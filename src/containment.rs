//! Polygon containment checking.
//!
//! Determines if one polygon (set of convex parts) is fully inside another.
//! Uses vertex checking plus edge midpoint sampling (t=1/3, t=2/3) for
//! multi-part outer polygons where the composite shape may be concave.
//!
//! Ported from polygon.move::contains_polygon / part_is_inside_parts.

use crate::aabb::Aabb;
use crate::spatial::point_inside_or_on_boundary;

/// True if point (x, y) is inside any of the given convex parts.
pub fn point_inside_any_part(parts: &[Vec<[i64; 2]>], x: i64, y: i64) -> bool {
    for part in parts {
        if point_inside_or_on_boundary(x, y, part) {
            return true;
        }
    }
    false
}

/// True if the inner polygon (set of parts) is fully contained within the outer polygon.
///
/// Algorithm (matching polygon.move::contains_polygon):
/// 1. AABB pre-filter: inner AABB must fit within outer AABB
/// 2. All inner vertices must be inside some outer part
/// 3. For multi-part outers only: sample edge points at t=1/3 and t=2/3 of each
///    inner edge (handles concave composite shapes where an inner edge might bridge
///    two outer parts through a gap)
///
/// Integer division for sample points: `(2*a + b) / 3` and `(a + 2*b) / 3`.
/// Single-part outers skip edge sampling (convex ⊂ convex is proven by vertices).
pub fn contains_polygon(outer_parts: &[Vec<[i64; 2]>], inner_parts: &[Vec<[i64; 2]>]) -> bool {
    if outer_parts.is_empty() || inner_parts.is_empty() {
        return false;
    }

    // AABB pre-filter
    let outer_aabb = compute_multi_part_aabb(outer_parts);
    let inner_aabb = compute_multi_part_aabb(inner_parts);

    if inner_aabb.min_x < outer_aabb.min_x
        || inner_aabb.max_x > outer_aabb.max_x
        || inner_aabb.min_y < outer_aabb.min_y
        || inner_aabb.max_y > outer_aabb.max_y
    {
        return false;
    }

    // Check each inner part is fully inside the outer polygon
    for part in inner_parts {
        if !part_is_inside(outer_parts, part) {
            return false;
        }
    }

    true
}

/// Check that a single inner part is fully inside the outer parts.
/// Matches polygon.move::part_is_inside_parts.
fn part_is_inside(outer_parts: &[Vec<[i64; 2]>], inner_part: &[[i64; 2]]) -> bool {
    // All vertices must be inside some outer part
    for &v in inner_part {
        if !point_inside_any_part(outer_parts, v[0], v[1]) {
            return false;
        }
    }

    // Edge sampling only for multi-part outers (concave composite shapes).
    // Single-part convex outers: vertex check is sufficient.
    if outer_parts.len() > 1 {
        let n = inner_part.len();
        for i in 0..n {
            let j = (i + 1) % n;
            let x1 = inner_part[i][0];
            let y1 = inner_part[i][1];
            let x2 = inner_part[j][0];
            let y2 = inner_part[j][1];

            // t=1/3: (2*x1 + x2) / 3
            let sx1 = (2 * x1 + x2) / 3;
            let sy1 = (2 * y1 + y2) / 3;
            if !point_inside_any_part(outer_parts, sx1, sy1) {
                return false;
            }

            // t=2/3: (x1 + 2*x2) / 3
            let sx2 = (x1 + 2 * x2) / 3;
            let sy2 = (y1 + 2 * y2) / 3;
            if !point_inside_any_part(outer_parts, sx2, sy2) {
                return false;
            }
        }
    }

    true
}

