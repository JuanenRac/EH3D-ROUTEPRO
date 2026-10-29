#!/usr/bin/env python3
"""Writes the example HP-GL files of the release into examples/, from the simplest to the largest.

The files are generated, not copied from anywhere: run `python3 tools/make_examples.py` and commit the result. One plotter unit is
0.025 mm (40 units per millimetre), the unit of the default configuration; everything is inside the 495 x 325 mm work area.
"""
import math
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "examples")
MM = 40  # plotter units per mm


def write(name, pens, note):
    """pens: {pen: [path, ...]} with a path a list of (x, y) in plotter units; written in pen order."""
    lines = ["IN;"]
    count = 0
    for pen, paths in pens.items():
        lines.append(f"SP{pen};")
        for path in paths:
            x0, y0 = path[0]
            lines.append(f"PU{int(round(x0))},{int(round(y0))};")
            rest = path[1:]
            for i in range(0, len(rest), 20):
                chunk = ",".join(f"{int(round(x))},{int(round(y))}" for x, y in rest[i : i + 20])
                lines.append(f"PD{chunk};")
            count += len(path)
    lines.append("PU;SP0;")
    with open(os.path.join(OUT, name), "w", newline="\r\n") as f:
        f.write("\n".join(lines) + "\n")
    return name, count, len(pens), note


def rect(x, y, w, h):
    return [(x, y), (x + w, y), (x + w, y + h), (x, y + h), (x, y)]


def circle(cx, cy, r, n):
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n + 1)]


def rounded(x, y, w, h, r, seg=8):
    pts = []
    for cx, cy, a0 in ((x + w - r, y + r, -90), (x + w - r, y + h - r, 0), (x + r, y + h - r, 90), (x + r, y + r, 180)):
        for i in range(seg + 1):
            a = math.radians(a0 + 90 * i / seg)
            pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    pts.append(pts[0])
    return pts


def gear(cx, cy, r_out, r_in, teeth):
    pts = []
    for t in range(teeth):
        base = 2 * math.pi * t / teeth
        step = 2 * math.pi / teeth
        for frac, r in ((0.0, r_in), (0.25, r_out), (0.5, r_out), (0.75, r_in)):
            a = base + step * frac
            pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    pts.append(pts[0])
    return pts


def spiral(cx, cy, turns, n, r_max):
    return [(cx + r_max * (i / n) * math.cos(2 * math.pi * turns * i / n), cy + r_max * (i / n) * math.sin(2 * math.pi * turns * i / n)) for i in range(n + 1)]


def lissajous(cx, cy, ax, ay, n, fa, fb):
    return [(cx + ax * math.sin(fa * 2 * math.pi * i / n), cy + ay * math.sin(fb * 2 * math.pi * i / n + 0.5)) for i in range(n + 1)]


def main():
    os.makedirs(OUT, exist_ok=True)
    rows = []
    rows.append(write("01_square.plt", {1: [rect(2000, 2000, 40 * MM, 40 * MM)]}, "a 40 mm square: the smallest job"))
    rows.append(write("02_two_pens.plt", {1: [rect(2000, 2000, 80 * MM, 50 * MM)], 2: [rect(2000 + 10 * MM, 2000 + 10 * MM, 60 * MM, 30 * MM)]}, "two pens, two tools: an outline and an inner slot"))
    rows.append(write("03_circle.plt", {1: [circle(4000, 4000, 20 * MM, 72)]}, "a 40 mm circle of 72 segments"))
    rows.append(write("04_rounded_rectangle.plt", {1: [rounded(2000, 2000, 100 * MM, 60 * MM, 8 * MM)]}, "a rectangle with rounded corners"))
    board = rect(2000, 2000, 100 * MM, 80 * MM)
    holes = [circle(2000 + 6 * MM + dx, 2000 + 6 * MM + dy, 1.6 * MM, 16) for dx in (0, 88 * MM) for dy in (0, 68 * MM)]
    rows.append(write("05_board_outline.plt", {1: [board], 2: holes}, "a board outline with four mounting holes cut with a second tool"))
    rows.append(write("06_gear_24_teeth.plt", {1: [gear(6000, 6000, 40 * MM, 34 * MM, 24)], 2: [circle(6000, 6000, 8 * MM, 48)]}, "a gear with a bore"))
    rows.append(write("07_spiral_2000_points.plt", {1: [spiral(8000, 6000, 12, 2000, 120 * MM)]}, "a spiral of 2000 points: many short segments"))
    rows.append(write("08_long_path_20000_points.plt", {1: [lissajous(9000, 6000, 8000, 5500, 20000, 7, 9)]}, "one path of 20000 points over most of the table"))
    rows.append(write("09_huge_100000_points.plt", {1: [lissajous(9000, 6000, 8200, 5800, 100000, 23, 29)], 2: [spiral(9000, 6000, 40, 20000, 150 * MM)]}, "the largest: 120000 points in two tools"))

    with open(os.path.join(OUT, "README.txt"), "w", newline="\r\n") as f:
        f.write("Example HP-GL files of ROUTEPRO (one plotter unit is 0.025 mm)\r\n\r\n")
        f.write("Load them with the Load button. They are listed from the simplest to the largest.\r\n")
        f.write("Start with the first ones, in the simulator (the default), before sending any job to a machine.\r\n\r\n")
        for name, count, npens, note in rows:
            f.write(f"{name:<32} {count:>7} points, {npens} pen(s): {note}\r\n")
    for r in rows:
        print(r)


if __name__ == "__main__":
    main()
