use wasm_bindgen::prelude::*;

use super::helpers::*;

#[wasm_bindgen]
pub fn is_ccw_ring(ring_flat: &[i64]) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::ring::is_ccw(&ring))
}

#[wasm_bindgen]
pub fn ensure_ccw_ring(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let mut ring = parse_flat_ring(ring_flat)?;
    crate::ring::ensure_ccw(&mut ring);
    serialize(&flatten_ring(&ring))
}

#[wasm_bindgen]
pub fn remove_collinear_ring(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    serialize(&flatten_ring(&crate::ring::remove_collinear(&ring)))
}

#[wasm_bindgen]
pub fn is_simple_ring(ring_flat: &[i64]) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::ring::is_simple(&ring))
}

#[wasm_bindgen]
pub fn normalize_polygon_ring(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    serialize(&crate::ring::normalize_ring(&ring).map(|normalized| flatten_ring(&normalized)))
}

#[wasm_bindgen]
pub fn rotate_polygon_ring(ring_flat: &[i64], start: usize) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    serialize(&flatten_ring(&crate::ring::rotate_ring(&ring, start)))
}
