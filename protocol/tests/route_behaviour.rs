//! The rules of the settings of a routing path at the exact edge of each limit, the length of a path, and the plans of the edge cases (no points, one point, a piece
//! longer than the path, a table that is flat), all in millimetres and without a machine.

use eh3d_routepro_protocol::bits::{Z_NEG, Z_POS};
use eh3d_routepro_protocol::motion::Geometry;
use eh3d_routepro_protocol::route::{
    length_mm, plan_polyline, plan_polyline_levelled, ParamsError, RouteParams, MAX_PASSES,
};
use eh3d_routepro_protocol::Step;

fn good() -> RouteParams {
    RouteParams {
        depth_mm: 0.5,
        passes: 2,
        feed_xy_mm_min: 1000.0,
        feed_z_mm_min: 500.0,
        approach_mm: 1.0,
    }
}

/// The steps down that the plan makes at most, and where Z ends, counted in steps from where it started.
fn z_reach(steps: &[Step]) -> (i64, i64) {
    let (mut z, mut deepest) = (0i64, 0i64);
    for s in steps {
        if s.byte == Z_POS {
            z += 1;
        } else if s.byte == Z_NEG {
            z -= 1;
        }
        deepest = deepest.max(z);
    }
    (deepest, z)
}

#[test]
fn good_settings_are_accepted() {
    assert_eq!(good().validate(37.3), Ok(()));
}

#[test]
fn the_number_of_passes_is_between_one_and_the_maximum() {
    let mut p = good();
    p.passes = 0;
    assert_eq!(p.validate(37.3), Err(ParamsError::Passes));
    p.passes = 1;
    assert_eq!(p.validate(37.3), Ok(()));
    p.passes = MAX_PASSES;
    assert_eq!(p.validate(37.3), Ok(()));
    p.passes = MAX_PASSES + 1;
    assert_eq!(p.validate(37.3), Err(ParamsError::Passes));
    assert_eq!(MAX_PASSES, 1000);
}

#[test]
fn the_depth_is_between_zero_and_the_depth_of_the_machine_both_included() {
    let mut p = good();
    p.depth_mm = 0.0;
    assert_eq!(p.validate(10.0), Ok(()));
    p.depth_mm = 10.0;
    assert_eq!(p.validate(10.0), Ok(()));
    p.depth_mm = 10.000001;
    assert_eq!(p.validate(10.0), Err(ParamsError::Depth));
    p.depth_mm = -0.000001;
    assert_eq!(p.validate(10.0), Err(ParamsError::Depth));
}

#[test]
fn a_depth_that_is_not_a_number_or_is_infinite_is_refused() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut p = good();
        p.depth_mm = bad;
        assert_eq!(p.validate(37.3), Err(ParamsError::Depth), "{bad}");
    }
}

#[test]
fn each_feed_must_be_more_than_zero_and_finite() {
    for bad in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let mut xy = good();
        xy.feed_xy_mm_min = bad;
        assert_eq!(xy.validate(37.3), Err(ParamsError::Feed), "xy {bad}");
        let mut z = good();
        z.feed_z_mm_min = bad;
        assert_eq!(z.validate(37.3), Err(ParamsError::Feed), "z {bad}");
    }
    let mut tiny = good();
    tiny.feed_xy_mm_min = f64::MIN_POSITIVE;
    assert_eq!(tiny.validate(37.3), Ok(()));
}

#[test]
fn the_approach_must_be_zero_or_more_and_finite() {
    let mut p = good();
    p.approach_mm = 0.0;
    assert_eq!(p.validate(37.3), Ok(()));
    for bad in [-0.001, f64::NAN, f64::INFINITY] {
        p.approach_mm = bad;
        assert_eq!(p.validate(37.3), Err(ParamsError::Approach), "{bad}");
    }
}

#[test]
fn the_first_rule_that_fails_is_the_one_that_is_told() {
    let p = RouteParams {
        depth_mm: f64::NAN,
        passes: 0,
        feed_xy_mm_min: 0.0,
        feed_z_mm_min: 0.0,
        approach_mm: -1.0,
    };
    assert_eq!(p.validate(1.0), Err(ParamsError::Passes));
    let q = RouteParams { passes: 1, ..p };
    assert_eq!(q.validate(1.0), Err(ParamsError::Depth));
    let r = RouteParams { depth_mm: 0.1, ..q };
    assert_eq!(r.validate(1.0), Err(ParamsError::Feed));
    let s = RouteParams {
        feed_xy_mm_min: 1.0,
        feed_z_mm_min: 1.0,
        ..r
    };
    assert_eq!(s.validate(1.0), Err(ParamsError::Approach));
}

#[test]
fn the_length_of_a_path_is_the_sum_of_its_segments() {
    assert_eq!(length_mm(&[]), 0.0);
    assert_eq!(length_mm(&[(3.0, 4.0)]), 0.0);
    assert_eq!(length_mm(&[(0.0, 0.0), (3.0, 4.0)]), 5.0);
    assert_eq!(length_mm(&[(0.0, 0.0), (3.0, 4.0), (3.0, 10.0)]), 11.0);
    assert_eq!(length_mm(&[(1.0, 1.0), (1.0, 1.0)]), 0.0);
}

