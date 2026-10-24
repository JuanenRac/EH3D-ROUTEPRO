//! Routing job: converts HP-GL polylines into the byte stream sent to the machine.
//! The planning itself lives in `eh3d-routepro-protocol`; this module adapts HP-GL data to it.

use eh3d_drillpro::job::Geometry;
use eh3d_drillpro::protocol::job::{Item, JobBuilder, JobMachine, Pause};
use eh3d_drillpro::protocol::Step;
use eh3d_routepro_protocol::route::{plan_polyline, RouteParams};

use crate::hpgl::{Polyline, UNIT_MM};
use crate::tools::Tool;

/// Plans one polyline. `from` is the current XY position in plotter units;
/// returns the steps and the final position, also in plotter units.
pub fn plan_path(
    path: &Polyline,
    tool: &Tool,
    from: (i32, i32),
    g: &Geometry,
) -> (Vec<Step>, (i32, i32)) {
    plan_path_with(path, tool, from, g, 0.0)
}

/// Like [`plan_path`], with the distance from the travel height (where the head moves between paths) down to the surface of the
/// board: the plunge goes through it first, then the tool's depth into the board.
pub fn plan_path_with(
    path: &Polyline,
    tool: &Tool,
    from: (i32, i32),
    g: &Geometry,
    approach_mm: f64,
) -> (Vec<Step>, (i32, i32)) {
    let mm = |p: (i32, i32)| (p.0 as f64 * UNIT_MM, p.1 as f64 * UNIT_MM);
    let points: Vec<(f64, f64)> = path.points.iter().map(|&p| mm(p)).collect();
    let params = RouteParams {
        depth_mm: tool.depth_mm,
        passes: tool.iterations,
        feed_xy_mm_min: tool.feed_xy_mm_min as f64,
        feed_z_mm_min: tool.feed_z_mm_min as f64,
        approach_mm,
    };
    let plan = plan_polyline(mm(from), &points, &params, g);
    let end = (
        (plan.end_mm.0 / UNIT_MM).round() as i32,
        (plan.end_mm.1 / UNIT_MM).round() as i32,
    );
    (plan.steps, end)
}

/// The tool to use for a pen number: the one in the table, or the cautious default.
pub fn tool_for(tools: &[Tool], pen: u8) -> Tool {
    tools
        .iter()
        .find(|t| t.number == pen)
        .cloned()
        .unwrap_or(Tool {
            number: pen,
            depth_mm: 0.1,
            iterations: 1,
            feed_xy_mm_min: 1000,
            feed_z_mm_min: 500,
            spindle_rpm: 60000,
        })
}

/// Plans a whole routing job for a real machine: the reference runs, then for each tool in the order the paths use it the
/// operator's dialog, the spindle, the paths (rapid at the travel height, plunge through the free stroke and into the board,
/// cut, lift) and the spindle off, then back to the origin. The coordinates of the plot are shifted by the configuration's offset.
pub fn plan_route_job(paths: &[Polyline], tools: &[Tool], m: &JobMachine) -> Vec<Item> {
    let g = m.geometry;
    let mut b = JobBuilder::new(m);
    // Z of the plan is the travel height: the free stroke above the surface of the board.
    let travel_mm =
        (m.max_depth_mm + 0.5 - m.base_mm - m.board_mm - m.distance_tool_board_mm).max(0.0);
    let travel_steps = g.z_steps(travel_mm);
    let shift = (
        (m.offset_mm.0 / UNIT_MM).round() as i32,
        (m.offset_mm.1 / UNIT_MM).round() as i32,
    );
    let mut at = (0i32, 0i32);
    let mut current: Option<u8> = None;
    let mut first_tool = true;
    for path in paths {
        if path.points.is_empty() {
            continue;
        }
        let tool = tool_for(tools, path.pen);
        if current != Some(path.pen) {
            if current.is_some() {
                b.z_to(0);
                b.spindle_off();
            }
            b.pause(Pause::ToolChange { tool: path.pen });
            if first_tool {
                b.pause(Pause::SpindleStart);
                first_tool = false;
            }
            b.spindle_on(tool.spindle_rpm);
            b.z_to(travel_steps);
            current = Some(path.pen);
        }
        let moved = Polyline {
            pen: path.pen,
            points: path
                .points
                .iter()
                .map(|&(x, y)| (x + shift.0, y + shift.1))
                .collect(),
        };
        let (steps, end) = plan_path_with(&moved, &tool, at, &g, m.distance_tool_board_mm);
        at = end;
        let end_steps = (
            g.x_steps(end.0 as f64 * UNIT_MM),
            g.y_steps(end.1 as f64 * UNIT_MM),
        );
        b.steps(steps, end_steps);
    }
    if current.is_some() {
        b.z_to(0);
        b.spindle_off();
    }
    b.finish()
}
