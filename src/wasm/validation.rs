use wasm_bindgen::prelude::*;

use super::helpers::*;

#[wasm_bindgen]
pub fn is_convex_ring(ring_flat: &[i64]) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::validation::is_convex(&ring))
}

#[wasm_bindgen]
pub fn validate_edge_lengths_ring(
    ring_flat: &[i64],
    config: Option<JsValue>,
) -> Result<Option<String>, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let config = parse_config(config)?;
    Ok(crate::validation::validate_edge_lengths(&ring, &config))
}

#[wasm_bindgen]
pub fn perimeter_l1_ring(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::validation::perimeter_l1(&ring).to_string())
}

/// Boundary-level compactness check. Apply to a whole polygon's outer
/// boundary (single part, or the union boundary of a multipart polygon).
/// NOT intended for individual parts of a multipart polygon — that would be
/// stricter than on-chain and reject legitimate decompositions.
#[wasm_bindgen]
pub fn validate_compactness_values(
    twice_area: &str,
    perimeter: &str,
    config: Option<JsValue>,
) -> Result<Option<String>, JsValue> {
    let twice_area = parse_u128_str(twice_area, "twice_area")?;
    let perimeter = parse_u128_str(perimeter, "perimeter")?;
    let config = parse_config(config)?;
    Ok(crate::validation::validate_compactness(
        twice_area, perimeter, &config,
    ))
}

/// Per-part structural validation mirroring polygon.move's `part()` entry:
/// vertex-count bounds, weak convexity, and minimum edge length.
///
/// Breaking change (was: also enforced compactness). Compactness is a
/// boundary-level property; call `validate_multipart_topology` (or the
/// full on-chain validator) to check the assembled polygon instead.
#[wasm_bindgen]
pub fn validate_part_ring(
    ring_flat: &[i64],
    config: Option<JsValue>,
) -> Result<Option<String>, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let config = parse_config(config)?;
    Ok(crate::validation::validate_part(&ring, &config))
}
