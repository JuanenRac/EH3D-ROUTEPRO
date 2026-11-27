# EH3D-ROUTEPRO: scope

Status: **simulator**. EH3D-ROUTEPRO is a program with a text-mode interface that reads HP-GL plot files, plans multi-pass milling paths and runs them against a simulator or, when the machine file says so, on a Bungard CCD machine. It is tried on a computer and on the simulator; the serial protocol it shares with EH3D-DRILLPRO has moved a real machine, but a whole routing job has not. This page says what it is for, what it will not do and where it sits among the other programs of the suite.

## Purpose

The routing half of the Bungard workflow: it reads the plot of the outline and the cuts, the tool table and the machine file, turns each path into a rapid move, a plunge, the cut and a lift for every pass, and sends the moves to the machine, to the simulator or to the server of the suite. It reproduces the screens of the original MS-DOS program and offers a modern interface and a console version.

## What it does

* Reads HP-GL plot files (`IN`, `SP`, `PU`, `PD`, `PA`, `PR`; any other command is counted and ignored), a DXF drawing or the outline of a Gerber board, and the first routing file of an archive.
* Plans each path with the depth split evenly across the passes and the tool always back at the surface, and plans a cut that follows a levelled table with the depth bounded.
* Runs the job on the simulator by default, and on a real machine through the serial port, a USB adapter or the node when the machine file says so; with a tool changer it takes every tool from its place and never asks the operator for one.
* Leaves out the pens that are switched off, the paths outside the working area and the paths already done when a job goes on, and cuts the outside of a panel at its tabs.
* Draws the classic screens, checked against stored images of the original, in eight languages, and edits and saves the machine file and the tool table from the dialogs.
* Sends the job to the server of the suite and follows its progress when the connection is `server`.

## What it will not do

* No emergency stop of its own: `Esc` stops the job and switches the drivers off, and the control unit of the machine watches the limit switches (see below).
* No drilling: the drilling program is EH3D-DRILLPRO, whose protocol and machine file this program shares.
* No claim that the steps per millimetre are right for a machine: they are only checked against the original configuration.
* No toolpath for anything but the paths of the plot: it does not offset a path by the radius of the cutter.

## Where it sits

* **It reads:** HP-GL plot files, DXF drawings, Gerber outlines, the tool table `ROUTEPRO.RTD` and the machine file `ROUTEPRO.RCF` (the formats of the drilling program), and the answers of the machine or of the server.
* **It writes:** the machine file and the tool table when they are edited from the dialogs, the bytes of the job to the serial port or the server, and nothing over a plot file.
* **It is read by:** EH3D-BUNGARD-GUI, which runs its engine for the routing half of a job, and EH3D-SERVER, which receives the jobs it sends.
* **It depends on:** `EH3D-DRILLPRO` (the protocol, the machine file and the screens), built next to it as a sibling folder, and the protocol crate of this repository.

## What it never decides

The limits of a machine, its soft limits and its emergency stop belong to the machine and to its control unit, not to this program. A planned job is a job written cleanly, not a verified one: nothing here knows where the machine stops, so a person must check the plan and keep a hand on the power switch of the machine.

The requirements of the program, with the state of each and the test that shows it, are in `REQUIREMENTS.md`; the state in a paragraph is in `STATUS.md`.