/// Compute the bounding AABB of all parts combined.
fn compute_multi_part_aabb(parts: &[Vec<[i64; 2]>]) -> Aabb {
    let ring: Vec<[i64; 2]> = parts.iter().flat_map(|p| p.iter().copied()).collect();
    Aabb::from_ring(&ring)
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    fn square(ox: i64, oy: i64, size: i64) -> Vec<[i64; 2]> {
        vec![
            [ox, oy],
            [ox + size, oy],
            [ox + size, oy + size],
            [ox, oy + size],
        ]
    }

    #[test]
    fn small_square_inside_large_square() {
        let outer = vec![square(0, 0, 10 * M)];
        let inner = vec![square(2 * M, 2 * M, 3 * M)];
        assert!(contains_polygon(&outer, &inner));
    }

    #[test]
    fn touching_outer_boundary_is_not_contained() {
        let outer = vec![square(0, 0, 5 * M), square(7 * M, 0, 5 * M)];
        let inner = vec![square(5 * M, 2 * M, 4 * M)];

        assert!(!contains_polygon(&outer, &inner));
    }

    #[test]
    fn partially_outside_returns_false() {
        let outer = vec![square(0, 0, 10 * M)];
        let inner = vec![square(8 * M, 8 * M, 5 * M)]; // extends beyond outer
        assert!(!contains_polygon(&outer, &inner));
    }

    #[test]
    fn completely_outside_returns_false() {
        let outer = vec![square(0, 0, M)];
        let inner = vec![square(3 * M, 3 * M, M)];
        assert!(!contains_polygon(&outer, &inner));
    }

    #[test]
    fn same_polygon_contains_itself() {
        let polygon = vec![square(0, 0, 10 * M)];
        assert!(contains_polygon(&polygon, &polygon));
    }

    #[test]
    fn multipart_outer_contains_inner() {
        // Two-part outer (two adjacent rectangles)
        let outer = vec![square(0, 0, 10 * M), square(10 * M, 0, 10 * M)];
        // Inner well inside the right part
        let inner = vec![square(12 * M, 2 * M, 5 * M)];
        assert!(contains_polygon(&outer, &inner));
    }

    #[test]
    fn multipart_outer_inner_bridges_gap_returns_false() {
        // Two outer parts with a gap between them
        // Left part: (0,0)-(5M, 10M), Right part: (7M,0)-(12M, 10M)
        // Gap from x=5M to x=7M
        let outer = vec![square(0, 0, 5 * M), square(7 * M, 0, 5 * M)];
        // Inner spans the gap: x=4M to x=8M
        let inner = vec![square(4 * M, 2 * M, 4 * M)];
        assert!(!contains_polygon(&outer, &inner));
    }

    #[test]
    fn edge_sampling_catches_concave_bridge() {
        // Two outer parts that share an edge but form an L-shape.
        // Bottom: (0,0)-(10M,5M), Right: (5M,5M)-(10M,10M)
        // Inner triangle whose vertices are all inside but edge crosses the void.
        let outer = vec![
            vec![[0, 0], [10 * M, 0], [10 * M, 5 * M], [0, 5 * M]],
            vec![
                [5 * M, 5 * M],
                [10 * M, 5 * M],
                [10 * M, 10 * M],
                [5 * M, 10 * M],
            ],
        ];
        // All 3 vertices inside parts, but edge (2M,4M)→(6M,6M) crosses void.
        // t=2/3 sample lands at (4_666_666, 5_333_333) — outside both parts.
        let inner = vec![vec![[2 * M, 4 * M], [6 * M, 6 * M], [6 * M, 4 * M]]];
        assert!(!contains_polygon(&outer, &inner));
    }

    #[test]
    fn empty_outer_returns_false() {
        let inner = vec![square(0, 0, M)];
        assert!(!contains_polygon(&[], &inner));
    }

    #[test]
    fn empty_inner_returns_false() {
        let outer = vec![square(0, 0, M)];
        assert!(!contains_polygon(&outer, &[]));
    }

    #[test]
    fn point_inside_any_part_works() {
        let parts = vec![
            square(0, 0, 10 * M),
            square(12 * M, 0, 10 * M),
            square(24 * M, 0, 10 * M),
        ];
        assert!(point_inside_any_part(&parts, 5 * M, 5 * M));
        assert!(point_inside_any_part(&parts, 17 * M, 5 * M));
        assert!(point_inside_any_part(&parts, 12 * M, 5 * M));
        assert!(!point_inside_any_part(&parts, 35 * M, 5 * M));
    }

    #[test]
    fn multipart_inner_fully_contained() {
        let outer = vec![square(0, 0, 20 * M)];
        let inner = vec![square(1 * M, 1 * M, 3 * M), square(5 * M, 5 * M, 3 * M)];
        assert!(contains_polygon(&outer, &inner));
    }

    #[test]
    fn multipart_inner_one_part_outside() {
        let outer = vec![square(0, 0, 10 * M)];
        let inner = vec![square(1 * M, 1 * M, 3 * M), square(15 * M, 15 * M, 3 * M)];
        assert!(!contains_polygon(&outer, &inner));
    }
}
