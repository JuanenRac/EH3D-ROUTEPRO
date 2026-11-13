# Changelog

All notable changes to this project are documented here.

## [Unreleased]

- **The tests that load a plot no longer share a file:** each call of the helper of the input limits writes a file of its own, so two tests that run at the same time cannot delete the file of each other (a test failed now and then with a file that was not there).
- **The CI and the release build take the sibling repositories at their released tags:** `EH3D-COMMON` at `v0.2.1`, and the other siblings at the version they were released in, instead of at `main`; the manifest asks for `min_common` 0.2.1.

## [0.1.6] - Shared path selection, reader limits and warnings of the server

- **A pull request must say what it changes:** the validation runs `tools/changelog_pr_check.py` on a pull request (it fails when the CHANGELOG has no new line under [Unreleased]; the label `no-changelog` skips it), the same check as the other repositories of the suite.
- **The Rust toolchain is pinned:** `rust-toolchain.toml` fixes the version of Rust (the same one in every repository of the suite) with `rustfmt` and `clippy`, the workflows install that version and not `stable`, so a new release of Rust cannot turn the CI red by itself.
- **The CI keeps its actions up to date and does not pile up runs:** a `dependabot.yml` groups the updates of the GitHub Actions in one monthly request, a new push to a branch cancels the run of the older one that is still going, and the Rust jobs have a time limit, so a stuck job cannot hold a runner for hours.
- **Tests of the plots that cannot be loaded:** a plot that is not there is an error on the screen and the plot already loaded stays; plots of nothing (empty, blank, a bare command) are taken empty or turned down, never a panic.
- **The selection of the paths takes the range of vectors, the panel and the working area from the code shared with the drilling:** the first and last vector, the copies on the panel and the test of the working area are the functions of `eh3d_drillpro::job` that the selection of the holes uses, so a change in one is a change in the other; what is selected does not change.
- **A plot, a drawing or an outline past the limits of the readers is refused with its code, and a big one is said:** loading goes through `hpgl::parse_limited` (and the same check for the DXF drawing and the Gerber outline) with the limits of `reader_limits.cfg` of the shared crate, so a file past a hard limit is refused with `EH3D-FORMATS-011` in the language of the screen and a file past a warning limit is read and the line under the file says that it is big, in the 37 languages. A test loads a plot past the warning limit.
- **A closing or a place of the tool changer that cannot be right is turned down before the job is planned:** when a job starts, the checks the server makes of a `rack` (`bad_rack`) are run on the options of the job, and the screen shows the same sentence the server gives; a job with a valid rack starts as it did.
- **The warnings of the server about the request are shown under the job in the language of the program:** the unknown field of the rack, the version of the request read as the newest the server knows, the tool changer left out, the machine that changes its tools by hand and the work that goes past the course of an axis are told in up to four rows of the window of a routing job on the server, in the 37 languages of the shared texts.

## [0.1.5] - Guards for the input files, the contract of the job with the server and the zip library at 9

