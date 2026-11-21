# EH3D-ROUTEPRO: not yet

Good ideas that are not in the current version. Writing them here keeps anybody from implementing them early or twice.

| Idea | Why not yet | Revisit when |
|---|---|---|
| Offsetting a path by the radius of the cutter | The plan cuts along the path as drawn, and the design is expected to put the path where the centre of the cutter goes | the design programs of the suite offer the offset and a test compares it with a known case |
| A whole routing job on a machine | Only the serial protocol has moved a real machine; a job needs a board, a cutter and a person who is ready to stop it | a person runs the first job on a real machine |
| Height map of the table read from the server | The levelled plan takes a map that the caller gives; the server keeps maps, but no probe has measured a real table | a probe measures a real table and the server stores the map |
| Arcs and curves of the plot | Only the straight moves of HP-GL are understood and the other commands are ignored | a real plot with arcs is given and a test compares the result |
| Cutting several boards in one job without a panel | A panel repeats one board; separate boards are separate jobs | a person asks for it with a real example |
| Packages with the classic programs inside | The release carries this program alone, and the drilling program is another repository | the suite has a single package for the Bungard workflow |
