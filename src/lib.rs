pub mod aabb;
pub mod area;
pub mod bayazit;
pub mod constants;
pub mod containment;
pub mod decompose;
pub mod ear_clip;
pub mod exact_partition;
pub mod hertel_mehlhorn;
pub mod overlap;
pub mod primitives;
pub mod ring;
pub mod sat;
pub mod shared_edge;
pub mod signed;
pub mod spatial;
pub mod topology;
pub mod types;
pub mod validate_onchain;
pub mod validation;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WasmDecomposeResult {
    pub parts: Vec<Vec<i64>>,
    pub steiner_points: Vec<i64>,
    pub strategy: crate::types::Strategy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Vec<crate::types::Attempt>>,
}

#[derive(Serialize, Deserialize)]
pub struct WasmIndexPair {
    pub a_index: usize,
    pub b_index: usize,
}

fn invalid_input(message: impl Into<String>) -> JsValue {
    JsValue::from_str(&message.into())
}

fn parse_flat_ring(ring_flat: &[i64]) -> Result<Vec<[i64; 2]>, JsValue> {
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

fn split_xy_from_flat(ring_flat: &[i64]) -> Result<(Vec<i64>, Vec<i64>), JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(split_xy(&ring))
}

fn split_xy(ring: &[[i64; 2]]) -> (Vec<i64>, Vec<i64>) {
    (
        ring.iter().map(|vertex| vertex[0]).collect(),
        ring.iter().map(|vertex| vertex[1]).collect(),
    )
}

fn flatten_ring(ring: &[[i64; 2]]) -> Vec<i64> {
    ring.iter().flat_map(|&[x, y]| [x, y]).collect()
}

fn flatten_parts(parts: &[Vec<[i64; 2]>]) -> Vec<Vec<i64>> {
    parts.iter().map(|part| flatten_ring(part)).collect()
}

fn parse_flat_parts(parts_flat: JsValue) -> Result<Vec<Vec<[i64; 2]>>, JsValue> {
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

fn serialize<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value).map_err(|err| invalid_input(err.to_string()))
}

fn parse_u128_str(value: &str, field: &str) -> Result<u128, JsValue> {
    value
        .parse::<u128>()
        .map_err(|_| invalid_input(format!("{field} must be a valid u128 string")))
}

fn parse_config(config_js: Option<JsValue>) -> Result<crate::types::ProtocolConfig, JsValue> {
    match config_js {
        None => Ok(crate::types::ProtocolConfig::merca()),
        Some(js) => serde_wasm_bindgen::from_value(js)
            .map_err(|e| JsValue::from_str(&format!("invalid config: {e}"))),
    }
}

fn has_zero_length_edge(ring: &[[i64; 2]]) -> bool {
    ring.iter()
        .zip(ring.iter().cycle().skip(1))
        .take(ring.len())
        .any(|(a, b)| a == b)
}

#[wasm_bindgen]
pub fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

#[wasm_bindgen]
pub fn decompose_polygon(
    ring_flat: &[i64],
    allow_steiner: bool,
    collect_trace: Option<bool>,
    minimize_parts: Option<bool>,
    config: Option<JsValue>,
) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let config = parse_config(config)?;
    let options = crate::types::DecomposeOptions {
        allow_steiner,
        collect_trace: collect_trace.unwrap_or(false),
        minimize_parts: minimize_parts.unwrap_or(false),
        ..Default::default()
    };

    let result = crate::decompose::decompose(&ring, &options, &config)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    serialize(&WasmDecomposeResult {
        parts: flatten_parts(&result.parts),
        steiner_points: flatten_ring(&result.steiner_points),
        strategy: result.strategy,
        trace: result.trace,
    })
}

#[wasm_bindgen]
pub fn collect_steiner_points(ring_flat: &[i64], parts_flat: JsValue) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = parse_flat_parts(parts_flat)?;
    let steiner = crate::decompose::collect_steiner_points(&ring, &parts);
    serialize(&flatten_ring(&steiner))
}

#[wasm_bindgen]
pub fn bayazit_decompose_polygon(
    ring_flat: &[i64],
    allow_steiner: bool,
) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = crate::bayazit::bayazit_decompose(&ring, allow_steiner)
        .map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}

#[wasm_bindgen]
pub fn exact_vertex_partition_polygon(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = crate::exact_partition::exact_vertex_partition(&ring)
        .map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}

#[wasm_bindgen]
pub fn exact_partition_only_original_vertices(
    ring_flat: &[i64],
    parts_flat: JsValue,
) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = parse_flat_parts(parts_flat)?;
    Ok(crate::exact_partition::only_original_vertices(
        &ring, &parts,
    ))
}

#[wasm_bindgen]
pub fn ear_clip_triangulate_polygon(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts =
        crate::ear_clip::ear_clip_triangulate(&ring).map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}

#[wasm_bindgen]
pub fn twice_area(ring_flat: &[i64]) -> Result<String, JsValue> {
    let (xs, ys) = split_xy_from_flat(ring_flat)?;
    Ok(crate::area::twice_area_fp2(&xs, &ys).to_string())
}

