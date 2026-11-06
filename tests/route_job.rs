use eh3d_drillpro::job::Geometry;
use eh3d_drillpro::protocol::execute::{run_items, Hooks, Outcome};
use eh3d_drillpro::protocol::job::{Item, JobMachine, Pause};
use eh3d_drillpro::protocol::safety::{Machine, Verified};
use eh3d_drillpro::protocol::{Pacing, Simulator};
use eh3d_routepro::hpgl::Polyline;
use eh3d_routepro::job::{
    default_tool, plan_path_levelled, plan_path_with, plan_route_job, plan_route_job_marked,
    tool_for,
};
use eh3d_routepro::tools::Tool;

struct Auto {
    pauses: Vec<Pause>,
}

impl Hooks for Auto {
    fn pause(&mut self, p: Pause) -> bool {
        self.pauses.push(p);
        true
    }
    fn progress(&mut self, _next: usize, _total: usize) {}
    fn cancelled(&mut self) -> bool {
        false
    }
}

fn machine() -> JobMachine {
    JobMachine {
        geometry: Geometry::with_correction(0.9992, 0.9998, 0.8988),
        offset_mm: (2.5, 2.5),
        max_depth_mm: 37.3,
        base_mm: 12.0,
        board_mm: 1.6,
        distance_tool_board_mm: 16.0,
        xy_search_steps: 26_613,
    }
}

fn tool(number: u8) -> Tool {
    Tool {
        number,
        depth_mm: 0.4,
        iterations: 2,
        feed_xy_mm_min: 1000,
        feed_z_mm_min: 500,
        spindle_rpm: 60_000,
    }
}

fn paths() -> Vec<Polyline> {
    vec![
        Polyline {
            pen: 1,
            points: vec![(0, 0), (400, 0), (400, 400)],
        },
        Polyline {
            pen: 1,
            points: vec![(800, 800), (1200, 800)],
        },
        Polyline {
            pen: 2,
            points: vec![(200, 200), (600, 200)],
        },
    ]
}

