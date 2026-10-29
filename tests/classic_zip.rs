//! A routing job packed in a ZIP archive loads from the command line path of the classic screen.

use std::io::Write;
use std::path::Path;

use eh3d_drillpro::classic::ClassicApp;
use eh3d_drillpro::i18n::Lang;
use eh3d_drillpro::screen::Screen;
use eh3d_drillpro::textapp::TextApp;
use eh3d_routepro::app::App;

#[test]
fn the_first_routing_file_of_an_archive_loads() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dir = std::env::temp_dir().join(format!("eh3d-rzip-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let plt = std::fs::read(root.join("examples/01_square.plt")).unwrap();
    let zip_path = dir.join("JOB.ZIP");
    let mut w = zip::ZipWriter::new(std::fs::File::create(&zip_path).unwrap());
    let o = zip::write::SimpleFileOptions::default();
    w.start_file("PART.NCD", o).unwrap();
    w.write_all(b"M48\n").unwrap();
    w.start_file("outline/SQUARE.PLT", o).unwrap();
    w.write_all(&plt).unwrap();
    w.finish().unwrap();
    let mut a = ClassicApp::with_engine(App::new(Lang::En, &root.join("testdata")));
    a.load_zip(&zip_path);
    let mut s = Screen::default();
    a.render(&mut s);
    assert!(s.to_text().contains("SQUARE"), "{}", s.to_text());
}