#[wasm_bindgen]
pub fn twice_area_ring(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::area::twice_area_fp2_ring(&ring).to_string())
}

#[wasm_bindgen]
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

#[wasm_bindgen]
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
pub fn signed_area_2x_ring(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::ring::signed_area_2x(&ring).to_string())
}

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

#[wasm_bindgen]
pub fn is_convex_ring(ring_flat: &[i64]) -> Result<bool, JsValue> {
    let (xs, ys) = split_xy_from_flat(ring_flat)?;
    Ok(crate::validation::is_convex(&xs, &ys))
}

#[wasm_bindgen]
pub fn validate_edge_lengths_ring(
    ring_flat: &[i64],
    config: Option<JsValue>,
) -> Result<Option<String>, JsValue> {
    let (xs, ys) = split_xy_from_flat(ring_flat)?;
    let config = parse_config(config)?;
    Ok(crate::validation::validate_edge_lengths(&xs, &ys, &config))
}

#[wasm_bindgen]
pub fn perimeter_l1_ring(ring_flat: &[i64]) -> Result<String, JsValue> {
    let (xs, ys) = split_xy_from_flat(ring_flat)?;
    Ok(crate::validation::perimeter_l1(&xs, &ys).to_string())
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
    let (xs, ys) = split_xy_from_flat(ring_flat)?;
    let config = parse_config(config)?;
    Ok(crate::validation::validate_part(&xs, &ys, &config))
}

#[wasm_bindgen]
pub fn sat_overlap(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    if has_zero_length_edge(&a) || has_zero_length_edge(&b) {
        return Err(invalid_input(
            "SAT polygons must not contain zero-length edges",
        ));
    }
    let (a_xs, a_ys) = split_xy(&a);
    let (b_xs, b_ys) = split_xy(&b);
    Ok(crate::sat::sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys))
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
    let (a_xs, a_ys) = split_xy(&a);
    let (b_xs, b_ys) = split_xy(&b);
    Ok(crate::sat::sat_overlaps_with_aabb(
        &a_xs, &a_ys, &b_xs, &b_ys,
    ))
}

#[wasm_bindgen]
pub fn point_strictly_inside_convex_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let (xs, ys) = split_xy_from_flat(ring_flat)?;
    Ok(crate::spatial::point_strictly_inside_convex(
        px, py, &xs, &ys,
    ))
}

#[wasm_bindgen]
pub fn point_on_polygon_boundary_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let (xs, ys) = split_xy_from_flat(ring_flat)?;
    Ok(crate::spatial::point_on_polygon_boundary(px, py, &xs, &ys))
}

#[wasm_bindgen]
pub fn point_inside_or_on_boundary_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let (xs, ys) = split_xy_from_flat(ring_flat)?;
    Ok(crate::spatial::point_inside_or_on_boundary(
        px, py, &xs, &ys,
    ))
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
    let (a_xs, a_ys) = split_xy_from_flat(a_ring_flat)?;
    let (b_xs, b_ys) = split_xy_from_flat(b_ring_flat)?;
    Ok(crate::spatial::collinear_segments_overlap_area(
        a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y, &a_xs, &a_ys, &b_xs, &b_ys,
    ))
}

#[wasm_bindgen]
pub fn has_exact_shared_edge(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let (a_xs, a_ys) = split_xy_from_flat(a_flat)?;
    let (b_xs, b_ys) = split_xy_from_flat(b_flat)?;
    Ok(crate::shared_edge::has_exact_shared_edge(
        &a_xs, &a_ys, &b_xs, &b_ys,
    ))
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
    let (a_xs, a_ys) = split_xy_from_flat(a_flat)?;
    let (b_xs, b_ys) = split_xy_from_flat(b_flat)?;
    let kind = crate::shared_edge::classify_contact(&a_xs, &a_ys, &b_xs, &b_ys);
    Ok(match kind {
        crate::shared_edge::ContactKind::SharedEdge => "shared_edge",
        crate::shared_edge::ContactKind::PartialContact => "partial_contact",
        crate::shared_edge::ContactKind::None => "none",
    }
    .into())
}

#[wasm_bindgen]
pub fn convex_parts_overlap(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let (a_xs, a_ys) = split_xy_from_flat(a_flat)?;
    let (b_xs, b_ys) = split_xy_from_flat(b_flat)?;
    Ok(crate::overlap::convex_parts_overlap(
        &a_xs, &a_ys, &b_xs, &b_ys,
    ))
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

#[wasm_bindgen]
pub fn optimize_partition(parts_flat: JsValue) -> Result<JsValue, JsValue> {
    let parts = parse_flat_parts(parts_flat)?;
    let optimized = crate::hertel_mehlhorn::optimize_partition(&parts);
    serialize(&flatten_parts(&optimized))
}

#[wasm_bindgen]
pub fn merge_convex_pair(a_flat: &[i64], b_flat: &[i64]) -> Result<JsValue, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    let result = crate::hertel_mehlhorn::merge_convex_pair(&a, &b);
    serialize(&result.map(|r| flatten_ring(&r)))
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
