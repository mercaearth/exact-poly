//! Point-in-polygon and segment intersection for convex polygons.
//!
//! All algorithms use exact integer arithmetic.
//! Reference implementations: deploy/app/src/lib/overlap.ts (bigint)

use crate::primitives::{cross2d, point_on_segment};

/// True if point (px, py) is strictly inside the convex polygon.
/// Uses cross-product sign consistency: for CCW polygon, point is inside iff
/// it's on the left side of ALL edges (all cross products positive).
///
/// Matches polygon.move::point_inside_convex_part_or_on_boundary behavior
/// but this is strict interior only (returns false for boundary).
/// Reference: overlap.ts::pointStrictlyInsideConvex (lines 7-19)
pub fn point_strictly_inside_convex(px: i64, py: i64, xs: &[i64], ys: &[i64]) -> bool {
    let n = xs.len();
    if n < 3 {
        return false;
    }
    let mut all_pos = true;
    let mut all_neg = true;
    for i in 0..n {
        let j = (i + 1) % n;
        let cp = cross2d(xs[i], ys[i], xs[j], ys[j], px, py);
        if cp <= 0 {
            all_pos = false;
        }
        if cp >= 0 {
            all_neg = false;
        }
    }
    all_pos || all_neg
}

/// True if point (px, py) is on the boundary of the polygon (on any edge).
pub fn point_on_polygon_boundary(px: i64, py: i64, xs: &[i64], ys: &[i64]) -> bool {
    let n = xs.len();
    for i in 0..n {
        let j = (i + 1) % n;
        if point_on_segment(px, py, xs[i], ys[i], xs[j], ys[j]) {
            return true;
        }
    }
    false
}

/// True if point is strictly inside OR on the boundary of a convex polygon.
/// Matches polygon.move::point_inside_convex_part_or_on_boundary.
pub fn point_inside_or_on_boundary(px: i64, py: i64, xs: &[i64], ys: &[i64]) -> bool {
    point_strictly_inside_convex(px, py, xs, ys) || point_on_polygon_boundary(px, py, xs, ys)
}

