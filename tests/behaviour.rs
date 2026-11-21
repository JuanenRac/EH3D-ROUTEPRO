//! Tests of the behaviour of the routing job at the exact edge of each rule: the checks of the settings of a tool, the choice of the cautious tool, the order and the
//! number of the dialogs, the marks of the paths, the paths that are chosen and left out, and the texts of the eight languages. Everything is deterministic.

use eh3d_drillpro::i18n::{Lang, CORE_LANGS};
use eh3d_drillpro::job::{Geometry, JobOptions};
use eh3d_drillpro::protocol::execute::{run_items, Hooks, Outcome};
use eh3d_drillpro::protocol::job::{Item, JobMachine, Pause};
use eh3d_drillpro::protocol::safety::{Machine, Verified};
use eh3d_drillpro::protocol::{Pacing, Simulator};
use eh3d_routepro::hpgl::Polyline;
use eh3d_routepro::job::{
    check_tools, default_tool, plan_path, plan_path_with, plan_route_job, plan_route_job_marked,
    select_paths, tool_for,
};
use eh3d_routepro::texts::{tr, Text, ALL_TEXTS};
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

fn path(pen: u8, points: &[(i32, i32)]) -> Polyline {
    Polyline {
        pen,
        points: points.to_vec(),
    }
}

fn pauses_of(items: &[Item]) -> Vec<Pause> {
    items
        .iter()
        .filter_map(|i| match i {
            Item::Pause(p) => Some(*p),
            _ => None,
        })
        .collect()
}

const CORE: [Lang; CORE_LANGS] = [
    Lang::En,
    Lang::Es,
    Lang::It,
    Lang::De,
    Lang::Fr,
    Lang::Pt,
    Lang::Zh,
    Lang::Ja,
];

// ---------------------------------------------------------------- the cautious tool

#[test]
fn the_cautious_tool_has_fixed_cautious_values_and_the_number_of_the_pen() {
    let d = default_tool(9);
    assert_eq!(d.number, 9);
    assert_eq!((d.depth_mm, d.iterations), (0.1, 1));
    assert_eq!(
        (d.feed_xy_mm_min, d.feed_z_mm_min, d.spindle_rpm),
        (1000, 500, 60_000)
    );
}

#[test]
fn the_first_tool_of_the_table_with_the_number_of_the_pen_is_the_one_used() {
    let mut first = tool(3);
    first.depth_mm = 0.2;
    let mut second = tool(3);
    second.depth_mm = 0.9;
    assert_eq!(tool_for(&[tool(1), first.clone(), second], 3), first);
    assert_eq!(tool_for(&[], 0), default_tool(0));
    assert_eq!(tool_for(&[tool(1)], 255), default_tool(255));
}

// ---------------------------------------------------------------- the check of the tools

#[test]
fn no_paths_and_paths_without_points_never_refuse_a_job() {
    let mut bad = tool(1);
    bad.iterations = 0;
    assert_eq!(check_tools(&[], &[bad.clone()], &machine()), Ok(()));
    assert_eq!(check_tools(&[path(1, &[])], &[bad], &machine()), Ok(()));
}

#[test]
fn the_pen_that_is_named_is_the_first_bad_one_in_the_order_of_the_paths() {
    let mut bad2 = tool(2);
    bad2.iterations = 0;
    let mut bad3 = tool(3);
    bad3.iterations = 0;
    let paths = [
        path(1, &[(0, 0), (4, 0)]),
        path(3, &[(0, 0), (4, 0)]),
        path(2, &[(0, 0), (4, 0)]),
    ];
    assert_eq!(
        check_tools(&paths, &[tool(1), bad2, bad3], &machine()),
        Err(3)
    );
}

#[test]
fn the_number_of_passes_is_accepted_up_to_one_thousand() {
    let paths = [path(1, &[(0, 0), (4, 0)])];
    let mut t = tool(1);
    t.iterations = 1000;
    assert_eq!(check_tools(&paths, &[t.clone()], &machine()), Ok(()));
    t.iterations = 1001;
    assert_eq!(check_tools(&paths, &[t], &machine()), Err(1));
}

