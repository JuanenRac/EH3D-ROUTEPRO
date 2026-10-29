//! The routing program through the classic front end: its files, editing and the mouse.

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

/// A scratch copy of the test data, so the tests can write files.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("eh3d-routepro-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for f in ["ROUTEPRO.RCF", "ROUTEPRO.RTD", "FRAME.PLT"] {
        std::fs::copy(testdata().join(f), dir.join(f)).unwrap();
    }
    dir
}

fn app_in(dir: &Path) -> ClassicApp<App> {
    ClassicApp::with_engine(App::new(Lang::En, dir))
}

fn press(a: &mut ClassicApp<App>, keys: &str) {
    for c in keys.chars() {
        a.handle(Input::Char(c));
    }
}

fn text(a: &ClassicApp<App>) -> String {
    let mut s = Screen::default();
    a.render(&mut s);
    s.to_text()
}

fn click_on(a: &mut ClassicApp<App>, needle: &str) {
    let t = text(a);
    for (y, line) in t.lines().enumerate() {
        if let Some(x) = line.find(needle) {
            a.click(line[..x].chars().count(), y);
            return;
        }
    }
    panic!("{needle:?} is not on the screen:\n{t}");
}

fn read(path: &Path) -> String {
    std::fs::read(path)
        .unwrap()
        .iter()
        .map(|&c| c as char)
        .collect()
}

#[test]
fn the_chord_angle_is_edited_and_saved_to_the_real_file() {
    let dir = scratch("chord");
    let mut a = app_in(&dir);
    press(&mut a, "c");
    // the chord angle field is where the drilling program has the sorting
    a.click(57, 5);
    press(&mut a, "12");
    a.handle(Input::Enter);
    click_on(&mut a, "Save");
    assert!(text(&a).contains("Saved"), "{}", text(&a));
    let written = read(&dir.join("ROUTEPRO.RCF"));
    assert!(
        written.contains("12            * HP/GL chord angle") || written.contains("12 "),
        "{written}"
    );
    let cfg = eh3d_drillpro::config::parse(&written).unwrap();
    assert_eq!(cfg.route_settings().unwrap().chord_angle, 12);
    assert!(dir.join("ROUTEPRO.RCF.BAK").exists());
    // the comments and the rest of the layout survive
    assert!(written.contains("Spindle start time"));
}

#[test]
fn a_zero_chord_angle_is_refused() {
    let dir = scratch("chord0");
    let mut a = app_in(&dir);
    press(&mut a, "c");
    a.click(57, 5);
    press(&mut a, "0");
    a.handle(Input::Enter);
    assert!(text(&a).contains("Input error!"), "{}", text(&a));
}

#[test]
fn routing_tool_data_is_edited_and_saved_to_the_tool_table() {
    let dir = scratch("tools");
    let mut a = app_in(&dir);
    a.load_file(&dir.join("FRAME.PLT"));
    press(&mut a, "t");
    // the first place holds pen 1: its X feed
    a.click(33, 4);
    press(&mut a, "800");
    a.handle(Input::Enter);
    click_on(&mut a, "Save");
    assert!(text(&a).contains("Saved"), "{}", text(&a));
    let table = eh3d_routepro::tools::parse(&read(&dir.join("ROUTEPRO.RTD")));
    let t1 = table.iter().find(|t| t.number == 1).unwrap();
    assert_eq!(t1.feed_xy_mm_min, 800);
    assert!(table.len() >= 10, "the other tools are still in the table");
}

#[test]
fn a_plot_is_opened_from_the_list_with_the_mouse() {
    let dir = scratch("files");
    let mut a = app_in(&dir);
    press(&mut a, "l");
    let t = text(&a);
    assert!(t.contains("FRAME.PLT"), "{t}");
    click_on(&mut a, "FRAME.PLT");
    click_on(&mut a, "FRAME.PLT");
    assert!(text(&a).contains("Tool Data"));
    click_on(&mut a, "Ok");
    let t = text(&a);
    assert!(t.contains("FRAME.PLT") && t.contains("00006"), "{t}");
}

#[test]
fn the_about_dialog_names_the_program_and_the_machines() {
    let mut a = ClassicApp::with_engine(App::new(Lang::En, &testdata()));
    click_on(&mut a, "About");
    let t = text(&a);
    assert!(t.contains("EH3D-ROUTEPRO"), "{t}");
    assert!(t.contains("routing program"), "{t}");
    assert!(
        t.contains("CCD/2") && t.contains("MTC") && t.contains("ATC"),
        "{t}"
    );
    assert!(t.contains("Bungard"));
}

#[test]
fn the_language_changes_from_the_config_dialog() {
    let dir = scratch("lang");
    let mut a = app_in(&dir);
    press(&mut a, "c");
    a.click(65, 17);
    a.handle(Input::Right);
    assert!(text(&a).contains("Español"), "{}", text(&a));
}