- **The plots and drawings that are read have limits:** a plot, a drawing or an outline that is bigger than 64 MiB, has a character 0, a drawing with a line of more than 16384 characters or a file with a number of more than 40 digits in a row is turned down when it is loaded, with the stable code (`input_too_large`, `input_binary`, `input_line_too_long`, `input_number_too_long`) and the number of its line, through the limits of EH3D-DRILLPRO; a plot with all its commands on one line is not held to the length of a line, and the examples and sample plots load as before. With tests of the plot and the drawing through the window and of every example.
- **The routing request is held to the contract of the server:** the request is sent with the fields of the second version only when it chains the tool changer, carries only the fields of `rack` that the server knows, passes the checks of the server for the places given to the tools and is read back as the very options of the program. With tests.
- Update the zip library to the version without the known vulnerabilities.
- Screenshots of the application in the README.
- **A routing job on the tool changer cuts where the design is:** after a tool was taken from the bar the plan of the next path went on from where the last path had ended, and not from where the head really was (at the bar), so the cut began shifted by the distance from the design to the bar - past the edge of the table. After every exchange the head now goes from the bar to the start of the path first. With tests that follow the planned steps from the switches and compare the end of every path with the design, with the tools in the places in the order of the job and in other places.
- **The cutters are taken from the places of the rack that the person gave them:** the exchanges of a routing job - taking the cutter, putting the used one back, the one the drilling left in the chuck and the last one at the end - go to the place given to that cutter (`rack_slots` of the job options), and a cutter without a place is taken from the place the order of the job gives it, as before. With tests of the order of the chuck.
- **A routing job sent to the server carries the chaining of the tool changer:** the request has the optional `rack` field (`first`, `carry_in`, `carry_out`, with `request_version` 2) when the job is one of a chain, and is the request of before when it is not. A server that turns the field down (400 or 422) gets the job again without it, and one that does not answer `rack_applied` is noticed too: the run window says in red that the tool changes were not chained. With tests of the request with and without chaining.

## [0.1.4] - Audit fixes, the new reading limits, tests and the application icon

- **A folder with no machine file starts from the one that comes with the program,** as the drilling program does: before, the routing program had no settings there (offset, scale, area) while the drilling program had the offset of 2.5 mm, so a job of drilling and routing from such a folder put the paths 2.5 mm away from the holes.
- **The Windows executable carries the icon of the program:** the file shows it in the explorer and the taskbar, together with the name, the description and the version, embedded by `build.rs`; other systems are not affected.
- **The window carries the icon of the program:** the title bar and the task bar show the icon of ROUTEPRO (the shared window of the classic screens takes the icon as a parameter).
- **The application icon and the logos of the program:** `images/brand/` holds the icon (SVG, PNG sizes, `.ico` and `.icns`), the horizontal and vertical logos in a light and a dark version and the social preview image.
- **Release builds are checked and attested:** `release-binaries.yml` takes the owner's go-ahead only (a manual start from main, the commit must be part of main), builds the same packages as before, writes `SHA256SUMS` and a build manifest, attests their provenance, verifies the artifact as a stranger would, prepares a draft release of unsigned files only after the owner approves it (a dry run is the default) and opens an issue `build-failed` when anything does not finish.
- **Shared checks updated:** the validator also compares the badge and the Version row of the README with the manifest, fails when one of its checks cannot run, and the CHANGELOG step of the workflow reads the base branch from the environment.

- **A plan on paper with no machine (`routepro-plan`):** `routepro-plan FILE.PLT [--data DIR] [--lang CODE]` and `App::plan_report` work out what a job would do with the paths, pens, rack and machine file that a start uses and print the facts (paths, pens, steps, paths left out) and the summary (how far each axis goes, the time, the checks of the work and the verdict against the course), in any of the languages of the program. It needs no connection and sends nothing; an option it does not know, a language it does not have or a file it cannot read is an error with exit code 1.
- **Stored plans of the sample files:** `tests/plan_golden.rs` compares the plans of `FRAME.PLT`, `FDEMO1.PLT` and `MEASURE.PLT` (English) and of `FRAME.PLT` in Spanish with the copies in `tests/golden/`; a change that is meant is stored again with `EH3D_UPDATE_GOLDEN=1`. It also checks that the facts do not depend on the language, that the same files give the same plan twice and the command line.

- **The README says what the ATC models do:** it no longer says that the tool changer is not supported; a routing job takes the tools from the rack of the machine file and has not run on a real machine.
- **A routing job on a machine with the tool changer does not ask for the tools:** `plan_route_job_atc_marked` takes the pens from the places of the rack in the order the paths use them (place 1 for the first, place 0 is the test pin), changes them itself and puts the last one back; the operator is asked for the start of the spindle only. `plan_route_job_marked` is the same job without a rack, as before.
- **The classic engine tells where the head is during a simulated job** (`head_mm`, from the steps of the plan), for the views of the modern interface.
- **The workflows are hardened:** every action is pinned by its commit with its version in a comment, the checkouts keep no credentials, and the release workflow runs only for the owner on main, with no cache and the values of the matrix passed by environment.
- **More tests of the planning of a routing path:** the approach, the passes, a path of one point, a table that is not flat and the length of a path.