#[test]
fn the_depth_is_accepted_up_to_the_depth_of_the_machine_and_not_a_hair_more() {
    let paths = [path(1, &[(0, 0), (4, 0)])];
    let m = machine();
    let mut t = tool(1);
    t.depth_mm = m.max_depth_mm;
    assert_eq!(check_tools(&paths, &[t.clone()], &m), Ok(()));
    t.depth_mm = m.max_depth_mm + 0.01;
    assert_eq!(check_tools(&paths, &[t.clone()], &m), Err(1));
    t.depth_mm = 0.0;
    assert_eq!(check_tools(&paths, &[t.clone()], &m), Ok(()));
    t.depth_mm = -0.01;
    assert_eq!(check_tools(&paths, &[t], &m), Err(1));
}

#[test]
fn a_feed_of_zero_in_either_direction_and_a_negative_approach_refuse_the_job() {
    let paths = [path(2, &[(0, 0), (4, 0)])];
    let mut t = tool(2);
    t.feed_z_mm_min = 0;
    assert_eq!(check_tools(&paths, &[t], &machine()), Err(2));
    let mut m = machine();
    m.distance_tool_board_mm = -1.0;
    assert_eq!(check_tools(&paths, &[tool(2)], &m), Err(2));
    m.distance_tool_board_mm = 0.0;
    assert_eq!(check_tools(&paths, &[tool(2)], &m), Ok(()));
}

// ---------------------------------------------------------------- the plan of one path

#[test]
fn a_path_without_points_plans_no_steps_and_stays_where_the_head_is() {
    let g = machine().geometry;
    let (steps, end) = plan_path(&path(1, &[]), &tool(1), (37, 91), &g);
    assert!(steps.is_empty());
    assert_eq!(end, (37, 91));
}

#[test]
fn the_head_ends_on_the_last_point_whatever_the_number_of_passes() {
    let g = machine().geometry;
    let line = path(1, &[(0, 0), (400, 0), (400, 400)]);
    for passes in [1u32, 2, 3, 5] {
        let mut t = tool(1);
        t.iterations = passes;
        let (steps, end) = plan_path_with(&line, &t, (0, 0), &g, 1.0);
        assert_eq!(end, (400, 400), "{passes}");
        assert!(!steps.is_empty());
    }
}

#[test]
fn more_passes_plan_more_steps_and_the_same_plan_comes_out_every_time() {
    let g = machine().geometry;
    let line = path(1, &[(0, 0), (400, 0)]);
    let mut one = tool(1);
    one.iterations = 1;
    let mut three = tool(1);
    three.iterations = 3;
    let a = plan_path_with(&line, &one, (0, 0), &g, 0.5);
    let b = plan_path_with(&line, &three, (0, 0), &g, 0.5);
    assert!(b.0.len() > a.0.len());
    assert_eq!(b, plan_path_with(&line, &three, (0, 0), &g, 0.5));
}

#[test]
fn a_path_of_one_point_ends_on_that_point() {
    let g = machine().geometry;
    let (_, end) = plan_path(&path(1, &[(200, 120)]), &tool(1), (0, 0), &g);
    assert_eq!(end, (200, 120));
}

// ---------------------------------------------------------------- the job

#[test]
fn the_same_paths_always_plan_the_same_job() {
    let paths = [
        path(1, &[(0, 0), (400, 0)]),
        path(2, &[(40, 40), (400, 400)]),
    ];
    let a = plan_route_job(&paths, &[tool(1), tool(2)], &machine());
    let b = plan_route_job(&paths, &[tool(1), tool(2)], &machine());
    assert_eq!(a, b);
}

#[test]
fn paths_of_one_pen_ask_for_the_tool_once_and_for_the_spindle_once() {
    let paths = [
        path(1, &[(0, 0), (400, 0)]),
        path(1, &[(0, 40), (400, 40)]),
        path(1, &[(0, 80), (400, 80)]),
    ];
    let items = plan_route_job(&paths, &[tool(1)], &machine());
    assert_eq!(
        pauses_of(&items),
        vec![Pause::ToolChange { tool: 1 }, Pause::SpindleStart]
    );
}

#[test]
fn a_pen_that_comes_back_asks_for_its_tool_again_but_the_spindle_only_once() {
    let paths = [
        path(1, &[(0, 0), (400, 0)]),
        path(2, &[(0, 40), (400, 40)]),
        path(1, &[(0, 80), (400, 80)]),
    ];
    let items = plan_route_job(&paths, &[tool(1), tool(2)], &machine());
    assert_eq!(
        pauses_of(&items),
        vec![
            Pause::ToolChange { tool: 1 },
            Pause::SpindleStart,
            Pause::ToolChange { tool: 2 },
            Pause::ToolChange { tool: 1 }
        ]
    );
}

