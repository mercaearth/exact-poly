//! Part validation matching polygon.move on-chain rules.
//! All validation functions must produce identical results to the Move contract.
//!
//! ## Compactness is a boundary-level property, not a per-part property.
//!
//! polygon.move calls `validate_compactness` in exactly two places
//! (polygon.move::validate_multipart_topology, count==1 and count>=2 branches):
//! - count == 1: on the single part's (twice_area, L1 perimeter), which is the
//!   polygon's outer boundary because the single part IS the boundary.
//! - count >= 2: on the union's outer boundary, computed by
//!   `validate_boundary_graph` (edges appearing exactly once) and the SUM of
//!   all per-part twice_areas.
//!
//! Per-part checks in Move are strictly: convexity, `validate_part_edges` (min
//! edge length), and max vertex count. There is NO per-part compactness check.
//!
//! This module therefore provides:
//! - `validate_compactness`: the isoperimetric check, to be applied to a
//!   polygon boundary (whole single part, or union boundary of a multipart
//!   polygon). Do NOT call this on an individual part of a multipart polygon.
//! - `validate_part`: mirrors Move's per-part rules — convex + edges +
//!   vertex count bounds. NO compactness here.
//!
//! Rules (in order):
//! 1. `is_convex`: weakly convex (collinear runs OK, reflex turns rejected)
//! 2. `validate_edge_lengths`: all edges squared-length >= `MIN_EDGE_LENGTH_SQUARED`
//! 3. `validate_compactness` (boundary-level only): `L1 perimeter^2 * MIN_COMPACTNESS_PPM <= 8_000_000 * twice_area`
//! 4. `validate_part`: structural per-part checks (1 + 2 + vertex-count bounds)

use crate::primitives::cross2d;
use crate::types::ProtocolConfig;

/// True if the polygon vertices form a weakly convex polygon.
///
/// "Weakly convex" = collinear runs are ALLOWED, but any non-zero cross product
/// must be the same sign throughout. Matches polygon.move::is_convex_vertices().
///
/// A polygon is convex if all cross products are:
/// - The same sign (all positive for CCW, all negative for CW), OR
/// - Zero (collinear)
/// Any mix of positive and negative cross products → not convex.
///
/// Coordinates are i64; cross products use i128.
pub fn is_convex(ring: &[[i64; 2]]) -> bool {
    let n = ring.len();
    if n < 3 {
        return false;
    }

    let mut direction: i32 = 0; // 0 = undecided, 1 = CCW (positive), -1 = CW (negative)

    for i in 0..n {
        let prev = (i + n - 1) % n;
        let next = (i + 1) % n;

        let cross = cross2d(
            ring[prev][0],
            ring[prev][1],
            ring[i][0],
            ring[i][1],
            ring[next][0],
            ring[next][1],
        );

        if cross == 0 {
            // Collinear — skip this vertex (weakly convex allows this)
            continue;
        }

        let cross_dir = if cross > 0 { 1i32 } else { -1i32 };

        if direction == 0 {
            direction = cross_dir;
        } else if direction != cross_dir {
            return false; // Mixed signs — not convex
        }
    }

    true // All non-zero cross products are same sign
}

/// Validate all edges have squared length >= MIN_EDGE_LENGTH_SQUARED.
/// Returns None if valid, Some(error message) if any edge is too short.
/// Matches polygon.move::validate_part_edges().
pub fn validate_edge_lengths(ring: &[[i64; 2]], config: &ProtocolConfig) -> Option<String> {
    let n = ring.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let dx = (ring[j][0] as i128) - (ring[i][0] as i128);
        let dy = (ring[j][1] as i128) - (ring[i][1] as i128);
        let sq_len = (dx * dx + dy * dy) as u128;
        if sq_len < config.min_edge_length_squared {
            return Some(format!(
                "edge {i}→{j} too short: {sq_len} < {}",
                config.min_edge_length_squared
            ));
        }
    }
    None
}

/// Compute the L1 (Manhattan) perimeter of a polygon.
/// Uses |dx| + |dy| per edge — matches polygon.move's perimeter formula.
/// MUST use L1, NOT Euclidean (matching on-chain compactness check).
pub fn perimeter_l1(ring: &[[i64; 2]]) -> u128 {
    let n = ring.len();
    let mut perimeter: u128 = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        let dx = ring[j][0].abs_diff(ring[i][0]) as u128;
        let dy = ring[j][1].abs_diff(ring[i][1]) as u128;
        perimeter += dx + dy;
    }
    perimeter
}

/// Outcome of the isoperimetric compactness check.
///
/// `ratio_ppm` is the computed compactness score in parts-per-million, directly
/// comparable to `min_ppm` (the configured threshold). It equals
/// `8_000_000 * twice_area / perimeter²`; for a 1:1 square this is ~1_000_000.
/// Saturated to `u128::MAX` on lhs overflow (a massive-area polygon is
/// unambiguously compact) and to 0 on perimeter² / rhs overflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactnessOutcome {
    pub passes: bool,
    pub ratio_ppm: u128,
    pub min_ppm: u128,
}

