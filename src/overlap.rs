//! Convex parts area overlap detection.
//!
//! Distinguishes area overlap from edge sharing (adjacent parts).
//! Adjacent parts share edges but have NO area overlap.
//!
//! Reference: deploy/app/src/lib/overlap.ts

use crate::aabb::Aabb;
use crate::spatial::{collinear_segments_overlap_area, point_strictly_inside_convex};

/// True if two convex polygons share area (not just edges).
/// Adjacent parts that share an edge have opposite interior sides → return false.
/// Overlapping parts share area → return true.
///
/// Algorithm (matching overlap.ts::convexPartsOverlap):
/// 1. Check if any vertex of B is strictly inside A
/// 2. Check if any vertex of A is strictly inside B
/// 3. Check all edge pairs for proper intersection
/// 4. Check all collinear edge pairs for area overlap (same-side interior check)
pub fn convex_parts_overlap(a_xs: &[i64], a_ys: &[i64], b_xs: &[i64], b_ys: &[i64]) -> bool {
    // AABB pre-filter: if bounding boxes don't overlap, no need for detailed check
    let aabb_a = Aabb::from_vertices(a_xs, a_ys);
    let aabb_b = Aabb::from_vertices(b_xs, b_ys);
    if !aabb_a.intersects(&aabb_b) {
        return false;
    }

    // Check if any vertex of B is strictly inside A
    for (&bx, &by) in b_xs.iter().zip(b_ys.iter()) {
        if point_strictly_inside_convex(bx, by, a_xs, a_ys) {
            return true;
        }
    }

    // Check if any vertex of A is strictly inside B
    for (&ax, &ay) in a_xs.iter().zip(a_ys.iter()) {
        if point_strictly_inside_convex(ax, ay, b_xs, b_ys) {
            return true;
        }
    }

    // Check all edge pairs
    let na = a_xs.len();
    let nb = b_xs.len();

    for i in 0..na {
        let ni = (i + 1) % na;
        for j in 0..nb {
            let nj = (j + 1) % nb;

            // Proper intersection (edges cross each other)
            if crate::primitives::segments_properly_intersect(
                a_xs[i], a_ys[i], a_xs[ni], a_ys[ni], b_xs[j], b_ys[j], b_xs[nj], b_ys[nj],
            ) {
                return true;
            }

            // Collinear area overlap (same-side interiors)
            if collinear_segments_overlap_area(
                a_xs[i], a_ys[i], a_xs[ni], a_ys[ni], b_xs[j], b_ys[j], b_xs[nj], b_ys[nj], a_xs,
                a_ys, b_xs, b_ys,
            ) {
                return true;
            }
        }
    }

    false
}

/// Find all overlapping part pairs between two sets of parts.
/// Returns list of (index_from_a, index_from_b) pairs.
pub fn find_overlapping_parts(
    a_parts: &[Vec<[i64; 2]>],
    b_parts: &[Vec<[i64; 2]>],
) -> Vec<(usize, usize)> {
    let mut overlaps = Vec::new();

    for (i, a) in a_parts.iter().enumerate() {
        let a_xs: Vec<i64> = a.iter().map(|v| v[0]).collect();
        let a_ys: Vec<i64> = a.iter().map(|v| v[1]).collect();

        for (j, b) in b_parts.iter().enumerate() {
            let b_xs: Vec<i64> = b.iter().map(|v| v[0]).collect();
            let b_ys: Vec<i64> = b.iter().map(|v| v[1]).collect();

            if convex_parts_overlap(&a_xs, &a_ys, &b_xs, &b_ys) {
                overlaps.push((i, j));
            }
        }
    }

    overlaps
}

