use eh3d_routepro_protocol::motion::Geometry;
use eh3d_routepro_protocol::route::*;
use eh3d_routepro_protocol::safety::{Machine, MachineError, Verified};
use eh3d_routepro_protocol::{run_steps, Simulator};

fn params(passes: u32) -> RouteParams {
    RouteParams {
        depth_mm: 0.5,
        passes,
        feed_xy_mm_min: 1000.0,
        feed_z_mm_min: 500.0,
        approach_mm: 0.0,
    }
}

#[test]
fn a_plan_returns_the_tool_to_the_surface_and_reaches_the_end() {
    let g = Geometry::default();
    let plan = plan_polyline(
        (0.0, 0.0),
        &[(2.5, 2.5), (10.0, 2.5), (10.0, 7.5)],
        &params(1),
        &g,
    );
    let mut sim = Simulator::default();
    run_steps(&mut sim, &plan.steps).unwrap();
    assert_eq!(sim.z_steps, 0);
    assert_eq!(plan.end_mm, (10.0, 7.5));
    assert_eq!(
        (sim.x_steps, sim.y_steps),
        (g.xy_steps(10.0) as i64, g.xy_steps(7.5) as i64)
    );
}

#[test]
fn several_passes_cost_more_time_and_end_at_the_last_point() {
    let g = Geometry::default();
    let pts = [(5.0, 5.0), (15.0, 5.0)];
    let one = plan_polyline((0.0, 0.0), &pts, &params(1), &g);
    let three = plan_polyline((0.0, 0.0), &pts, &params(3), &g);
    assert_eq!(three.end_mm, (15.0, 5.0));
    assert!(three.steps.len() > one.steps.len());
    let mut sim = Simulator::default();
    run_steps(&mut sim, &three.steps).unwrap();
    assert_eq!(sim.z_steps, 0);
}

#[test]
fn empty_path_does_nothing() {
    let p = plan_polyline((1.0, 2.0), &[], &params(1), &Geometry::default());
    assert!(p.steps.is_empty());
    assert_eq!(p.end_mm, (1.0, 2.0));
}

#[test]
fn routing_goes_through_the_safety_gate() {
    let g = Geometry::default();
    let plan = plan_polyline((0.0, 0.0), &[(1.0, 1.0), (3.0, 1.0)], &params(1), &g);
    let mut m = Machine::new(Simulator::default());
    m.connect().unwrap();
    assert!(matches!(m.run(&plan.steps), Err(MachineError::NotArmed)));
    m.arm(Verified::all()).unwrap();
    m.run(&plan.steps).unwrap();
}

#[test]
fn polyline_length() {
    assert!((length_mm(&[(0.0, 0.0), (3.0, 4.0), (3.0, 10.0)]) - 11.0).abs() < 1e-9);
}
