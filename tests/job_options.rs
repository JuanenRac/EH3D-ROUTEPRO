//! What the main form and the Panelization dialog add to a routing job.

use eh3d_drillpro::job::{JobOptions, Panel};
use eh3d_routepro::hpgl::Polyline;
use eh3d_routepro::job::select_paths;

fn path(pen: u8, points: &[(i32, i32)]) -> Polyline {
    Polyline {
        pen,
        points: points.to_vec(),
    }
}

fn plot() -> Vec<Polyline> {
    vec![
        path(1, &[(0, 0), (400, 0)]),
        path(2, &[(0, 40), (400, 40)]),
        path(1, &[(0, 80), (400, 80)]),
        path(1, &[(0, 120), (400, 120)]),
    ]
}

#[test]
fn the_vectors_asked_for_are_the_only_ones_routed() {
    let (all, skipped) = select_paths(&plot(), &JobOptions::default(), (0.0, 0.0));
    assert_eq!((all.len(), skipped), (4, 0));
    let o = JobOptions {
        from: 2,
        to: 3,
        ..Default::default()
    };
    let (some, _) = select_paths(&plot(), &o, (0.0, 0.0));
    assert_eq!(some.len(), 2);
    assert_eq!(some[0].pen, 2);
}

#[test]
fn a_path_that_leaves_the_working_area_is_left_out_whole() {
    // 400 plotter units are 10 mm; a clip of 12 mm in X lets them in, but an offset of 3 mm pushes them out
    let o = JobOptions {
        clip_mm: Some((12.0, 20.0)),
        ..Default::default()
    };
    let (inside, none) = select_paths(&plot(), &o, (0.0, 0.0));
    assert_eq!((inside.len(), none), (4, 0));
    let (outside, skipped) = select_paths(&plot(), &o, (3.0, 0.0));
    assert_eq!((outside.len(), skipped), (0, 4));
}

#[test]
fn a_panel_repeats_each_pen_over_all_the_copies() {
    let o = JobOptions {
        panel: Some(Panel {
            columns: 2,
            rows: 2,
            pitch_mm: (20.0, 10.0),
            tabs: None,
        }),
        ..Default::default()
    };
    let (all, _) = select_paths(&plot(), &o, (0.0, 0.0));
    assert_eq!(all.len(), 16);
    // pen 1 (three paths) in the four copies first, then pen 2: one pen change
    assert!(all[..12].iter().all(|p| p.pen == 1));
    assert!(all[12..].iter().all(|p| p.pen == 2));
    // the second copy of the first path is 20 mm (800 units) to the right
    assert_eq!(all[3].points[0], (800, 0));
    // the third copy is 10 mm (400 units) up
    assert_eq!(all[6].points[0], (0, 400));
}

#[test]
fn the_outside_of_a_panel_is_cut_at_its_tabs() {
    use eh3d_drillpro::job::{TabPlacement, Tabs};
    let outline = Polyline {
        pen: 1,
        points: vec![(0, 0), (2000, 0), (2000, 1000), (0, 1000), (0, 0)],
    };
    let tabs = Tabs {
        per_side: 1,
        width_mm: 4.0,
        placement: TabPlacement::Middle,
    };
    let o = JobOptions {
        panel: Some(Panel {
            columns: 2,
            rows: 1,
            pitch_mm: (60.0, 0.0),
            tabs: Some(tabs),
        }),
        ..Default::default()
    };
    // a board is 50 mm by 25 mm: one tab on each of its four sides leaves four pieces, in each of the two copies
    let (cut, _) = select_paths(&[outline.clone()], &o, (0.0, 0.0));
    assert_eq!(cut.len(), 8);
    // without tabs the outside stays whole
    let plain = JobOptions {
        panel: Some(Panel {
            tabs: None,
            ..o.panel.unwrap()
        }),
        ..Default::default()
    };
    let (whole, _) = select_paths(&[outline], &plain, (0.0, 0.0));
    assert_eq!(whole.len(), 2);
}