#[test]
fn the_job_has_the_dialogs_of_each_tool_and_ends_with_the_drivers_off() {
    let items = plan_route_job(&paths(), &[tool(1), tool(2)], &machine());
    let pauses: Vec<Pause> = items
        .iter()
        .filter_map(|i| {
            if let Item::Pause(p) = i {
                Some(*p)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(
        pauses,
        vec![
            Pause::ToolChange { tool: 1 },
            Pause::SpindleStart,
            Pause::ToolChange { tool: 2 }
        ]
    );
    // the last item switches the drivers off (0xC0 latch value)
    let Some(Item::Latch(ops)) = items.last() else {
        panic!("the job must end with a latch write")
    };
    assert!(format!("{ops:?}").contains("192"), "{ops:?}");
}

#[test]
fn the_job_runs_on_the_machine_and_the_head_comes_back_to_the_origin() {
    let items = plan_route_job(&paths(), &[tool(1), tool(2)], &machine());
    let mut m = Machine::new(Simulator::default());
    m.set_pacing(Pacing::Bytes);
    m.connect().unwrap();
    m.arm(Verified::all()).unwrap();
    let mut h = Auto { pauses: vec![] };
    assert_eq!(
        run_items(&mut m, &items, &mut h).unwrap(),
        Outcome::Completed
    );
    assert_eq!(h.pauses.len(), 3);
    assert!(m.transport().elapsed_us > 1_000_000);
}

#[test]
fn an_empty_plot_still_homes_and_finishes_without_touching_the_spindle() {
    let items = plan_route_job(&[], &[], &machine());
    assert!(!items.iter().any(|i| matches!(i, Item::Pause(_))));
    assert!(items.len() > 5);
}

#[test]
fn every_path_has_a_mark_on_the_item_that_cuts_it() {
    let (items, marks) = plan_route_job_marked(&paths(), &[tool(1), tool(2)], &machine(), None);
    assert_eq!(marks.len(), 3, "one mark for each path");
    assert!(
        marks.windows(2).all(|w| w[0] < w[1]),
        "in the order of the plan"
    );
    for &m in &marks {
        assert!(matches!(&items[m], Item::Steps(s) if s.len() > 100), "{m}");
    }
    // the plan is the same with or without the marks
    assert_eq!(
        items,
        plan_route_job(&paths(), &[tool(1), tool(2)], &machine())
    );
}

#[test]
fn a_path_with_no_points_is_done_at_once() {
    let mut p = paths();
    p.insert(
        1,
        Polyline {
            pen: 1,
            points: Vec::new(),
        },
    );
    let (_, marks) = plan_route_job_marked(&p, &[tool(1), tool(2)], &machine(), None);
    assert_eq!(marks.len(), 4, "the count of marks is the count of paths");
    assert!(marks.windows(2).all(|w| w[0] <= w[1]));
}

#[test]
fn the_pen_the_table_does_not_give_is_cut_with_the_one_cautious_tool() {
    assert_eq!(tool_for(&[tool(1)], 7), default_tool(7));
    assert_eq!(tool_for(&[tool(1)], 1), tool(1));
    let d = default_tool(3);
    assert_eq!(d.number, 3);
    assert_eq!((d.iterations, d.depth_mm), (1, 0.1));
}

#[test]
fn a_flat_table_cuts_the_same_with_the_levelled_and_the_plain_planner() {
    let path = &paths()[0];
    let g = machine().geometry;
    let plain = plan_path_with(path, &tool(1), (0, 0), &g, 1.0);
    let levelled = plan_path_levelled(path, &tool(1), (0, 0), &g, 1.0, &|_, _| 0.0);
    assert_eq!(plain, levelled);
}

fn rack(places: usize) -> eh3d_drillpro::protocol::job::Rack {
    eh3d_drillpro::protocol::job::Rack::new(
        (0..places)
            .map(|i| (540.0, 20.0 + 20.0 * i as f64, 30.0))
            .collect(),
    )
}

#[test]
fn on_a_machine_with_the_changer_the_operator_is_never_asked_for_a_tool() {
    use eh3d_routepro::job::plan_route_job_atc_marked;
    let (items, marks) = plan_route_job_atc_marked(
        &paths(),
        &[tool(1), tool(2)],
        &machine(),
        None,
        Some(&rack(4)),
    );
    let pauses: Vec<Pause> = items
        .iter()
        .filter_map(|i| match i {
            Item::Pause(p) => Some(*p),
            _ => None,
        })
        .collect();
    // only the start of the spindle is the operator's (once); the tools are taken from the rack
    assert_eq!(pauses, vec![Pause::SpindleStart]);
    assert_eq!(marks.len(), 3, "a mark for each path");
    // the exchanges (the auxiliary output opens and closes the chuck) make the plan longer than the same job changed by hand
    let (by_hand, _) = plan_route_job_marked(&paths(), &[tool(1), tool(2)], &machine(), None);
    assert!(items.len() > by_hand.len() + 8);
    // the head goes to the bar: it travels as far as place 2 (the last tool used) along X more than the paths need
    let far = items
        .iter()
        .filter(|i| matches!(i, Item::Steps(s) if s.len() > 4000))
        .count();
    assert!(far >= 4, "{far}");
}

#[test]
fn the_job_with_the_changer_runs_to_the_end_and_comes_back_to_the_origin() {
    use eh3d_routepro::job::plan_route_job_atc_marked;
    let (items, _) = plan_route_job_atc_marked(
        &paths(),
        &[tool(1), tool(2)],
        &machine(),
        None,
        Some(&rack(4)),
    );
    let mut m = Machine::new(Simulator::default());
    m.set_pacing(Pacing::Bytes);
    m.connect().unwrap();
    m.arm(Verified::all()).unwrap();
    let mut h = Auto { pauses: vec![] };
    assert_eq!(
        run_items(&mut m, &items, &mut h).unwrap(),
        Outcome::Completed
    );
    assert_eq!(h.pauses, vec![Pause::SpindleStart]);
}

#[test]
fn without_a_rack_the_plan_is_the_one_of_before() {
    use eh3d_routepro::job::plan_route_job_atc_marked;
    let a = plan_route_job_marked(&paths(), &[tool(1), tool(2)], &machine(), None);
    let b = plan_route_job_atc_marked(&paths(), &[tool(1), tool(2)], &machine(), None, None);
    assert_eq!(a, b);
}

#[test]
fn by_hand_the_router_stops_the_spindle_before_asking_for_the_next_cutter_and_starts_it_after() {
    for node in [false, true] {
        let mut m = machine();
        m.geometry.tuning.spindle_node = node;
        let mut two = paths();
        two.push(Polyline {
            pen: 2,
            points: vec![(0, 0), (300, 300)],
        });
        let items = plan_route_job(&two, &[tool(1), tool(2)], &m);
        let at: Vec<usize> = items
            .iter()
            .enumerate()
            .filter(|(_, i)| matches!(i, Item::Pause(Pause::ToolChange { .. })))
            .map(|(k, _)| k)
            .collect();
        assert_eq!(at.len(), 2, "node: {node}");
        let stops = |s: &[Item]| {
            if node {
                s.iter().any(|i| matches!(i, Item::Spindle { rpm: 0, .. }))
            } else {
                eh3d_drillpro::protocol::job::flatten_bytes(s).contains(&0xD2)
            }
        };
        let starts = |s: &[Item]| {
            if node {
                s.iter()
                    .any(|i| matches!(i, Item::Spindle { rpm, .. } if *rpm > 0))
            } else {
                eh3d_drillpro::protocol::job::flatten_bytes(s).contains(&0x90)
            }
        };
        assert!(stops(&items[at[0]..at[1]]), "node: {node}");
        assert!(starts(&items[at[1]..]), "node: {node}");
    }
}
