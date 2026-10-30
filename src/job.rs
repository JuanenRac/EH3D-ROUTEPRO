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

/// The paths of a job: the vectors the form asks for, copied over the panel and clipped to the working area. Pens are done one
/// after the other (a tool change is the operator's), every copy of the paths of a pen in turn. Returns the paths and how many
/// were left out because they end outside the working area.
pub fn select_paths(
    paths: &[Polyline],
    opts: &eh3d_drillpro::job::JobOptions,
    offset_mm: (f64, f64),
) -> (Vec<Polyline>, usize) {
    use crate::hpgl::UNIT_MM;
    let first = opts.from.max(1);
    let last = if opts.to == 0 { usize::MAX } else { opts.to };
    let kept: Vec<&Polyline> = paths
        .iter()
        .enumerate()
        .filter(|(i, _)| (first..=last).contains(&(i + 1)))
        .map(|(_, p)| p)
        .collect();
    let mut pens: Vec<u8> = Vec::new();
    for p in &kept {
        if !pens.contains(&p.pen) {
            pens.push(p.pen);
        }
    }
    let copies: Vec<(f64, f64)> = match opts.panel {
        Some(p) if p.columns * p.rows > 1 => (0..p.rows)
            .flat_map(|iy| {
                (0..p.columns).map(move |ix| (ix as f64 * p.pitch_mm.0, iy as f64 * p.pitch_mm.1))
            })
            .collect(),
        _ => vec![(0.0, 0.0)],
    };
    let mut out = Vec::new();
    let mut skipped = 0;
    for pen in pens {
        for &(dx, dy) in &copies {
            let (ux, uy) = ((dx / UNIT_MM).round() as i32, (dy / UNIT_MM).round() as i32);
            for p in kept.iter().filter(|p| p.pen == pen) {
                let moved = Polyline {
                    pen: p.pen,
                    points: p.points.iter().map(|&(x, y)| (x + ux, y + uy)).collect(),
                };
                let inside = opts.clip_mm.is_none_or(|(cx, cy)| {
                    moved.points.iter().all(|&(x, y)| {
                        let (mx, my) = (
                            f64::from(x) * UNIT_MM + offset_mm.0,
                            f64::from(y) * UNIT_MM + offset_mm.1,
                        );
                        mx >= 0.0 && my >= 0.0 && mx <= cx && my <= cy
                    })
                });
                if inside {
                    match opts.panel.and_then(|p| p.tabs) {
                        Some(tabs) => out.extend(with_tabs(moved, &tabs)),
                        None => out.push(moved),
                    }
                } else {
                    skipped += 1;
                }
            }
        }
    }
    out.drain(..opts.skip.min(out.len()));
    (out, skipped)
}

/// A path with the tabs of the panel cut into it: a closed path (the outside of a board) becomes the pieces between the tabs, so the router lifts at each tab
/// and the board stays joined to the panel there; any other path is left whole.
fn with_tabs(path: Polyline, tabs: &eh3d_drillpro::job::Tabs) -> Vec<Polyline> {
    use crate::hpgl::UNIT_MM;
    let mm: Vec<(f64, f64)> = path
        .points
        .iter()
        .map(|&(x, y)| (f64::from(x) * UNIT_MM, f64::from(y) * UNIT_MM))
        .collect();
    let pieces = eh3d_drillpro::job::cut_tabs(&mm, tabs);
    if pieces.len() < 2 {
        return vec![path];
    }
    pieces
        .into_iter()
        .map(|piece| Polyline {
            pen: path.pen,
            points: piece
                .into_iter()
                .map(|(x, y)| ((x / UNIT_MM).round() as i32, (y / UNIT_MM).round() as i32))
                .collect(),
        })
        .collect()
}
