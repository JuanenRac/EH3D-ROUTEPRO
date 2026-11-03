//! More cases of the planning of a routing path: the approach through the surface, a number of passes of zero, one point, a table that is not flat, and the length of a path.

use eh3d_routepro_protocol::bits::{Z_NEG, Z_POS};
use eh3d_routepro_protocol::motion::Geometry;
use eh3d_routepro_protocol::route::*;
use eh3d_routepro_protocol::{run_steps, Simulator};

fn params(depth: f64, passes: u32, approach: f64) -> RouteParams {
    RouteParams {
        depth_mm: depth,
        passes,
        feed_xy_mm_min: 1000.0,
        feed_z_mm_min: 500.0,
        approach_mm: approach,
    }
}

/// The deepest and the highest the Z gets, in steps down from where it started, and where it ends.
fn z_range(steps: &[eh3d_routepro_protocol::Step]) -> (i64, i64, i64) {
    let (mut z, mut deepest, mut highest) = (0i64, 0i64, 0i64);
    for s in steps {
        if s.byte == Z_POS {
            z += 1;
        } else if s.byte == Z_NEG {
            z -= 1;
        }
        deepest = deepest.max(z);
        highest = highest.min(z);
    }
    (deepest, highest, z)
}

#[test]
fn the_plunge_goes_through_the_approach_first_and_then_the_depth_into_the_board() {
    let g = Geometry::default();
    let plan = plan_polyline(
        (0.0, 0.0),
        &[(5.0, 5.0), (15.0, 5.0)],
        &params(0.5, 1, 3.0),
        &g,
    );
    let (deepest, highest, end) = z_range(&plan.steps);
    assert_eq!(deepest, g.z_steps(3.5) as i64);
    assert_eq!((highest, end), (0, 0));
}

#[test]
fn a_number_of_passes_of_zero_is_one_pass() {
    let g = Geometry::default();
    let pts = [(5.0, 5.0), (15.0, 5.0)];
    let zero = plan_polyline((0.0, 0.0), &pts, &params(0.5, 0, 0.0), &g);
    let one = plan_polyline((0.0, 0.0), &pts, &params(0.5, 1, 0.0), &g);
    assert_eq!(zero, one);
}

#[test]
fn every_pass_goes_a_part_of_the_depth_deeper_and_the_last_one_is_the_whole_depth() {
    let g = Geometry::default();
    for passes in [2u32, 4, 5] {
        let plan = plan_polyline(
            (0.0, 0.0),
            &[(5.0, 5.0), (15.0, 5.0)],
            &params(1.0, passes, 0.0),
            &g,
        );
        let (deepest, highest, end) = z_range(&plan.steps);
        assert_eq!(deepest, g.z_steps(1.0) as i64, "{passes}");
        assert_eq!((highest, end), (0, 0));
    }
}

#[test]
fn a_path_of_one_point_goes_there_and_goes_down_and_up_without_cutting() {
    let g = Geometry::default();
    let plan = plan_polyline((0.0, 0.0), &[(4.0, 6.0)], &params(0.5, 1, 0.0), &g);
    assert_eq!(plan.end_mm, (4.0, 6.0));
    let mut sim = Simulator::default();
    run_steps(&mut sim, &plan.steps).unwrap();
    assert_eq!(
        (sim.x_steps, sim.y_steps, sim.z_steps),
        (g.xy_steps(4.0) as i64, g.xy_steps(6.0) as i64, 0)
    );
    assert!(z_range(&plan.steps).0 > 0);
}

#[test]
fn a_path_that_starts_where_the_tool_is_does_not_move_in_xy_before_going_down() {
    let g = Geometry::default();
    let from_start = plan_polyline(
        (5.0, 5.0),
        &[(5.0, 5.0), (9.0, 5.0)],
        &params(0.5, 1, 0.0),
        &g,
    );
    let elsewhere = plan_polyline(
        (0.0, 0.0),
        &[(5.0, 5.0), (9.0, 5.0)],
        &params(0.5, 1, 0.0),
        &g,
    );
    assert!(from_start.steps.len() < elsewhere.steps.len());
}

#[test]
fn the_plan_is_the_same_every_time() {
    let g = Geometry::default();
    let pts = [(1.0, 1.0), (7.5, 2.0), (3.0, 9.0), (1.0, 1.0)];
    let a = plan_polyline((0.0, 0.0), &pts, &params(0.6, 3, 0.2), &g);
    let b = plan_polyline((0.0, 0.0), &pts, &params(0.6, 3, 0.2), &g);
    assert_eq!(a, b);
}

#[test]
fn a_closed_path_ends_where_it_began_and_the_simulator_agrees() {
    let g = Geometry::default();
    let pts = [(2.0, 2.0), (12.0, 2.0), (12.0, 8.0), (2.0, 8.0), (2.0, 2.0)];
    let plan = plan_polyline((0.0, 0.0), &pts, &params(0.5, 2, 0.0), &g);
    assert_eq!(plan.end_mm, (2.0, 2.0));
    let mut sim = Simulator::default();
    run_steps(&mut sim, &plan.steps).unwrap();
    assert_eq!(
        (sim.x_steps, sim.y_steps),
        (g.xy_steps(2.0) as i64, g.xy_steps(2.0) as i64)
    );
}

