//! ROUTEPRO plan: works out, on paper, what a job would do and prints it. No machine, no connection and no window are needed, and nothing is sent anywhere.
//!
//! Usage: `routepro-plan FILE.PLT [--data DIR] [--lang en|es|it|de|fr|pt|zh|ja]`
//!
//! `DIR` is the folder of `ROUTEPRO.RCF` and `ROUTEPRO.RTD` (the folder of the program by default). The lines are the same for the same files: the paths, the pens, the steps of the
//! plan, how far each axis goes, the time, the checks of the work and the verdict of the dry run against the course of the machine file.
//!
//! The `.eh3d` project of the suite: `routepro-plan project info FILE.eh3d` shows the layers of the job and the plans kept in the project (it ends with the code 2 when they are out
//! of date), and `routepro-plan project plan FILE.eh3d [--layer NAME] [--data DIR] [--lang CODE]` plans each outline layer of the job (or the one named) and keeps each plan in the
//! branch `routepro/` only.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use eh3d_drillpro::i18n::Lang;
use eh3d_project::manufacturing as suite;
use eh3d_routepro::app::App;
use eh3d_routepro::eh3dfile;

const USAGE: &str = "usage: routepro-plan FILE.PLT [--data DIR] [--lang CODE]\n       routepro-plan project info FILE.eh3d\n       routepro-plan project plan FILE.eh3d [--layer NAME] [--data DIR] [--lang CODE]";

/// `project info FILE` and `project plan FILE`: the code the program ends with (0 clean, 2 out of date).
fn project(args: &[String], layer: Option<&str>, lang: Lang, data: &Path) -> Result<u8, String> {
    let (what, file) = match args {
        [what, file] => (what.as_str(), Path::new(file)),
        _ => return Err(USAGE.into()),
    };
    match what {
        "info" if layer.is_none() => {
            let (lines, state) = suite::describe(&eh3dfile::KIT, file)?;
            for line in lines {
                println!("{line}");
            }
            if state.out_of_date {
                eprintln!("{}", suite::out_of_date_message(&eh3dfile::KIT));
                return Ok(2);
            }
            Ok(0)
        }
        "plan" => {
            // the mark is taken before the layers are read: what another program saves after it is not written over
            let seen = eh3d_drillpro::eh3dfile::mark_of(&eh3dfile::KIT, file)?;
            let layers = eh3dfile::outline_layers(file)?;
            let chosen = match layer {
                Some(name) => vec![eh3dfile::choose(&layers, name)?.clone()],
                None => layers.clone(),
            };
            // every plan is made before one is kept, so a refusal leaves the project as it was
            let mut made = Vec::new();
            for layer in chosen {
                let mut app = App::new(lang, data);
                app.load_layer(file, &layer)?;
                let lines = app.plan_report()?;
                for line in &lines {
                    println!("{line}");
                }
                made.push((layer, lines));
            }
            for (layer, lines) in made {
                for notice in eh3dfile::save_plan_since(file, &seen, &layer, &lines)? {
                    eprintln!("note: {notice}");
                }
                println!(
                    "{} kept in the branch {} of {}",
                    eh3dfile::result_name(&layers, &layer),
                    eh3dfile::KIT.branch,
                    file.display()
                );
            }
            Ok(0)
        }
        _ => Err(USAGE.into()),
    }
}

fn run(mut args: Vec<String>) -> Result<u8, String> {
    let mut take = |name: &str| -> Result<Option<String>, String> {
        match args.iter().position(|a| a == name) {
            Some(i) => {
                let v = args
                    .get(i + 1)
                    .ok_or_else(|| format!("{name} needs a value"))?
                    .clone();
                args.drain(i..=i + 1);
                Ok(Some(v))
            }
            None => Ok(None),
        }
    };
    let lang = match take("--lang")? {
        Some(c) => Lang::from_code(&c)
            .ok_or_else(|| format!("--lang '{c}' is not a language of the program"))?,
        None => Lang::En,
    };
    let data = take("--data")?.map_or_else(|| PathBuf::from("."), PathBuf::from);
    let layer = take("--layer")?;
    if let Some(o) = args.iter().find(|a| a.starts_with("--")) {
        return Err(format!("{o}: unknown option"));
    }
    if args.len() > 1 && args[0] == "project" {
        return project(&args[1..], layer.as_deref(), lang, &data);
    }
    if args.len() != 1 || layer.is_some() {
        return Err(USAGE.into());
    }
    let mut app = App::new(lang, &data);
    app.load_file(Path::new(&args[0]));
    for line in app.plan_report()? {
        println!("{line}");
    }
    Ok(0)
}

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}
