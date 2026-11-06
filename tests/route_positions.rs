//! Where the head goes in a routing job on a machine with the tool changer: after every exchange the head is at the bar, and the next path has to be gone to from
//! there. Checked on the planned steps, followed from the switches as the control unit would.

use eh3d_drillpro::job::Geometry;
use eh3d_drillpro::progress::Progress;
use eh3d_drillpro::protocol::job::{Item, JobMachine, Rack};
use eh3d_routepro::hpgl::Polyline;
use eh3d_routepro::job::plan_route_job_atc_marked;
use eh3d_routepro::tools::Tool;

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

/// Three paths of two pens, in plotter units of 1/40 mm: they end at (500, 500), (1300, 900) and (700, 300) once the offset of 2.5 mm (100 units) is added.
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

/// The test pin and six places 20 mm apart along Y at X 540 mm, far outside the work area: place `i` is at Y = 20 + 20 i mm.
fn rack() -> Rack {
    Rack::new(
        (0..7)
            .map(|i| (540.0, 20.0 + 20.0 * i as f64, 30.0))
            .collect(),
    )
}

fn plan(rack: &Rack) -> (Vec<Item>, Vec<usize>) {
    plan_route_job_atc_marked(&paths(), &[tool(1), tool(2)], &machine(), None, Some(rack))
}

/// Where the head is (X, Y in mm from the switches) after each path of the plan, in the order of the paths.
fn ends(items: &[Item], marks: &[usize]) -> Vec<(f64, f64)> {
    let progress = Progress::new(items, marks).with_track(items, &machine().geometry);
    marks
        .iter()
        .map(|&m| {
            let h = progress.head_mm(m + 1, 0.0).expect("the plan is followed");
            (h.0, h.1)
        })
        .collect()
}

#[test]
fn after_a_change_of_tool_the_next_path_is_cut_where_the_design_is_and_not_at_the_bar() {
    let (items, marks) = plan(&rack());
    assert_eq!(marks.len(), 3);
    // the end of each path: its last point plus the offset of the machine, 1/40 mm each unit
    let want = [(12.5, 12.5), (32.5, 22.5), (17.5, 7.5)];
    let got = ends(&items, &marks);
    for (k, (g, w)) in got.iter().zip(want).enumerate() {
        assert!(
            (g.0 - w.0).abs() < 0.2 && (g.1 - w.1).abs() < 0.2,
            "path {k}: the head ends at {g:?}, the path ends at {w:?}"
        );
    }
}

#[test]
fn the_head_never_cuts_outside_the_work_area_whatever_the_places_of_the_tools() {
    for slots in [
        vec![],
        vec![(1u8, 6usize), (2, 3)],
        vec![(1u8, 2usize), (2, 1)],
    ] {
        let given = rack().with_slots(slots.clone(), None);
        let (items, marks) = plan(&given);
        // every end of a path is inside a table of 300 x 200 mm
        for (k, h) in ends(&items, &marks).into_iter().enumerate() {
            assert!(
                (0.0..=300.0).contains(&h.0) && (0.0..=200.0).contains(&h.1),
                "{slots:?}: path {k} ends at {h:?}"
            );
        }
    }
}

/// The position in steps (X, Y) at every latch byte that opens (0xA0) or closes (0x80) the chuck at the bar, in the order they are sent.
fn chuck_events(items: &[Item], g: &Geometry) -> Vec<(u8, i32, i32)> {
    let bytes = eh3d_drillpro::protocol::job::flatten_bytes(items);
    let (mut x, mut y) = (0i32, 0i32);
    let mut out = Vec::new();
    for b in bytes {
        if b & 0x80 != 0 {
            if (b == 0xA0 || b == 0x80) && x > g.x_steps(500.0) {
                out.push((b, x, y));
            }
            continue;
        }
        let over = b & 0x40 != 0;
        for (step, dir, pos) in [(0x01u8, 0x02u8, &mut x), (0x04, 0x08, &mut y)] {
            if b & step == 0 {
                continue;
            }
            if b & dir != 0 {
                *pos += 1;
            } else if *pos > 0 || over {
                *pos -= 1;
            }
        }
    }
    out
}

/// For each opening ('o') and closing ('c') of the chuck, the place of the rack it happens at.
fn places(rack: &Rack, items: &[Item]) -> Vec<(char, usize)> {
    let g = machine().geometry;
    chuck_events(items, &g)
        .into_iter()
        .map(|(byte, x, y)| {
            let (i, d) = rack
                .places
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let dx = (x - g.x_steps(p.0)) as f64 / g.x_steps(1.0) as f64;
                    let dy = (y - g.y_steps(p.1)) as f64 / g.y_steps(1.0) as f64;
                    (i, dx.hypot(dy))
                })
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap();
            assert!(d < 0.2, "the chuck works {d} mm from the place {i}");
            (if byte == 0xA0 { 'o' } else { 'c' }, i)
        })
        .collect()
}

#[test]
fn without_places_the_cutters_are_taken_from_the_places_in_the_order_of_the_job() {
    let r = rack();
    let (items, _) = plan(&r);
    assert_eq!(
        places(&r, &items),
        vec![('o', 0), ('c', 1), ('o', 1), ('c', 2), ('o', 2), ('c', 0)]
    );
}

#[test]
fn a_cutter_is_taken_from_the_place_it_was_given_and_put_back_in_that_place() {
    let r = rack().with_slots(vec![(1, 5), (2, 3)], None);
    let (items, marks) = plan(&r);
    assert_eq!(
        places(&r, &items),
        vec![('o', 0), ('c', 5), ('o', 5), ('c', 3), ('o', 3), ('c', 0)]
    );
    // and it cuts where the design is all the same
    let got = ends(&items, &marks);
    assert!((got[2].0 - 17.5).abs() < 0.2 && (got[2].1 - 7.5).abs() < 0.2);
}

#[test]
fn a_routing_job_that_follows_a_drilling_job_takes_the_chuck_from_the_place_of_the_last_drill() {
    // the drilling left its last drill (in place 4) in the chuck: the first exchange puts it back there and takes cutter 1 from place 2, cutter 2 from place 6
    let r = rack()
        .in_load(3, true, false)
        .with_slots(vec![(1, 2), (2, 6)], Some(4));
    let (items, marks) = plan(&r);
    assert_eq!(
        places(&r, &items),
        vec![('o', 4), ('c', 2), ('o', 2), ('c', 6), ('o', 6), ('c', 0)]
    );
    let got = ends(&items, &marks);
    assert!((got[0].0 - 12.5).abs() < 0.2 && (got[0].1 - 12.5).abs() < 0.2);
}
