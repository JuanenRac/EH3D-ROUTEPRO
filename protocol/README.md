# eh3d-routepro-protocol

Routing (milling) layer of the Bungard CCD protocol. It re-exports everything from
`eh3d-drillpro-protocol` (byte link, motion primitives, safety gate) and adds `route`:

```rust
use eh3d_routepro_protocol::route::{plan_polyline, RouteParams};
use eh3d_routepro_protocol::motion::Geometry;

let params = RouteParams { depth_mm: 0.5, passes: 2, feed_xy_mm_min: 1000.0, feed_z_mm_min: 500.0 };
let plan = plan_polyline((0.0, 0.0), &[(5.0, 5.0), (20.0, 5.0)], &params, &Geometry::default());
// plan.steps goes through safety::Machine::run, which refuses motion until the machine is verified.
```

A plan is: rapid to the start, then for each pass plunge, cut along the polyline and lift; with
several passes the tool returns to the start between passes. Depth is split evenly across passes.

No GUI and no file formats (HP-GL parsing lives in the `eh3d-routepro` application crate).
See the DRILLPRO protocol README for the wire facts and for what is still unverified on hardware.