#[test]
fn paths_without_points_ask_for_nothing_and_each_one_still_has_its_mark() {
    for count in 0..6usize {
        let paths: Vec<Polyline> = (0..count).map(|i| path(i as u8, &[])).collect();
        let (items, marks) = plan_route_job_marked(&paths, &[], &machine(), None);
        assert!(pauses_of(&items).is_empty(), "{count}");
        assert_eq!(marks.len(), count, "{count}");
    }
}

#[test]
fn the_marks_point_at_items_in_order_and_inside_the_plan() {
    let paths = [
        path(1, &[(0, 0), (400, 0)]),
        path(1, &[]),
        path(2, &[(40, 40), (400, 400)]),
    ];
    let (items, marks) = plan_route_job_marked(&paths, &[tool(1), tool(2)], &machine(), None);
    assert_eq!(marks.len(), 3);
    assert!(marks.windows(2).all(|w| w[0] <= w[1]));
    assert!(marks.iter().all(|m| *m <= items.len()));
}

#[test]
fn the_pen_without_a_tool_in_the_table_is_cut_with_the_cautious_one_and_the_job_still_runs() {
    let paths = [path(7, &[(0, 0), (400, 0), (400, 200)])];
    let items = plan_route_job(&paths, &[], &machine());
    assert_eq!(
        pauses_of(&items),
        vec![Pause::ToolChange { tool: 7 }, Pause::SpindleStart]
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
    assert_eq!(h.pauses.len(), 2);
}

#[test]
fn small_paths_made_with_a_fixed_seed_all_run_to_the_end_on_the_simulated_machine() {
    let mut seed = 20_241_121u64;
    let mut next = |bound: u64| {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (seed >> 33) % bound
    };
    for _ in 0..3 {
        let mut paths = Vec::new();
        for _ in 0..3 {
            let pen = 1 + next(2) as u8;
            let points: Vec<(i32, i32)> = (0..3)
                .map(|k| (200 * k + next(100) as i32, next(600) as i32))
                .collect();
            paths.push(Polyline { pen, points });
        }
        let items = plan_route_job(&paths, &[tool(1), tool(2)], &machine());
        let mut m = Machine::new(Simulator::default());
        m.set_pacing(Pacing::Bytes);
        m.connect().unwrap();
        m.arm(Verified::all()).unwrap();
        let mut h = Auto { pauses: vec![] };
        assert_eq!(
            run_items(&mut m, &items, &mut h).unwrap(),
            Outcome::Completed
        );
    }
}

// ---------------------------------------------------------------- the paths that are chosen

fn plot() -> Vec<Polyline> {
    vec![
        path(1, &[(0, 0), (400, 0)]),
        path(2, &[(0, 40), (400, 40)]),
        path(1, &[(0, 80), (400, 80)]),
        path(1, &[(0, 120), (400, 120)]),
    ]
}

#[test]
fn the_paths_are_grouped_by_pen_and_keep_their_own_order_inside_a_pen() {
    let (out, skipped) = select_paths(&plot(), &JobOptions::default(), (0.0, 0.0));
    assert_eq!(skipped, 0);
    let pens: Vec<u8> = out.iter().map(|p| p.pen).collect();
    assert_eq!(pens, vec![1, 1, 1, 2]);
    let ys: Vec<i32> = out.iter().map(|p| p.points[0].1).collect();
    assert_eq!(ys, vec![0, 80, 120, 40]);
}

#[test]
fn the_tools_that_are_switched_off_leave_their_paths_out() {
    let o = JobOptions {
        skip_tools: vec![1],
        ..Default::default()
    };
    let (out, _) = select_paths(&plot(), &o, (0.0, 0.0));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].pen, 2);
    let all_off = JobOptions {
        skip_tools: vec![1, 2],
        ..Default::default()
    };
    assert!(select_paths(&plot(), &all_off, (0.0, 0.0)).0.is_empty());
}

