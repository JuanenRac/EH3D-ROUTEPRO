# The machine protocol

EH3D-ROUTEPRO talks to the control unit of the machine with exactly the same serial protocol as EH3D-DRILLPRO: the same link, handshake, input lines and output bytes. What is known about it, what has been confirmed on a machine and what is still a hypothesis is written once, in [`docs/PROTOCOL.md` of EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO/blob/main/docs/PROTOCOL.md). The machine file, the three connections (the computer's serial port, a USB serial adapter and the EH3D-NODE-BUNGARD) and every tuning value are in [`docs/CONNECTION.md` of EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO/blob/main/docs/CONNECTION.md); the file is shared by both programs. Read them before connecting anything. With a `node` connection ([EH3D-NODE-BUNGARD](https://github.com/JuanenRac/EH3D-NODE-BUNGARD)) a routing job can also use the node's own spindle (`spindle = node`, a variable-frequency drive, any speed up to the node's maximum) and the machine's lamps, as the drilling job does; see "What the node adds" in the first document.

**Status: the protocol was tried on one machine (a CCD MTC), but a routing job has not run on it yet.** The reference run of the three axes and the moves of every axis were checked step by step; the routing plan below has only run on the simulator.

## What routing adds

The `eh3d-routepro-protocol` crate re-exports the protocol crate of EH3D-DRILLPRO and adds one thing, the routing plan (`route`): a rapid move to the start of a path and, for every pass, a plunge, the cut along the path and a lift. The depth is split evenly across the passes and the tool always ends at the surface. The plan goes through the same safety gate, which refuses to move anything until the machine has been armed.

For a real machine the plan is wrapped in the same job as a drilling one: the drivers on, the reference run of Z, X and Y, the operator's dialogs for each tool, the spindle, the paths, and the way back to the origin. The plan's zero is the travel height, the free stroke above the surface of the board, so the plunge goes through it first and then the depth into the board.

The depth, number of passes, feed rates in XY and Z and spindle speed of each tool come from the tool table; HP-GL plot files give the paths (0.025 mm per plotter unit, origin at the lower left), shifted by the offset of the machine configuration.

## Open points

- A whole routing job on a real machine, and the steps per millimetre of Z, so lengths and depths of a real cut are not confirmed yet.
- Tool compensation (the width of the cutter) is not applied: the path is followed as drawn.
- The limit switches are watched by the control unit, and `Esc` stops the job and switches the stepper drivers off; this software has no emergency stop of its own.