## [0.1.3] - Shared summary, marks and progress by time
- **The summary before a start comes from EH3D-COMMON (`eh3d-summary`):** the refusal of a plan that leaves the course takes the thresholds of the tuning through `eh3d_drillpro::summary::limits`, and the clock of the time left is the shared one.

- **A summary before a job moves a real machine:** the exact plan of the routing job is followed on paper first (the dry run of the shared library) and a window shows what the job does, how long it takes, how far each axis goes and what looks wrong in the file, in the 37 languages. The job starts only when the summary is accepted, in the text screens and in the classic ones; a plan that would take an axis past its course is refused (`dry_run = reject`, the default, or only warned with `warn`). The keys `confirm_start`, `dry_run`, `dry_run_margin_mm`, `sanity_tiny_mm` and `sanity_far_mm` of the machine file set it.
- **The demo follows the same plan as the machine:** the simulation runs the very items that go to a real machine (the reference runs, the spindle, the dialogs of the tool change, the cuts with the depth of the tool table and of a measured table) instead of planning each path on its own; the tests compare the bytes it sends with the plan.
- **The plan says where each path ends:** `plan_route_job_marked` gives the item that ends every path (a path with no points is done at once), and the paths done come from those marks.
- **Progress by time, and the time left:** the window of a run shows the share of the job done by the time the steps take on the machine, the time left, and the paths done out of the paths of the job (before, out of all the paths of the file).
- **One cautious tool and one construction of the parameters of the planner:** `default_tool` is the only place the values for a pen the table does not give are written, and the parameters of the planner are built in one function for the plain and the levelled cut.
- **`test.sh` and `test.bat` run the tests of the whole workspace,** the protocol crate included.

## [0.1.2] - A failure inside the work is told and stops the machine, a record of crashes and a CI that builds the interface

- **A failure inside the work is told, and the machine is stopped:** when the thread that runs a job (or the Offset window) panics, the run no longer stays at "running" for ever with nothing said: the machine is stopped first and the state becomes failed with "internal error" and the words of the panic.
- **A record of crashes:** a panic is also appended to `eh3d-crash.log` in the configuration folder (`%APPDATA%\EH3D`, `~/.config/eh3d`) with the program, its version, the words and the place, because a window program on Windows has no console to show it.
- **The CI builds and tests the graphical interface too** (`--features gui`, with the libraries of the window system on Linux), and checks the pictures of the README; the architecture picture pointed at files that no longer exist and now points at the ones that do.

## [0.1.1] - Cautious values named, acceleration and spindle start counted, cut copy and paste, and full screen

- **The pens that are cut with the cautious values are named:** a pen of the plot that the tool table does not give (or whose line could not be read, which is then shown) is cut with the cautious defaults as before, but the screen says which pens they are; the decimal comma is read in the table.
- **The acceleration and the spindle start time of the machine file count,** as in DRILLPRO (the same code builds the machine of the job; not tried on a real machine).
- **The request to the server cannot be split** and the token's clear transit is documented (see DRILLPRO).
- **The height map and the server:** a job that goes through the server is not started with a height map in use, because the server plans on a flat table.
- **Tests that prove what they say:** the levelling test now needs the tool to go down strictly less, and a new test checks that the cut reaches exactly the depth asked, going down with `Z_POS`, and is back up at the end.
- **Two new texts in the 37 languages;** the notes of the project no longer say the protocol was never tried on a machine and that the program has eight languages.