#[test]
fn the_paths_already_done_are_left_out_from_the_front_and_never_more_than_there_are() {
    let one = JobOptions {
        skip: 1,
        ..Default::default()
    };
    let (rest, _) = select_paths(&plot(), &one, (0.0, 0.0));
    assert_eq!(rest.len(), 3);
    assert_eq!(rest[0].points[0].1, 80);
    let many = JobOptions {
        skip: 100,
        ..Default::default()
    };
    assert!(select_paths(&plot(), &many, (0.0, 0.0)).0.is_empty());
}

#[test]
fn an_empty_plot_selects_nothing_and_skips_nothing() {
    let (out, skipped) = select_paths(&[], &JobOptions::default(), (1.0, 2.0));
    assert!(out.is_empty());
    assert_eq!(skipped, 0);
}

// ---------------------------------------------------------------- the texts

#[test]
fn the_eight_core_languages_are_the_first_eight_in_the_fixed_order() {
    for (index, lang) in CORE.iter().enumerate() {
        assert_eq!(*lang as usize, index);
    }
    assert_eq!(CORE_LANGS, 8);
}

#[test]
fn a_few_texts_are_exactly_what_the_person_reads_in_each_language() {
    for (lang, done) in [
        (Lang::En, "Done"),
        (Lang::Es, "Terminado"),
        (Lang::It, "Fatto"),
        (Lang::De, "Fertig"),
        (Lang::Fr, "Terminé"),
        (Lang::Pt, "Concluído"),
        (Lang::Zh, "完成"),
        (Lang::Ja, "完了"),
    ] {
        assert_eq!(tr(lang, Text::Done), done);
    }
    assert_eq!(tr(Lang::Pt, Text::BtnYes), "Sim");
    assert_eq!(tr(Lang::Fr, Text::BtnNo), "Non");
}

#[test]
fn the_name_of_a_text_in_the_files_of_the_languages_is_the_table_and_the_name() {
    assert_eq!(Text::Done.locale_key(), "Text.Done");
    assert_eq!(Text::MenuLoad.locale_key(), "Text.MenuLoad");
    let mut keys: Vec<&str> = ALL_TEXTS.iter().map(|t| t.locale_key()).collect();
    let total = keys.len();
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), total);
}

#[test]
fn a_text_that_has_a_place_for_a_number_has_one_in_every_language() {
    for text in [Text::WaitToolChange, Text::RealMachine] {
        for lang in CORE {
            assert_eq!(tr(lang, text).matches("{}").count(), 1, "{text:?} {lang:?}");
        }
    }
    for &text in ALL_TEXTS {
        if matches!(text, Text::WaitToolChange | Text::RealMachine) {
            continue;
        }
        for lang in CORE {
            assert!(!tr(lang, text).contains("{}"), "{text:?} {lang:?}");
        }
    }
}

#[test]
fn every_menu_text_has_exactly_one_hot_key_between_marks_and_no_two_menus_share_it() {
    let menus = [
        Text::MenuLoad,
        Text::MenuPaths,
        Text::MenuTools,
        Text::MenuConfig,
        Text::MenuStart,
        Text::MenuLanguage,
        Text::MenuQuit,
    ];
    for lang in CORE {
        let mut keys: Vec<String> = Vec::new();
        for menu in menus {
            let text = tr(lang, menu);
            assert_eq!(text.matches('~').count(), 2, "{menu:?} {lang:?}");
            let inside = text.split('~').nth(1).unwrap();
            assert_eq!(inside.chars().count(), 1, "{menu:?} {lang:?}");
            keys.push(inside.to_lowercase());
        }
        let total = keys.len();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), total, "{lang:?}");
    }
}

#[test]
fn no_text_has_blanks_at_its_ends_or_a_line_break() {
    for &text in ALL_TEXTS {
        for lang in CORE {
            let t = tr(lang, text);
            assert_eq!(t, t.trim(), "{text:?} {lang:?}");
            assert!(!t.contains('\n') && !t.contains('\r'), "{text:?} {lang:?}");
        }
    }
}

#[test]
fn the_texts_of_the_other_languages_differ_from_the_english_ones_for_most_sentences() {
    let mut same = 0usize;
    let mut total = 0usize;
    for &text in ALL_TEXTS {
        for lang in CORE.iter().skip(1) {
            total += 1;
            if tr(*lang, text) == tr(Lang::En, text) {
                same += 1;
            }
        }
    }
    assert!(same * 10 < total, "{same} of {total}");
}
