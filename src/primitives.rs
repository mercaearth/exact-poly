//! Fundamental integer geometry primitives.
//! All coordinates are i64 (fixed-point). All intermediate computations use i128.
//! ZERO floating point. ZERO epsilon. Exact integer arithmetic throughout.
//!
//! Ground truth: deploy/app/src/lib/overlap.ts (bigint implementation)
//! On-chain reference: deploy/onchain/protocol/sources/math/signed.move

/// Orientation of three points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    CounterClockwise,
    Clockwise,
    Collinear,
}

/// 2D cross product of vectors (A→B) × (A→C).
/// Equivalent to: (bx-ax)*(cy-ay) - (by-ay)*(cx-ax)
/// Positive = CCW (left turn), Negative = CW (right turn), Zero = collinear.
/// Casts to i128 before multiply — no overflow for coords up to MAX_WORLD=40_075_017_000_000.
pub fn cross2d(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> i128 {
    let dx1 = (bx as i128) - (ax as i128);
    let dy1 = (by as i128) - (ay as i128);
    let dx2 = (cx as i128) - (ax as i128);
    let dy2 = (cy as i128) - (ay as i128);

    dx1 * dy2 - dy1 * dx2
}

/// Orientation of three points A, B, C.
pub fn orientation(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> Orientation {
    match cross2d(ax, ay, bx, by, cx, cy).cmp(&0) {
        std::cmp::Ordering::Greater => Orientation::CounterClockwise,
        std::cmp::Ordering::Less => Orientation::Clockwise,
        std::cmp::Ordering::Equal => Orientation::Collinear,
    }
}

/// True if P is strictly left of line A→B (cross product > 0 for CCW polygon).
pub fn is_left(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    cross2d(ax, ay, bx, by, px, py) > 0
}

/// True if P is strictly left of or on line A→B.
pub fn is_left_or_on(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    cross2d(ax, ay, bx, by, px, py) >= 0
}

/// True if P is strictly right of line A→B.
pub fn is_right(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    cross2d(ax, ay, bx, by, px, py) < 0
}

/// True if P is strictly right of or on line A→B.
pub fn is_right_or_on(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    cross2d(ax, ay, bx, by, px, py) <= 0
}

/// True if P is collinear with A and B (cross product == 0).
pub fn is_collinear_pts(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> bool {
    cross2d(ax, ay, bx, by, px, py) == 0
}

/// True if vertex `curr` is a reflex vertex in a CCW polygon.
/// A reflex vertex has a CW turn (cross product < 0) in a CCW polygon.
/// prev → curr → next should be a left turn for convex, right turn for reflex.
pub fn is_reflex(
    prev_x: i64,
    prev_y: i64,
    curr_x: i64,
    curr_y: i64,
    next_x: i64,
    next_y: i64,
) -> bool {
    cross2d(prev_x, prev_y, curr_x, curr_y, next_x, next_y) < 0
}

/// Squared Euclidean distance between two points (using i128 to avoid overflow).
/// Returns dx² + dy². Used for edge length validation (compare against MIN_EDGE_LENGTH_SQUARED).
pub fn edge_squared_length(ax: i64, ay: i64, bx: i64, by: i64) -> u128 {
    let dx = (bx as i128) - (ax as i128);
    let dy = (by as i128) - (ay as i128);

    (dx * dx + dy * dy) as u128
}

/// True if point P lies on segment A→B (collinear AND within the bounding box of A→B).
pub fn point_on_segment(px: i64, py: i64, ax: i64, ay: i64, bx: i64, by: i64) -> bool {
    if cross2d(ax, ay, bx, by, px, py) != 0 {
        return false;
    }

    px >= ax.min(bx) && px <= ax.max(bx) && py >= ay.min(by) && py <= ay.max(by)
}

/// True if segments A1→A2 and B1→B2 properly intersect (cross each other, not collinear).
/// Proper intersection: each segment straddles the other's line (both endpoints on opposite sides).
/// Does NOT count T-junctions (endpoint on segment) as proper intersection.
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
    let d1 = cross2d(b1x, b1y, b2x, b2y, a1x, a1y);
    let d2 = cross2d(b1x, b1y, b2x, b2y, a2x, a2y);
    let d3 = cross2d(a1x, a1y, a2x, a2y, b1x, b1y);
    let d4 = cross2d(a1x, a1y, a2x, a2y, b2x, b2y);

    ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
}

/// True if segments A1→A2 and B1→B2 intersect (proper crossing OR endpoint on segment).
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
    if segments_properly_intersect(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y) {
        return true;
    }

    point_on_segment(b1x, b1y, a1x, a1y, a2x, a2y)
        || point_on_segment(b2x, b2y, a1x, a1y, a2x, a2y)
        || point_on_segment(a1x, a1y, b1x, b1y, b2x, b2y)
        || point_on_segment(a2x, a2y, b1x, b1y, b2x, b2y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;
    const MAX_WORLD: i64 = 40_075_017_000_000;

    #[test]
    fn cross2d_sign_and_collinearity_cases() {
        assert!(cross2d(0, 0, M, 0, 0, M) > 0);
        assert!(cross2d(0, 0, 0, M, M, 0) < 0);
        assert_eq!(cross2d(0, 0, M, 0, 2 * M, 0), 0);
    }

    #[test]
    fn cross2d_matches_expected_magnitude() {
        assert_eq!(
            cross2d(0, 0, 3 * M, 0, 0, 4 * M),
            12 * (M as i128) * (M as i128)
        );
        assert_eq!(cross2d(-M, -M, M, -M, -M, M), 4 * (M as i128) * (M as i128));
        assert_eq!(
            cross2d(5 * M, 5 * M, 6 * M, 7 * M, 9 * M, 12 * M),
            -(M as i128) * (M as i128)
        );
    }

    #[test]
    fn cross2d_handles_max_world_without_overflow() {
        let result = cross2d(0, 0, MAX_WORLD, 0, 0, MAX_WORLD);
        let expected = (MAX_WORLD as i128) * (MAX_WORLD as i128);
        assert_eq!(result, expected);
        assert!(cross2d(-MAX_WORLD, 0, MAX_WORLD, 0, 0, MAX_WORLD) > 0);
        assert_eq!(
            cross2d(0, 0, MAX_WORLD, MAX_WORLD, 2 * MAX_WORLD, 2 * MAX_WORLD),
            0
        );
    }

    #[test]
    fn orientation_classifies_three_cases() {
        assert_eq!(orientation(0, 0, M, 0, 0, M), Orientation::CounterClockwise);
        assert_eq!(orientation(0, 0, 0, M, M, 0), Orientation::Clockwise);
        assert_eq!(
            orientation(0, 0, M, M, 2 * M, 2 * M),
            Orientation::Collinear
        );
    }

    #[test]
    fn orientation_handles_shifted_and_negative_points() {
        assert_eq!(
            orientation(-M, -M, M, -M, 0, M),
            Orientation::CounterClockwise
        );
        assert_eq!(orientation(-M, -M, 0, M, M, -M), Orientation::Clockwise);
        assert_eq!(
            orientation(-2 * M, -2 * M, -M, -M, 0, 0),
            Orientation::Collinear
        );
    }

    #[test]
    fn orientation_handles_degenerate_repeated_points() {
        assert_eq!(orientation(0, 0, 0, 0, M, M), Orientation::Collinear);
        assert_eq!(orientation(0, 0, M, M, M, M), Orientation::Collinear);
        assert_eq!(orientation(7, 11, 7, 11, 7, 11), Orientation::Collinear);
    }

    #[test]
    fn side_predicates_classify_left_right_and_on() {
        assert!(is_left(0, 0, M, 0, 0, M));
        assert!(!is_left(0, 0, M, 0, M, 0));
        assert!(!is_left(0, 0, M, 0, 0, -M));

        assert!(is_left_or_on(0, 0, M, 0, 0, M));
        assert!(is_left_or_on(0, 0, M, 0, M, 0));
        assert!(!is_left_or_on(0, 0, M, 0, 0, -M));

        assert!(is_right(0, 0, M, 0, 0, -M));
        assert!(!is_right(0, 0, M, 0, M, 0));
        assert!(!is_right(0, 0, M, 0, 0, M));

        assert!(is_right_or_on(0, 0, M, 0, 0, -M));
        assert!(is_right_or_on(0, 0, M, 0, M, 0));
        assert!(!is_right_or_on(0, 0, M, 0, 0, M));

        assert!(is_collinear_pts(0, 0, M, M, 2 * M, 2 * M));
        assert!(is_collinear_pts(0, 0, 2 * M, 0, M, 0));
        assert!(!is_collinear_pts(0, 0, M, 0, M, M));
    }

    #[test]
    fn side_predicates_work_with_vertical_lines() {
        assert!(is_left(0, 0, 0, M, -M, 0));
        assert!(is_left_or_on(0, 0, 0, M, 0, M / 2));
        assert!(is_right(0, 0, 0, M, M, 0));
        assert!(is_right_or_on(0, 0, 0, M, 0, M / 2));
        assert!(is_collinear_pts(0, 0, 0, 2 * M, 0, M));
    }

    #[test]
    fn side_predicates_work_with_negative_coordinates() {
        assert!(is_left(-M, -M, M, -M, 0, 0));
        assert!(is_left_or_on(-M, -M, M, -M, -M, -M));
        assert!(is_right(-M, -M, M, -M, 0, -2 * M));
        assert!(is_right_or_on(-M, -M, M, -M, M, -M));
        assert!(!is_collinear_pts(-M, -M, M, -M, 0, 0));
    }

    #[test]
    fn is_reflex_distinguishes_concave_and_convex_vertices() {
        assert!(is_reflex(0, 0, M, M, 2 * M, 0));
        assert!(!is_reflex(0, 2 * M, 0, 0, 2 * M, 0));
        assert!(!is_reflex(0, 0, M, 0, 2 * M, 0));
    }

    #[test]
    fn is_reflex_handles_shifted_vertices() {
        assert!(is_reflex(-2 * M, -2 * M, -M, -M, 0, -2 * M));
        assert!(!is_reflex(-2 * M, -2 * M, -M, -2 * M, -M, -M));
        assert!(!is_reflex(5 * M, 5 * M, 6 * M, 6 * M, 7 * M, 7 * M));
    }

    #[test]
    fn is_reflex_uses_exact_orientation_semantics() {
        assert_eq!(is_reflex(0, 0, M, 0, M, M), false);
        assert_eq!(is_reflex(0, 0, M, 0, M, -M), true);
        assert_eq!(is_reflex(0, 0, M, 0, 2 * M, 0), false);
    }

    #[test]
    fn edge_squared_length_matches_known_distances() {
        assert_eq!(
            edge_squared_length(0, 0, 3 * M, 4 * M),
            25 * (M as u128) * (M as u128)
        );
        assert_eq!(edge_squared_length(0, 0, M, 0), (M as u128) * (M as u128));
        assert_eq!(
            edge_squared_length(-M, -M, M, M),
            8 * (M as u128) * (M as u128)
        );
    }

    #[test]
    fn edge_squared_length_is_symmetric_and_zero_for_same_point() {
        let ab = edge_squared_length(-7, 11, 13, -17);
        let ba = edge_squared_length(13, -17, -7, 11);
        assert_eq!(ab, ba);
        assert_eq!(edge_squared_length(5, 5, 5, 5), 0);
        assert_eq!(
            edge_squared_length(-M, 0, -M, 3 * M),
            9 * (M as u128) * (M as u128)
        );
    }

    #[test]
    fn edge_squared_length_handles_max_world() {
        let len_sq = edge_squared_length(0, 0, MAX_WORLD, MAX_WORLD);
        assert_eq!(len_sq, 2 * (MAX_WORLD as u128) * (MAX_WORLD as u128));
        assert!(len_sq > 0);
        assert_eq!(
            edge_squared_length(-MAX_WORLD, 0, MAX_WORLD, 0),
            4 * (MAX_WORLD as u128) * (MAX_WORLD as u128)
        );
    }

    #[test]
    fn point_on_segment_handles_midpoint_endpoint_and_outside() {
        assert!(point_on_segment(M, 0, 0, 0, 2 * M, 0));
        assert!(point_on_segment(0, 0, 0, 0, 2 * M, 0));
        assert!(!point_on_segment(3 * M, 0, 0, 0, 2 * M, 0));
    }

    #[test]
    fn point_on_segment_handles_diagonal_and_vertical_segments() {
        assert!(point_on_segment(M, M, 0, 0, 2 * M, 2 * M));
        assert!(point_on_segment(0, M, 0, 0, 0, 2 * M));
        assert!(!point_on_segment(M, M + 1, 0, 0, 2 * M, 2 * M));
    }

    #[test]
    fn point_on_segment_handles_reversed_bounds() {
        assert!(point_on_segment(M, 0, 2 * M, 0, 0, 0));
        assert!(point_on_segment(0, M, 0, 2 * M, 0, 0));
        assert!(!point_on_segment(-1, 0, 2 * M, 0, 0, 0));
    }

    #[test]
    fn segments_properly_intersect_detects_crossings() {
        assert!(segments_properly_intersect(
            0,
            0,
            2 * M,
            2 * M,
            0,
            2 * M,
            2 * M,
            0
        ));
        assert!(segments_properly_intersect(0, 0, 3 * M, 0, M, -M, M, M));
        assert!(segments_properly_intersect(-M, -M, M, M, -M, M, M, -M));
    }

    #[test]
    fn segments_properly_intersect_excludes_parallel_touching_and_collinear_cases() {
        assert!(!segments_properly_intersect(0, 0, 2 * M, 0, 0, M, 2 * M, M));
        assert!(!segments_properly_intersect(0, 0, 2 * M, 0, M, 0, M, 2 * M));
        assert!(!segments_properly_intersect(0, 0, 2 * M, 0, M, 0, 3 * M, 0));
    }

    #[test]
    fn segments_properly_intersect_excludes_separated_segments() {
        assert!(!segments_properly_intersect(
            0,
            0,
            M,
            M,
            2 * M,
            2 * M,
            3 * M,
            3 * M
        ));
        assert!(!segments_properly_intersect(0, 0, 0, M, M, 0, M, M));
        assert!(!segments_properly_intersect(
            -2 * M,
            0,
            -M,
            0,
            M,
            0,
            2 * M,
            0
        ));
    }

    #[test]
    fn segments_intersect_detects_proper_and_endpoint_intersections() {
        assert!(segments_intersect(0, 0, 2 * M, 2 * M, 0, 2 * M, 2 * M, 0));
        assert!(segments_intersect(0, 0, 2 * M, 0, M, 0, M, 2 * M));
        assert!(segments_intersect(0, 0, 2 * M, 0, 2 * M, 0, 3 * M, 0));
    }

    #[test]
    fn segments_intersect_detects_collinear_overlap_and_containment() {
        assert!(segments_intersect(0, 0, 4 * M, 0, M, 0, 3 * M, 0));
        assert!(segments_intersect(M, 0, 3 * M, 0, 0, 0, 4 * M, 0));
        assert!(segments_intersect(0, 0, 0, 4 * M, 0, M, 0, 3 * M));
    }

    #[test]
    fn segments_intersect_excludes_disjoint_segments() {
        assert!(!segments_intersect(0, 0, M, 0, 2 * M, 0, 3 * M, 0));
        assert!(!segments_intersect(0, 0, 0, M, M, 0, M, M));
        assert!(!segments_intersect(
            -3 * M,
            -3 * M,
            -2 * M,
            -2 * M,
            2 * M,
            2 * M,
            3 * M,
            3 * M
        ));
    }
}