- **Cut, copy and paste, and full screen:** the window of ROUTEPRO is the one of DRILLPRO, so it has the same clipboard in every field (the keys and the menu of the right button) and the same full-screen setting (Parameter window and F11); see DRILLPRO.

## [0.1.0] - Spindle warnings of the server and the head of the drill on screen

- **What the server tells the run screen:** a job on the server shows the excesses of the spindle that the server cut (`spindle_warnings` of its view, one short text for each: the tool, the value asked for and the limit) under the progress, and, when the machine has the node of a drill head, the turns of the spindle and its temperature, asked of the server every two seconds and let go if the machine has no node or the node does not answer. The line is named in all 37 languages. The reading of the notes is the library of DRILLPRO's, tested there.

## [0.0.9] - No console window on Windows, hot keys in 37 languages and the pens that are off sent to the server

- **No black window on Windows:** the program no longer opens a console; started from a terminal it attaches to it, so the help and the errors are printed there as before.

- **Hot keys in the menu of the 29 languages read from files:** the items of the menu bar have a highlighted letter in each of them (`~X~` in `locales/*.tsv`), written by `tools/make_hotkeys.py` (the same tool as EH3D-DRILLPRO, which also writes the letters of the classic screens); Cyrillic and Greek letters are typed with that alphabet's keyboard or with the Latin key that sits in the same place (Russian, Ukrainian, Greek; Bulgarian and Serbian need their own layout). 2 tests check the menu and the parameter window in all 37 languages.

- **The pens that are off go to the server too:** a job sent to the EH3D server (`connection = server`) carries `skip_tools` along with the paths the program chose, so the server leaves the same pens out and never asks the operator for them (`app::server_route_request`). 1 test.

## [0.0.8] - 37 languages and pens that can be left out of a job

- **Room for 37 languages:** the words of the new languages are read from the files of `locales/` (the same scheme as DRILLPRO), and the list of languages scrolls.

- **Pens switched off:** the pens listed in `JobOptions::skip_tools` are left out of the routing job (`select_paths`). 1 test.

- **Tabs only on the outside, and on a single board:** the tabs of a panel are cut into the outside of each board only (the pockets and slots inside are routed whole), and a panel of one copy with tabs is cut at its tabs too.

## [0.0.7] - A table that is not flat and spoken notices

- **A table that is not flat:** with the height map of the table the cut follows its height in pieces of 10 mm, so the tool keeps the same depth into the board from one end of the table to the other; without a map nothing changes.
## [0.0.6] - Tabs for the outside of a panel, jobs that go on and the connection drop-down

- **A job can go on:** the paths already done are left out when the job is started again from where it was.
- **Outside routed with tabs:** the closed paths of a panel are cut at its tabs, so the router lifts there and the boards stay joined to the panel.
- **The connection chosen in the Config window:** a drop-down with the five ways the job reaches the machine and what each needs, applied with OK and written into the machine file (it comes with the classic screens of EH3D-DRILLPRO).
- **A blinking cursor:** the cursor of a field being edited blinks.
- **Several machines and stations at once:** with `connection = server`, `routepro --machines` lists every machine of the server, grouped by station, with what each one is doing, its job and the jobs ready; `--machine ID` sends the jobs to that machine instead of the one in the machine file.
- **The buttons answer to the pointer:** the button under the pointer lights up, and while it is pressed it sinks (it loses its shadow, goes darker and moves its letters a little); `--point COLUMN,ROW[,down]` puts the pointer there for the pictures.
## [0.0.5] - Plotter arcs and circles, DXF drawings and Gerber outlines, sounds, the machine files from a shared crate and screenshots

