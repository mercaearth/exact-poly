use wasm_bindgen::prelude::*;

use super::helpers::*;

#[wasm_bindgen]
pub fn point_strictly_inside_convex_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::spatial::point_strictly_inside_convex(px, py, &ring))
}

#[wasm_bindgen]
pub fn point_on_polygon_boundary_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::spatial::point_on_polygon_boundary(px, py, &ring))
}

#[wasm_bindgen]
pub fn point_inside_or_on_boundary_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::spatial::point_inside_or_on_boundary(px, py, &ring))
}

#[wasm_bindgen]
pub fn collinear_segments_overlap_area_rings(
    a1x: i64,
    a1y: i64,
    a2x: i64,
    a2y: i64,
    b1x: i64,
    b1y: i64,
    b2x: i64,
    b2y: i64,
    a_ring_flat: &[i64],
    b_ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let a_ring = parse_flat_ring(a_ring_flat)?;
    let b_ring = parse_flat_ring(b_ring_flat)?;
    Ok(crate::spatial::collinear_segments_overlap_area(
        a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y, &a_ring, &b_ring,
    ))
}

#[wasm_bindgen]
pub fn point_inside_any_part(parts_flat: JsValue, x: i64, y: i64) -> Result<bool, JsValue> {
    let parts = parse_flat_parts(parts_flat)?;
    Ok(crate::containment::point_inside_any_part(&parts, x, y))
}

#[wasm_bindgen]
pub fn contains_polygon(
    outer_parts_flat: JsValue,
    inner_parts_flat: JsValue,
) -> Result<bool, JsValue> {
    let outer_parts = parse_flat_parts(outer_parts_flat)?;
    let inner_parts = parse_flat_parts(inner_parts_flat)?;
    Ok(crate::containment::contains_polygon(
        &outer_parts,
        &inner_parts,
    ))
}
