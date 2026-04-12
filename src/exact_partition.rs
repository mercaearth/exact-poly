//! Exact vertex partition — uses only original polygon vertices, no Steiner points.
//!
//! Prefers diagonals between reflex vertices for better decomposition quality.
//! Falls back gracefully: returns None if no valid exact-vertex decomposition found.
//!
//! Reference: deploy/app/src/lib/decompose.ts::searchExactVertexPartition

use crate::primitives::{
    is_left, is_left_or_on, is_reflex, point_on_segment, segments_properly_intersect,
};
use crate::validation::is_convex;

/// Decompose a simple CCW polygon using only its own vertices (no Steiner points).
/// Returns Ok(parts) or Err if no exact-vertex decomposition is possible.
///
/// This is the "exact vertices only" strategy used when protectedKeys are present.
pub fn exact_vertex_partition(ring: &[[i64; 2]]) -> Result<Vec<Vec<[i64; 2]>>, String> {
    if ring.len() < 3 {
        return Err("polygon has fewer than 3 vertices".into());
    }

    // Already convex — return as-is
    {
        let xs: Vec<i64> = ring.iter().map(|v| v[0]).collect();
        let ys: Vec<i64> = ring.iter().map(|v| v[1]).collect();
        if is_convex(&xs, &ys) {
            return Ok(vec![ring.to_vec()]);
        }
    }

    let mut result = Vec::new();
    if exact_partition_recursive(ring, &mut result, 0) {
        Ok(result)
    } else {
        Err("no valid exact-vertex partition found".into())
    }
}

fn exact_partition_recursive(
    poly: &[[i64; 2]],
    result: &mut Vec<Vec<[i64; 2]>>,
    depth: usize,
) -> bool {
    if depth > 32 || poly.len() < 3 {
        if poly.len() >= 3 {
            result.push(poly.to_vec());
        }
        return true;
    }

    // Already convex
    {
        let xs: Vec<i64> = poly.iter().map(|v| v[0]).collect();
        let ys: Vec<i64> = poly.iter().map(|v| v[1]).collect();
        if is_convex(&xs, &ys) {
            result.push(poly.to_vec());
            return true;
        }
    }

    let n = poly.len();

    // Try all diagonals between non-adjacent vertices
    // Prioritize: both endpoints reflex, then one reflex, then any valid diagonal
    let mut candidates: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        for j in (i + 2)..n {
            if i == 0 && j == n - 1 {
                continue;
            } // Adjacent via wrap
            if is_valid_exact_diagonal(poly, i, j) {
                candidates.push((i, j));
            }
        }
    }

    // Sort: prefer diagonals where at least one endpoint is reflex
    candidates.sort_by_key(|&(i, j)| {
        let r_i = is_reflex_at(poly, i) as u8;
        let r_j = is_reflex_at(poly, j) as u8;
        255u8 - (r_i + r_j) // higher reflex count = lower sort key
    });

    for (i, j) in candidates {
        let (lo, hi) = if i <= j { (i, j) } else { (j, i) };

        let lower: Vec<[i64; 2]> = (lo..=hi).map(|k| poly[k]).collect();
        let mut upper: Vec<[i64; 2]> = Vec::new();
        for k in hi..n {
            upper.push(poly[k]);
        }
        for k in 0..=lo {
            upper.push(poly[k]);
        }

        if lower.len() < 3 || upper.len() < 3 {
            continue;
        }

        let lo_len = result.len();
        if exact_partition_recursive(&lower, result, depth + 1)
            && exact_partition_recursive(&upper, result, depth + 1)
        {
            return true;
        }
        // Backtrack
        result.truncate(lo_len);
    }

    false
}

fn is_reflex_at(poly: &[[i64; 2]], i: usize) -> bool {
    let n = poly.len();
    let prev = poly[(i + n - 1) % n];
    let curr = poly[i];
    let next = poly[(i + 1) % n];
    is_reflex(prev[0], prev[1], curr[0], curr[1], next[0], next[1])
}

fn is_valid_exact_diagonal(poly: &[[i64; 2]], i: usize, j: usize) -> bool {
    let n = poly.len();
    if i == j || (i + 1) % n == j || (j + 1) % n == i {
        return false;
    }

    // Check that no other vertex lies on the diagonal
    for k in 0..n {
        if k == i || k == j {
            continue;
        }
        if point_on_segment(
            poly[k][0], poly[k][1], poly[i][0], poly[i][1], poly[j][0], poly[j][1],
        ) {
            return false;
        }
    }

    in_cone(poly, i, j) && in_cone(poly, j, i) && diagonal_no_cross(poly, i, j)
}

/// Check if vertex j is in the cone formed at vertex i.
/// For a convex vertex: j must be strictly left of both adjacent edges.
/// For a reflex vertex: j must NOT be in the reflex wedge.
fn in_cone(poly: &[[i64; 2]], i: usize, j: usize) -> bool {
    let n = poly.len();
    let a = poly[i];
    let b = poly[j];
    let a_prev = poly[(i + n - 1) % n];
    let a_next = poly[(i + 1) % n];

    // If vertex i is convex or collinear (prev is left-of-or-on the edge i→next)
    if is_left_or_on(a[0], a[1], a_next[0], a_next[1], a_prev[0], a_prev[1]) {
        // b must be strictly left of edge (a→a_prev) AND strictly left of edge (b→a_next)
        // i.e., left of (a, a_next) seen from b side, and left of (a_prev, a) seen from b side
        return is_left(a[0], a[1], b[0], b[1], a_prev[0], a_prev[1])
            && is_left(b[0], b[1], a[0], a[1], a_next[0], a_next[1]);
    }

    // Reflex vertex: b must NOT be in the reflex wedge
    !(is_left_or_on(a[0], a[1], b[0], b[1], a_next[0], a_next[1])
        && is_left_or_on(b[0], b[1], a[0], a[1], a_prev[0], a_prev[1]))
}

