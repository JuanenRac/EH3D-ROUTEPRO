//! Properties of the planning of a routing path on many paths made by a generator with a fixed seed: the same cases are tried every time.

use eh3d_routepro_protocol::bits::{Z_NEG, Z_POS};
use eh3d_routepro_protocol::motion::Geometry;
use eh3d_routepro_protocol::route::*;

/// A small generator with a fixed seed (the same numbers every run, on every system).
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }

    /// A number from `low` to `high`, with a step of a hundredth.
    fn between(&mut self, low: f64, high: f64) -> f64 {
        let hundredths = ((high - low) * 100.0) as u64 + 1;
        low + (self.next() % hundredths) as f64 / 100.0
    }
}

fn path(rng: &mut Lcg) -> Vec<(f64, f64)> {
    let points = 1 + (rng.next() % 6) as usize;
    (0..points)
        .map(|_| (rng.between(0.0, 120.0), rng.between(0.0, 90.0)))
        .collect()
}

fn params(rng: &mut Lcg) -> RouteParams {
    RouteParams {
        depth_mm: rng.between(0.1, 2.5),
        passes: 1 + (rng.next() % 5) as u32,
        feed_xy_mm_min: rng.between(100.0, 1500.0),
        feed_z_mm_min: rng.between(50.0, 400.0),
        approach_mm: rng.between(0.0, 3.0),
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
fn whatever_the_path_the_tool_never_rises_above_where_it_began_and_ends_there() {
    let g = Geometry::default();
    let mut rng = Lcg(20_240_517);
    for case in 0..300 {
        let (points, p) = (path(&mut rng), params(&mut rng));
        let plan = plan_polyline((0.0, 0.0), &points, &p, &g);
        let (deepest, highest, end) = z_range(&plan.steps);
        assert_eq!((highest, end), (0, 0), "case {case}: {points:?} {p:?}");
        assert_eq!(
            deepest,
            g.z_steps(p.approach_mm + p.depth_mm) as i64,
            "case {case}: {points:?} {p:?}"
        );
        assert_eq!(plan.end_mm, *points.last().unwrap(), "case {case}");
    }
}

#[test]
fn on_a_flat_table_the_levelled_plan_keeps_the_depth_and_the_end_whatever_the_size_of_the_pieces() {
    let g = Geometry::default();
    let mut rng = Lcg(77);
    for case in 0..200 {
        let (points, p) = (path(&mut rng), params(&mut rng));
        let piece = [0.0, 0.5, 3.0, 25.0, 1000.0][(rng.next() % 5) as usize];
        let plain = plan_polyline((0.0, 0.0), &points, &p, &g);
        let levelled = plan_polyline_levelled((0.0, 0.0), &points, &p, &g, &|_, _| 0.0, piece);
        assert_eq!(levelled.end_mm, plain.end_mm, "case {case}, piece {piece}");
        assert_eq!(
            z_range(&levelled.steps),
            z_range(&plain.steps),
            "case {case}, piece {piece}: {points:?} {p:?}"
        );
    }
}

#[test]
fn more_passes_never_make_a_plan_shorter() {
    let g = Geometry::default();
    let mut rng = Lcg(5);
    for case in 0..150 {
        let (points, mut p) = (path(&mut rng), params(&mut rng));
        p.passes = 1;
        let one = plan_polyline((0.0, 0.0), &points, &p, &g).steps.len();
        p.passes = 1 + (rng.next() % 4) as u32 + 1;
        let several = plan_polyline((0.0, 0.0), &points, &p, &g).steps.len();
        assert!(
            several >= one,
            "case {case}: {one} steps in one pass, {several} in {}",
            p.passes
        );
    }
}

#[test]
fn a_piece_far_smaller_than_the_path_is_bounded_and_does_not_hang() {
    let g = Geometry::default();
    let p = RouteParams {
        depth_mm: 0.5,
        passes: 1,
        feed_xy_mm_min: 1000.0,
        feed_z_mm_min: 300.0,
        approach_mm: 0.0,
    };
    let points = [(0.0, 0.0), (30.0, 0.0)];
    let plan = plan_polyline_levelled((0.0, 0.0), &points, &p, &g, &|_, _| 0.0, 1e-12);
    assert_eq!(plan.end_mm, (30.0, 0.0));
    assert_eq!(z_range(&plan.steps).2, 0);
    // a piece that is not a number or is negative cuts the segment whole
    for piece in [f64::NAN, -1.0, 0.0] {
        let whole = plan_polyline_levelled((0.0, 0.0), &points, &p, &g, &|_, _| 0.0, piece);
        assert_eq!(whole.end_mm, (30.0, 0.0), "{piece}");
    }
}
