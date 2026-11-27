//! The `.eh3d` project: the outline layers are read from the job, each plan is kept in the branch of this program only, a job that changes makes the plans out of date, and the screens
//! open the first outline layer and keep its plan. Every file is made here from invented data.

use std::path::{Path, PathBuf};
use std::process::Command;

use eh3d_drillpro::app::Input;
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

/// A job whose outline layers have these paths inside `manufacturing/`.
fn write_layers(path: &Path, files: &[(&str, &str)]) {
    let layers: Vec<String> = files
        .iter()
        .map(|(rel, _)| format!(r#"{{"path":"{rel}","kind":"outline","name":"{rel}"}}"#))
        .collect();
    let record = format!(
        r#"{{"format_version":"1.0.0","layers":[{}]}}"#,
        layers.join(",")
    );
    let mut entries: Vec<(&str, &[u8])> = vec![("project.json", record.as_bytes())];
    entries.extend(files.iter().map(|(rel, text)| (*rel, text.as_bytes())));
    write_branch(path, SOURCE_BRANCH, &entries);
}

#[test]
fn two_layers_with_the_same_stem_each_keep_their_own_plan() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    write_layers(
        &path,
        &[("layers/board.gko", OUTLINE), ("layers/board.gm1", OTHER)],
    );
    let out = run(&path, &["plan"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut both = [
        "route+board.gko.txt".to_string(),
        "route+board.gm1.txt".to_string(),
    ];
    both.sort();
    assert_eq!(suite::state(&KIT, &path).unwrap().results, both);
    for ext in ["gko", "gm1"] {
        let name = format!("route+board.{ext}.txt");
        let plan = String::from_utf8(suite::read_result(&KIT, &path, &name).unwrap()).unwrap();
        assert!(plan.contains(&format!("board.{ext}")), "{name}: {plan}");
    }
    // the screen that opens the project keeps the plan under the same name
    let mut app = App::new(Lang::En, &data());
    app.load_file(&path);
    app.save_project();
    let shown = text(&app);
    assert!(shown.contains("route+board.gko.txt"), "{shown}");
    assert_eq!(suite::state(&KIT, &path).unwrap().results, both);
}

#[test]
fn a_file_name_that_two_layers_share_is_not_chosen_by_guess() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    write_layers(
        &path,
        &[("layers/a/top.gko", OUTLINE), ("layers/b/top.gko", OTHER)],
    );
    let layers = eh3dfile::outline_layers(&path).unwrap();
    let why = eh3dfile::choose(&layers, "top.gko").unwrap_err();
    assert!(
        why.contains("layers/a/top.gko") && why.contains("layers/b/top.gko"),
        "{why}"
    );
    let second = eh3dfile::choose(&layers, "layers/b/top.gko").unwrap();
    assert_eq!(
        eh3dfile::result_name(&layers, second),
        "route+b+top.gko.txt"
    );
    let before = std::fs::read(&path).unwrap();
    assert!(!run(&path, &["plan", "--layer", "top.gko"]).status.success());
    assert_eq!(std::fs::read(&path).unwrap(), before);
}

#[test]
fn the_screen_says_when_keeping_a_plan_removed_the_signature_of_the_project() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    let signer = eh3d_project::Ed25519Signer::from_seed(&[7; 32]);
    Project::open(&path, &OpenOptions::default())
        .unwrap()
        .sign_project(&signer, &SaveOptions::default())
        .unwrap();
    let mut app = App::new(Lang::En, &data());
    app.load_file(&path);
    // the screen asks first, and nothing is written until the person agrees
    let before = std::fs::read(&path).unwrap();
    app.save_project();
    assert!(app.is_asking());
    let shown = text(&app);
    assert!(
        shown.contains("writing removes the signature") && shown.contains("Write to this project?"),
        "{shown}"
    );
    assert_eq!(std::fs::read(&path).unwrap(), before);
    // Enter, Esc and any other letter leave the project as it is
    for no in [Input::Enter, Input::Esc, Input::Char('n')] {
        app.save_project();
        app.handle(no);
        assert!(!app.is_asking());
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }
    // opening again forgets the question
    app.save_project();
    app.load_file(&path);
    assert!(!app.is_asking());
    app.handle(Input::Char('y'));
    assert_eq!(std::fs::read(&path).unwrap(), before);
    // agreeing writes and tells what was removed
    app.save_project();
    app.handle(Input::Char('y'));
    let shown = text(&app);
    assert!(
        shown.contains("signature of the project was removed"),
        "{shown}"
    );
    assert_ne!(std::fs::read(&path).unwrap(), before);
    // with no signature left there is nothing to ask
    app.save_project();
    assert!(!app.is_asking());
}

#[test]
fn the_classic_screen_asks_before_removing_a_signature() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    let signer = eh3d_project::Ed25519Signer::from_seed(&[7; 32]);
    Project::open(&path, &OpenOptions::default())
        .unwrap()
        .sign_project(&signer, &SaveOptions::default())
        .unwrap();
    let mut classic = ClassicApp::with_engine(App::new(Lang::En, &data()));
    classic.load_file(&path);
    let before = std::fs::read(&path).unwrap();
    classic.save_project();
    let shown = text(&classic);
    assert!(
        shown.contains("writing removes the signature") && shown.contains("Write to this project?"),
        "{shown}"
    );
    // `No` has the focus: Enter and Esc leave the project as it is
    for no in [Input::Enter, Input::Esc] {
        classic.save_project();
        classic.handle(no);
        assert!(!text(&classic).contains("Write to this project?"));
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }
    // choosing `Yes` writes
    classic.save_project();
    classic.handle(Input::Left);
    classic.handle(Input::Enter);
    assert_ne!(std::fs::read(&path).unwrap(), before);
}