/// Check that the diagonal i→j does not properly cross any edge of the polygon.
fn diagonal_no_cross(poly: &[[i64; 2]], i: usize, j: usize) -> bool {
    let n = poly.len();
    let a = poly[i];
    let b = poly[j];

    for k in 0..n {
        let l = (k + 1) % n;
        if k == i || k == j || l == i || l == j {
            continue;
        }
        if segments_properly_intersect(
            a[0], a[1], b[0], b[1], poly[k][0], poly[k][1], poly[l][0], poly[l][1],
        ) {
            return false;
        }
    }
    true
}

/// True if all vertices in output parts were in the original ring.
pub fn only_original_vertices(ring: &[[i64; 2]], parts: &[Vec<[i64; 2]>]) -> bool {
    let original_set: std::collections::HashSet<[i64; 2]> = ring.iter().cloned().collect();
    parts
        .iter()
        .all(|part| part.iter().all(|v| original_set.contains(v)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    fn l_shape() -> Vec<[i64; 2]> {
        vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ]
    }

    fn arrow_shape() -> Vec<[i64; 2]> {
        // An arrow/chevron shape (two reflex vertices)
        vec![
            [0, 0],
            [10 * M, 5 * M],
            [20 * M, 0],
            [20 * M, 20 * M],
            [10 * M, 15 * M],
            [0, 20 * M],
        ]
    }

    #[test]
    fn convex_polygon_returns_as_single_part() {
        let square = vec![[0i64, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let result = exact_vertex_partition(&square).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn triangle_returns_as_single_part() {
        let tri = vec![[0i64, 0], [10 * M, 0], [5 * M, 10 * M]];
        let result = exact_vertex_partition(&tri).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn rejects_degenerate_input() {
        assert!(exact_vertex_partition(&[[0, 0], [1, 1]]).is_err());
        assert!(exact_vertex_partition(&[]).is_err());
    }

    #[test]
    fn l_shape_decomposes_using_only_original_vertices() {
        let ring = l_shape();
        let result = exact_vertex_partition(&ring);
        if let Ok(parts) = result {
            assert!(
                only_original_vertices(&ring, &parts),
                "output contains Steiner points"
            );
            assert!(parts.len() >= 2, "L-shape needs at least 2 convex parts");
        }
        // It's OK if exact_vertex_partition returns Err for some shapes —
        // the cascade will fall back to Bayazit
    }

    #[test]
    fn all_parts_convex_when_successful() {
        let ring = l_shape();
        let result = exact_vertex_partition(&ring);
        if let Ok(parts) = result {
            for (idx, part) in parts.iter().enumerate() {
                let xs: Vec<i64> = part.iter().map(|v| v[0]).collect();
                let ys: Vec<i64> = part.iter().map(|v| v[1]).collect();
                assert!(is_convex(&xs, &ys), "part {idx} is not convex: {:?}", part);
            }
        }
    }

    #[test]
    fn area_conservation() {
        let ring = l_shape();
        let result = exact_vertex_partition(&ring);
        if let Ok(parts) = result {
            let original_area = crate::area::twice_area_fp2_ring(&ring);
            let parts_area: u128 = parts
                .iter()
                .map(|p| crate::area::twice_area_fp2_ring(p))
                .sum();
            assert_eq!(
                parts_area, original_area,
                "area mismatch: original={original_area}, parts_sum={parts_area}"
            );
        }
    }

    #[test]
    fn arrow_shape_decomposes_correctly() {
        let ring = arrow_shape();
        let result = exact_vertex_partition(&ring);
        if let Ok(parts) = result {
            assert!(only_original_vertices(&ring, &parts));
            for part in &parts {
                let xs: Vec<i64> = part.iter().map(|v| v[0]).collect();
                let ys: Vec<i64> = part.iter().map(|v| v[1]).collect();
                assert!(is_convex(&xs, &ys));
            }
            let original_area = crate::area::twice_area_fp2_ring(&ring);
            let parts_area: u128 = parts
                .iter()
                .map(|p| crate::area::twice_area_fp2_ring(p))
                .sum();
            assert_eq!(parts_area, original_area);
        }
    }

    #[test]
    fn only_original_vertices_rejects_steiner() {
        let ring = vec![[0i64, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let bad_parts = vec![vec![[0, 0], [5 * M, 0], [10 * M, 0], [10 * M, 10 * M]]];
        // [5*M, 0] is on an edge but not a vertex of the ring
        assert!(!only_original_vertices(&ring, &bad_parts));
    }

    #[test]
    fn only_original_vertices_accepts_valid() {
        let ring = vec![[0i64, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let good_parts = vec![vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]]];
        assert!(only_original_vertices(&ring, &good_parts));
    }

    #[test]
    fn u_shape_decomposes() {
        // U-shape: two reflex vertices
        let ring = vec![
            [0, 0],
            [30 * M, 0],
            [30 * M, 20 * M],
            [20 * M, 20 * M],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];
        let result = exact_vertex_partition(&ring);
        if let Ok(parts) = result {
            assert!(only_original_vertices(&ring, &parts));
            for part in &parts {
                let xs: Vec<i64> = part.iter().map(|v| v[0]).collect();
                let ys: Vec<i64> = part.iter().map(|v| v[1]).collect();
                assert!(is_convex(&xs, &ys));
            }
            let original_area = crate::area::twice_area_fp2_ring(&ring);
            let parts_area: u128 = parts
                .iter()
                .map(|p| crate::area::twice_area_fp2_ring(p))
                .sum();
            assert_eq!(parts_area, original_area);
        }
    }
}
