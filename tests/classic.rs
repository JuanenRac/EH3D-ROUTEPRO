//! The classic front end of ROUTEPRO: its own tool table, the chord angle and the keyboard flow.

use std::path::{Path, PathBuf};

use eh3d_drillpro::app::Input;
use eh3d_drillpro::classic::ClassicApp;
use eh3d_drillpro::i18n::Lang;
use eh3d_drillpro::screen::Screen;
use eh3d_drillpro::textapp::TextApp;
use eh3d_routepro::app::App;

fn testdata() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata")
}

fn app(lang: Lang) -> ClassicApp<App> {
    ClassicApp::with_engine(App::new(lang, &testdata()))
}

fn press(app: &mut ClassicApp<App>, keys: &str) {
    for c in keys.chars() {
        app.handle(Input::Char(c));
    }
}

fn text(app: &ClassicApp<App>) -> String {
    let mut s = Screen::default();
    app.render(&mut s);
    s.to_text()
}

#[test]
fn the_main_screen_names_the_program() {
    let t = text(&app(Lang::En));
    assert!(t.contains("RoutePro"), "{t}");
    assert!(t.contains("--------.---"));
}

#[test]
fn loading_a_plot_counts_its_paths_and_shows_its_size() {
    let mut a = app(Lang::En);
    a.load_file(&testdata().join("FRAME.PLT"));
    let t = text(&a);
    assert!(t.contains("00006"), "six paths:\n{t}");
    assert!(t.contains("FRAME.PLT"));
    assert!(t.contains("020.00") && t.contains("030.00"), "{t}");
}

#[test]
fn the_parameter_dialog_shows_the_chord_angle_instead_of_the_sorting() {
    let mut a = app(Lang::En);
    press(&mut a, "c");
    let t = text(&a);
    assert!(t.contains("Chord angle") && t.contains("+05"), "{t}");
    assert!(!t.contains("X first"), "no sorting in the routing program");
    assert!(t.contains("*.PLT"));
}

#[test]
fn the_tool_table_has_the_routing_columns() {
    let mut a = app(Lang::En);
    press(&mut a, "t");
    let t = text(&a);
    for want in ["Depth", "Iter.", "X Feed", "Z Feed", "Spindle"] {
        assert!(t.contains(want), "{want} missing:\n{t}");
    }
}

#[test]
fn panelization_counts_the_copies_that_fit() {
    let mut a = app(Lang::En);
    a.load_file(&testdata().join("FRAME.PLT"));
    press(&mut a, "p");
    let t = text(&a);
    assert!(t.contains("Panelization"));
    assert!(t.contains("0029") && t.contains("0013"), "{t}");
}

#[test]
fn every_language_draws_the_classic_screens() {
    for lang in Lang::ALL {
        let mut a = app(lang);
        a.load_file(&testdata().join("FRAME.PLT"));
        assert!(text(&a).lines().count() > 10, "{lang:?}");
        a.handle(Input::Esc);
        assert!(text(&a).lines().count() > 10, "{lang:?} quit dialog");
    }
}
