use wasm_bindgen::prelude::*;

use super::helpers::*;

#[wasm_bindgen]
pub fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

#[wasm_bindgen]
pub fn cross_sign(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> String {
    crate::signed::cross_sign(ax, ay, bx, by, cx, cy).to_string()
}

#[wasm_bindgen]
pub fn sub_u64(a: u64, b: u64) -> String {
    crate::signed::sub_u64(a, b).to_string()
}

#[wasm_bindgen]
pub fn sign_i128(value: &str) -> Result<i32, JsValue> {
    let parsed = value
        .parse::<i128>()
        .map_err(|_| invalid_input("value must be a valid i128 string"))?;
    Ok(crate::signed::sign(parsed))
}

#[wasm_bindgen]
pub fn is_left_turn(cross: &str) -> Result<bool, JsValue> {
    let parsed = cross
        .parse::<i128>()
        .map_err(|_| invalid_input("cross must be a valid i128 string"))?;
    Ok(crate::signed::is_left_turn(parsed))
}

#[wasm_bindgen]
pub fn is_right_turn(cross: &str) -> Result<bool, JsValue> {
    let parsed = cross
        .parse::<i128>()
        .map_err(|_| invalid_input("cross must be a valid i128 string"))?;
    Ok(crate::signed::is_right_turn(parsed))
}

#[wasm_bindgen]
pub fn is_collinear(cross: &str) -> Result<bool, JsValue> {
    let parsed = cross
        .parse::<i128>()
        .map_err(|_| invalid_input("cross must be a valid i128 string"))?;
    Ok(crate::signed::is_collinear(parsed))
}
