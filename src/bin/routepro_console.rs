//! ROUTEPRO front end (console): the same screens in a terminal, with the keyboard and the mouse; no window system needed.
//!
//! Usage: `routepro-console [FILE.PLT] [--lang en|es|it|de|fr|pt|zh|ja] [--data DIR] [--connection FILE] [--modern] [--dump]`
//!
//! The terminal must be at least 80 columns by 25 rows. `Ctrl+C` leaves at once.

use eh3d_drillpro::cli;
use eh3d_drillpro::console;
use eh3d_routepro::app::App;

const SETTINGS: &str = "eh3d-routepro.lang";

fn main() -> std::io::Result<()> {
    let o = cli::parse(SETTINGS, "ROUTEPRO.RCF");
    if o.modern {
        let mut app = App::new(o.lang, &o.data);
        match eh3d_drillpro::protocol::connection::load(o.connection.as_deref()) {
            Ok(c) => app.set_connection(c),
            Err(e) => app.set_error(e),
        }
        if let Some(f) = &o.file {
            app.load_file(f);
        }
        if o.dump {
            let mut s = eh3d_drillpro::screen::Screen::default();
            app.render(&mut s);
            print!("{}", s.to_text());
            return Ok(());
        }
        return console::run(app, SETTINGS);
    }
    let app = cli::start_classic(App::new(o.lang, &o.data), &o);
    if o.dump {
        print!("{}", cli::dump_text(&app));
        return Ok(());
    }
    console::run(app, SETTINGS)
}
