//! exact-poly — integer polygon geometry for deterministic computation.
//! All coordinates are i64 fixed-point (SCALE=1_000_000 units = 1 meter).
//! No float arithmetic. WASM bindings in src/wasm/.

pub mod aabb;
pub mod area;
pub mod bayazit;
pub mod constants;
pub mod containment;
pub mod decompose;
pub mod ear_clip;
pub mod exact_partition;
pub mod hertel_mehlhorn;
pub mod overlap;
pub mod primitives;
pub mod ring;
pub mod sat;
pub mod shared_edge;
pub mod signed;
pub mod spatial;
pub mod topology;
pub mod types;
pub mod validate_onchain;
pub mod validation;
pub mod wasm;