#[test]
fn on_a_flat_table_the_levelled_plan_cuts_to_the_same_depth_and_ends_in_the_same_place() {
    let g = Geometry::default();
    let pts = [(5.0, 5.0), (15.0, 5.0), (15.0, 12.0)];
    let p = params(0.5, 2, 0.0);
    let plain = plan_polyline((0.0, 0.0), &pts, &p, &g);
    let levelled = plan_polyline_levelled((0.0, 0.0), &pts, &p, &g, &|_, _| 0.0, 2.0);
    assert_eq!(levelled.end_mm, plain.end_mm);
    assert_eq!(z_range(&levelled.steps).0, z_range(&plain.steps).0);
    assert_eq!(z_range(&levelled.steps).2, 0);
}

#[test]
fn a_table_that_is_higher_makes_the_cut_less_deep_and_one_that_is_lower_makes_it_deeper() {
    let g = Geometry::default();
    let pts = [(5.0, 5.0), (15.0, 5.0)];
    let p = params(0.5, 1, 0.0);
    let flat = z_range(&plan_polyline_levelled((0.0, 0.0), &pts, &p, &g, &|_, _| 0.0, 1.0).steps).0;
    let high = z_range(&plan_polyline_levelled((0.0, 0.0), &pts, &p, &g, &|_, _| 0.1, 1.0).steps).0;
    let low = z_range(&plan_polyline_levelled((0.0, 0.0), &pts, &p, &g, &|_, _| -0.1, 1.0).steps).0;
    assert!(high < flat && flat < low, "{high} {flat} {low}");
}

#[test]
fn a_table_much_higher_than_the_depth_never_lifts_the_tool_over_the_travel_height() {
    let g = Geometry::default();
    let p = params(0.5, 2, 0.0);
    let plan = plan_polyline_levelled(
        (0.0, 0.0),
        &[(5.0, 5.0), (9.0, 5.0)],
        &p,
        &g,
        &|_, _| 5.0,
        1.0,
    );
    let (_, highest, end) = z_range(&plan.steps);
    assert_eq!(highest, 0);
    assert_eq!(end, 0);
}

#[test]
fn a_tilted_table_makes_the_depth_follow_it_along_the_path() {
    let g = Geometry::default();
    // the table rises 0.2 mm over 20 mm along x: the Z at the end of the path is shallower than at its start
    let lift = |x: f64, _y: f64| 0.01 * x;
    let plan = plan_polyline_levelled(
        (0.0, 0.0),
        &[(0.0, 5.0), (20.0, 5.0)],
        &params(0.5, 1, 0.0),
        &g,
        &lift,
        1.0,
    );
    let mut z = 0i64;
    let mut zs = Vec::new();
    for s in &plan.steps {
        if s.byte == Z_POS {
            z += 1;
        } else if s.byte == Z_NEG {
            z -= 1;
        }
        zs.push(z);
    }
    let deepest = zs.iter().copied().max().unwrap();
    // the end of the path is cut shallower than the start: the Z at the last cutting move is above the deepest point
    let last_move_z = zs.iter().rev().copied().find(|v| *v > 0).unwrap();
    assert!(last_move_z <= deepest);
    // the deepest point is at the start of the path, where the table is lowest
    assert_eq!(deepest, g.z_steps(0.5) as i64);
    assert_eq!(*zs.last().unwrap(), 0);
}

#[test]
fn a_piece_length_of_zero_or_less_means_one_piece_per_segment_and_a_short_one_gives_more_steps() {
    let g = Geometry::default();
    let pts = [(5.0, 5.0), (25.0, 5.0)];
    let p = params(0.5, 1, 0.0);
    let one_piece = plan_polyline_levelled((0.0, 0.0), &pts, &p, &g, &|x, _| 0.005 * x, 0.0);
    let negative = plan_polyline_levelled((0.0, 0.0), &pts, &p, &g, &|x, _| 0.005 * x, -3.0);
    let fine = plan_polyline_levelled((0.0, 0.0), &pts, &p, &g, &|x, _| 0.005 * x, 0.5);
    assert_eq!(one_piece, negative);
    assert!(fine.steps.len() >= one_piece.steps.len());
    for plan in [&one_piece, &fine] {
        assert_eq!(plan.end_mm, (25.0, 5.0));
        assert_eq!(z_range(&plan.steps).2, 0);
    }
}

#[test]
fn the_levelled_plan_of_an_empty_path_does_nothing() {
    let plan = plan_polyline_levelled(
        (3.0, 4.0),
        &[],
        &params(0.5, 1, 0.0),
        &Geometry::default(),
        &|_, _| 0.0,
        1.0,
    );
    assert!(plan.steps.is_empty());
    assert_eq!(plan.end_mm, (3.0, 4.0));
}

#[test]
fn the_length_of_a_path_is_zero_without_two_points_and_does_not_depend_on_the_direction() {
    assert_eq!(length_mm(&[]), 0.0);
    assert_eq!(length_mm(&[(1.0, 1.0)]), 0.0);
    let path = [(0.0, 0.0), (3.0, 4.0), (6.0, 0.0)];
    let mut back = path;
    back.reverse();
    assert!((length_mm(&path) - 10.0).abs() < 1e-12);
    assert!((length_mm(&path) - length_mm(&back)).abs() < 1e-12);
    assert_eq!(length_mm(&[(2.0, 2.0), (2.0, 2.0)]), 0.0);
}
