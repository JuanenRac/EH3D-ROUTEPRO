#!/usr/bin/env python3
"""Writes the basic configuration of the release into config/: ROUTEPRO.RCF (the machine) and ROUTEPRO.RTD (the tools).

Every parameter is explained in plain words after its value. The files are plain text in the layout the program reads; run
`python3 tools/make_config.py` and commit the result. config/machine.cfg.example (the connection to a real machine) is the same one
DRILLPRO ships and is kept as it is.
"""
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "config")

RCF = [
    ("0000001", "SERIAL NUMBER of your machine. Only a label shown on the main screen; write anything up to 12 characters."),
    ("EH3D.000000A", "VERSION label of this file. Only a label; leave it."),
    (r".\examples", "FOLDER where the Load button looks for your HP-GL files first. Relative to the folder you start the program from."),
    ("*.PLT", "FILE PATTERN of the Load list. *.PLT shows the plotter files. The signs * and ? stand for any letters."),
    ("1.0000", "SCALE X of the job. 1.0000 = the drawing as it is; 1.0010 makes it 0.1 percent bigger. From 0 to 10."),
    ("1.0000", "SCALE Y of the job (same as X)."),
    ("02.50", "OFFSET X in mm: how far from the home corner the drawing starts. Typical 2.50. From -495 to 495. The Offset button sets it by moving the head."),
    ("02.50", "OFFSET Y in mm (same as X). From -325 to 325."),
    ("495.00", "CLIPPING X in mm: the usable length of your table in X. Nothing farther than this is ever cut. Set it to your real table (CCD 495, CCD/2 325)."),
    ("325.00", "CLIPPING Y in mm: the usable width of your table in Y (CCD 325, CCD/2 270)."),
    ("1.00", "BOARD THICKNESS in mm. The cutter goes through the board; the depth of each tool comes from the tool table."),
    ("12.00", "BASE SHEET THICKNESS in mm: the sacrificial plate under the board."),
    ("4000", "XY SPEED, FASTEST (mm/min) for the moves without cutting. If the motors lose steps or stall, lower it (try 2500)."),
    ("1000", "XY SPEED of the REFERENCE run (mm/min): the slow search for the home switches. Leave it."),
    ("1000", "XY START SPEED of the ramp (mm/min). Leave it."),
    ("5000", "Z SPEED, FASTEST (mm/min): the quick rise of the cutter. Lower it if Z stalls."),
    ("1000", "Z SPEED of the REFERENCE run (mm/min): the slow search of the Z home switch. Leave it."),
    ("1000", "Z START SPEED of the ramp (mm/min). Leave it."),
    ("10", "ACCELERATION of X and Y (how gently they speed up). If the motors stall at the start of a move, try 5."),
    ("20", "ACCELERATION of Z."),
    ("0.9992", "CORRECTION X: calibration so that 100 mm cut are 100 mm. Cut two marks 200 mm apart, measure, multiply this by wanted/measured. 1.0000 = none."),
    ("0.9998", "CORRECTION Y (same as X)."),
    ("0.8988", "CORRECTION Z (the same for the depth). Leave it unless you measured a depth error."),
    ("37.30", "MAXIMUM Z DEPTH in mm: the whole travel of the Z axis from its top switch. Too large drives the cutter into the table."),
    ("16.00", "DISTANCE TOOL TO BOARD in mm: how far above the board the quick approach stops and the slow plunge begins."),
    ("0.50", "SPINDLE START TIME: seconds to wait for each 10000 rpm for the spindle to reach its speed after it starts."),
    ("2", "UNIT of the numbers in your HP-GL files: 0 = 1/10 mil, 1 = mil, 2 = 0.025 mm (the plotter step, the usual one), 3 = 0.01 mm. A wrong unit scales the whole drawing."),
    ("05", "HP-GL CHORD ANGLE in degrees (1 to 60, with a minus sign for the other direction): how finely circles are cut into straight pieces. Smaller = rounder and slower."),
    ("0 0", "RESERVED. Do not change."),
    ("4", "CONNECTION: 0 = COM1, 1 = COM2, 2 = COM3, 3 = COM4, 4 = NO MACHINE (the simulator: safe, try anything). A real machine is chosen in the machine file."),
    ("2", "SERIAL SPEED: 0 = 9600 baud, 1 = 19200, 2 = 38400 (the one the control unit uses)."),
]

SECTIONS = {
    0: ["* ROUTEPRO machine configuration, explained line by line.",
        "* Each line is a value, some spaces, a * and what it means. Change only the value, keep the * and the text after it.",
        "* Save a copy of this file before you change anything. The decimal separator is a point.",
        "*"],
    4: ["* MAIN SCREEN"],
    12: ["* MACHINE CONSTANTS"],
    26: ["* UNITS"],
    29: ["* SERIAL LINE"],
}

RTD = [
    "Basic tool table of EH3D-ROUTEPRO",
    "",
    "Written for this program; it is not a copy of any other file.",
    "",
    "Table of tool data. 15 lines of data begin at line 20; lines 1 to 19 are comments like these.",
    "Each data line is six numbers separated by spaces, with a point for decimals:",
    "",
    "nn   d.d   ii   xxxx   zzzz   sssss",
    "",
    "nn    = the tool number: pen SP1 of the HP-GL file cuts with tool 01, SP2 with tool 02, and so on",
    "d.d   = total depth to cut in mm (1.6 goes through a 1.6 mm board)",
    "ii    = passes: the depth is reached in that many passes (2 or 3 is gentler on a small cutter)",
    "xxxx  = feed of the cutter along the path in mm/min (500 to 1000 for FR4, slower for a small cutter)",
    "zzzz  = feed of the plunge in mm/min",
    "sssss = spindle speed in turns per minute: 30000 to 60000 on the CCD",
    "",
    "Change the numbers, never the number of lines. Too fast a feed breaks cutters; try on a scrap board first.",
    "",
    "Delimiter between the numbers: one or more spaces.",
    "01 1.8 02 0800 0300 60000",
    "02 1.8 02 0800 0300 60000",
    "03 1.8 02 0600 0300 60000",
    "04 1.0 01 0500 0200 60000",
    "05 1.0 01 0500 0200 60000",
    "06 0.5 01 0500 0200 60000",
    "07 0.5 01 1000 0200 30000",
    "08 0.5 01 1000 0200 30000",
    "09 0.5 01 1000 0100 30000",
    "10 0.5 01 1000 0100 30000",
    "11 0.5 01 1000 0100 30000",
    "12 0.5 01 1000 0100 30000",
    "13 0.5 01 1000 0100 30000",
    "14 0.5 01 1000 0100 30000",
    "15 0.5 01 1000 0100 30000",
]


def write(name, lines):
    with open(os.path.join(OUT, name), "w", newline="\r\n", encoding="latin-1") as f:
        f.write("\n".join(lines) + "\n")


def main():
    os.makedirs(OUT, exist_ok=True)
    lines = []
    for i, (value, text) in enumerate(RCF):
        for c in SECTIONS.get(i, []):
            lines.append(" " * 14 + c)
        lines.append(f"{value:<14}* {text}")
    lines.append(" " * 14 + "* End of the configuration file.")
    write("ROUTEPRO.RCF", lines)
    assert len(RTD) == 19 + 15, len(RTD)
    write("ROUTEPRO.RTD", RTD)
    print("config written:", sorted(os.listdir(OUT)))


if __name__ == "__main__":
    main()
