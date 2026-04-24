//! Convex parts area overlap detection.

/// True if two convex polygons share area (not just edges).
/// Adjacent parts that share an edge have opposite interior sides → return false.
/// Overlapping parts share area → return true.
///
pub fn convex_parts_overlap(a: &[[i64; 2]], b: &[[i64; 2]]) -> bool {
    crate::sat::sat_overlaps(a, b)
}

/// Find all overlapping part pairs between two sets of parts.
/// Returns list of (index_from_a, index_from_b) pairs.
pub fn find_overlapping_parts(
    a_parts: &[Vec<[i64; 2]>],
    b_parts: &[Vec<[i64; 2]>],
) -> Vec<(usize, usize)> {
    let mut overlaps = Vec::new();

    for (i, a) in a_parts.iter().enumerate() {
        for (j, b) in b_parts.iter().enumerate() {
            if convex_parts_overlap(a, b) {
                overlaps.push((i, j));
            }
        }
    }

    overlaps
}

/// True if any part from `a_parts` overlaps any part from `b_parts`.
pub fn parts_overlap(a_parts: &[Vec<[i64; 2]>], b_parts: &[Vec<[i64; 2]>]) -> bool {
    for a in a_parts {
        for b in b_parts {
            if convex_parts_overlap(a, b) {
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

    fn square(ox: i64, oy: i64, size: i64) -> Vec<[i64; 2]> {
        vec![
            [ox, oy],
            [ox + size, oy],
            [ox + size, oy + size],
            [ox, oy + size],
        ]
    }

    #[test]
    fn adjacent_squares_no_overlap() {
        // Adjacent squares sharing edge at x=M — NOT overlapping
        let a = square(0, 0, M);
        let b = square(M, 0, M);
        assert!(
            !convex_parts_overlap(&a, &b),
            "adjacent parts sharing an edge must NOT overlap"
        );
    }

    #[test]
    fn one_pixel_penetration_is_overlap() {
        let a = square(0, 0, M);
        let b = square(M - 1, 0, M);

        assert!(
            convex_parts_overlap(&a, &b),
            "a 1-unit interior intersection must count as overlap"
        );
    }

    #[test]
    fn overlapping_squares_overlap() {
        // Squares overlapping by 1M in each direction
        let a = square(0, 0, 3 * M);
        let b = square(2 * M, 2 * M, 3 * M);
        assert!(convex_parts_overlap(&a, &b));
    }

    #[test]
    fn separated_squares_no_overlap() {
        let a = square(0, 0, M);
        let b = square(3 * M, 0, M);
        assert!(!convex_parts_overlap(&a, &b));
    }

    #[test]
    fn identical_squares_overlap() {
        let square = square(0, 0, M);
        assert!(convex_parts_overlap(&square, &square));
    }

    #[test]
    fn contained_square_overlaps() {
        let outer = square(0, 0, 10 * M);
        let inner = square(2 * M, 2 * M, 3 * M);
        assert!(convex_parts_overlap(&outer, &inner));
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
