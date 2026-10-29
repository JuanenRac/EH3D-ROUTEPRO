//! ROUTEPRO: HP-GL routing front end for the Bungard CCD machine.
//! The serial protocol, text-mode screen and language support are shared with `eh3d-drillpro`.

pub mod app;
pub use eh3d_bungard_formats::hpgl;
pub mod job;
pub mod texts;
pub use eh3d_bungard_formats::route_tools as tools;
