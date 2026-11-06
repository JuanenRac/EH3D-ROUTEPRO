//! The files of the release packages and of the tests are far inside the limits of the files that are read.

use eh3d_drillpro::input_guard::{check, InputKind, InputProblem};

fn latin1(path: &std::path::Path) -> String {
    std::fs::read(path)
        .unwrap()
        .iter()
        .map(|&c| c as char)
        .collect()
}

fn plots_in(folder: &str) -> Vec<std::path::PathBuf> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(folder);
    let mut found: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.extension()
                .is_some_and(|e| e.to_string_lossy().eq_ignore_ascii_case("plt"))
        })
        .collect();
    found.sort();
    found
}

#[test]
fn every_example_and_every_test_plot_is_inside_the_limits() {
    for folder in ["examples", "testdata"] {
        let plots = plots_in(folder);
        assert!(!plots.is_empty(), "{folder} has plots");
        for plot in plots {
            assert_eq!(check(InputKind::Hpgl, &latin1(&plot)), Ok(()), "{plot:?}");
        }
    }
}

#[test]
fn a_plot_is_judged_by_the_line_the_problem_is_on() {
    let ok = latin1(
        &plots_in("testdata")
            .into_iter()
            .next()
            .expect("a plot of the tests"),
    );
    let lines = ok.split('\n').count();
    // the same plot with an absurd number added after its last line is turned down on the line that follows
    let broken = format!("{ok}\nPU{},5;", "1".repeat(60));
    assert_eq!(
        check(InputKind::Hpgl, &broken),
        Err(InputProblem::NumberTooLong(lines + 1))
    );
}
