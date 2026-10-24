//! Tool table (`ROUTEPRO.RTD`): `nn depth iterations feed_xy feed_z spindle`, data from line 20.

#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    pub number: u8,
    /// Total routing depth in mm.
    pub depth_mm: f64,
    /// Number of passes the depth is divided into.
    pub iterations: u32,
    pub feed_xy_mm_min: u32,
    pub feed_z_mm_min: u32,
    pub spindle_rpm: u32,
}

pub fn parse(text: &str) -> Vec<Tool> {
    text.lines()
        .skip(19)
        .filter_map(|l| {
            let mut it = l.split_whitespace();
            Some(Tool {
                number: it.next()?.parse().ok()?,
                depth_mm: it.next()?.parse().ok()?,
                iterations: it.next()?.parse().ok()?,
                feed_xy_mm_min: it.next()?.parse().ok()?,
                feed_z_mm_min: it.next()?.parse().ok()?,
                spindle_rpm: it.next()?.parse().ok()?,
            })
        })
        .collect()
}
