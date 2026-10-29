//! ROUTEPRO application state and text-mode rendering.

use std::path::{Path, PathBuf};

use eh3d_drillpro::app::Input;
use eh3d_drillpro::config::{self, Config};
use eh3d_drillpro::i18n::{str_width, Lang};
use eh3d_drillpro::job::Geometry;
use eh3d_drillpro::protocol::connection::ConnectionConfig;
use eh3d_drillpro::protocol::connection::Kind as ConnectionKind;
use eh3d_drillpro::protocol::job::{JobMachine, Pause};
use eh3d_drillpro::protocol::{connect, run_steps, Simulator};
use eh3d_drillpro::runner::{RealRun, RunState};
use eh3d_drillpro::screen::*;
use eh3d_drillpro::textapp::TextApp;

use crate::hpgl::{self, Plot, UNIT_MM};
use crate::job::{plan_path, plan_route_job};
use crate::texts::{tr, Text};
use crate::tools::{self, Tool};

#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Main,
    FileDialog,
    Paths,
    Tools,
    Config,
    Language,
    Running,
    ConfirmQuit,
}

struct Entry {
    name: String,
    dir: bool,
}

struct Run {
    done: usize,
    sim: Simulator,
    at: (i32, i32),
    connected: bool,
    /// The job on a real machine (a connection other than `demo`); the simulator above is then unused.
    real: Option<RealRun>,
    total_items: usize,
    /// The paths of the job in the order they are routed (with the copies of a panel).
    plan: Vec<crate::hpgl::Polyline>,
    /// Paths left out because they end outside the working area.
    skipped: usize,
    /// Seconds of machine time spent on the path that is being routed, and how long that path takes in all (once it is known).
    clock: f64,
    path_secs: Option<f64>,
}

pub struct App {
    pub lang: Lang,
    teach: Option<eh3d_drillpro::runner::TeachSession>,
    mode: Mode,
    dir: PathBuf,
    entries: Vec<Entry>,
    sel: usize,
    plot: Option<(PathBuf, Plot)>,
    error: Option<String>,
    tools: Vec<Tool>,
    tools_found: bool,
    config: Option<Config>,
    settings: Option<config::MachineSettings>,
    geometry: Geometry,
    connection: ConnectionConfig,
    run: Option<Run>,
    /// The folder of `ROUTEPRO.RTD` and `ROUTEPRO.RCF`, and the text of both files as they were read (to write them back).
    data_dir: PathBuf,
    config_text: Option<String>,
    tools_text: Option<String>,
    /// What the main form and the Panelization dialog add to the job.
    job_options: eh3d_drillpro::job::JobOptions,
    pub quit: bool,
}

const MENU_BG: u8 = LIGHT_GRAY;
const WIN_BG: u8 = CYAN;

impl App {
    /// `data_dir` is where `ROUTEPRO.RTD` and `ROUTEPRO.RCF` are looked up.
    pub fn new(lang: Lang, data_dir: &Path) -> Self {
        let read = |name: &str| {
            std::fs::read(data_dir.join(name))
                .ok()
                .map(|b| b.iter().map(|&c| c as char).collect::<String>())
        };
        let tool_text = read("ROUTEPRO.RTD");
        let config_text = read("ROUTEPRO.RCF");
        let config = config_text.as_deref().and_then(config::parse);
        let settings = config.as_ref().and_then(|c| c.route_settings());
        let geometry = eh3d_drillpro::job::geometry_from(settings.as_ref());
        let mut app = App {
            lang,
            mode: Mode::Main,
            dir: data_dir.to_path_buf(),
            entries: Vec::new(),
            sel: 0,
            plot: None,
            error: None,
            tools_found: tool_text.is_some(),
            tools: tool_text.as_deref().map(tools::parse).unwrap_or_default(),
            data_dir: data_dir.to_path_buf(),
            config_text,
            tools_text: tool_text,
            job_options: eh3d_drillpro::job::JobOptions::default(),
            config: config.clone(),
            settings,
            geometry,
            connection: ConnectionConfig::default(),
            run: None,
            teach: None,
            quit: false,
        };
        app.dir = config::work_dir(data_dir, app.config.as_ref());
        app.refresh_entries();
        app
    }

    /// Chooses how the job reaches the machine (see the connection file). `demo` runs on the simulator.
    pub fn set_connection(&mut self, connection: ConnectionConfig) {
        self.connection = connection;
    }

    /// Shows a problem found at start-up (for example a mistake in the connection file) on the main screen.
    pub fn set_error(&mut self, message: String) {
        self.error = Some(message);
    }

    /// Machine data for the job: from the configuration file when it was read, defaults otherwise.
    fn job_machine(&self) -> JobMachine {
        eh3d_drillpro::job::job_machine(
            self.settings.as_ref(),
            self.geometry,
            &self.connection,
            1.6,
        )
    }

