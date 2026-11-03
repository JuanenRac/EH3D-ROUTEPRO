//! ROUTEPRO plan: works out, on paper, what a job would do and prints it. No machine, no connection and no window are needed, and nothing is sent anywhere.
//!
//! Usage: `routepro-plan FILE.PLT [--data DIR] [--lang en|es|it|de|fr|pt|zh|ja]`
//!
//! `DIR` is the folder of `ROUTEPRO.RCF` and `ROUTEPRO.RTD` (the folder of the program by default). The lines are the same for the same files: the paths, the pens, the steps of the
//! plan, how far each axis goes, the time, the checks of the work and the verdict of the dry run against the course of the machine file.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use eh3d_drillpro::i18n::Lang;
use eh3d_routepro::app::App;

fn run(mut args: Vec<String>) -> Result<(), String> {
    let mut take = |name: &str| -> Result<Option<String>, String> {
        match args.iter().position(|a| a == name) {
            Some(i) => {
                let v = args.get(i + 1).ok_or_else(|| format!("{name} needs a value"))?.clone();
                args.drain(i..=i + 1);
                Ok(Some(v))
            }
            None => Ok(None),
        }
    };
    let lang = match take("--lang")? {
        Some(c) => Lang::from_code(&c).ok_or_else(|| format!("--lang '{c}' is not a language of the program"))?,
        None => Lang::En,
    };
    let data = take("--data")?.map_or_else(|| PathBuf::from("."), PathBuf::from);
    if let Some(o) = args.iter().find(|a| a.starts_with("--")) {
        return Err(format!("{o}: unknown option"));
    }
    if args.len() != 1 {
        return Err("usage: routepro-plan FILE.PLT [--data DIR] [--lang CODE]".into());
    }
    let mut app = App::new(lang, &data);
    app.load_file(Path::new(&args[0]));
    for line in app.plan_report()? {
        println!("{line}");
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}
