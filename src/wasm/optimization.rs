use wasm_bindgen::prelude::*;

use super::helpers::*;

#[wasm_bindgen(skip_typescript)]
pub fn optimize_partition(parts_flat: JsValue) -> Result<JsValue, JsValue> {
    let parts = parse_flat_parts(parts_flat)?;
    let optimized = crate::hertel_mehlhorn::optimize_partition(&parts);
    serialize(&flatten_parts(&optimized))
}

#[wasm_bindgen(skip_typescript)]
pub fn merge_convex_pair(a_flat: &[i64], b_flat: &[i64]) -> Result<JsValue, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    let result = crate::hertel_mehlhorn::merge_convex_pair(&a, &b);
    serialize(&result.map(|r| flatten_ring(&r)))
}