- **The main screen says it is ROUTEPRO:** the heading under the logo no longer reads as the name of the drilling program.
- **Screenshots in the README:** two pictures of the classic screens (the main screen and the tool data) in all the eight languages of the README.
- **The drawing of a board and its Gerber outline:** a `.DXF` drawing (lines, polylines with arcs, arcs and circles) or the outline layer of a Gerber board (`.GKO`, `.GM1`, `.GML` or a `.GBR` that calls itself the profile) is loaded as a routing path from the file window, from the command line and from ZIP archives; a Gerber layer of copper is refused with the name of the file, and the window offers the patterns `*.DXF` and `*.GKO` next to `*.PLT`.
- **Arcs, circles and scaling in the plotter files:** the HP-GL commands `AA`, `AR` and `CI` are cut into chords, and `SC` with `IP` lays the units of the file on the plotter, so the files of programs that draw circles with those commands are cut as they are drawn; the chord angle of the routing configuration decides how fine the chords are.
- **The files of the machine come from a shared crate:** the HP-GL file and the tool table are read by `eh3d-bungard-formats` of EH3D-COMMON; nothing changes for the operator.
- **Sounds when the job ends and when the machine asks for a tool:** `sound_finish` and `sound_tool_change` in the machine file are each off (the default), `default` (a tone the program makes, one for each cue) or the path of a WAV file of the operator's. The sound is played by the system (PowerShell, `afplay`, `paplay`/`aplay`/`play`) in a process of its own, so it never stops a job. It is explained in the machine file and in the guide of the configuration in the eight languages.

## [0.0.4] - Jobs in ZIP archives, an Offset window that moves the machine, file patterns and a simulation at the speed of the machine

- **The simulated job follows the clock of the machine** (every path takes the time it would take with the feeds of its tool), as in DRILLPRO.

- **A lighter debug build:** the `dev` profile keeps the file and line of a panic (`debug = "line-tables-only"`) but not the full debug symbols, so the `target` folder of a test run is about a tenth of its size.
- **The Offset window moves the real machine** and **the arrow of the file name opens a list of patterns** (`*.PLT`, `*.ZIP`, `*.*`), as in DRILLPRO.

- **Buttons that did nothing now work:** the Load button of the Tool Data and Parameter windows opens the Select File window for the tool files (`*.RTD`) or the configuration files (`*.RCF`) and loads the one chosen; Start and Panelz. without a file say so in a window.

- **Jobs packed in ZIP archives:** the Select File window goes inside a `.zip` file and shows the routing files in it; `routepro JOB.ZIP` loads the first routing file of the archive. The archive code is the shared `eh3d-archive` crate of EH3D-COMMON.

- **The build, build-test and test scripts show a live panel:** a banner with the name and version, one line per step with its time, the number of tests that passed and a summary at the end, with the whole output kept in a log file. `test.sh` and `test.bat` run only the tests and never change the version; without `tools/pretty_run.py` the scripts behave as they always did.

