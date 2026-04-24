use wasm_bindgen::prelude::*;

use super::helpers::*;

#[wasm_bindgen]
pub fn twice_area(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::area::twice_area_fp2(&ring).to_string())
}

#[wasm_bindgen(skip_typescript)]
pub fn area_display_from_twice_area(
    twice_area: &str,
    config: Option<JsValue>,
) -> Result<u64, JsValue> {
    let config = parse_config(config)?;
    Ok(crate::area::area_display(
        parse_u128_str(twice_area, "twice_area")?,
        config.area_divisor,
    ))
}

#[wasm_bindgen(skip_typescript)]
pub fn areas_conserved_values(original: &str, part_areas: JsValue) -> Result<bool, JsValue> {
    let original = parse_u128_str(original, "original")?;
    let part_area_strings: Vec<String> =
        serde_wasm_bindgen::from_value(part_areas).map_err(|err| invalid_input(err.to_string()))?;
    let parsed = part_area_strings
        .iter()
        .map(|value| parse_u128_str(value, "part area"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(crate::area::areas_conserved(original, &parsed))
}

#[wasm_bindgen]
pub fn signed_area_2x(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::ring::signed_area_2x(&ring).to_string())
}
