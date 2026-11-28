//! The path of the centre of the cutter: a closed path moved by the radius of the cutter, so the edge of the tool, and not its middle, follows the line
//! drawn. Off unless the machine file gives the diameter (`cutter_mm`).
//!
//! The outline of a board is moved outwards and a cut-out inside it inwards (a path inside a path inside a path outwards again, and so on). A path that
//! is not closed has no inside and stays as drawn. The result is checked point by point: where the cutter does not fit (a slot narrower than the tool,
//! a notch it cannot enter) the whole job is refused rather than cut something else.

use crate::hpgl::{Polyline, UNIT_MM};

type Pt = (f64, f64);

/// How far, in mm, a drawn circle may be from the true one and a point may be closer than the radius (the rounding of the plot's units included).
const TOLERANCE_MM: f64 = 0.02;

/// The cutter of a pen does not fit in one of its paths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoesNotFit {
    pub pen: u8,
    pub diameter_mm: f64,
}

/// `paths` with every closed one moved by half the diameter `diameter_of` gives its pen (0 leaves it as drawn).
pub fn offset_paths(
    paths: &[Polyline],
    diameter_of: impl Fn(u8) -> f64,
) -> Result<Vec<Polyline>, DoesNotFit> {
    let rings: Vec<Option<Vec<Pt>>> = paths.iter().map(|p| ring_of(&p.points)).collect();
    paths
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let diameter = diameter_of(path.pen);
            let Some(ring) = rings[i].as_ref().filter(|_| diameter > 0.0) else {
                return Ok(path.clone());
            };
            // how many of the other closed paths this one lies inside
            let depth = rings
                .iter()
                .enumerate()
                .filter(|(j, other)| {
                    *j != i
                        && other.as_ref().is_some_and(|o| {
                            area(o).abs() > area(ring).abs() && ring.iter().all(|&p| inside(o, p))
                        })
                })
                .count();
            let moved = offset_ring(ring, diameter / 2.0, depth % 2 == 0).ok_or(DoesNotFit {
                pen: path.pen,
                diameter_mm: diameter,
            })?;
            let mut points: Vec<(i32, i32)> = moved
                .iter()
                .map(|&(x, y)| ((x / UNIT_MM).round() as i32, (y / UNIT_MM).round() as i32))
                .collect();
            points.dedup();
            if let Some(&first) = points.first() {
                if points.last() != Some(&first) {
                    points.push(first);
                }
            }
            Ok(Polyline {
                pen: path.pen,
                points,
            })
        })
        .collect()
}

/// The corners in mm of a closed path (without the repeated last one), or `None` for a path that is open or encloses nothing.
fn ring_of(points: &[(i32, i32)]) -> Option<Vec<Pt>> {
    if points.len() < 4 || points.first() != points.last() {
        return None;
    }
    let mut ring: Vec<(i32, i32)> = points[..points.len() - 1].to_vec();
    ring.dedup();
    let ring: Vec<Pt> = ring
        .iter()
        .map(|&(x, y)| (f64::from(x) * UNIT_MM, f64::from(y) * UNIT_MM))
        .collect();
    (ring.len() >= 3 && area(&ring).abs() > 1e-9).then_some(ring)
}