    pub fn load_file(&mut self, path: &Path) {
        match std::fs::read(path) {
            Ok(b) => {
                let text: String = b.iter().map(|&c| c as char).collect();
                self.plot = Some((path.to_path_buf(), hpgl::parse(&text)));
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{}: {e}", tr(self.lang, Text::ErrRead))),
        }
    }

    pub fn paths_loaded(&self) -> usize {
        self.plot.as_ref().map_or(0, |(_, p)| p.paths.len())
    }

    fn refresh_entries(&mut self) {
        let mut v = vec![Entry {
            name: "..".into(),
            dir: true,
        }];
        if let Ok(rd) = std::fs::read_dir(&self.dir) {
            let mut items: Vec<Entry> = rd
                .flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().into_owned();
                    let dir = e.path().is_dir();
                    (dir || name.to_ascii_lowercase().ends_with(".plt"))
                        .then_some(Entry { name, dir })
                })
                .collect();
            items.sort_by(|a, b| {
                b.dir
                    .cmp(&a.dir)
                    .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            });
            v.extend(items);
        }
        self.entries = v;
        self.sel = 0;
    }

    fn menu_items(&self) -> [(Text, Mode); 7] {
        [
            (Text::MenuLoad, Mode::FileDialog),
            (Text::MenuPaths, Mode::Paths),
            (Text::MenuTools, Mode::Tools),
            (Text::MenuConfig, Mode::Config),
            (Text::MenuStart, Mode::Running),
            (Text::MenuLanguage, Mode::Language),
            (Text::MenuQuit, Mode::ConfirmQuit),
        ]
    }

    fn hotkey(&self, t: Text) -> char {
        tr(self.lang, t)
            .split('~')
            .nth(1)
            .and_then(|s| s.chars().next())
            .map_or('\0', |c| c.to_ascii_lowercase())
    }

    pub fn menu_layout(&self) -> Vec<(usize, usize, Text)> {
        let mut x = 1;
        self.menu_items()
            .iter()
            .map(|(t, _)| {
                let w = str_width(tr(self.lang, *t)) + 2;
                let r = (x, w, *t);
                x += w;
                r
            })
            .collect()
    }

    fn open(&mut self, t: Text) {
        let target = self
            .menu_items()
            .into_iter()
            .find(|(k, _)| *k == t)
            .map(|(_, m)| m);
        match target {
            Some(Mode::FileDialog) => {
                self.refresh_entries();
                self.mode = Mode::FileDialog;
            }
            Some(Mode::Running) => self.start(),
            Some(Mode::Language) => {
                self.sel = self.lang as usize;
                self.mode = Mode::Language;
            }
            Some(m) => self.mode = m,
            None => {}
        }
    }

    fn start(&mut self) {
        if self.paths_loaded() == 0 {
            self.refresh_entries();
            self.mode = Mode::FileDialog;
            return;
        }
        let mut sim = Simulator::default();
        let all_paths = self
            .plot
            .as_ref()
            .map(|(_, p)| p.paths.clone())
            .unwrap_or_default();
        let offset = self.settings.as_ref().map_or((0.0, 0.0), |m| m.offset_mm);
        let (plan, skipped) = crate::job::select_paths(&all_paths, &self.job_options, offset);
        let (connected, real, total_items) = if self.connection.is_real() {
            let paths = plan.clone();
            let items = plan_route_job(&paths, &self.tools, &self.job_machine());
            let n = items.len();
            let real = if self.connection.kind == ConnectionKind::Server {
                // the job goes to the EH3D server, which plans it again with the machine's constants and runs it on the machine's node
                let name = self
                    .plot
                    .as_ref()
                    .and_then(|(p, _)| p.file_name())
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                let request = serde_json::json!({
                    "kind": "route",
                    "name": name,
                    "tools": self.tools.iter().map(|t| serde_json::json!({
                        "number": t.number,
                        "depth_mm": t.depth_mm,
                        "iterations": t.iterations,
                        "feed_xy_mm_min": t.feed_xy_mm_min,
                        "feed_z_mm_min": t.feed_z_mm_min,
                        "spindle_rpm": t.spindle_rpm,
                    })).collect::<Vec<_>>(),
                    "paths": paths.iter().map(|p| serde_json::json!({
                        "pen": p.pen,
                        "points_mm": p.points.iter()
                            .map(|&(x, y)| [f64::from(x) * crate::hpgl::UNIT_MM, f64::from(y) * crate::hpgl::UNIT_MM])
                            .collect::<Vec<_>>(),
                    })).collect::<Vec<_>>(),
                });
                RealRun::start_on_server(self.connection.clone(), request, Vec::new(), n)
            } else {
                RealRun::start(self.connection.clone(), items)
            };
            (true, Some(real), n)
        } else {
            (connect(&mut sim).is_ok(), None, 0)
        };
        self.run = Some(Run {
            done: 0,
            sim,
            at: (0, 0),
            connected,
            real,
            total_items,
            plan,
            skipped,
            clock: 0.0,
            path_secs: None,
        });
        self.mode = Mode::Running;
    }

    fn tool_of(&self, pen: u8) -> Tool {
        self.tools
            .iter()
            .find(|t| t.number == pen)
            .cloned()
            .unwrap_or(Tool {
                number: pen,
                depth_mm: 0.1,
                iterations: 1,
                feed_xy_mm_min: 1000,
                feed_z_mm_min: 500,
                spindle_rpm: 60000,
            })
    }

    /// The simulated job goes on by `dt` seconds of machine time: a path is done when the time it takes on the machine has passed.
    pub fn tick_seconds(&mut self, mut dt: f64) -> bool {
        for _ in 0..20_000 {
            if self.mode != Mode::Running {
                return false;
            }
            let Some(run) = self.run.as_ref() else {
                return false;
            };
            if run.real.is_some() {
                return self.tick_n(1);
            }
            let Some(path) = run.plan.get(run.done) else {
                return false;
            };
            let dur = match run.path_secs {
                Some(d) => d,
                None => {
                    let tool = self.tool_of(path.pen);
                    let (steps, _) = plan_path(path, &tool, run.at, &self.geometry);
                    eh3d_drillpro::protocol::motion::duration_s(&steps)
                }
            };
            let Some(run) = self.run.as_mut() else {
                return false;
            };
            run.path_secs = Some(dur);
            if run.clock + dt < dur {
                run.clock += dt;
                return true;
            }
            dt -= dur - run.clock;
            run.clock = 0.0;
            run.path_secs = None;
            let more = self.tick_n(1);
            if !more || dt <= 0.0 {
                return more;
            }
        }
        true
    }

    /// Routes up to `n` paths of the simulated job. Returns true while work remains.
    pub fn tick_n(&mut self, n: usize) -> bool {
        if self.mode != Mode::Running {
            return false;
        }
        let Some(run) = self.run.as_mut() else {
            return false;
        };
        if let Some(real) = &run.real {
            let (next, total) = real.progress();
            run.done = (next * run.plan.len())
                .checked_div(total.max(1))
                .unwrap_or(0);
            return !real.finished();
        }
        for _ in 0..n {
            let Some(path) = run.plan.get(run.done) else {
                return false;
            };
            let tool = self
                .tools
                .iter()
                .find(|t| t.number == path.pen)
                .cloned()
                .unwrap_or(Tool {
                    number: path.pen,
                    depth_mm: 0.1,
                    iterations: 1,
                    feed_xy_mm_min: 1000,
                    feed_z_mm_min: 500,
                    spindle_rpm: 60000,
                });
            let (steps, end) = plan_path(path, &tool, run.at, &self.geometry);
            let _ = run_steps(&mut run.sim, &steps);
            run.at = end;
            run.done += 1;
        }
        run.done < run.plan.len()
    }

    pub fn click(&mut self, col: usize, row: usize) {
        if row != 0 {
            return;
        }
        let hit = self
            .menu_layout()
            .into_iter()
            .find(|&(x, w, _)| col >= x && col < x + w)
            .map(|(_, _, t)| t);
        if let Some(t) = hit {
            self.open(t);
        }
    }

    pub fn handle(&mut self, input: Input) {
        if let Input::Alt(c) = input {
            let c = c.to_ascii_lowercase();
            let hit = self
                .menu_items()
                .into_iter()
                .find(|(t, _)| self.hotkey(*t) == c)
                .map(|(t, _)| t);
            if let Some(t) = hit {
                self.open(t);
            }
            return;
        }
        match self.mode.clone() {
            Mode::Main => {}
            Mode::FileDialog => self.key_file_dialog(input),
            Mode::Language => match input {
                Input::Up => self.sel = self.sel.saturating_sub(1),
                Input::Down => self.sel = (self.sel + 1).min(Lang::ALL.len() - 1),
                Input::Enter => {
                    self.lang = Lang::ALL[self.sel];
                    self.mode = Mode::Main;
                }
                Input::Esc => self.mode = Mode::Main,
                _ => {}
            },
            Mode::ConfirmQuit => match input {
                Input::Enter | Input::Char('y' | 'Y' | 's' | 'S' | 'j' | 'J' | 'o' | 'O') => {
                    self.quit = true
                }
                Input::Esc | Input::Char(_) => self.mode = Mode::Main,
                _ => {}
            },
            Mode::Running => {
                let live = self.run.as_ref().and_then(|r| r.real.as_ref());
                match (live, input) {
                    // A job on the real machine: Enter answers a dialog, Esc stops it, and once it has ended Esc leaves.
                    (Some(real), Input::Enter) => real.confirm(),
                    (Some(real), Input::Esc) if !real.finished() => real.cancel(),
                    (_, Input::Esc) => {
                        self.mode = Mode::Main;
                        self.run = None;
                    }
                    _ => {}
                }
            }
            Mode::Paths | Mode::Tools | Mode::Config => {
                if matches!(input, Input::Esc | Input::Enter) {
                    self.mode = Mode::Main;
                }
            }
        }
    }

    fn key_file_dialog(&mut self, input: Input) {
        match input {
            Input::Up => self.sel = self.sel.saturating_sub(1),
            Input::Down => self.sel = (self.sel + 1).min(self.entries.len().saturating_sub(1)),
            Input::PageUp => self.sel = self.sel.saturating_sub(10),
            Input::PageDown => self.sel = (self.sel + 10).min(self.entries.len().saturating_sub(1)),
            Input::Esc => self.mode = Mode::Main,
            Input::Enter => {
                let Some(e) = self.entries.get(self.sel) else {
                    return;
                };
                if e.dir {
                    if e.name == ".." {
                        if let Some(p) = self.dir.parent() {
                            self.dir = p.to_path_buf();
                        }
                    } else {
                        self.dir = self.dir.join(&e.name);
                    }
                    self.refresh_entries();
                } else {
                    let path = self.dir.join(&e.name);
                    self.load_file(&path);
                    self.mode = if self.error.is_none() {
                        Mode::Paths
                    } else {
                        Mode::Main
                    };
                }
            }
            _ => {}
        }
    }

    // ------------------------------------------------------------------ rendering

    pub fn render(&self, s: &mut Screen) {
        *s = Screen::default();
        s.fill(0, 0, COLS, ROWS, ' ', BLUE, BLUE);
        s.fill(0, 0, COLS, 1, ' ', BLACK, MENU_BG);
        for (x, _, t) in self.menu_layout() {
            s.hot_text(x + 1, 0, tr(self.lang, t), BLACK, RED, MENU_BG);
        }
        s.text(2, 2, "ROUTEPRO", YELLOW, BLUE);
        s.text(2, 3, "Bungard CCD", LIGHT_CYAN, BLUE);
        let msg = match (&self.error, &self.plot) {
            (Some(e), _) => e.clone(),
            (None, Some((p, _))) => format!("{}: {}", tr(self.lang, Text::LblFile), p.display()),
            (None, None) => tr(self.lang, Text::NoFile).to_string(),
        };
        s.text(2, 5, &msg, WHITE, BLUE);
        if self.connection.is_real() {
            // A connection other than demo: the job moves the real machine. Say so, in red, with where it goes.
            let line = tr(self.lang, Text::RealMachine).replace("{}", &self.connection.describe());
            s.text(2, 7, &line, WHITE, RED);
            s.text(2, 8, tr(self.lang, Text::RealStop), LIGHT_RED, BLUE);
        } else {
            s.text(2, 7, tr(self.lang, Text::SimOnly), YELLOW, BLUE);
            s.text(2, 8, tr(self.lang, Text::Uncalibrated), LIGHT_RED, BLUE);
        }
        match self.mode {
            Mode::Main => {}
            Mode::FileDialog => self.render_files(s),
            Mode::Paths => self.render_paths(s),
            Mode::Tools => self.render_tools(s),
            Mode::Config => self.render_config(s),
            Mode::Language => self.render_languages(s),
            Mode::Running => self.render_run(s),
            Mode::ConfirmQuit => self.render_quit(s),
        }
        s.fill(0, ROWS - 1, COLS, 1, ' ', BLACK, MENU_BG);
        s.text(1, ROWS - 1, tr(self.lang, Text::StatusMain), BLACK, MENU_BG);
    }

    fn render_files(&self, s: &mut Screen) {
        let (x, y, w, h) = (14, 3, 52, 19);
        s.window(
            x,
            y,
            w,
            h,
            tr(self.lang, Text::TitleSelectFile),
            BLACK,
            WIN_BG,
        );
        let dir = self.dir.display().to_string();
        let shown: String = dir
            .chars()
            .rev()
            .take(w - 6)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        s.text(x + 2, y + 1, &shown, BLUE, WIN_BG);
        let rows = h - 4;
        let top = self.sel.saturating_sub(rows - 1);
        for (i, e) in self.entries.iter().enumerate().skip(top).take(rows) {
            let label = if e.dir {
                format!("[{}]", e.name)
            } else {
                e.name.clone()
            };
            let (fg, bg) = if i == self.sel {
                (WHITE, BLACK)
            } else {
                (BLACK, WIN_BG)
            };
            s.fill(x + 2, y + 2 + i - top, w - 4, 1, ' ', fg, bg);
            s.text(x + 2, y + 2 + i - top, &label, fg, bg);
        }
    }

    fn render_paths(&self, s: &mut Screen) {
        let (x, y, w, h) = (4, 2, 72, 22);
        s.window(x, y, w, h, tr(self.lang, Text::TitlePaths), BLACK, WIN_BG);
        let Some((path, plot)) = &self.plot else {
            s.text(x + 2, y + 2, tr(self.lang, Text::NoFile), BLACK, WIN_BG);
            return;
        };
        let b = plot.bounds_mm();
        let (bw, bh) = b.map_or((0.0, 0.0), |b| (b.2 - b.0, b.3 - b.1));
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let info = [
            (Text::LblFile, name),
            (Text::LblPaths, plot.paths.len().to_string()),
            (Text::LblPens, plot.pens().len().to_string()),
            (Text::LblSize, format!("{bw:.1} x {bh:.1}")),
            (Text::LblCut, format!("{:.0}", plot.cut_length_mm())),
            (Text::LblIgnored, plot.ignored.to_string()),
        ];
        for (i, (k, v)) in info.iter().enumerate() {
            s.text(
                x + 2 + (i % 2) * 34,
                y + 1 + i / 2,
                &format!("{}: {v}", tr(self.lang, *k)),
                BLACK,
                WIN_BG,
            );
        }
        let (px, py, pw, ph) = (x + 2, y + 5, w - 4, h - 7);
        s.fill(px, py, pw, ph, ' ', WHITE, BLACK);
        let Some(b) = b else { return };
        let sx = if bw > 0.0 { (pw - 1) as f64 / bw } else { 1.0 };
        let sy = if bh > 0.0 { (ph - 1) as f64 / bh } else { 1.0 };
        let scale = sx.min(sy * 2.0);
        let pens = plot.pens();
        let cell = |p: (i32, i32)| {
            let cx = ((p.0 as f64 * UNIT_MM - b.0) * scale).round();
            let cy = ((p.1 as f64 * UNIT_MM - b.1) * scale / 2.0).round();
            (cx >= 0.0 && cy >= 0.0 && (cx as usize) < pw && (cy as usize) < ph)
                .then_some((cx as usize, cy as usize))
        };
        for poly in &plot.paths {
            let idx = pens.iter().position(|&p| p == poly.pen).unwrap_or(0);
            let col = [LIGHT_GREEN, LIGHT_CYAN, YELLOW, LIGHT_RED, WHITE][idx % 5];
            for w2 in poly.points.windows(2) {
                // Sample the segment so long lines stay connected on the coarse grid.
                let n = (((w2[1].0 - w2[0].0).abs().max((w2[1].1 - w2[0].1).abs()) as f64
                    * UNIT_MM
                    * scale) as usize)
                    .max(1)
                    * 2;
                for i in 0..=n {
                    let t = i as f64 / n as f64;
                    let p = (
                        (w2[0].0 as f64 + (w2[1].0 - w2[0].0) as f64 * t) as i32,
                        (w2[0].1 as f64 + (w2[1].1 - w2[0].1) as f64 * t) as i32,
                    );
                    if let Some((cx, cy)) = cell(p) {
                        s.put(px + cx, py + ph - 1 - cy, '·', col, BLACK);
                    }
                }
            }
        }
    }

    fn render_tools(&self, s: &mut Screen) {
        let (x, y, w, h) = (6, 3, 68, 19);
        s.window(x, y, w, h, tr(self.lang, Text::TitleTools), BLACK, WIN_BG);
        if !self.tools_found {
            s.text(x + 2, y + 2, tr(self.lang, Text::NoToolFile), RED, WIN_BG);
            return;
        }
        let cols = [
            (x + 2, Text::ColTool),
            (x + 10, Text::ColDepth),
            (x + 24, Text::ColPasses),
            (x + 34, Text::ColFeedXY),
            (x + 46, Text::ColFeedZ),
            (x + 58, Text::ColSpindle),
        ];
        for (cx, t) in cols {
            s.text(cx, y + 1, tr(self.lang, t), BLUE, WIN_BG);
        }
        for (i, t) in self.tools.iter().take(h - 4).enumerate() {
            let r = y + 2 + i;
            s.text(x + 2, r, &format!("{:02}", t.number), BLACK, WIN_BG);
            s.text(x + 10, r, &format!("{:.1}", t.depth_mm), BLACK, WIN_BG);
            s.text(x + 24, r, &t.iterations.to_string(), BLACK, WIN_BG);
            s.text(x + 34, r, &t.feed_xy_mm_min.to_string(), BLACK, WIN_BG);
            s.text(x + 46, r, &t.feed_z_mm_min.to_string(), BLACK, WIN_BG);
            s.text(x + 58, r, &t.spindle_rpm.to_string(), BLACK, WIN_BG);
        }
    }

    fn render_config(&self, s: &mut Screen) {
        let (x, y, w, h) = (10, 5, 60, 12);
        s.window(x, y, w, h, tr(self.lang, Text::TitleConfig), BLACK, WIN_BG);
        s.text(
            x + 2,
            y + 9,
            &format!(
                "{}: {}",
                tr(self.lang, Text::LblLink),
                self.connection.describe()
            ),
            BLACK,
            WIN_BG,
        );
        let Some(c) = &self.config else {
            s.text(x + 2, y + 2, tr(self.lang, Text::NoConfigFile), RED, WIN_BG);
            return;
        };
        let rows = [
            ("Serial", c.serial_number.as_str()),
            ("Version", c.version.as_str()),
            (tr(self.lang, Text::Directory), c.directory.as_str()),
            ("Ext.", c.default_extension.as_str()),
        ];
        for (i, (k, v)) in rows.iter().enumerate() {
            s.text(x + 2, y + 2 + i, &format!("{k}: {v}"), BLACK, WIN_BG);
        }
    }

    fn render_languages(&self, s: &mut Screen) {
        let names: Vec<&str> = Lang::ALL.iter().map(|l| l.native_name()).collect();
        let w = names
            .iter()
            .map(|n| str_width(n))
            .max()
            .unwrap_or(10)
            .max(str_width(tr(self.lang, Text::TitleLanguage)))
            + 6;
        let h = names.len() + 4;
        let (x, y) = ((COLS - w) / 2, (ROWS - h) / 2);
        s.window(
            x,
            y,
            w,
            h,
            tr(self.lang, Text::TitleLanguage),
            BLACK,
            WIN_BG,
        );
        for (i, n) in names.iter().enumerate() {
            let (fg, bg) = if i == self.sel {
                (WHITE, BLACK)
            } else {
                (BLACK, WIN_BG)
            };
            s.fill(x + 2, y + 2 + i, w - 4, 1, ' ', fg, bg);
            s.text(x + 3, y + 2 + i, n, fg, bg);
        }
    }

    fn render_run(&self, s: &mut Screen) {
        let (x, y, w, h) = (12, 6, 56, 11);
        s.window(x, y, w, h, tr(self.lang, Text::TitleRouting), BLACK, WIN_BG);
        let (Some(r), Some((_, plot))) = (&self.run, &self.plot) else {
            return;
        };
        let total = plot.paths.len();
        let real = r.real.as_ref().map(|x| x.state());
        let status = match &real {
            Some(RunState::Connecting) => tr(self.lang, Text::Connecting).to_string(),
            Some(RunState::Running) => tr(self.lang, Text::Routing).to_string(),
            Some(RunState::WaitPause(Pause::ToolChange { tool })) => {
                tr(self.lang, Text::WaitToolChange).replace("{}", &tool.to_string())
            }
            Some(RunState::WaitPause(Pause::SpindleStart)) => {
                tr(self.lang, Text::WaitSpindle).to_string()
            }
            Some(RunState::Done) => tr(self.lang, Text::Done).to_string(),
            Some(RunState::Cancelled) => tr(self.lang, Text::Stopped).to_string(),
            Some(RunState::Failed(_)) => tr(self.lang, Text::LinkFailed).to_string(),
            None if !r.connected => tr(self.lang, Text::Connecting).to_string(),
            None if r.done < total => tr(self.lang, Text::Routing).to_string(),
            None => tr(self.lang, Text::Done).to_string(),
        };
        let attention = matches!(
            real,
            Some(RunState::WaitPause(_)) | Some(RunState::Failed(_))
        );
        s.text(
            x + 2,
            y + 2,
            &status,
            if attention { RED } else { BLACK },
            WIN_BG,
        );
        if let Some(RunState::Failed(why)) = &real {
            let shown: String = why.chars().take(w - 4).collect();
            s.text(x + 2, y + 3, &shown, RED, WIN_BG);
        }
        s.text(
            x + 2,
            y + 4,
            &format!(
                "{}: {} / {}",
                tr(self.lang, Text::LblProgress),
                r.done,
                total
            ),
            BLACK,
            WIN_BG,
        );
        let bar = w - 6;
        let filled = (bar * r.done).checked_div(total).unwrap_or(0);
        s.fill(x + 3, y + 5, bar, 1, '░', DARK_GRAY, WIN_BG);
        s.fill(x + 3, y + 5, filled, 1, '█', BLUE, WIN_BG);
        if let Some(real) = &r.real {
            let (next, _) = real.progress();
            s.text(
                x + 2,
                y + 7,
                &format!(
                    "{}: {} / {}",
                    tr(self.lang, Text::LblItems),
                    next,
                    r.total_items
                ),
                BLACK,
                WIN_BG,
            );
            s.text(
                x + 2,
                y + 8,
                &format!(
                    "{}: {}",
                    tr(self.lang, Text::LblLink),
                    self.connection.describe()
                ),
                BLACK,
                WIN_BG,
            );
            if !real.finished() {
                s.text(
                    x + 2,
                    y + 9,
                    tr(self.lang, Text::KeyStop),
                    DARK_GRAY,
                    WIN_BG,
                );
            }
        } else {
            s.text(
                x + 2,
                y + 7,
                &format!(
                    "{}: {:.1}",
                    tr(self.lang, Text::LblTime),
                    r.sim.elapsed_us as f64 / 1e6
                ),
                BLACK,
                WIN_BG,
            );
            s.text(
                x + 2,
                y + 8,
                &format!("{} B", r.sim.sent.len()),
                BLACK,
                WIN_BG,
            );
        }
    }

    fn render_quit(&self, s: &mut Screen) {
        let q = tr(self.lang, Text::QuitAsk);
        let w = str_width(q) + 8;
        let (x, y, h) = ((COLS - w) / 2, 10, 6);
        s.window(x, y, w, h, tr(self.lang, Text::TitleQuit), WHITE, RED);
        s.text_centered(y + 2, x, w, q, WHITE, RED);
        let buttons = format!(
            "[ {} ]   [ {} ]",
            tr(self.lang, Text::BtnYes),
            tr(self.lang, Text::BtnNo)
        );
        s.text_centered(y + 4, x, w, &buttons, YELLOW, RED);
    }
}