/// Saves the project the way another program does: the change is made on a copy and the file is replaced with it, so this process has no memory of that
/// write.
fn saved_by_another(path: &Path, text: &str) {
    let copy = path.with_file_name("another.eh3d");
    std::fs::copy(path, &copy).unwrap();
    write_branch(&copy, "pcb", &[("board.json", text.as_bytes())]);
    std::fs::copy(&copy, path).unwrap();
    std::fs::remove_file(&copy).unwrap();
}

#[test]
fn a_project_another_program_saved_is_not_written_over_and_can_be_read_again_or_saved_apart() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    let mut app = App::new(Lang::En, &data());
    app.load_file(&path);
    saved_by_another(&path, "{\"made\":1}");
    let bytes = std::fs::read(&path).unwrap();
    app.save_project();
    assert!(app.changed().is_some());
    let shown = text(&app);
    for word in [
        "board.eh3d changed on disk",
        "[ Read again ]",
        "[ Save suite project as... ]",
        "[ Close ]",
    ] {
        assert!(shown.contains(word), "{word}: {shown}");
    }
    // `Close` has the focus: Enter and Esc leave the project as it is
    for no in [Input::Enter, Input::Esc] {
        app.save_project();
        app.handle(no);
        assert!(app.changed().is_none());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
    // a copy under the name that is suggested takes the plan
    app.save_project();
    app.handle(Input::Down);
    app.handle(Input::Down);
    app.handle(Input::Enter);
    assert!(app.changed().is_none(), "{}", text(&app));
    let apart = dir.path().join("board-2.eh3d");
    assert_eq!(suite::state(&KIT, &apart).unwrap().results.len(), 1);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    // reading again what another program saved lets the plan be kept there
    app.load_file(&path);
    saved_by_another(&path, "{\"made\":2}");
    app.save_project();
    app.handle(Input::Down);
    app.handle(Input::Enter);
    assert!(app.changed().is_none(), "{}", text(&app));
    app.save_project();
    assert!(app.changed().is_none());
    assert_eq!(suite::state(&KIT, &path).unwrap().results.len(), 1);
}

#[test]
fn a_save_by_another_program_while_a_question_waits_refuses_the_agreed_write() {
    let dir = tempfile::tempdir().unwrap();
    let path = project(dir.path());
    let signer = eh3d_project::Ed25519Signer::from_seed(&[7; 32]);
    Project::open(&path, &OpenOptions::default())
        .unwrap()
        .sign_project(&signer, &SaveOptions::default())
        .unwrap();
    let mut app = App::new(Lang::En, &data());
    app.load_file(&path);
    app.save_project();
    assert!(app.is_asking());
    saved_by_another(&path, "{\"made\":3}");
    let bytes = std::fs::read(&path).unwrap();
    app.handle(Input::Char('y'));
    assert!(
        app.changed().is_some() && !app.is_asking(),
        "{}",
        text(&app)
    );
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
}

#[test]
fn the_classic_screen_says_a_project_was_saved_by_another_in_the_eight_languages() {
    for lang in Lang::ALL {
        let dir = tempfile::tempdir().unwrap();
        let path = project(dir.path());
        let mut classic = ClassicApp::with_engine(App::new(lang, &data()));
        classic.load_file(&path);
        saved_by_another(&path, "{\"made\":4}");
        let bytes = std::fs::read(&path).unwrap();
        classic.save_project();
        let shown = text(&classic);
        for word in [
            eh3d_project::suitetext::S::ReadAgain,
            eh3d_project::suitetext::S::MenuSaveAs,
            eh3d_project::suitetext::S::Close,
        ] {
            let word = word.tr(lang.code());
            assert!(shown.contains(word), "{word}: {shown}");
        }
        classic.handle(Input::Esc);
        let again = eh3d_project::suitetext::S::ReadAgain.tr(lang.code());
        assert!(!text(&classic).contains(again));
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
}