/// True if any part from `a_parts` overlaps any part from `b_parts`.
pub fn parts_overlap(a_parts: &[Vec<[i64; 2]>], b_parts: &[Vec<[i64; 2]>]) -> bool {
    for a in a_parts {
        let a_xs: Vec<i64> = a.iter().map(|v| v[0]).collect();
        let a_ys: Vec<i64> = a.iter().map(|v| v[1]).collect();

        for b in b_parts {
            let b_xs: Vec<i64> = b.iter().map(|v| v[0]).collect();
            let b_ys: Vec<i64> = b.iter().map(|v| v[1]).collect();

            if convex_parts_overlap(&a_xs, &a_ys, &b_xs, &b_ys) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    fn sq_xs(ox: i64, size: i64) -> Vec<i64> {
        vec![ox, ox + size, ox + size, ox]
    }
    fn sq_ys(oy: i64, size: i64) -> Vec<i64> {
        vec![oy, oy, oy + size, oy + size]
    }

    #[test]
    fn adjacent_squares_no_overlap() {
        // Adjacent squares sharing edge at x=M — NOT overlapping
        let (a_xs, a_ys) = (sq_xs(0, M), sq_ys(0, M));
        let (b_xs, b_ys) = (sq_xs(M, M), sq_ys(0, M));
        assert!(
            !convex_parts_overlap(&a_xs, &a_ys, &b_xs, &b_ys),
            "adjacent parts sharing an edge must NOT overlap"
        );
    }

    #[test]
    fn one_pixel_penetration_is_overlap() {
        let (a_xs, a_ys) = (sq_xs(0, M), sq_ys(0, M));
        let (b_xs, b_ys) = (sq_xs(M - 1, M), sq_ys(0, M));

        assert!(
            convex_parts_overlap(&a_xs, &a_ys, &b_xs, &b_ys),
            "a 1-unit interior intersection must count as overlap"
        );
    }

    #[test]
    fn overlapping_squares_overlap() {
        // Squares overlapping by 1M in each direction
        let (a_xs, a_ys) = (sq_xs(0, 3 * M), sq_ys(0, 3 * M));
        let (b_xs, b_ys) = (sq_xs(2 * M, 3 * M), sq_ys(2 * M, 3 * M));
        assert!(convex_parts_overlap(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn separated_squares_no_overlap() {
        let (a_xs, a_ys) = (sq_xs(0, M), sq_ys(0, M));
        let (b_xs, b_ys) = (sq_xs(3 * M, M), sq_ys(0, M));
        assert!(!convex_parts_overlap(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn identical_squares_overlap() {
        let (xs, ys) = (sq_xs(0, M), sq_ys(0, M));
        assert!(convex_parts_overlap(&xs, &ys, &xs, &ys));
    }

    #[test]
    fn contained_square_overlaps() {
        let (outer_xs, outer_ys) = (sq_xs(0, 10 * M), sq_ys(0, 10 * M));
        let (inner_xs, inner_ys) = (sq_xs(2 * M, 3 * M), sq_ys(2 * M, 3 * M));
        assert!(convex_parts_overlap(
            &outer_xs, &outer_ys, &inner_xs, &inner_ys
        ));
    }

    #[test]
    fn parts_overlap_multipart() {
        let a_parts = vec![vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]]];
        let b_parts_overlap = vec![vec![
            [5 * M, 5 * M],
            [15 * M, 5 * M],
            [15 * M, 15 * M],
            [5 * M, 15 * M],
        ]];
        let b_parts_adjacent = vec![vec![
            [10 * M, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
        ]];
        assert!(parts_overlap(&a_parts, &b_parts_overlap));
        assert!(!parts_overlap(&a_parts, &b_parts_adjacent));
    }

    #[test]
    fn find_overlapping_parts_returns_empty_when_disjoint() {
        let a_parts = vec![
            vec![[0, 0], [M, 0], [M, M], [0, M]],
            vec![[3 * M, 0], [4 * M, 0], [4 * M, M], [3 * M, M]],
            vec![[6 * M, 0], [7 * M, 0], [7 * M, M], [6 * M, M]],
        ];
        let b_parts = vec![
            vec![[0, 3 * M], [M, 3 * M], [M, 4 * M], [0, 4 * M]],
            vec![
                [3 * M, 3 * M],
                [4 * M, 3 * M],
                [4 * M, 4 * M],
                [3 * M, 4 * M],
            ],
            vec![
                [6 * M, 3 * M],
                [7 * M, 3 * M],
                [7 * M, 4 * M],
                [6 * M, 4 * M],
            ],
        ];

        assert!(find_overlapping_parts(&a_parts, &b_parts).is_empty());
    }

    #[test]
    fn find_overlapping_parts_returns_matching_indices() {
        let a_parts = vec![
            vec![[0, 0], [M, 0], [M, M], [0, M]],
            vec![[3 * M, 0], [4 * M, 0], [4 * M, M], [3 * M, M]],
            vec![[6 * M, 0], [7 * M, 0], [7 * M, M], [6 * M, M]],
        ];
        let b_parts = vec![
            vec![[0, 3 * M], [M, 3 * M], [M, 4 * M], [0, 4 * M]],
            vec![
                [3 * M, 3 * M],
                [4 * M, 3 * M],
                [4 * M, 4 * M],
                [3 * M, 4 * M],
            ],
            vec![
                [6 * M - 1, 0],
                [7 * M - 1, 0],
                [7 * M - 1, M],
                [6 * M - 1, M],
            ],
        ];

        assert_eq!(find_overlapping_parts(&a_parts, &b_parts), vec![(2, 2)]);
    }

    #[test]
    fn parts_overlap_false_when_all_pairs_separated() {
        let a_parts = vec![
            vec![[0, 0], [M, 0], [M, M], [0, M]],
            vec![[3 * M, 0], [4 * M, 0], [4 * M, M], [3 * M, M]],
        ];
        let b_parts = vec![
            vec![[0, 3 * M], [M, 3 * M], [M, 4 * M], [0, 4 * M]],
            vec![
                [3 * M, 3 * M],
                [4 * M, 3 * M],
                [4 * M, 4 * M],
                [3 * M, 4 * M],
            ],
        ];

        assert!(!parts_overlap(&a_parts, &b_parts));
    }

    #[test]
    fn parts_overlap_true_when_one_pair_overlaps() {
        let a_parts = vec![
            vec![[0, 0], [M, 0], [M, M], [0, M]],
            vec![[3 * M, 0], [4 * M, 0], [4 * M, M], [3 * M, M]],
        ];
        let b_parts = vec![
            vec![[0, 3 * M], [M, 3 * M], [M, 4 * M], [0, 4 * M]],
            vec![[M - 1, 0], [2 * M - 1, 0], [2 * M - 1, M], [M - 1, M]],
        ];

        assert!(parts_overlap(&a_parts, &b_parts));
    }
}