#[test]
fn a_path_has_the_same_length_walked_backwards_and_a_closed_square_has_four_sides() {
    let path = [(0.0, 0.0), (10.0, 0.0), (10.0, 7.0), (2.0, 7.0)];
    let mut back = path;
    back.reverse();
    assert!((length_mm(&path) - length_mm(&back)).abs() < 1e-12);
    let square = [(0.0, 0.0), (5.0, 0.0), (5.0, 5.0), (0.0, 5.0), (0.0, 0.0)];
    assert_eq!(length_mm(&square), 20.0);
}

#[test]
fn a_plan_without_points_has_no_steps_and_stays_where_it_is() {
    let g = Geometry::default();
    let plan = plan_polyline((12.5, 3.25), &[], &good(), &g);
    assert!(plan.steps.is_empty());
    assert_eq!(plan.end_mm, (12.5, 3.25));
    let levelled = plan_polyline_levelled((12.5, 3.25), &[], &good(), &g, &|_, _| 0.0, 10.0);
    assert!(levelled.steps.is_empty());
    assert_eq!(levelled.end_mm, (12.5, 3.25));
}

#[test]
fn the_plan_ends_on_the_last_point_with_the_head_back_up_for_any_number_of_passes() {
    let g = Geometry::default();
    let points = [(5.0, 5.0), (15.0, 5.0), (15.0, 12.0)];
    for passes in [1u32, 2, 3, 7] {
        let p = RouteParams { passes, ..good() };
        let plan = plan_polyline((0.0, 0.0), &points, &p, &g);
        assert_eq!(plan.end_mm, (15.0, 12.0), "{passes}");
        let (deepest, end) = z_reach(&plan.steps);
        assert_eq!(end, 0, "{passes}");
        // the last pass goes down to the approach plus the whole depth (a step more or less for the rounding of each pass)
        assert!(
            (deepest - g.z_steps(1.0 + 0.5) as i64).abs() <= 1,
            "{passes}"
        );
    }
}

#[test]
fn a_path_of_one_point_goes_there_and_ends_there() {
    let g = Geometry::default();
    let plan = plan_polyline((0.0, 0.0), &[(8.0, 6.0)], &good(), &g);
    assert_eq!(plan.end_mm, (8.0, 6.0));
}

#[test]
fn the_same_inputs_give_the_same_plan() {
    let g = Geometry::default();
    let points = [(5.0, 5.0), (25.0, 9.0)];
    let a = plan_polyline((1.0, 2.0), &points, &good(), &g);
    assert_eq!(a, plan_polyline((1.0, 2.0), &points, &good(), &g));
    let flat = |_: f64, _: f64| 0.0;
    let b = plan_polyline_levelled((1.0, 2.0), &points, &good(), &g, &flat, 10.0);
    assert_eq!(
        b,
        plan_polyline_levelled((1.0, 2.0), &points, &good(), &g, &flat, 10.0)
    );
}

#[test]
fn more_passes_make_a_longer_plan() {
    let g = Geometry::default();
    let points = [(5.0, 5.0), (25.0, 5.0)];
    let one = plan_polyline(
        (0.0, 0.0),
        &points,
        &RouteParams {
            passes: 1,
            ..good()
        },
        &g,
    );
    let four = plan_polyline(
        (0.0, 0.0),
        &points,
        &RouteParams {
            passes: 4,
            ..good()
        },
        &g,
    );
    assert!(four.steps.len() > one.steps.len());
}

#[test]
fn a_piece_longer_than_the_path_or_no_piece_at_all_is_the_same_plan() {
    let g = Geometry::default();
    let points = [(5.0, 5.0), (8.0, 5.0)];
    let flat = |_: f64, _: f64| 0.0;
    let long = plan_polyline_levelled((0.0, 0.0), &points, &good(), &g, &flat, 10.0);
    let none = plan_polyline_levelled((0.0, 0.0), &points, &good(), &g, &flat, 0.0);
    let negative = plan_polyline_levelled((0.0, 0.0), &points, &good(), &g, &flat, -5.0);
    assert_eq!(long, none);
    assert_eq!(long, negative);
}

#[test]
fn a_flat_table_ends_where_the_plain_plan_ends_and_goes_as_deep() {
    let g = Geometry::default();
    let points = [(5.0, 5.0), (30.0, 5.0), (30.0, 20.0)];
    let plain = plan_polyline((0.0, 0.0), &points, &good(), &g);
    let flat = |_: f64, _: f64| 0.0;
    let levelled = plan_polyline_levelled((0.0, 0.0), &points, &good(), &g, &flat, 5.0);
    assert_eq!(plain.end_mm, levelled.end_mm);
    assert_eq!(z_reach(&plain.steps).0, z_reach(&levelled.steps).0);
    assert_eq!(z_reach(&levelled.steps).1, 0);
}

#[test]
fn a_table_that_is_higher_makes_the_cut_less_deep_and_the_head_still_comes_back_up() {
    let g = Geometry::default();
    let points = [(5.0, 5.0), (30.0, 5.0)];
    let flat = |_: f64, _: f64| 0.0;
    let high = |_: f64, _: f64| 0.4;
    let low = plan_polyline_levelled((0.0, 0.0), &points, &good(), &g, &flat, 10.0);
    let raised = plan_polyline_levelled((0.0, 0.0), &points, &good(), &g, &high, 10.0);
    assert!(z_reach(&raised.steps).0 < z_reach(&low.steps).0);
    assert_eq!(z_reach(&raised.steps).1, 0);
    assert_eq!(raised.end_mm, low.end_mm);
}