## [0.0.3] - The Offset teach-in, the interrupted-job question, examples and a 32-bit Windows build
- **A basic configuration and examples in every release, and Windows 32 bits:** the packages now carry the machine and tool files with every line explained in plain words, a guide to them in the eight languages, the example connection file and an `examples` folder of HP-GL files from a square to a hundred thousand points; all of it is generated by scripts in `tools/` and read back by a test. The release builds also include Windows 32 bits (not tried on a 32-bit Windows).
- **The operator's dialogs in the classic screens:** the job asks, as the original does, to confirm that the spindle may start (Attention!) and, for every tool, to put it in (Tool Change: the tool, the length of the run, the spindle, the Z speed and the stroke), with Ok and Abort on the keyboard (`Enter`, `Esc`, the letters) and the mouse. The simulation stops there too, so the whole flow can be seen without a machine. Before, a job on a real machine had no way to answer these dialogs from the classic screens (the menu-driven interface did) and would wait for ever.
- **Steps on a half step:** a position that falls exactly half way between two steps now rounds to the even step, as the original does.
- **Vacuum cleaner on the auxiliary output (optional):** `vacuum = aux` in the machine file switches the auxiliary output (latch A bit 5) on with the spindle, at its speed, and off with it. The default, `vacuum = central`, writes exactly what the original software writes on the manual machine (which never touches that bit: a relay wired to the control unit follows its spindle bits).
- **The Offset button works:** it asks "Delete old offset?" (Yes starts at home, No at the offset in the form) and opens the Offset Teach window as the original has it. The head moves in steps of 1, 10, 100 or 1000 mil (0.001 inch) with `E`/`S`/`D`/`X` (Y+, X-, X+, Y-) or the arrow keys, `R`/`C` or `PageUp`/`PageDown` for Z, `1`-`4` for the step, and the mouse on every button. The position is shown in mm and Z as the depth left. **Ok** writes the position into the Offset fields of the main screen. **Teach** starts the Teach-In mode: positions are relative to the point where it was pressed, `+ Pos` records the head position, `- Pos` takes the last one back, `Mod` puts the last one where the head is, and Ok asks "Write Teach-In file?" and writes `TEACH.NCD` (a drill file this program reads). Screens checked against the original.
- **Esc during the job asks what next:** as the original does, `Esc` while drilling or routing opens INTERRUPTED / What next? with Cont. and Abort (keys `Enter`/`C` and `Esc`/`A`, or the mouse). The job waits, with the machine still, until the operator answers; Abort stops it and brings the form back. Before the first move (homing) `Esc` still stops at once.
- **The mouse wheel** scrolls the file list (window and console).
- **The percentage** of the job window counts the holes done, as the original does.
- **Moving through the folders of the computer:** the Select File dialog lists the files, then the folders, then `..\` last, as the original does; it works from the absolute folder (a folder written as `.` could not go up before), shows the drives as `[-C-]` entries on Windows, hides dotted files, ends a long name with `~`, and tells when a folder cannot be opened. A path written in the name field (`D:\jobs\*.NCD`, `..\other`, `C:\`) goes to its folder and takes the rest as the pattern or the file; the name field takes a path of any length. The mouse wheel scrolls the list.
- **The form drives the job:** the vectors `from` and `to` choose the holes (or paths) that are done, the clip leaves out the ones that end outside the working area (counted), and the Panelization dialog's Ok repeats the job over the copies, a tool at a time so that there is one tool change; the copies are `board - spacing` apart, as the original's count works out.
- **The job window shows the job:** the position, the tool, its stroke, the speeds, the count and the drilled depth follow the planned job (the values are the planned ones, not read from the machine).
- **Enter walks the form:** accepting a field goes on to the next one, as in the original.
## [0.0.2] - The classic screens, the mouse, the real files and a console version

- **Console version:** `routepro-console` draws the same screens in a terminal, with the keyboard and the mouse, and needs no window system: it runs over SSH, on a text console and on an old PC, in the MS-DOS spirit of the original. The terminal must be 80 columns by 25 rows or larger; a larger one shows the screen in the middle.
- **Packages:** the release carries, for Windows, Linux and macOS (Apple silicon and Intel), the program with its window and the console version, each with its SHA-256.
- **One command line:** the window and the console read the same options (`--lang`, `--data`, `--connection`, `--modern`, `--dump`).
- **Classic screens (the default):** the screens of the MS-DOS routing program (main form, Parameter with the chord angle, the routing Tool Data table with depth, iterations and both feeds, Select File, Panelization, Quit and the homing window), shared with DRILLPRO through its `classic` front end, in eight languages. `--modern` starts the menu-driven interface as before.
- **Machine file:** `ROUTEPRO.RCF` is read with its own layout (the chord angle where the drilling program keeps its sorting), so the speeds, ramps and corrections of the file are used.
- **Editing and the mouse:** every field takes typing, `Backspace`, `Delete`, `Home`, `End` and the arrow keys (the first key typed replaces the whole value), `Tab` and `Shift+Tab` move between fields, `Enter` accepts and `Esc` cancels. Every field, button, radio button, list entry and dialog button answers to the mouse; a double click opens a file.
- **A window that scales:** it can be maximized or resized; the screen grows from 640x400 up to any larger size (480p, 720p, 1080p and everything between) keeping its proportions, and clicks follow the scaling.
- **Real files:** the Parameter dialog edits the machine configuration and writes it back to the real file (the old one stays as `.BAK`, every comment and reserved line is kept), reloads it, and holds the language; Tool Data edits the tool table and saves it; Select File filters with patterns (`*.NCD`, `T*.PLT`, ...) and lists a folder of any size in pages; the program starts in the folder the configuration names; leaving offers to save the changed settings. Without `--data` the files are looked for in the current folder, the folder of the program and the user's configuration folder.
- **About:** the program, its version, licence and source, and the Bungard machines it knows: CCD and CCD/2 with manual tool change (MTC) supported, with automatic tool changer (ATC) not yet, tried on a CCD MTC only, and a note that the project is independent of Bungard Elektronik.
- **The layout of the README is checked in the CI:** the validator runs `tools/universal_readme_check.py` and lists, as a warning, what differs from the universal layout of the README.

## [0.0.1] - First version: file readers, job planning, text-mode interface and a simulated machine

- **HP-GL reader:** `.PLT` plot files with `IN`, `SP`, `PU`, `PD`, `PA` and `PR`; any other command is counted and ignored. The tool table (`.RTD`: depth, passes, feeds in XY and Z, spindle speed) and the machine configuration (`.RCF`).
- **Path planner:** every path becomes a rapid move to its start and, for each pass, a plunge, the cut along the path and a lift; the depth is split evenly across the passes and the tool ends at the surface.
- **Interface:** an 80x25 text-mode screen with CGA colours, menus, a file dialog and a path view, in eight languages, shared with EH3D-DRILLPRO.
- **Protocol library (`eh3d-routepro-protocol`):** re-exports the protocol library of EH3D-DRILLPRO and adds the routing plan.
- **Simulator:** the whole job runs to the end without hardware.
- **Real machine:** a machine file (`machine.cfg`, shared by DRILLPRO and ROUTEPRO) chooses the simulator, the computer's serial port, a USB serial adapter or the EH3D-NODE-BUNGARD; every tuning value lives in it. `Esc` stops a routing job and switches the drivers off, and a red banner says the machine is real.
- **Reference run:** Z, X and Y to their limit switches with the control unit's "all home" line read once the last search has gone out (reading it in the middle of the stream made the Y motor lose steps on the CH340), a short unstick move when parked, a Z search that covers the whole travel (`z_search_steps`) and a smooth calibration jump. Tried on a Bungard CCD MTC; the CCD/2 MTC is driven the same way and has not been tried. Spindle and vacuum outputs have not been tried; the two ATC models are not covered.
- **Spindle of the node:** `spindle = node` in the machine file makes the job ask the EH3D-NODE-BUNGARD for the spindle (a variable-frequency drive on the node's own outputs, any speed up to the node's maximum) instead of writing the control unit's spindle bits; the spindle is stopped on any stop. The node link also reads the machine's lamps and can ask the node to drop the rest of a search as soon as an axis is home.
- **Server connection:** `connection = server` sends the routing job to the EH3D-SERVER, which plans it with the constants of the machine, runs it through the machine's node and tells the program how it goes; the program shows the progress, answers the dialogs and can stop the job, and closing it does not stop the job.
- **Steady motion:** moves go out as one continuous stream paced by the adapter's UART, Z moves byte by byte.
- **Documentation:** protocol notes, the connection guide with the CH340 findings, and a machine file example with every value.
- 21 tests, `cargo clippy -D warnings` and `cargo fmt --check` in CI on Linux and Windows, and a repository check of the manifest, the versions, the changelog and the eight README files.
- **Ready-made packages for Windows, Linux and macOS:** the `Release binaries` workflow builds the program with its window for Windows (zip), Linux (tar.gz; X11 and Wayland) and macOS (Apple silicon and Intel, tar.gz), each package with its SHA-256, and the release carries them.
