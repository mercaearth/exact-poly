use serde::Serialize;
use wasm_bindgen::prelude::*;

pub(crate) fn invalid_input(message: impl Into<String>) -> JsValue {
    JsValue::from_str(&message.into())
}

pub(crate) fn parse_flat_ring(ring_flat: &[i64]) -> Result<Vec<[i64; 2]>, JsValue> {
    if ring_flat.len() < 6 || ring_flat.len() % 2 != 0 {
        return Err(invalid_input(
            "ring must have >= 3 vertices encoded as [x0,y0,x1,y1,...]",
        ));
    }

    Ok(ring_flat
        .chunks_exact(2)
        .map(|chunk| [chunk[0], chunk[1]])
        .collect())
}

pub(crate) fn flatten_ring(ring: &[[i64; 2]]) -> Vec<i64> {
    ring.iter().flat_map(|&[x, y]| [x, y]).collect()
}

pub(crate) fn flatten_parts(parts: &[Vec<[i64; 2]>]) -> Vec<Vec<i64>> {
    parts.iter().map(|part| flatten_ring(part)).collect()
}

pub(crate) fn parse_flat_parts(parts_flat: JsValue) -> Result<Vec<Vec<[i64; 2]>>, JsValue> {
    let raw_parts: Vec<Vec<i64>> =
        serde_wasm_bindgen::from_value(parts_flat).map_err(|err| invalid_input(err.to_string()))?;

    if raw_parts.is_empty() {
        return Err(invalid_input("parts array must not be empty"));
    }

    raw_parts
        .into_iter()
        .map(|part| parse_flat_ring(&part))
        .collect()
}

pub(crate) fn serialize<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value).map_err(|err| invalid_input(err.to_string()))
}

pub(crate) fn parse_u128_str(value: &str, field: &str) -> Result<u128, JsValue> {
    value
        .parse::<u128>()
        .map_err(|_| invalid_input(format!("{field} must be a valid u128 string")))
}

pub(crate) fn parse_config(
    config_js: Option<JsValue>,
) -> Result<crate::types::ProtocolConfig, JsValue> {
    match config_js {
        None => Ok(crate::types::ProtocolConfig::permissive()),
        Some(js) => serde_wasm_bindgen::from_value(js)
            .map_err(|e| JsValue::from_str(&format!("invalid config: {e}"))),
    }
}

pub(crate) fn has_zero_length_edge(ring: &[[i64; 2]]) -> bool {
    ring.iter()
        .zip(ring.iter().cycle().skip(1))
        .take(ring.len())
        .any(|(a, b)| a == b)
}
