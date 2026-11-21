# AGENTS.md: rules for anyone who changes EH3D-ROUTEPRO

EH3D-ROUTEPRO: the routing program of the Bungard CCD workflow: it reads plot files, plans the cuts and runs them on a simulator or on a machine. Software licence: GPL-3.0-or-later; every source file starts with its header.

## 1. Before you start

1. Read `docs/STATUS.md`: what exists, what is missing, what the last change decided.
2. Read `docs/SCOPE.md` (what the program is for and what it never decides) and `docs/REQUIREMENTS.md` (the register of requirements and the test that shows each one).
3. Read `docs/NOT_YET.md` before adding an idea: it may be there on purpose.

## 2. Build and test

- The repository builds next to `EH3D-DRILLPRO` (a sibling folder); the version it needs is in the workflows and in `eh3d.project.json`.
- Run before every commit, with the same flags as the CI: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --workspace`, `python -I tools/ci_validate.py` and `python tools/make_structure.py --check --readmes`.
- A heavy build runs in the CI, not on a machine that is short of memory; a change that adds a file adds it to `docs/structure.json` and regenerates the tree and the picture.

## 3. Code rules

- A reader never panics and never guesses. What it cannot understand is an error that names what and, when it has one, the number of its line.
- Every input file is hostile until it is shown otherwise: sizes, counts and number formats are checked before use, and everything is bounded.
- No `unwrap` or `expect` in code that handles an external value. No `unsafe`.
- A rule of safety only ever makes a job refused or a warning louder. A change that would relax one after a trouble is a defect: the tests that fail for it must stay.
- Every value that is tuned (a limit, a tolerance, a time) is a setting in a configuration file, not a constant in the code; every option of security has a switch with the conservative value as its default.
- Output carries no date, host name or user name that the person did not give.

## 4. What the program must not do

- It does not decide that a job is safe to run, and it does not replace the emergency stop or the limit switches of the machine.
- It does not write over a plot file, and it does not send a byte to a machine unless the machine file says so.
- Its documents and its code do not name anything private: no dates, no internal notes, no names of people or of tools that wrote the change.

## 5. Evidence

- A state in `docs/REQUIREMENTS.md` is `validated` only when a person has run the capability on the real machine it names. Nothing is green that has never run on the real thing.
- A requirement names a test that exists; when a requirement changes state, the row and `docs/STATUS.md` change in the same commit, and `tests/reqs.rs` says whether they agree.

## 6. Documents and languages

- The README exists in eight languages (English, Spanish, German, French, Italian, Japanese, Chinese and Portuguese of Portugal) and every word of the window and of the messages exists in the eight where the program has them.
- The file tree and the pictures are generated from `docs/structure.json` and `docs/architecture.json`; they are not edited by hand. The count of tests in the READMEs is the real one.
- Each change has a line in the `[Unreleased]` part of `CHANGELOG.md`, in English.

## 7. Clean room

- Ideas, flows and behaviours that can be seen from outside may be taken from other programs; their code, their texts, their files and their wording are not copied.
- Formats are learned from public specifications and from files of this project. The files of another program are read, never written.

## 8. Reporting

- A problem with a part of the program is reported with a small file that shows it. Real boards and real machine files are private: do not paste them into a report.
