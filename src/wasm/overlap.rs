use wasm_bindgen::prelude::*;

use super::helpers::*;
use super::types::WasmIndexPair;

#[wasm_bindgen]
pub fn sat_overlap(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    if has_zero_length_edge(&a) || has_zero_length_edge(&b) {
        return Err(invalid_input(
            "SAT polygons must not contain zero-length edges",
        ));
    }
    Ok(crate::sat::sat_overlaps(&a, &b))
}

#[wasm_bindgen]
pub fn sat_overlap_with_aabb(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    if has_zero_length_edge(&a) || has_zero_length_edge(&b) {
        return Err(invalid_input(
            "SAT polygons must not contain zero-length edges",
        ));
    }
    Ok(crate::sat::sat_overlaps_with_aabb(&a, &b))
}

#[wasm_bindgen]
pub fn convex_parts_overlap(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    Ok(crate::overlap::convex_parts_overlap(&a, &b))
}

#[wasm_bindgen]
pub fn find_overlapping_parts(
    a_parts_flat: JsValue,
    b_parts_flat: JsValue,
) -> Result<JsValue, JsValue> {
    let a_parts = parse_flat_parts(a_parts_flat)?;
    let b_parts = parse_flat_parts(b_parts_flat)?;
    let overlaps = crate::overlap::find_overlapping_parts(&a_parts, &b_parts)
        .into_iter()
        .map(|(a_index, b_index)| WasmIndexPair { a_index, b_index })
        .collect::<Vec<_>>();
    serialize(&overlaps)
}

#[wasm_bindgen]
pub fn parts_overlap(a_parts_flat: JsValue, b_parts_flat: JsValue) -> Result<bool, JsValue> {
    let a_parts = parse_flat_parts(a_parts_flat)?;
    let b_parts = parse_flat_parts(b_parts_flat)?;
    Ok(crate::overlap::parts_overlap(&a_parts, &b_parts))
}
