# EH3D-ROUTEPRO: status

## Current state

EH3D-ROUTEPRO works on a computer and on the simulator: the readers, the planner, the job, the changer, the screens in eight languages and the connection to the server are covered by tests. The serial protocol it shares with EH3D-DRILLPRO has been tried on one Bungard CCD MTC (the reference run and the moves of each axis), but a whole routing job has not run on a machine, the spindle and vacuum outputs are untried and the steps per millimetre are only checked against the original configuration.

## Capabilities

The official state of each capability is in `REQUIREMENTS.md`. The register has 48 requirements: 38 implemented, 5 prototype, 5 planned and 0 validated. By progress: 38 done, 5 partial, 1 pending and 4 blocked. This page never states a state that `REQUIREMENTS.md` does not.

## Open questions

* Whether the plan should offset a path by the radius of the cutter, or leave that to the design.
* Whether the levelled plan should read the height map of the table from the server.

## What it needs from other repositories

* `EH3D-DRILLPRO`: the protocol, the machine file and the screens, as a sibling folder at the version named in the workflows.
* A Bungard CCD machine and a person at it, for any routing job that is not simulated.
* `EH3D-SERVER`, for the connection `server`; that path has only been exercised against the simulator.

## Known technical debt and omitted tests

* No plot from a real design has been read; the readers were tried on small files written for the tests and on the examples of the release.
* The spindle and vacuum outputs have never been switched on a real machine.
* The text-mode screens are compared with stored images of the original, not with the original program on a screen.
