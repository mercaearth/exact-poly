use wasm_bindgen::prelude::*;

use super::helpers::*;

#[wasm_bindgen]
pub fn has_exact_shared_edge(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    Ok(crate::shared_edge::has_exact_shared_edge(&a, &b))
}

#[wasm_bindgen]
pub fn segments_contact(
    ax1: i64,
    ay1: i64,
    ax2: i64,
    ay2: i64,
    bx1: i64,
    by1: i64,
    bx2: i64,
    by2: i64,
) -> bool {
    crate::shared_edge::segments_contact(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2)
}

/// Classify contact between two polygon parts.
///
/// Returns: `"shared_edge"`, `"partial_contact"`, or `"none"`.
///
/// - `"shared_edge"`: at least one edge appears in both parts (valid adjacency).
/// - `"partial_contact"`: collinear overlap without exact match (T-junction —
///   on-chain aborts `EInvalidMultipartContact`).
/// - `"none"`: no collinear contact at all.
#[wasm_bindgen]
pub fn classify_contact(a_flat: &[i64], b_flat: &[i64]) -> Result<String, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    let kind = crate::shared_edge::classify_contact(&a, &b);
    Ok(match kind {
        crate::shared_edge::ContactKind::SharedEdge => "shared_edge",
        crate::shared_edge::ContactKind::PartialContact => "partial_contact",
        crate::shared_edge::ContactKind::None => "none",
    }
    .into())
}

#[wasm_bindgen]
pub fn validate_multipart_topology(
    parts_flat: JsValue,
    allow_vertex_contact: Option<bool>,
    config: Option<JsValue>,
) -> Result<JsValue, JsValue> {
    let parts = parse_flat_parts(parts_flat)?;
    let config = parse_config(config)?;
    match crate::topology::validate_multipart_topology(
        &parts,
        allow_vertex_contact.unwrap_or(false),
        &config,
    ) {
        Ok(()) => serialize(&Option::<crate::types::TopologyError>::None),
        Err(topo_err) => serialize(&Some(topo_err)),
    }
}

#[wasm_bindgen]
pub fn validate_decomposition(
    ring_flat: &[i64],
    parts_flat: JsValue,
    config: Option<JsValue>,
) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = parse_flat_parts(parts_flat)?;
    let config = parse_config(config)?;
    let report = crate::validate_onchain::validate_decomposition(&ring, &parts, &config);
    serialize(&report)
}
