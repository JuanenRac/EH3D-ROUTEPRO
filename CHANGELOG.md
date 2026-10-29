# Changelog

All notable changes to this project are documented here.

## [Unreleased]

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
