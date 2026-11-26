//! The `.eh3d` project of the suite: the outline layers are read from the job of the project (the branch `manufacturing/`) and the routing plan made on paper from one of them is kept in
//! the branch `routepro/` only, with a copy of the layer and a reference to the content of the job, so that a job that changes makes the plans **out of date**. The reading, the writing and
//! the references are the ones every manufacturing program shares (`eh3d_project::manufacturing` of EH3D-COMMON), and a window keeps its layer as DRILLPRO does. A plan is a text: no file
//! for the machine is made and no machine is needed.

use std::path::Path;

use eh3d_drillpro::eh3dfile::plan_bytes;
use eh3d_project::appbranch::BranchKit;
use eh3d_project::manufacturing::{self as suite, JobFile};

/// The branch of this program.
pub const KIT: BranchKit<'static> = suite::kit(
    "routepro",
    "routepro.json",
    env!("CARGO_PKG_VERSION"),
);

/// The outline layers of the job of a project; an error when the job has none.
pub fn outline_layers(path: &Path) -> Result<Vec<JobFile>, String> {
    suite::layers_of(path, "outline")
}

/// The outline layer of the job called `layer` (its path inside the job or its file name).
pub fn choose<'a>(layers: &'a [JobFile], layer: &str) -> Result<&'a JobFile, String> {
    suite::choose(layers, "outline", layer)
}

/// The name of the plan of a layer inside `result/`: `route-<name of the layer>.txt`.
pub fn result_name(layer: &JobFile) -> String {
    suite::result_name("route", layer, "txt")
}

/// Keeps the plan of one layer in the branch of this program. The plans of the other layers stay while the job is the one they were made from. Returns what a person should be told.
pub fn save_plan(path: &Path, layer: &JobFile, lines: &[String]) -> Result<Vec<String>, String> {
    suite::keep_result(&KIT, path, layer, &result_name(layer), plan_bytes(lines))
}
