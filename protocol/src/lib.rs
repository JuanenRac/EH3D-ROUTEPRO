//! Routing (milling) layer of the Bungard CCD protocol.
//!
//! The byte link, motion primitives and the safety gate are the same as for drilling and are
//! re-exported from `eh3d-drillpro-protocol`; this crate adds the planning of routing paths.
//! No GUI and no file formats, so it can be embedded in a combined drill-and-route application.

pub use eh3d_drillpro_protocol::*;

pub mod route;
