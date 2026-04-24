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
pub mod wasm;

use wasm_bindgen::prelude::*;

use wasm::helpers::*;
use wasm::types::{WasmDecomposeResult, WasmIndexPair};

pub fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

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

pub fn collect_steiner_points(ring_flat: &[i64], parts_flat: JsValue) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = parse_flat_parts(parts_flat)?;
    let steiner = crate::decompose::collect_steiner_points(&ring, &parts);
    serialize(&flatten_ring(&steiner))
}

pub fn bayazit_decompose_polygon(
    ring_flat: &[i64],
    allow_steiner: bool,
) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = crate::bayazit::bayazit_decompose(
        &ring,
        allow_steiner,
        &crate::types::ProtocolConfig::merca(),
    )
    .map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}

pub fn exact_vertex_partition_polygon(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts = crate::exact_partition::exact_vertex_partition(&ring)
        .map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}

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

pub fn ear_clip_triangulate_polygon(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let parts =
        crate::ear_clip::ear_clip_triangulate(&ring).map_err(|err| JsValue::from_str(&err))?;
    serialize(&flatten_parts(&parts))
}

pub fn twice_area(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::area::twice_area_fp2(&ring).to_string())
}

pub fn twice_area_ring(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::area::twice_area_fp2(&ring).to_string())
}

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

pub fn signed_area_2x_ring(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::ring::signed_area_2x(&ring).to_string())
}

pub fn is_ccw_ring(ring_flat: &[i64]) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::ring::is_ccw(&ring))
}

pub fn ensure_ccw_ring(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let mut ring = parse_flat_ring(ring_flat)?;
    crate::ring::ensure_ccw(&mut ring);
    serialize(&flatten_ring(&ring))
}

pub fn remove_collinear_ring(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    serialize(&flatten_ring(&crate::ring::remove_collinear(&ring)))
}

pub fn is_simple_ring(ring_flat: &[i64]) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::ring::is_simple(&ring))
}

pub fn normalize_polygon_ring(ring_flat: &[i64]) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    serialize(&crate::ring::normalize_ring(&ring).map(|normalized| flatten_ring(&normalized)))
}

pub fn rotate_polygon_ring(ring_flat: &[i64], start: usize) -> Result<JsValue, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    serialize(&flatten_ring(&crate::ring::rotate_ring(&ring, start)))
}

pub fn is_convex_ring(ring_flat: &[i64]) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::validation::is_convex(&ring))
}

pub fn validate_edge_lengths_ring(
    ring_flat: &[i64],
    config: Option<JsValue>,
) -> Result<Option<String>, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let config = parse_config(config)?;
    Ok(crate::validation::validate_edge_lengths(&ring, &config))
}

pub fn perimeter_l1_ring(ring_flat: &[i64]) -> Result<String, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::validation::perimeter_l1(&ring).to_string())
}

/// Boundary-level compactness check. Apply to a whole polygon's outer
/// boundary (single part, or the union boundary of a multipart polygon).
/// NOT intended for individual parts of a multipart polygon — that would be
/// stricter than on-chain and reject legitimate decompositions.
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
pub fn validate_part_ring(
    ring_flat: &[i64],
    config: Option<JsValue>,
) -> Result<Option<String>, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    let config = parse_config(config)?;
    Ok(crate::validation::validate_part(&ring, &config))
}

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

pub fn point_strictly_inside_convex_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::spatial::point_strictly_inside_convex(px, py, &ring))
}

pub fn point_on_polygon_boundary_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::spatial::point_on_polygon_boundary(px, py, &ring))
}

pub fn point_inside_or_on_boundary_ring(
    px: i64,
    py: i64,
    ring_flat: &[i64],
) -> Result<bool, JsValue> {
    let ring = parse_flat_ring(ring_flat)?;
    Ok(crate::spatial::point_inside_or_on_boundary(px, py, &ring))
}

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

pub fn has_exact_shared_edge(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    Ok(crate::shared_edge::has_exact_shared_edge(&a, &b))
}

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
pub fn classify_contact(a_flat: &[i64], b_flat: &[i64]) -> Result<String, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    let kind = crate::shared_edge::classify_contact(&a, &b);
    Ok(match kind {
        crate::shared_edge::ContactKind::SharedEdge => "shared_edge",
        crate::shared_edge::ContactKind::PartialContact => "partial_contact",
        crate::shared_edge::ContactKind::None => "none",
    }
    .into())
}