/// The signed area of the ring, positive when the corners go counterclockwise.
fn area(ring: &[Pt]) -> f64 {
    let n = ring.len();
    (0..n)
        .map(|i| {
            let (a, b) = (ring[i], ring[(i + 1) % n]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        / 2.0
}

fn inside(ring: &[Pt], p: Pt) -> bool {
    let n = ring.len();
    let mut inside = false;
    for i in 0..n {
        let (a, b) = (ring[i], ring[(i + 1) % n]);
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) / (b.1 - a.1) * (b.0 - a.0) {
            inside = !inside;
        }
    }
    inside
}

fn distance_to_ring(ring: &[Pt], p: Pt) -> f64 {
    let n = ring.len();
    (0..n)
        .map(|i| {
            let (a, b) = (ring[i], ring[(i + 1) % n]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
            (p.0 - a.0 - t * dx).hypot(p.1 - a.1 - t * dy)
        })
        .fold(f64::MAX, f64::min)
}

/// The ring moved by `radius` outwards (`grow`) or inwards, in the direction the original goes, or `None` where the circle of that radius does not fit.
fn offset_ring(ring: &[Pt], radius: f64, grow: bool) -> Option<Vec<Pt>> {
    let clockwise = area(ring) < 0.0;
    let mut r: Vec<Pt> = ring.to_vec();
    if clockwise {
        r[1..].reverse();
    }
    let n = r.len();
    let side = if grow { 1.0 } else { -1.0 };
    let direction = |i: usize| {
        let (a, b) = (r[i], r[(i + 1) % n]);
        let len = (b.0 - a.0).hypot(b.1 - a.1);
        ((b.0 - a.0) / len, (b.1 - a.1) / len)
    };
    // the step of the arcs that keeps them within the tolerance of the true circle
    let step = 2.0 * (1.0 - (TOLERANCE_MM / 2.0 / radius).min(1.0)).acos();
    let mut out: Vec<Pt> = Vec::new();
    for i in 0..n {
        let v = r[i];
        let (d1, d2) = (direction((i + n - 1) % n), direction(i));
        let (n1, n2) = ((side * d1.1, -side * d1.0), (side * d2.1, -side * d2.0));
        let turn = d1.0 * d2.1 - d1.1 * d2.0;
        if side * turn > 1e-12 {
            // the two moved sides leave a gap at this corner: an arc round it
            let a1 = n1.1.atan2(n1.0);
            let mut sweep = n2.1.atan2(n2.0) - a1;
            while sweep > std::f64::consts::PI {
                sweep -= std::f64::consts::TAU;
            }
            while sweep < -std::f64::consts::PI {
                sweep += std::f64::consts::TAU;
            }
            let pieces = (sweep.abs() / step).ceil().max(1.0) as usize;
            for k in 0..=pieces {
                let a = a1 + sweep * k as f64 / pieces as f64;
                out.push((v.0 + radius * a.cos(), v.1 + radius * a.sin()));
            }
        } else {
            // they cross: the corner is where they meet
            let k = 1.0 + n1.0 * n2.0 + n1.1 * n2.1;
            if k < 1e-6 {
                return None;
            }
            out.push((
                v.0 + radius * (n1.0 + n2.0) / k,
                v.1 + radius * (n1.1 + n2.1) / k,
            ));
        }
    }
    // every point of the way of the centre keeps the radius from the line drawn, on the side asked for
    let m = out.len();
    let sample = (radius / 4.0).clamp(0.02, 0.2);
    for i in 0..m {
        let (a, b) = (out[i], out[(i + 1) % m]);
        let pieces = ((b.0 - a.0).hypot(b.1 - a.1) / sample).ceil().max(1.0) as usize;
        for k in 0..pieces {
            let t = k as f64 / pieces as f64;
            let p = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
            if inside(&r, p) == grow || distance_to_ring(&r, p) < radius - TOLERANCE_MM {
                return None;
            }
        }
    }
    if area(&out) <= 0.0 {
        return None;
    }
    if clockwise {
        out[1..].reverse();
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A closed path through these corners, in mm.
    fn path(pen: u8, corners: &[(f64, f64)]) -> Polyline {
        let mut points: Vec<(i32, i32)> = corners
            .iter()
            .map(|&(x, y)| ((x / UNIT_MM).round() as i32, (y / UNIT_MM).round() as i32))
            .collect();
        points.push(points[0]);
        Polyline { pen, points }
    }

    fn square(pen: u8, x: f64, y: f64, side: f64) -> Polyline {
        path(
            pen,
            &[(x, y), (x + side, y), (x + side, y + side), (x, y + side)],
        )
    }

    /// The box of a path, in mm.
    fn bounds(p: &Polyline) -> (f64, f64, f64, f64) {
        p.points
            .iter()
            .fold((f64::MAX, f64::MAX, f64::MIN, f64::MIN), |b, &(x, y)| {
                let (x, y) = (f64::from(x) * UNIT_MM, f64::from(y) * UNIT_MM);
                (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y))
            })
    }

    fn near(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) {
        for (x, y) in [(a.0, b.0), (a.1, b.1), (a.2, b.2), (a.3, b.3)] {
            assert!((x - y).abs() <= 0.03, "{a:?} is not {b:?}");
        }
    }

    #[test]
    fn the_cut_is_offset_by_the_radius_of_the_cutter() {
        // a board of 40 mm with a window of 10 mm inside it, cut with a 2 mm cutter
        let paths = [square(1, 0.0, 0.0, 40.0), square(1, 10.0, 10.0, 10.0)];
        let moved = offset_paths(&paths, |_| 2.0).unwrap();
        near(bounds(&moved[0]), (-1.0, -1.0, 41.0, 41.0));
        near(bounds(&moved[1]), (11.0, 11.0, 19.0, 19.0));
        // the corners of the outline are rounded, those of the window are sharp
        assert!(moved[0].points.len() > 10);
        assert_eq!(moved[1].points.len(), 5);
        assert_eq!(moved[0].points.first(), moved[0].points.last());
    }

    #[test]
    fn no_cutter_and_open_paths_stay_as_drawn() {
        let open = Polyline {
            pen: 2,
            points: vec![(0, 0), (400, 0), (400, 400)],
        };
        let paths = [square(1, 0.0, 0.0, 40.0), open];
        let same = offset_paths(&paths, |pen| if pen == 2 { 2.0 } else { 0.0 }).unwrap();
        assert_eq!(same, paths);
    }

    #[test]
    fn the_direction_of_the_cut_is_kept() {
        let clockwise = path(1, &[(0.0, 0.0), (0.0, 20.0), (20.0, 20.0), (20.0, 0.0)]);
        let moved = offset_paths(std::slice::from_ref(&clockwise), |_| 3.0).unwrap();
        let ring = ring_of(&moved[0].points).unwrap();
        assert!(area(&ring) < 0.0, "still clockwise");
        near(bounds(&moved[0]), (-1.5, -1.5, 21.5, 21.5));
    }

    #[test]
    fn a_cutter_that_does_not_fit_refuses_the_job() {
        // a window of 2 mm and a cutter of 3 mm
        let paths = [square(1, 0.0, 0.0, 40.0), square(4, 10.0, 10.0, 2.0)];
        assert_eq!(
            offset_paths(&paths, |_| 3.0),
            Err(DoesNotFit {
                pen: 4,
                diameter_mm: 3.0
            })
        );
        // a notch of 1 mm in the outline that a 2 mm cutter cannot enter
        let notched = path(
            1,
            &[
                (0.0, 0.0),
                (20.0, 0.0),
                (20.0, 20.0),
                (11.0, 20.0),
                (11.0, 10.0),
                (10.0, 10.0),
                (10.0, 20.0),
                (0.0, 20.0),
            ],
        );
        assert!(offset_paths(std::slice::from_ref(&notched), |_| 2.0).is_err());
        // and a cutter that does enter it
        assert!(offset_paths(&[notched], |_| 0.8).is_ok());
    }
}
