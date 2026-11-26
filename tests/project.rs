//! The `.eh3d` project: the outline layers are read from the job, each plan is kept in the branch of this program only, a job that changes makes the plans out of date, and the screens
//! open the first outline layer and keep its plan. Every file is made here from invented data.

use std::path::{Path, PathBuf};
use std::process::Command;

use eh3d_drillpro::classic::ClassicApp;
use eh3d_drillpro::i18n::Lang;
use eh3d_drillpro::screen::Screen;
use eh3d_drillpro::textapp::TextApp;
use eh3d_project::manufacturing::{self as suite, SOURCE_BRANCH};
use eh3d_project::{BranchUpdate, NewProject, OpenOptions, Project, SaveOptions, Writer};
use eh3d_routepro::app::App;
use eh3d_routepro::eh3dfile::{self, KIT};

const BIN: &str = env!("CARGO_BIN_EXE_routepro-plan");
/// A rectangle of 20 by 10 mm.
const OUTLINE: &str = "%FSLAX24Y24*%\n%MOMM*%\n%ADD10C,0.1*%\nD10*\nG01*\nX100000Y100000D02*\nX300000Y100000D01*\nX300000Y200000D01*\nX100000Y200000D01*\nX100000Y100000D01*\nM02*\n";
/// A rectangle of 5 by 5 mm.
const OTHER: &str = "%FSLAX24Y24*%\n%MOMM*%\n%ADD10C,0.1*%\nD10*\nG01*\nX100000Y100000D02*\nX150000Y100000D01*\nX150000Y150000D01*\nX100000Y150000D01*\nX100000Y100000D01*\nM02*\n";

fn data() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata")
}

fn write_branch(path: &Path, branch: &str, files: &[(&str, &[u8])]) {
    let mut update = BranchUpdate::new(branch, "1.0", "0.1.0");
    for (name, bytes) in files {
        update.entries.insert((*name).to_string(), bytes.to_vec());
    }
    Project::open(path, &OpenOptions::default())
        .unwrap()
        .save_branch(&Writer::for_app(branch), update, &SaveOptions::default())
        .unwrap();
}

fn write_job(path: &Path, board: &str) {
    let record = br#"{"format_version":"1.0.0","layers":[
        {"path":"layers/board.gko","kind":"outline","name":"Board"},
        {"path":"layers/window.gko","kind":"outline","name":"Window"}]}"#;
    write_branch(
        path,
        SOURCE_BRANCH,
        &[
            ("project.json", record),
            ("layers/board.gko", board.as_bytes()),
            ("layers/window.gko", OTHER.as_bytes()),
        ],
    );
}

fn project(dir: &Path) -> PathBuf {
    let path = dir.join("board.eh3d");
    let new = NewProject {
        id: "test-id".into(),
        name: "Invented board".into(),
        units: "millimetres".into(),
        created: None,
    };
    Project::create(
        &path,
        new,
        &OpenOptions::default(),
        &SaveOptions::default().atomic,
    )
    .unwrap();
    write_branch(&path, "pcb", &[("board.json", b"{}")]);
    write_job(&path, OUTLINE);
    path
}

fn hash(path: &Path, branch: &str) -> String {
    Project::open(path, &OpenOptions::default())
        .unwrap()
        .branch(branch)
        .unwrap()
        .hash
        .clone()
}

fn run(path: &Path, args: &[&str]) -> std::process::Output {
    Command::new(BIN)
        .arg("project")
        .arg(args[0])
        .arg(path)
        .args(&args[1..])
        .arg("--data")
        .arg(data())
        .output()
        .expect("the program runs")
}

fn text(app: &impl TextApp) -> String {
    let mut screen = Screen::default();
    app.render(&mut screen);
    screen.to_text()
}

#[test]
fn plan_keeps_a_plan_for_each_outline_layer_in_its_own_branch_only() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    let (pcb, job) = (hash(&path, "pcb"), hash(&path, SOURCE_BRANCH));
    let out = run(&path, &["plan"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(hash(&path, "pcb"), pcb);
    assert_eq!(hash(&path, SOURCE_BRANCH), job);
    let state = suite::state(&KIT, &path).unwrap();
    assert_eq!(state.results, ["route-board.txt", "route-window.txt"]);
    assert_eq!(state.inputs, ["layers/board.gko", "layers/window.gko"]);
    assert!(!state.out_of_date);
    let plan = suite::read_result(&KIT, &path, "route-board.txt").unwrap();
    let plan = String::from_utf8(plan).unwrap();
    assert!(plan.contains("board.gko"), "{plan}");
    assert_eq!(run(&path, &["info"]).status.code(), Some(0));
}

#[test]
fn a_job_that_changes_makes_the_plans_out_of_date_and_a_refusal_writes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    assert!(run(&path, &["plan"]).status.success());
    write_job(&path, OTHER);
    assert_eq!(run(&path, &["info"]).status.code(), Some(2));
    assert!(run(&path, &["plan", "--layer", "board.gko"])
        .status
        .success());
    let state = suite::state(&KIT, &path).unwrap();
    assert_eq!(state.results, ["route-board.txt"]);
    assert!(!state.out_of_date);

    let before = std::fs::read(&path).unwrap();
    for args in [
        &["plan", "--layer", "missing.gko"][..],
        &["plan", "--nothing-like-this"],
        &["info", "--layer", "board.gko"],
        &["mill"],
    ] {
        assert_ne!(run(&path, args).status.code(), Some(0), "{args:?}");
        assert_eq!(std::fs::read(&path).unwrap(), before, "{args:?}");
    }
    assert!(eh3dfile::outline_layers(Path::new("missing.eh3d")).is_err());
}

#[test]
fn the_screens_open_the_first_outline_layer_and_keep_its_plan() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    let mut modern = App::new(Lang::En, &data());
    modern.load_file(&path);
    let shown = text(&modern);
    assert!(shown.contains("board.gko"), "{shown}");
    assert!(shown.contains("layer 1 of 2"), "{shown}");
    modern.save_project();
    assert!(text(&modern).contains("route-board.txt saved"));
    assert_eq!(
        suite::state(&KIT, &path).unwrap().results,
        ["route-board.txt"]
    );

    write_job(&path, OTHER);
    let mut classic = ClassicApp::with_engine(App::new(Lang::En, &data()));
    classic.load_file(&path);
    let shown = text(&classic);
    assert!(shown.contains("OUT OF DATE"), "{shown}");
    classic.save_project();
    assert!(!suite::state(&KIT, &path).unwrap().out_of_date);

    // a plot of its own takes the place of the project: Ctrl+S then writes nothing
    let before = std::fs::read(&path).unwrap();
    classic.load_file(&data().join("FRAME.PLT"));
    classic.save_project();
    assert_eq!(std::fs::read(&path).unwrap(), before);
}
