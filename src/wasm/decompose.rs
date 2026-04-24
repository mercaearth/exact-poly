use wasm_bindgen::prelude::*;

use super::helpers::*;
use super::types::WasmDecomposeResult;

#[wasm_bindgen]
pub fn decompose_polygon(
    ring_flat: &[i64],
    allow_steiner: bool,
    collect_trace: Option<bool>,
    minimize_parts: Option<bool>,
    config: Option<JsValue>,
) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let config = parse_config(config)?;
    let options = crate::types::DecomposeOptions {
        allow_steiner,
        collect_trace: collect_trace.unwrap_or(false),
        minimize_parts: minimize_parts.unwrap_or(false),
        ..Default::default()
    };

    let result = crate::decompose::decompose(&ring, &options, &config)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    serialize(&WasmDecomposeResult {
        parts: flatten_parts(&result.parts),
        steiner_points: flatten_ring(&result.steiner_points),
        strategy: result.strategy,
        trace: result.trace,
    })
}

#[wasm_bindgen]
pub fn collect_steiner_points(ring_flat: &[i64], parts_flat: JsValue) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = parse_flat_parts(parts_flat)?;
    let steiner = crate::decompose::collect_steiner_points(&ring, &parts);
    serialize(&flatten_ring(&steiner))
}

#[wasm_bindgen]
pub fn bayazit_decompose_polygon(
    ring_flat: &[i64],
    allow_steiner: bool,
) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = crate::bayazit::bayazit_decompose(
        &ring,
        allow_steiner,
        &crate::types::ProtocolConfig::merca(),
    )
    .map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}

#[wasm_bindgen]
pub fn exact_vertex_partition_polygon(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = crate::exact_partition::exact_vertex_partition(&ring)
        .map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}

#[wasm_bindgen]
pub fn exact_partition_only_original_vertices(
    ring_flat: &[i64],
    parts_flat: JsValue,
) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = parse_flat_parts(parts_flat)?;
    Ok(crate::exact_partition::only_original_vertices(
        &ring, &parts,
    ))
}

#[wasm_bindgen]
pub fn ear_clip_triangulate_polygon(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts =
        crate::ear_clip::ear_clip_triangulate(&ring).map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}