/// Collinear segment overlap: true if two collinear segments share more than a point.
///
/// Steps:
/// 1. Segments must be parallel (same direction or opposite)
/// 2. Must be collinear (b1 lies on line through a1→a2)
/// 3. 1D projections must strictly overlap (not just touch)
/// 4. Interiors must be on the same side of the shared line (area overlap, not adjacency)
///
/// Reference: overlap.ts::collinearEdgesOverlapArea (lines 38-78)
pub fn collinear_segments_overlap_area(
    a1x: i64,
    a1y: i64,
    a2x: i64,
    a2y: i64,
    b1x: i64,
    b1y: i64,
    b2x: i64,
    b2y: i64,
    a_xs: &[i64],
    a_ys: &[i64], // full polygon A for interior side check
    b_xs: &[i64],
    b_ys: &[i64], // full polygon B
) -> bool {
    let dax = (a2x as i128) - (a1x as i128);
    let day = (a2y as i128) - (a1y as i128);
    let dbx = (b2x as i128) - (b1x as i128);
    let dby = (b2y as i128) - (b1y as i128);

    // Must be parallel: cross of directions == 0
    if dax * dby != day * dbx {
        return false;
    }

    // Must be collinear: b1 lies on the line through a1→a2
    let collinear_check = cross2d(a1x, a1y, a2x, a2y, b1x, b1y);
    if collinear_check != 0 {
        return false;
    }

    // Strict 1D interval overlap along dominant axis
    let has_overlap = if dax != 0 || dbx != 0 {
        // Horizontal-ish: project onto X
        let (a_lo, a_hi) = (a1x.min(a2x), a1x.max(a2x));
        let (b_lo, b_hi) = (b1x.min(b2x), b1x.max(b2x));
        a_lo.max(b_lo) < a_hi.min(b_hi)
    } else {
        // Vertical: project onto Y
        let (a_lo, a_hi) = (a1y.min(a2y), a1y.max(a2y));
        let (b_lo, b_hi) = (b1y.min(b2y), b1y.max(b2y));
        a_lo.max(b_lo) < a_hi.min(b_hi)
    };

    if !has_overlap {
        return false;
    }

    // Interior side check: find first off-line vertex of each polygon
    let mut side_a: i128 = 0;
    for (&x, &y) in a_xs.iter().zip(a_ys.iter()) {
        let cp = cross2d(a1x, a1y, a2x, a2y, x, y);
        if cp != 0 {
            side_a = cp;
            break;
        }
    }

    let mut side_b: i128 = 0;
    for (&x, &y) in b_xs.iter().zip(b_ys.iter()) {
        let cp = cross2d(a1x, a1y, a2x, a2y, x, y);
        if cp != 0 {
            side_b = cp;
            break;
        }
    }

    if side_a == 0 || side_b == 0 {
        return false;
    } // degenerate

    // Same side = interiors interpenetrate = area overlap
    // Opposite side = adjacent parcels sharing edge = no area overlap
    (side_a > 0) == (side_b > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    fn square_xs() -> Vec<i64> {
        vec![0, M, M, 0]
    }
    fn square_ys() -> Vec<i64> {
        vec![0, 0, M, M]
    }

    #[test]
    fn point_strictly_inside() {
        assert!(point_strictly_inside_convex(
            M / 2,
            M / 2,
            &square_xs(),
            &square_ys()
        ));
    }

    #[test]
    fn point_on_boundary_not_strictly_inside() {
        // Point on edge: (0.5M, 0)
        assert!(!point_strictly_inside_convex(
            M / 2,
            0,
            &square_xs(),
            &square_ys()
        ));
    }

    #[test]
    fn point_at_vertex_not_strictly_inside() {
        assert!(!point_strictly_inside_convex(
            0,
            0,
            &square_xs(),
            &square_ys()
        ));
    }

    #[test]
    fn point_outside() {
        assert!(!point_strictly_inside_convex(
            2 * M,
            2 * M,
            &square_xs(),
            &square_ys()
        ));
    }

    #[test]
    fn point_inside_or_on_boundary_interior() {
        assert!(point_inside_or_on_boundary(
            M / 2,
            M / 2,
            &square_xs(),
            &square_ys()
        ));
    }

    #[test]
    fn point_inside_or_on_boundary_edge() {
        assert!(point_inside_or_on_boundary(
            M / 2,
            0,
            &square_xs(),
            &square_ys()
        ));
    }

    #[test]
    fn point_inside_or_on_boundary_vertex() {
        assert!(point_inside_or_on_boundary(
            0,
            0,
            &square_xs(),
            &square_ys()
        ));
    }

    #[test]
    fn point_inside_or_on_boundary_outside() {
        assert!(!point_inside_or_on_boundary(
            2 * M,
            0,
            &square_xs(),
            &square_ys()
        ));
    }

    #[test]
    fn collinear_overlap_same_side_returns_true() {
        // Two squares overlapping via collinear edges
        // Polygon A: bottom row (0,0)→(2M,0)→(2M,M)→(0,M)
        // Polygon B: same bottom edge but offset right: (M,0)→(3M,0)→(3M,M)→(M,M)
        // Edge A: (0,0)→(2M,0), Edge B: (M,0)→(3M,0) — same horizontal line, overlap at M..2M
        // Both polygons are ABOVE the shared edge line (same side)
        let a_xs = vec![0, 2 * M, 2 * M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 3 * M, 3 * M, M];
        let b_ys = vec![0, 0, M, M];

        let result = collinear_segments_overlap_area(
            0,
            0,
            2 * M,
            0, // edge from A
            M,
            0,
            3 * M,
            0, // edge from B
            &a_xs,
            &a_ys,
            &b_xs,
            &b_ys,
        );
        // Both rectangles are above y=0 — same side — area overlap → true
        assert!(result);
    }

    #[test]
    fn adjacent_polygons_opposite_sides_no_overlap() {
        // Two rectangles sharing the x-axis edge: A above, B below
        let a_xs = vec![0, 2 * M, 2 * M, 0];
        let a_ys = vec![0, 0, M, M]; // above y=0
        let b_xs = vec![0, 2 * M, 2 * M, 0];
        let b_ys = vec![0, 0, -M, -M]; // below y=0

        let result = collinear_segments_overlap_area(
            0,
            0,
            2 * M,
            0,
            0,
            0,
            2 * M,
            0,
            &a_xs,
            &a_ys,
            &b_xs,
            &b_ys,
        );
        // Opposite sides → adjacent parcels → no area overlap → false
        assert!(!result);
    }

    #[test]
    fn non_collinear_segments_return_false() {
        // Perpendicular edges — not collinear
        let a_xs = vec![0, M, M, 0]; // square A
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![2 * M, 3 * M, 3 * M, 2 * M]; // square B far away
        let b_ys = vec![0, 0, M, M];

        let result = collinear_segments_overlap_area(
            0,
            0,
            M,
            0, // horizontal edge from A
            2 * M,
            0,
            2 * M,
            M, // vertical edge from B — NOT collinear
            &a_xs,
            &a_ys,
            &b_xs,
            &b_ys,
        );
        assert!(!result);
    }
}
