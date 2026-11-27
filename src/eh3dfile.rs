//! The `.eh3d` project of the suite: the outline layers are read from the job of the project (the branch `manufacturing/`) and the routing plan made on paper from one of them is kept in
//! the branch `routepro/` only, with a copy of the layer and a reference to the content of the job, so that a job that changes makes the plans **out of date**. The reading, the writing and
//! the references are the ones every manufacturing program shares (`eh3d_project::manufacturing` of EH3D-COMMON), and a window keeps its layer as DRILLPRO does. A plan is a text: no file
//! for the machine is made and no machine is needed.

use std::path::Path;

use eh3d_drillpro::eh3dfile::plan_bytes;
use eh3d_project::appbranch::BranchKit;
use eh3d_project::manufacturing::{self as suite, JobFile};
use eh3d_project::suiteview;

/// The branch of this program.
pub const KIT: BranchKit<'static> =
    suite::kit("routepro", "routepro.json", env!("CARGO_PKG_VERSION"));

/// What keeping a plan in the project at `path` would take away (its signature, or the one of the branch of this program), in the language of a code such as `es`: the person has to
/// agree to it before anything is written. Empty when there is nothing to ask. Refused when another program saved the project since the mark `seen` was taken
/// (see [`suiteview::is_changed_refusal`]).
pub fn signature_warnings(path: &Path, seen: &str, code: &str) -> Result<Vec<String>, String> {
    let said = suiteview::warnings(path, Some(path), Some(seen), KIT.branch)?;
    Ok(said.iter().map(|w| w.text(code)).collect())
}

/// The outline layers of the job of a project; an error when the job has none.
pub fn outline_layers(path: &Path) -> Result<Vec<JobFile>, String> {
    suite::layers_of(path, "outline")
}

/// The outline layer of the job called `layer` (its path inside the job or its file name). A file name that two layers share is refused: the path says which.
pub fn choose<'a>(layers: &'a [JobFile], layer: &str) -> Result<&'a JobFile, String> {
    let shared: Vec<&str> = layers
        .iter()
        .filter(|f| f.file_name() == layer)
        .map(|f| f.rel.as_str())
        .collect();
    if shared.len() > 1 {
        return Err(format!(
            "{layer} is the name of more than one outline layer of the job ({}): give its path",
            shared.join(", ")
        ));
    }
    suite::choose(layers, "outline", layer)
}

/// The name of the plan of a layer inside `result/`: `route-<name of the layer>.txt`. When another of the `layers` would get the same name (`board.gko` and `board.gm1`), the
/// whole path of the layer is kept after `route+`, so that the plan of one layer never takes the place of the plan of another.
pub fn result_name(layers: &[JobFile], layer: &JobFile) -> String {
    let short = suite::result_name("route", layer, "txt");
    let shared = layers.iter().any(|other| {
        other.rel != layer.rel
            && suite::result_name("route", other, "txt").to_lowercase() == short.to_lowercase()
    });
    if !shared {
        return short;
    }
    let whole = layer.rel.strip_prefix("layers/").unwrap_or(&layer.rel);
    format!("route+{}.txt", whole.replace('/', "+"))
}

/// [`result_name`] with the layers of the job as the project at `path` has them now.
pub fn plan_name(path: &Path, layer: &JobFile) -> String {
    let layers = outline_layers(path).unwrap_or_default();
    result_name(&layers, layer)
}

/// Keeps the plan of one layer in the branch of this program. The plans of the other layers stay while the job is the one they were made from. Returns what a person should be told.
///
/// `seen` is the mark of the project as it was read ([`eh3d_drillpro::eh3dfile::mark_of`]). When another program saved the file since, nothing is written
/// (`EH3D-PKG-025`); what this program wrote itself does not count.
pub fn save_plan_since(
    path: &Path,
    seen: &str,
    layer: &JobFile,
    lines: &[String],
) -> Result<Vec<String>, String> {
    suite::keep_result_since(
        &KIT,
        path,
        seen,
        layer,
        &plan_name(path, layer),
        plan_bytes(lines),
    )
}
