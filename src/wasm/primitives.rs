use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn cross2d(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> String {
    crate::primitives::cross2d(ax, ay, bx, by, cx, cy).to_string()
}

#[wasm_bindgen]
pub fn orientation(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> String {
    match crate::primitives::orientation(ax, ay, bx, by, cx, cy) {
        crate::primitives::Orientation::CounterClockwise => "CounterClockwise",
        crate::primitives::Orientation::Clockwise => "Clockwise",
        crate::primitives::Orientation::Collinear => "Collinear",
    }
    .into()
}

#[wasm_bindgen]
pub fn is_left(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_left(ax, ay, bx, by, px, py)
}

#[wasm_bindgen]
pub fn is_left_or_on(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_left_or_on(ax, ay, bx, by, px, py)
}

#[wasm_bindgen]
pub fn is_right(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_right(ax, ay, bx, by, px, py)
}

#[wasm_bindgen]
pub fn is_right_or_on(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_right_or_on(ax, ay, bx, by, px, py)
}

#[wasm_bindgen]
pub fn is_collinear_pts(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_collinear_pts(ax, ay, bx, by, px, py)
}

#[wasm_bindgen]
pub fn is_reflex(
    prev_x: i64,
    prev_y: i64,
    curr_x: i64,
    curr_y: i64,
    next_x: i64,
    next_y: i64,
) -> bool {
    crate::primitives::is_reflex(prev_x, prev_y, curr_x, curr_y, next_x, next_y)
}

#[wasm_bindgen]
pub fn edge_squared_length(ax: i64, ay: i64, bx: i64, by: i64) -> String {
    crate::primitives::edge_squared_length(ax, ay, bx, by).to_string()
}

#[wasm_bindgen]
pub fn point_on_segment(px: i64, py: i64, ax: i64, ay: i64, bx: i64, by: i64) -> bool {
    crate::primitives::point_on_segment(px, py, ax, ay, bx, by)
}

#[wasm_bindgen]
pub fn segments_properly_intersect(
    a1x: i64,
    a1y: i64,
    a2x: i64,
    a2y: i64,
    b1x: i64,
    b1y: i64,
    b2x: i64,
    b2y: i64,
) -> bool {
    crate::primitives::segments_properly_intersect(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y)
}

#[wasm_bindgen]
pub fn segments_intersect(
    a1x: i64,
    a1y: i64,
    a2x: i64,
    a2y: i64,
    b1x: i64,
    b1y: i64,
    b2x: i64,
    b2y: i64,
) -> bool {
    crate::primitives::segments_intersect(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y)
}