/// Core compactness check matching polygon.move::validate_compactness.
///
/// Formula: `8_000_000 * twice_area >= min_compactness_ppm * perimeter_l1²`.
///
/// Overflow semantics (matches Move):
/// - `perimeter²` overflow → not compact (pathologically long boundary).
/// - `rhs` (`min_ppm * perimeter²`) overflow → not compact.
/// - `lhs` (`8_000_000 * twice_area`) overflow → compact (massive area wins).
///
/// This is the canonical implementation. Both the string-based
/// `validate_compactness` and the `TopologyError`-based wrapper in `topology.rs`
/// delegate to this function so all callers agree byte-for-byte with on-chain.
pub fn check_compactness(
    twice_area: u128,
    perimeter: u128,
    config: &ProtocolConfig,
) -> CompactnessOutcome {
    let min_ppm = config.min_compactness_ppm;

    let perimeter_sq = perimeter.checked_mul(perimeter);
    let rhs = perimeter_sq.and_then(|p| min_ppm.checked_mul(p));
    let lhs = 8_000_000u128.checked_mul(twice_area);

    let passes = match (lhs, rhs) {
        (None, _) => true,        // massive area — compact by definition
        (Some(_), None) => false, // pathological perimeter — not compact
        (Some(l), Some(r)) => l >= r,
    };

    // ratio_ppm = 8M * twice_area / perimeter², directly comparable to min_ppm.
    let ratio_ppm = match (lhs, perimeter_sq) {
        (None, _) => u128::MAX,
        (_, None) | (_, Some(0)) => 0,
        (Some(l), Some(p_sq)) => l / p_sq,
    };

    CompactnessOutcome {
        passes,
        ratio_ppm,
        min_ppm,
    }
}

/// Validate compactness using the isoperimetric ratio, returning a string error
/// on failure. Thin wrapper around `check_compactness` for callers that want a
/// human-readable message instead of a structured `TopologyError`.
///
/// Apply this to a polygon boundary (whole single-part polygon, or outer
/// boundary of a multipart polygon). Do NOT call on an isolated part.
pub fn validate_compactness(
    twice_area: u128,
    perimeter: u128,
    config: &ProtocolConfig,
) -> Option<String> {
    let outcome = check_compactness(twice_area, perimeter, config);
    if outcome.passes {
        None
    } else {
        Some(format!(
            "not compact: {} ppm < min {} ppm",
            outcome.ratio_ppm, outcome.min_ppm
        ))
    }
}

