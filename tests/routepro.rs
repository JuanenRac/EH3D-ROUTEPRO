use eh3d_drillpro::app::Input;
use eh3d_drillpro::i18n::{cell_width, Lang};
use eh3d_drillpro::job::Geometry;
use eh3d_drillpro::protocol::{run_steps, Simulator};
use eh3d_drillpro::screen::Screen;
use eh3d_routepro::app::App;
use eh3d_routepro::hpgl::{self, UNIT_MM};
use eh3d_routepro::job::plan_path;
use eh3d_routepro::texts::{tr, Text, ALL_TEXTS};
use eh3d_routepro::tools;
use std::path::PathBuf;

fn data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata")
}

fn read(name: &str) -> String {
    std::fs::read(data().join(name))
        .unwrap()
        .iter()
        .map(|&c| c as char)
        .collect()
}

fn render(app: &App) -> String {
    let mut s = Screen::default();
    app.render(&mut s);
    s.to_text()
}

#[test]
fn measure_plot_is_a_300_by_200_mm_rectangle() {
    let p = hpgl::parse(&read("MEASURE.PLT"));
    let b = p.bounds_mm().unwrap();
    assert!((b.2 - b.0 - 300.0).abs() < 1.0, "{b:?}");
    assert!((b.3 - b.1 - 200.0).abs() < 1.0, "{b:?}");
}

#[test]
fn pen_up_moves_split_polylines() {
    let p = hpgl::parse("IN;SP1;PU0,0;PD10,0,10,10;PU50,50;PD60,50;");
    assert_eq!(p.paths.len(), 2);
    assert_eq!(p.paths[0].points, vec![(0, 0), (10, 0), (10, 10)]);
    assert_eq!(p.paths[1].points, vec![(50, 50), (60, 50)]);
}

#[test]
fn absolute_and_relative_moves() {
    let p = hpgl::parse("PU0,0;PD;PA10,10;PR5,5;");
    assert_eq!(p.paths[0].points, vec![(0, 0), (10, 10), (15, 15)]);
}

#[test]
fn pens_select_the_tool() {
    let p = hpgl::parse("SP1;PU0,0;PD10,10;SP2;PU0,0;PD5,5;");
    assert_eq!(p.pens(), vec![1, 2]);
    assert_eq!(p.paths[1].pen, 2);
}

#[test]
fn unknown_commands_are_counted_not_fatal() {
    let p = hpgl::parse("IN;LT2;PU0,0;PD1,1;");
    assert_eq!(p.ignored, 1);
    assert_eq!(p.paths.len(), 1);
}

#[test]
fn every_real_plt_file_parses() {
    // `EH3D_LOCAL_DATA` may name a folder with more plot files; the sample files are always checked.
    let mut dirs = vec![data()];
    dirs.extend(std::env::var_os("EH3D_LOCAL_DATA").map(PathBuf::from));
    let mut files = vec![];
    for dir in dirs {
        if let Ok(rd) = std::fs::read_dir(dir) {
            files.extend(
                rd.flatten()
                    .map(|e| e.path())
                    .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("plt"))),
            );
        }
    }
    assert!(!files.is_empty());
    for f in files {
        let text: String = std::fs::read(&f)
            .unwrap()
            .iter()
            .map(|&c| c as char)
            .collect();
        let p = hpgl::parse(&text);
        assert!(
            p.paths.iter().all(|x| x.points.len() >= 2),
            "{}",
            f.display()
        );
    }
}

#[test]
fn tool_table_parses_fifteen_tools() {
    let t = tools::parse(&read("ROUTEPRO.RTD"));
    assert_eq!(t.len(), 15);
    assert_eq!((t[3].number, t[3].feed_xy_mm_min), (4, 500));
}

#[test]
fn planned_path_returns_to_the_surface() {
    let p = hpgl::parse("SP1;PU100,100;PD400,100,400,300;");
    let tool = tools::Tool {
        number: 1,
        depth_mm: 0.5,
        iterations: 2,
        feed_xy_mm_min: 1000,
        feed_z_mm_min: 500,
        spindle_rpm: 60000,
    };
    let (steps, end) = plan_path(&p.paths[0], &tool, (0, 0), &Geometry::default());
    let mut sim = Simulator::default();
    run_steps(&mut sim, &steps).unwrap();
    assert_eq!(sim.z_steps, 0, "Z must end where it started");
    assert!(steps.len() > 100);
    assert_eq!(
        end,
        (400, 300),
        "after the last pass the tool is at the last point of the path"
    );
    let _ = UNIT_MM;
}

#[test]
fn texts_exist_in_all_languages_and_cjk_is_wide() {
    for &k in ALL_TEXTS {
        let en = tr(Lang::En, k).contains('~');
        for l in Lang::ALL {
            let t = tr(l, k);
            assert!(!t.trim().is_empty());
            assert_eq!(t.contains('~'), en, "{k:?} {l:?}");
        }
        for l in [Lang::Zh, Lang::Ja] {
            assert!(tr(l, k).chars().any(|c| cell_width(c) == 2), "{k:?} {l:?}");
        }
    }
}

#[test]
fn menu_hotkeys_unique_per_language() {
    use Text::*;
    for l in Lang::ALL {
        let mut seen: Vec<String> = Vec::new();
        for k in [
            MenuLoad,
            MenuPaths,
            MenuTools,
            MenuConfig,
            MenuStart,
            MenuLanguage,
            MenuQuit,
        ] {
            let hot = tr(l, k).split('~').nth(1).unwrap().to_ascii_lowercase();
            assert!(!seen.contains(&hot), "{l:?}: {hot}");
            seen.push(hot);
        }
    }
}

#[test]
fn app_loads_a_plot_shows_paths_and_runs_to_the_end() {
    let mut app = App::new(Lang::En, &data());
    assert!(render(&app).contains("No file loaded"));
    app.load_file(&data().join("MEASURE.PLT"));
    app.handle(Input::Alt('p'));
    let t = render(&app);
    assert!(t.contains("Paths") && t.contains("Cut length"), "{t}");
    app.handle(Input::Esc);
    app.handle(Input::Alt('s'));
    let mut guard = 0;
    while app.tick_n(5) {
        guard += 1;
        assert!(guard < 100);
    }
    assert!(render(&app).contains("Done"));
}

#[test]
fn every_language_renders_every_screen() {
    for lang in Lang::ALL {
        let mut app = App::new(lang, &data());
        app.load_file(&data().join("FDEMO1.PLT"));
        for hot in ['l', 'p', 't', 'c', 's', 'g', 'q'] {
            app.handle(Input::Alt(hot));
            assert_eq!(render(&app).lines().count(), 25);
            app.handle(Input::Esc);
        }
    }
}

#[test]
fn language_menu_switches_to_japanese() {
    let mut app = App::new(Lang::En, &data());
    app.handle(Input::Alt('g'));
    for _ in 0..7 {
        app.handle(Input::Down);
    }
    app.handle(Input::Enter);
    assert_eq!(app.lang, Lang::Ja);
    assert!(render(&app).lines().next().unwrap().contains("読込"));
}
