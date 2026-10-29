//! The classic screens of ROUTEPRO against reference screens captured from the original program.
//!
//! The reference screens are not part of the repository. Point `EH3D_ORIGINAL_UI` at a folder that
//! holds them (`route_main.txt`, ...: a `# chars`, `# fg` and `# bg` block of 24 rows of 80 cells) to run
//! these tests; without it they pass trivially.

use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use eh3d_drillpro::app::Input;
use eh3d_drillpro::classic::ClassicApp;
use eh3d_drillpro::i18n::Lang;
use eh3d_drillpro::screen::{Screen, ROWS};
use eh3d_drillpro::textapp::TextApp;
use eh3d_routepro::app::App;

struct Reference {
    chars: Vec<Vec<char>>,
    fg: Vec<Vec<char>>,
    bg: Vec<Vec<char>>,
}

fn reference(name: &str) -> Option<Reference> {
    let dir = std::env::var_os("EH3D_ORIGINAL_UI")?;
    let text = std::fs::read_to_string(Path::new(&dir).join(name)).ok()?;
    let mut blocks: Vec<Vec<Vec<char>>> = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') {
            blocks.push(Vec::new());
        } else if let Some(b) = blocks.last_mut() {
            b.push(line.chars().collect());
        }
    }
    let [chars, fg, bg]: [Vec<Vec<char>>; 3] = blocks.try_into().ok()?;
    Some(Reference { chars, fg, bg })
}

fn testdata() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata")
}

fn app() -> ClassicApp<App> {
    ClassicApp::with_engine(App::new(Lang::En, &testdata()))
}

fn press(app: &mut ClassicApp<App>, keys: &str) {
    for c in keys.chars() {
        app.handle(Input::Char(c));
    }
}

fn hex(v: u8) -> char {
    char::from_digit(v as u32, 16).unwrap().to_ascii_uppercase()
}

/// Cells (columns, rows) whose characters are not compared (texts that differ on purpose).
type Mask = (RangeInclusive<usize>, RangeInclusive<usize>);

fn compare(app: &ClassicApp<App>, name: &str, text_mask: &[Mask]) {
    let Some(r) = reference(name) else { return };
    let mut s = Screen::default();
    app.render(&mut s);
    let inside =
        |m: &[Mask], x: usize, y: usize| m.iter().any(|(c, rw)| c.contains(&x) && rw.contains(&y));
    let mut bad = Vec::new();
    for y in 0..(ROWS - 1) {
        for x in 0..80 {
            // (74, 23) is where the capture shows the hardware text cursor, not a cell of the screen
            if (x, y) == (74, 23) {
                continue;
            }
            let c = s.get(x, y);
            let (rc, rf, rb) = (r.chars[y][x], r.fg[y][x], r.bg[y][x]);
            let masked = inside(text_mask, x, y);
            let blank = rc == ' ' && c.ch == ' ';
            let char_ok = masked || c.ch == rc;
            let fg_ok = blank || masked || hex(c.fg) == rf;
            if !(char_ok && fg_ok && hex(c.bg) == rb) {
                bad.push(format!(
                    "r{y} c{x}: ours {:?} {}/{} original {:?} {rf}/{rb}",
                    c.ch,
                    hex(c.fg),
                    hex(c.bg),
                    rc
                ));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{name}: {} cells differ\n{}",
        bad.len(),
        bad.iter().take(40).cloned().collect::<Vec<_>>().join("\n")
    );
}

/// Heading texts that are our own wording, and the machine number.
fn heading() -> Vec<Mask> {
    vec![(5..=51, 3..=5), (62..=74, 3..=4)]
}

#[test]
fn main_screen_matches_the_original() {
    compare(&app(), "route_main.txt", &heading());
}

#[test]
fn main_screen_with_a_file_matches_the_original() {
    let mut a = app();
    a.load_file(&testdata().join("FRAME.PLT"));
    // the file name differs from the one captured
    let mut mask = heading();
    mask.push((62..=73, 14..=14));
    compare(&a, "route_main_loaded.txt", &mask);
}

#[test]
fn parameter_dialog_matches_the_original() {
    let mut a = app();
    press(&mut a, "c");
    compare(&a, "route_parameter.txt", &[]);
}

#[test]
fn tool_data_dialog_matches_the_original() {
    let mut a = app();
    press(&mut a, "t");
    compare(&a, "route_tooldata.txt", &[]);
}

#[test]
fn select_file_dialog_matches_the_original() {
    let mut a = app();
    press(&mut a, "l");
    // the list and the information lines show the files of the folder, which are not the same files
    compare(
        &a,
        "route_selectfile.txt",
        &[(18..=62, 8..=15), (16..=62, 18..=19)],
    );
}

#[test]
fn panelization_dialog_matches_the_original() {
    let mut a = app();
    a.load_file(&testdata().join("FRAME.PLT"));
    press(&mut a, "p");
    compare(&a, "route_panelz.txt", &[]);
}