/// Validate a polygon part against the per-part on-chain rules.
///
/// Mirrors polygon.move's per-part checks — the ones enforced by `part()` and
/// `validate_part_edges`/`is_convex_vertices`. Compactness is deliberately NOT
/// checked here because on-chain applies it only at the polygon boundary
/// (`validate_multipart_topology`). Callers that need the full polygon
/// validation must also invoke `topology::validate_multipart_topology`.
///
/// Checks (in order):
/// 1. At least 3 vertices
/// 2. At most `max_vertices_per_part` vertices
/// 3. Weakly convex
/// 4. All edges `>= min_edge_length_squared`
pub fn validate_part(ring: &[[i64; 2]], config: &ProtocolConfig) -> Option<String> {
    let n = ring.len();

    if n < 3 {
        return Some(format!("part has {n} vertices, need >= 3"));
    }

    if n > config.max_vertices_per_part {
        return Some(format!(
            "part has {n} vertices, max is {}",
            config.max_vertices_per_part
        ));
    }

    if !is_convex(ring) {
        return Some("part is not convex".into());
    }

    if let Some(err) = validate_edge_lengths(ring, config) {
        return Some(err);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::area::twice_area_fp2;
    use crate::types::ProtocolConfig;

    const M: i64 = 1_000_000;

    fn merca_config() -> ProtocolConfig {
        ProtocolConfig::merca()
    }

    #[test]
    fn is_convex_accepts_square() {
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        assert!(is_convex(&ring));
    }

    #[test]
    fn is_convex_rejects_l_shape() {
        // L-shape has a reflex vertex
        let ring = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];
        assert!(!is_convex(&ring));
    }

    #[test]
    fn is_convex_accepts_weakly_convex_with_collinear() {
        // Rectangle with extra vertex at midpoint of top edge (collinear)
        let ring = vec![
            [0, 0],
            [10 * M, 0],
            [10 * M, 10 * M],
            [5 * M, 10 * M],
            [0, 10 * M],
        ];
        assert!(is_convex(&ring));
    }

    #[test]
    fn is_convex_accepts_triangle() {
        let ring = vec![[0, 0], [10 * M, 0], [5 * M, 10 * M]];
        assert!(is_convex(&ring));
    }

    #[test]
    fn is_convex_rejects_two_points() {
        let ring = vec![[0, 0], [1, 0]];
        assert!(!is_convex(&ring));
    }

    #[test]
    fn is_convex_accepts_convex_pentagon() {
        let ring = vec![[0, 0], [2 * M, 0], [3 * M, M], [2 * M, 2 * M], [0, 2 * M]];
        assert!(is_convex(&ring));
    }

    #[test]
    fn validate_edge_lengths_valid() {
        // All edges 10M each (= (10M)^2 = 1e14 >> 1e12 threshold)
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        assert!(validate_edge_lengths(&ring, &merca_config()).is_none());
    }

    #[test]
    fn validate_edge_lengths_rejects_short_edge() {
        // Edge of 0.5M length: (0.5M)^2 = 2.5e11 < MIN_EDGE_LENGTH_SQUARED=1e12
        let ring = vec![[0, 0], [M / 2, 0], [M / 2, M], [0, M]]; // 0.5M = 500_000
                                                                 // Edge 0→1: dx=500_000, dy=0, sq=250_000_000_000 < 1_000_000_000_000
        assert!(validate_edge_lengths(&ring, &merca_config()).is_some());
    }

    #[test]
    fn validate_edge_lengths_exact_threshold() {
        // Edge exactly at MIN_EDGE_LENGTH = 1M: sq = (1M)^2 = 1e12 = MIN_EDGE_LENGTH_SQUARED
        let ring = vec![[0, 0], [M, 0], [M, M], [0, M]];
        assert!(validate_edge_lengths(&ring, &merca_config()).is_none());
    }

    #[test]
    fn validate_edge_lengths_accepts_large_negative_coordinates() {
        let ring = vec![[-1_000_000, 0], [1_000_000, 0]];
        assert!(validate_edge_lengths(&ring, &merca_config()).is_none());
    }

    #[test]
    fn validate_edge_lengths_rejects_unit_edge() {
        let ring = vec![[0, 0], [1, 1], [1, 0]];
        assert!(validate_edge_lengths(&ring, &merca_config()).is_some());
    }

    #[test]
    fn perimeter_l1_uses_manhattan() {
        // Square 10M x 10M: L1 perimeter = 4 * (|10M| + |0|) = 4 * 10M = 40M
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let p = perimeter_l1(&ring);
        assert_eq!(p, 4 * 10 * M as u128);
    }

    #[test]
    fn perimeter_l1_not_euclidean() {
        // Diagonal edge: (0,0)→(3M,4M)
        // Euclidean = 5M, L1 = 7M
        let ring = vec![[0, 0], [3 * M, 0], [0, 4 * M]]; // right triangle
                                                         // Edge 0→1: |3M|+|0|=3M, Edge 1→2: |3M|+|4M|=7M, Edge 2→0: |0|+|4M|=4M
        let p = perimeter_l1(&ring);
        assert_eq!(p, (3 + 7 + 4) * M as u128);
    }

    #[test]
    fn perimeter_l1_handles_negative_coordinates() {
        let ring = vec![[-1, -1], [-1, 1], [1, 1], [1, -1]];
        assert_eq!(perimeter_l1(&ring), 8);
    }

    #[test]
    fn perimeter_l1_large_square_does_not_overflow() {
        let ring = vec![
            [10_000_000 * M, 10_000_000 * M],
            [11_000_000 * M, 10_000_000 * M],
            [11_000_000 * M, 11_000_000 * M],
            [10_000_000 * M, 11_000_000 * M],
        ];
        assert_eq!(perimeter_l1(&ring), 4 * 1_000_000_000_000u128);
    }

    #[test]
    fn validate_compactness_compact_square_passes() {
        // 10M x 10M square
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let twice_area = twice_area_fp2(&ring);
        let perimeter = perimeter_l1(&ring);
        assert!(validate_compactness(twice_area, perimeter, &merca_config()).is_none());
    }

    #[test]
    fn validate_compactness_uses_checked_mul() {
        // Large but valid polygon — should not panic
        let ring = vec![
            [0, 0],
            [10_000_000 * M, 0],
            [10_000_000 * M, 10_000_000 * M],
            [0, 10_000_000 * M],
        ];
        let twice_area = twice_area_fp2(&ring);
        let perimeter = perimeter_l1(&ring);
        // Should not panic, result doesn't matter (extreme case)
        let _ = validate_compactness(twice_area, perimeter, &merca_config());
    }

    #[test]
    fn validate_part_valid_square() {
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        assert!(validate_part(&ring, &merca_config()).is_none());
    }

    #[test]
    fn validate_part_square_passes_both_configs() {
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let permissive = ProtocolConfig::permissive();
        assert!(validate_part(&ring, &merca_config()).is_none());
        assert!(validate_part(&ring, &permissive).is_none());
    }

    #[test]
    fn validate_part_short_square_fails_merca_but_passes_permissive() {
        let ring = vec![[0, 0], [M / 2, 0], [M / 2, M / 2], [0, M / 2]];
        let permissive = ProtocolConfig::permissive();
        assert!(validate_part(&ring, &merca_config()).is_some());
        assert!(validate_part(&ring, &permissive).is_none());
    }

    #[test]
    fn validate_part_rejects_l_shape() {
        let ring = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];
        assert!(validate_part(&ring, &merca_config()).is_some());
    }

    #[test]
    fn validate_part_rejects_too_few_vertices() {
        assert!(validate_part(&[[0, 0], [M, 0]], &merca_config()).is_some());
    }
}
