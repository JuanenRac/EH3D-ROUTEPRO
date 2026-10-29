//! The files that travel with every release: the basic configuration and the examples, from a square to a hundred thousand points.

use eh3d_drillpro::config;
use eh3d_routepro::{hpgl, tools};

fn root(rel: &str) -> String {
    format!("{}/{rel}", env!("CARGO_MANIFEST_DIR"))
}

fn latin1(path: &str) -> String {
    std::fs::read(path)
        .unwrap()
        .iter()
        .map(|&c| c as char)
        .collect()
}

/// The examples with the points and pens each one has (the list in `examples/README.txt` is written by the same script).
const EXAMPLES: [(&str, usize, usize); 9] = [
    ("01_square.plt", 5, 1),
    ("02_two_pens.plt", 10, 2),
    ("03_circle.plt", 73, 1),
    ("04_rounded_rectangle.plt", 37, 1),
    ("05_board_outline.plt", 5 + 4 * 17, 2),
    ("06_gear_24_teeth.plt", 97 + 49, 2),
    ("07_spiral_2000_points.plt", 2001, 1),
    ("08_long_path_20000_points.plt", 20_001, 1),
    ("09_huge_100000_points.plt", 120_002, 2),
];

#[test]
fn every_example_is_read_with_its_points_and_pens() {
    for (name, points, pens) in EXAMPLES {
        let plot = hpgl::parse(&latin1(&root(&format!("examples/{name}"))));
        let got: usize = plot.paths.iter().map(|p| p.points.len()).sum();
        assert_eq!(got, points, "{name}");
        assert_eq!(plot.pens().len(), pens, "{name}");
        assert_eq!(plot.ignored, 0, "{name}");
        // everything is inside the table of the basic configuration (495 x 325 mm)
        let (x0, y0, x1, y1) = plot.bounds_mm().unwrap();
        assert!(
            x0 >= 0.0 && y0 >= 0.0 && x1 <= 495.0 && y1 <= 325.0,
            "{name}"
        );
    }
}

#[test]
fn no_example_is_missing_from_the_list() {
    let mut on_disk: Vec<String> = std::fs::read_dir(root("examples"))
        .unwrap()
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter(|n| n.ends_with(".plt"))
        .collect();
    on_disk.sort();
    let listed: Vec<String> = EXAMPLES.iter().map(|e| e.0.to_string()).collect();
    assert_eq!(on_disk, listed);
}

#[test]
fn the_basic_configuration_is_read_whole() {
    let c = config::parse(&latin1(&root("config/ROUTEPRO.RCF"))).expect("the machine file");
    let s = c.route_settings().expect("typed values");
    assert_eq!(s.clip_mm, (495.0, 325.0));
    assert_eq!(s.unit, config::CoordUnit::PlotterStep);
    assert_eq!(s.chord_angle, 5);
    assert_eq!(s.interface, 4, "no machine: the simulator");
    assert_eq!(c.directory, r".\examples");
    assert_eq!(c.default_extension, "*.PLT");
    let t = tools::parse(&latin1(&root("config/ROUTEPRO.RTD")));
    assert_eq!(t.len(), 15);
    assert!(t.iter().any(|t| t.number == 1));
}

#[test]
fn the_basic_configuration_is_rewritten_without_losing_its_explanations() {
    let text = latin1(&root("config/ROUTEPRO.RCF"));
    let c = config::parse(&text).unwrap();
    let s = c.route_settings().unwrap();
    let out = config::rewrite(&text, &s, r".\examples", "*.PLT", true);
    assert_eq!(
        out.matches('*').count(),
        text.matches('*').count(),
        "the comments are kept"
    );
    assert_eq!(config::parse(&out).unwrap().route_settings().unwrap(), s);
}

#[test]
fn the_configuration_guide_exists_in_the_eight_languages() {
    for lang in ["eng", "spa", "deu", "fra", "ita", "por", "zho", "jpn"] {
        let p = root(&format!("config/GUIDE_{lang}.txt"));
        let text = std::fs::read_to_string(&p).unwrap_or_else(|_| panic!("{p}"));
        for needle in [
            "ROUTEPRO.RCF",
            "ROUTEPRO.RTD",
            "machine.cfg",
            "01_square.plt",
        ] {
            assert!(text.contains(needle), "{lang}: {needle}");
        }
    }
}
