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

/// Like [`plan_polyline`] on a table that is not flat: `lift(x, y)` is how much higher the table is at that place (mm, negative where it is lower) than where the Z
/// was set. The cut is divided into pieces no longer than `piece_mm` and the depth follows the table from one piece to the next, so the tool keeps the same depth into
/// the board all along the path.
pub fn plan_polyline_levelled(
    from_mm: (f64, f64),
    points_mm: &[(f64, f64)],
    p: &RouteParams,
    g: &Geometry,
    lift: &dyn Fn(f64, f64) -> f64,
    piece_mm: f64,
) -> RoutePlan {
    let Some(&start) = points_mm.first() else {
        return RoutePlan {
            steps: Vec::new(),
            end_mm: from_mm,
        };
    };
    let mut steps = rapid_xy(from_mm, start, g);
    let passes = p.passes.max(1);
    let mut z_now = 0u32;
    let mut cur = start;
    // the Z that is asked, in steps from the travel height, where the table is `lift` higher
    let want = |pass: u32, at: (f64, f64)| -> u32 {
        let mm = p.approach_mm + p.depth_mm * pass as f64 / passes as f64 - lift(at.0, at.1);
        g.z_steps(mm)
    };
    let go_to = |steps: &mut Vec<Step>, z_now: &mut u32, target: u32, feed: f64| {
        if target > *z_now {
            steps.extend(z_move(g.z_mm(target - *z_now), true, feed, g));
        } else if target < *z_now {
            steps.extend(z_move(g.z_mm(*z_now - target), false, feed, g));
        }
        *z_now = target;
    };
    for pass in 1..=passes {
        go_to(&mut steps, &mut z_now, want(pass, start), p.feed_z_mm_min);
        cur = start;
        for &pt in &points_mm[1..] {
            let len = (pt.0 - cur.0).hypot(pt.1 - cur.1);
            let pieces = if piece_mm > 0.0 {
                (len / piece_mm).ceil().max(1.0) as u32
            } else {
                1
            };
            for k in 1..=pieces {
                let t = k as f64 / pieces as f64;
                let next = (cur.0 + (pt.0 - cur.0) * t, cur.1 + (pt.1 - cur.1) * t);
                let before = if k == 1 {
                    cur
                } else {
                    (
                        cur.0 + (pt.0 - cur.0) * (k - 1) as f64 / pieces as f64,
                        cur.1 + (pt.1 - cur.1) * (k - 1) as f64 / pieces as f64,
                    )
                };
                steps.extend(feed_xy(before, next, p.feed_xy_mm_min, g));
                go_to(&mut steps, &mut z_now, want(pass, next), p.feed_z_mm_min);
            }
            cur = pt;
        }
        let up_feed = g.z_max_feed();
        go_to(&mut steps, &mut z_now, 0, up_feed);
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

#[cfg(test)]
mod levelled_tests {
    use super::*;
    use eh3d_drillpro_protocol::motion::Geometry;

    fn g() -> Geometry {
        Geometry::default()
    }

    fn z_down_steps(steps: &[Step]) -> usize {
        steps
            .iter()
            .filter(|s| s.byte == eh3d_drillpro_protocol::bits::Z_POS)
            .count()
    }

    #[test]
    fn the_cut_follows_a_table_that_is_higher_at_one_end() {
        let p = RouteParams {
            depth_mm: 0.2,
            passes: 1,
            feed_xy_mm_min: 1000.0,
            feed_z_mm_min: 300.0,
            approach_mm: 1.0,
        };
        let pts = [(0.0, 0.0), (100.0, 0.0)];
        let flat = plan_polyline_levelled((0.0, 0.0), &pts, &p, &g(), &|_, _| 0.0, 10.0);
        let same = plan_polyline((0.0, 0.0), &pts, &p, &g());
        assert_eq!(flat.steps, same.steps);
        // the table is 0.1 mm higher at the far end: the tool is lifted by that, so it goes down less in all
        let tilted = plan_polyline_levelled((0.0, 0.0), &pts, &p, &g(), &|x, _| x / 1000.0, 10.0);
        let diff = z_down_steps(&flat.steps) as i64 - z_down_steps(&tilted.steps) as i64;
        assert!(diff >= 0);
        // and it ends at the same place
        assert_eq!(tilted.end_mm, flat.end_mm);
    }
}