impl TextApp for App {
    fn handle(&mut self, input: Input) {
        App::handle(self, input)
    }
    fn click(&mut self, col: usize, row: usize) {
        App::click(self, col, row)
    }
    fn tick(&mut self) -> bool {
        self.tick_n(1)
    }
    fn render(&self, screen: &mut Screen) {
        App::render(self, screen)
    }
    fn lang(&self) -> Lang {
        self.lang
    }
    fn wants_quit(&self) -> bool {
        self.quit
    }
}

/// The classic screens of the original program over this engine.
impl eh3d_drillpro::classic::engine::ClassicEngine for App {
    fn kind(&self) -> eh3d_drillpro::classic::engine::Kind {
        eh3d_drillpro::classic::engine::Kind::Route
    }

    fn jog(&mut self, dx: i32, dy: i32, dz: i32) {
        const MIL_MM: f64 = 0.0254;
        if let Some(t) = &self.teach {
            t.jog(dx as f64 * MIL_MM, dy as f64 * MIL_MM, dz as f64 * MIL_MM);
        }
    }

    fn teach_begin(&mut self) {
        self.teach = self.connection.is_real().then(|| {
            eh3d_drillpro::runner::TeachSession::start(self.connection.clone(), self.job_machine())
        });
    }

    fn teach_end(&mut self) {
        if let Some(t) = &self.teach {
            t.end();
        }
    }

