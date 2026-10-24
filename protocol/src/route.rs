//! Routing path planning in millimetres.

use eh3d_drillpro_protocol::motion::{feed_xy, rapid_xy, z_move, Geometry};
use eh3d_drillpro_protocol::Step;

/// Parameters of one routing tool.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RouteParams {
    /// Total depth in mm.
    pub depth_mm: f64,
    /// Number of passes the depth is divided into (at least one).
    pub passes: u32,
    pub feed_xy_mm_min: f64,
    pub feed_z_mm_min: f64,
    /// Distance from the travel height (Z = 0 of the plan) down to the surface of the board. The plunge goes through it
    /// first, then `depth_mm` into the board. Zero when the plan's Z = 0 is the surface.
    pub approach_mm: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoutePlan {
    pub steps: Vec<Step>,
    /// XY position after the plan, in mm.
    pub end_mm: (f64, f64),
}

/// Plans one polyline: rapid to its start, then `passes` times plunge, cut, lift.
/// With several passes the tool returns to the start between passes; after the last pass it
/// stays at the last point of the path.
pub fn plan_polyline(
    from_mm: (f64, f64),
    points_mm: &[(f64, f64)],
    p: &RouteParams,
    g: &Geometry,
) -> RoutePlan {
    let Some(&start) = points_mm.first() else {
        return RoutePlan {
            steps: Vec::new(),
            end_mm: from_mm,
        };
    };
    let mut steps = rapid_xy(from_mm, start, g);
    let passes = p.passes.max(1);
    let mut z_now_steps = 0u32;
    let mut cur = start;
    for pass in 1..=passes {
        let target = g.z_steps(p.approach_mm + p.depth_mm * pass as f64 / passes as f64);
        let extra_mm = g.z_mm(target.saturating_sub(z_now_steps));
        steps.extend(z_move(extra_mm, true, p.feed_z_mm_min, g));
        z_now_steps = target;
        cur = start;
        for &pt in &points_mm[1..] {
            steps.extend(feed_xy(cur, pt, p.feed_xy_mm_min, g));
            cur = pt;
        }
        let up_feed = g.z_max_feed();
        steps.extend(z_move(g.z_mm(z_now_steps), false, up_feed, g));
        z_now_steps = 0;
        if pass < passes {
            steps.extend(rapid_xy(cur, start, g));
            cur = start;
        }
    }
    RoutePlan { steps, end_mm: cur }
}

/// Length of a polyline in mm.
pub fn length_mm(points_mm: &[(f64, f64)]) -> f64 {
    points_mm
        .windows(2)
        .map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt())
        .sum()
}
