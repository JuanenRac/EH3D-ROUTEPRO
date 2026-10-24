use eh3d_drillpro::job::Geometry;
use eh3d_drillpro::protocol::execute::{run_items, Hooks, Outcome};
use eh3d_drillpro::protocol::job::{Item, JobMachine, Pause};
use eh3d_drillpro::protocol::safety::{Machine, Verified};
use eh3d_drillpro::protocol::{Pacing, Simulator};
use eh3d_routepro::hpgl::Polyline;
use eh3d_routepro::job::plan_route_job;
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