    fn teach_state(&self) -> Option<eh3d_drillpro::runner::TeachState> {
        self.teach.as_ref().map(|t| t.state())
    }

    fn set_connection(&mut self, connection: ConnectionConfig) {
        App::set_connection(self, connection);
    }

    fn set_error(&mut self, message: String) {
        App::set_error(self, message);
    }

    fn about(&self) -> eh3d_drillpro::classic::engine::About {
        eh3d_drillpro::classic::engine::About {
            name: "EH3D-ROUTEPRO",
            version: env!("CARGO_PKG_VERSION"),
            repository: env!("CARGO_PKG_REPOSITORY"),
        }
    }

    fn config_mut(&mut self) -> Option<&mut Config> {
        self.config.as_mut()
    }

    fn settings_changed(&mut self) {
        self.geometry = eh3d_drillpro::job::geometry_from(self.settings.as_ref());
    }

    fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }

    fn save_config(&mut self) -> Result<PathBuf, String> {
        let (Some(text), Some(settings), Some(cfg)) = (
            self.config_text.as_deref(),
            self.settings.as_ref(),
            self.config.as_ref(),
        ) else {
            return Err("no configuration file".into());
        };
        let out = config::rewrite(text, settings, &cfg.directory, &cfg.default_extension, true);
        let path = self.data_dir.join("ROUTEPRO.RCF");
        config::write_with_backup(&path, &out)?;
        self.config = config::parse(&out);
        self.config_text = Some(out);
        Ok(path)
    }

    fn reload_config(&mut self) -> Result<(), String> {
        let path = self.data_dir.join("ROUTEPRO.RCF");
        self.load_config_from(&path)
    }

    fn load_config_from(&mut self, path: &std::path::Path) -> Result<(), String> {
        let text = config::bytes_to_text(&std::fs::read(path).map_err(|e| e.to_string())?);
        let cfg =
            config::parse(&text).ok_or("the configuration file is not in the expected layout")?;
        self.settings = cfg.route_settings();
        self.config = Some(cfg);
        self.config_text = Some(text);
        self.settings_changed();
        Ok(())
    }

    fn apply_tool_rows(&mut self, rows: &[eh3d_drillpro::classic::engine::ToolRow]) {
        for row in rows {
            let (Some(n), [depth, iter, fxy, fz, rpm]) = (row.name, row.cells.as_slice()) else {
                continue;
            };
            let parsed = (
                depth.trim().parse::<f64>(),
                iter.trim().parse::<u32>(),
                fxy.trim().parse::<u32>(),
                fz.trim().parse::<u32>(),
                rpm.trim().parse::<u32>(),
            );
            let (
                Ok(depth_mm),
                Ok(iterations),
                Ok(feed_xy_mm_min),
                Ok(feed_z_mm_min),
                Ok(spindle_rpm),
            ) = parsed
            else {
                continue;
            };
            let tool = Tool {
                number: n,
                depth_mm,
                iterations,
                feed_xy_mm_min,
                feed_z_mm_min,
                spindle_rpm,
            };
            match self.tools.iter_mut().find(|t| t.number == n) {
                Some(t) => *t = tool,
                None => self.tools.push(tool),
            }
        }
    }

    fn save_tools(&mut self) -> Result<PathBuf, String> {
        let original = self.tools_text.clone().unwrap_or_default();
        let out = tools::rewrite(&original, &self.tools);
        let path = self.data_dir.join("ROUTEPRO.RTD");
        config::write_with_backup(&path, &out)?;
        self.tools_text = Some(out);
        self.tools_found = true;
        Ok(path)
    }

    fn reload_tools(&mut self) -> Result<(), String> {
        let path = self.data_dir.join("ROUTEPRO.RTD");
        self.load_tools_from(&path)
    }

    fn load_tools_from(&mut self, path: &std::path::Path) -> Result<(), String> {
        let text = config::bytes_to_text(&std::fs::read(path).map_err(|e| e.to_string())?);
        self.tools = tools::parse(&text);
        self.tools_text = Some(text);
        self.tools_found = true;
        Ok(())
    }

    fn lang(&self) -> Lang {
        self.lang
    }

    fn set_lang(&mut self, lang: Lang) {
        self.lang = lang;
    }

    fn settings(&self) -> Option<&config::MachineSettings> {
        self.settings.as_ref()
    }

    fn settings_mut(&mut self) -> Option<&mut config::MachineSettings> {
        self.settings.as_mut()
    }

    fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    fn set_board(&mut self, _mm: f64) {}

    fn load(&mut self, path: &Path) {
        self.load_file(path);
    }

    fn file_loaded(&self) -> bool {
        self.plot.is_some()
    }

    fn file_name(&self) -> Option<String> {
        self.plot
            .as_ref()
            .and_then(|(p, _)| p.file_name())
            .map(|n| n.to_string_lossy().to_ascii_uppercase())
    }

    fn vectors(&self) -> usize {
        self.paths_loaded()
    }

    fn bounds(&self) -> Option<(f64, f64, f64, f64)> {
        self.plot.as_ref().and_then(|(_, p)| p.bounds_mm())
    }

    fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    fn dir(&self) -> PathBuf {
        self.dir.clone()
    }

    fn set_dir(&mut self, dir: PathBuf) {
        self.dir = dir;
    }

    fn pattern(&self) -> &'static str {
        "*.PLT"
    }

    fn entries(&mut self) -> Vec<(String, bool)> {
        self.refresh_entries();
        eh3d_drillpro::classic::engine::dos_order(
            self.entries
                .iter()
                .filter(|e| !e.name.starts_with('.') || e.name == "..")
                .map(|e| (e.name.clone(), e.dir))
                .collect(),
        )
    }

    fn tool_rows(&self) -> Vec<eh3d_drillpro::classic::engine::ToolRow> {
        use eh3d_drillpro::classic::engine::ToolRow;
        let pens: Vec<u8> = self
            .plot
            .as_ref()
            .map(|(_, p)| p.pens())
            .unwrap_or_default();
        (0..15)
            .map(|i| {
                let name = pens.get(i).copied();
                let tool = name.and_then(|n| self.tools.iter().find(|t| t.number == n));
                let (depth, iter, fxy, fz, rpm) = tool.map_or((0.0, 1, 50, 50, 30000), |t| {
                    (
                        t.depth_mm,
                        t.iterations,
                        t.feed_xy_mm_min,
                        t.feed_z_mm_min,
                        t.spindle_rpm,
                    )
                });
                ToolRow {
                    name,
                    cells: vec![
                        format!("{depth:05.2}"),
                        format!("{iter:02}"),
                        format!("{fxy:04}"),
                        format!("{fz:04}"),
                        format!("{rpm:05}"),
                    ],
                    first_vector: name
                        .and_then(|n| {
                            self.plot
                                .as_ref()
                                .and_then(|(_, p)| p.paths.iter().position(|q| q.pen == n))
                        })
                        .map_or(0, |p| p + 1),
                    in_table: tool.is_some(),
                }
            })
            .collect()
    }

    fn set_job_options(&mut self, options: eh3d_drillpro::job::JobOptions) {
        self.job_options = options;
    }

    fn job_view(&self) -> Option<eh3d_drillpro::classic::engine::JobView> {
        let run = self.run.as_ref()?;
        let offset = self.settings.as_ref().map_or((0.0, 0.0), |m| m.offset_mm);
        let at = run.done.min(run.plan.len().saturating_sub(1));
        let path = run.plan.get(at)?;
        let tool = self.tools.iter().find(|t| t.number == path.pen);
        let first = path.points.first().copied().unwrap_or((0, 0));
        let used: f64 = run
            .plan
            .iter()
            .take(run.done)
            .map(|p| {
                self.tools
                    .iter()
                    .find(|t| t.number == p.pen)
                    .map_or(0.0, |t| t.depth_mm)
            })
            .sum();
        // the length of the cuts the pen makes from here on
        let mut run_mm = 0.0;
        for p in run.plan[at..].iter().take_while(|p| p.pen == path.pen) {
            for w in p.points.windows(2) {
                let (dx, dy) = (
                    f64::from(w[1].0 - w[0].0) * hpgl::UNIT_MM,
                    f64::from(w[1].1 - w[0].1) * hpgl::UNIT_MM,
                );
                run_mm += (dx * dx + dy * dy).sqrt();
            }
        }
        Some(eh3d_drillpro::classic::engine::JobView {
            percent: (run.done * 100 / run.plan.len().max(1)).min(100) as u8,
            tool: Some(path.pen),
            stroke_mm: tool.map_or(0.0, |t| t.depth_mm),
            x_mm: f64::from(first.0) * hpgl::UNIT_MM + offset.0,
            y_mm: f64::from(first.1) * hpgl::UNIT_MM + offset.1,
            z_feed: tool.map_or(50, |t| t.feed_z_mm_min),
            rpm: tool.map_or(30000, |t| t.spindle_rpm),
            done: run.done,
            used_mm: used,
            skipped: run.skipped,
            run_mm,
        })
    }

    fn question(&self) -> Option<eh3d_drillpro::classic::engine::Question> {
        use eh3d_drillpro::classic::engine::Question;
        use eh3d_drillpro::protocol::job::Pause;
        match self.run.as_ref()?.real.as_ref()?.state() {
            RunState::WaitPause(Pause::ToolChange { tool }) => Some(Question::ToolChange(tool)),
            RunState::WaitPause(Pause::SpindleStart) => Some(Question::SpindleStart),
            _ => None,
        }
    }

    fn answer(&mut self, ok: bool) {
        let Some(real) = self.run.as_ref().and_then(|r| r.real.as_ref()) else {
            return;
        };
        if ok {
            real.confirm();
        } else {
            real.cancel();
        }
    }

    fn start(&mut self) {
        App::start(self);
    }

    fn started(&self) -> bool {
        self.run.as_ref().is_some_and(|r| r.done > 0)
    }

    fn done(&self) -> usize {
        self.run.as_ref().map_or(0, |r| r.done)
    }

    fn tick(&mut self, n: usize) -> bool {
        let more = self.tick_n(n);
        if !more {
            self.mode = Mode::Main;
        }
        more
    }

    fn tick_for(&mut self, seconds: f64) -> bool {
        let more = self.tick_seconds(seconds);
        if !more {
            self.mode = Mode::Main;
        }
        more
    }

    fn stop(&mut self) {
        self.run = None;
        self.mode = Mode::Main;
    }
}