pub fn convex_parts_overlap(a_flat: &[i64], b_flat: &[i64]) -> Result<bool, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    Ok(crate::overlap::convex_parts_overlap(&a, &b))
}

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

pub fn parts_overlap(a_parts_flat: JsValue, b_parts_flat: JsValue) -> Result<bool, JsValue> {
    let a_parts = parse_flat_parts(a_parts_flat)?;
    let b_parts = parse_flat_parts(b_parts_flat)?;
    Ok(crate::overlap::parts_overlap(&a_parts, &b_parts))
}

pub fn point_inside_any_part(parts_flat: JsValue, x: i64, y: i64) -> Result<bool, JsValue> {
    let parts = parse_flat_parts(parts_flat)?;
    Ok(crate::containment::point_inside_any_part(&parts, x, y))
}

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

pub fn cross2d(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> String {
    crate::primitives::cross2d(ax, ay, bx, by, cx, cy).to_string()
}

pub fn orientation(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> String {
    match crate::primitives::orientation(ax, ay, bx, by, cx, cy) {
        crate::primitives::Orientation::CounterClockwise => "CounterClockwise",
        crate::primitives::Orientation::Clockwise => "Clockwise",
        crate::primitives::Orientation::Collinear => "Collinear",
    }
    .into()
}

pub fn is_left(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_left(ax, ay, bx, by, px, py)
}

pub fn is_left_or_on(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_left_or_on(ax, ay, bx, by, px, py)
}

pub fn is_right(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_right(ax, ay, bx, by, px, py)
}

pub fn is_right_or_on(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_right_or_on(ax, ay, bx, by, px, py)
}

pub fn is_collinear_pts(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    crate::primitives::is_collinear_pts(ax, ay, bx, by, px, py)
}

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

pub fn edge_squared_length(ax: i64, ay: i64, bx: i64, by: i64) -> String {
    crate::primitives::edge_squared_length(ax, ay, bx, by).to_string()
}

pub fn point_on_segment(px: i64, py: i64, ax: i64, ay: i64, bx: i64, by: i64) -> bool {
    crate::primitives::point_on_segment(px, py, ax, ay, bx, by)
}

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

pub fn cross_sign(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> String {
    crate::signed::cross_sign(ax, ay, bx, by, cx, cy).to_string()
}

pub fn sub_u64(a: u64, b: u64) -> String {
    crate::signed::sub_u64(a, b).to_string()
}

pub fn sign_i128(value: &str) -> Result<i32, JsValue> {
    let parsed = value
        .parse::<i128>()
        .map_err(|_| invalid_input("value must be a valid i128 string"))?;
    Ok(crate::signed::sign(parsed))
}

pub fn is_left_turn(cross: &str) -> Result<bool, JsValue> {
    let parsed = cross
        .parse::<i128>()
        .map_err(|_| invalid_input("cross must be a valid i128 string"))?;
    Ok(crate::signed::is_left_turn(parsed))
}

pub fn is_right_turn(cross: &str) -> Result<bool, JsValue> {
    let parsed = cross
        .parse::<i128>()
        .map_err(|_| invalid_input("cross must be a valid i128 string"))?;
    Ok(crate::signed::is_right_turn(parsed))
}

pub fn is_collinear(cross: &str) -> Result<bool, JsValue> {
    let parsed = cross
        .parse::<i128>()
        .map_err(|_| invalid_input("cross must be a valid i128 string"))?;
    Ok(crate::signed::is_collinear(parsed))
}

pub fn optimize_partition(parts_flat: JsValue) -> Result<JsValue, JsValue> {
    let parts = parse_flat_parts(parts_flat)?;
    let optimized = crate::hertel_mehlhorn::optimize_partition(&parts);
    serialize(&flatten_parts(&optimized))
}

pub fn merge_convex_pair(a_flat: &[i64], b_flat: &[i64]) -> Result<JsValue, JsValue> {
    let a = parse_flat_ring(a_flat)?;
    let b = parse_flat_ring(b_flat)?;
    let result = crate::hertel_mehlhorn::merge_convex_pair(&a, &b);
    serialize(&result.map(|r| flatten_ring(&r)))
}

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
