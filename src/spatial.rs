//! Point-in-polygon and segment intersection for convex polygons.

use crate::primitives::{cross2d, point_on_segment};

/// True if point (px, py) is strictly inside the convex polygon.
/// Uses cross-product sign consistency: for CCW polygon, point is inside iff
/// it's on the left side of ALL edges (all cross products positive).
///
/// Matches polygon.move::point_inside_convex_part_or_on_boundary behavior
/// but this is strict interior only (returns false for boundary).
/// Reference: overlap.ts::pointStrictlyInsideConvex (lines 7-19)
pub fn point_strictly_inside_convex(px: i64, py: i64, ring: &[[i64; 2]]) -> bool {
    let n = ring.len();
    if n < 3 {
        return false;
    }
    let mut all_pos = true;
    let mut all_neg = true;
    for i in 0..n {
        let j = (i + 1) % n;
        let cp = cross2d(ring[i][0], ring[i][1], ring[j][0], ring[j][1], px, py);
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
pub fn point_on_polygon_boundary(px: i64, py: i64, ring: &[[i64; 2]]) -> bool {
    let n = ring.len();
    for i in 0..n {
        let j = (i + 1) % n;
        if point_on_segment(px, py, ring[i][0], ring[i][1], ring[j][0], ring[j][1]) {
            return true;
        }
    }
    false
}

/// Ray-casting point-in-polygon for general (including non-convex) polygons.
/// Counts how many times a rightward ray from (px, py) crosses the polygon edges.
/// Odd count = inside. Uses exact integer arithmetic — no division, no float.
fn point_inside_polygon_ray_cast(px: i64, py: i64, ring: &[[i64; 2]]) -> bool {
    let n = ring.len();
    if n < 3 {
        return false;
    }
    let mut crossings = 0i32;
    for i in 0..n {
        let j = (i + 1) % n;
        let ax = ring[i][0];
        let ay = ring[i][1];
        let bx = ring[j][0];
        let by = ring[j][1];
        // Edge crosses the ray's horizontal level (asymmetric to handle vertices correctly)
        if (ay > py) != (by > py) {
            // x_intersect = ax + (py - ay) * (bx - ax) / (by - ay)
            // x_intersect > px  without division:
            //   (py - ay) * (bx - ax) [vs] (px - ax) * (by - ay)
            //   flip inequality when (by - ay) < 0
            let lhs = (py as i128 - ay as i128) * (bx as i128 - ax as i128);
            let rhs = (px as i128 - ax as i128) * (by as i128 - ay as i128);
            let to_right = if by > ay { lhs > rhs } else { lhs < rhs };
            if to_right {
                crossings += 1;
            }
        }
    }
    crossings % 2 == 1
}

/// True if point is inside OR on the boundary of a polygon (convex or non-convex).
pub fn point_inside_or_on_boundary(px: i64, py: i64, ring: &[[i64; 2]]) -> bool {
    point_on_polygon_boundary(px, py, ring) || point_inside_polygon_ray_cast(px, py, ring)
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
    a_ring: &[[i64; 2]], // full polygon A for interior side check
    b_ring: &[[i64; 2]], // full polygon B
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
    for point in a_ring {
        let cp = cross2d(a1x, a1y, a2x, a2y, point[0], point[1]);
        if cp != 0 {
            side_a = cp;
            break;
        }
    }

    let mut side_b: i128 = 0;
    for point in b_ring {
        let cp = cross2d(a1x, a1y, a2x, a2y, point[0], point[1]);
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

    fn square() -> Vec<[i64; 2]> {
        vec![[0, 0], [M, 0], [M, M], [0, M]]
    }

    fn rhombus() -> Vec<[i64; 2]> {
        vec![[0, 4 * M], [2 * M, 0], [0, -4 * M], [-2 * M, 0]]
    }

    #[test]
    fn point_strictly_inside() {
        assert!(point_strictly_inside_convex(M / 2, M / 2, &square()));
    }

    #[test]
    fn point_strictly_inside_convex_rhombus_centroid_and_edge_neighbors() {
        let ring = rhombus();

        assert!(point_strictly_inside_convex(0, 0, &ring));

        let edge_mid_x = M;
        let edge_mid_y = 2 * M;
        assert!(point_strictly_inside_convex(
            edge_mid_x - 2,
            edge_mid_y - 1,
            &ring
        ));
        assert!(!point_strictly_inside_convex(
            edge_mid_x + 2,
            edge_mid_y + 1,
            &ring
        ));
    }

    #[test]
    fn point_on_boundary_not_strictly_inside() {
        // Point on edge: (0.5M, 0)
        assert!(!point_strictly_inside_convex(M / 2, 0, &square()));
    }

    #[test]
    fn point_at_vertex_not_strictly_inside() {
        assert!(!point_strictly_inside_convex(0, 0, &square()));
    }

    #[test]
    fn point_outside() {
        assert!(!point_strictly_inside_convex(2 * M, 2 * M, &square()));
    }

    #[test]
    fn point_inside_or_on_boundary_interior() {
        assert!(point_inside_or_on_boundary(M / 2, M / 2, &square()));
    }

    #[test]
    fn point_inside_or_on_boundary_edge() {
        assert!(point_inside_or_on_boundary(M / 2, 0, &square()));
    }

    #[test]
    fn point_inside_or_on_boundary_vertex() {
        assert!(point_inside_or_on_boundary(0, 0, &square()));
    }

    #[test]
    fn point_inside_or_on_boundary_outside() {
        assert!(!point_inside_or_on_boundary(2 * M, 0, &square()));
    }

    #[test]
    fn point_on_polygon_boundary_edge_midpoint_and_off_edge() {
        let ring = rhombus();

        assert!(point_on_polygon_boundary(M, 2 * M, &ring));
        assert!(!point_on_polygon_boundary(M + 2, 2 * M + 1, &ring));
    }

    #[test]
    fn point_inside_or_on_boundary_inclusive_cases() {
        let ring = rhombus();

        assert!(point_inside_or_on_boundary(0, 4 * M, &ring));
        assert!(point_inside_or_on_boundary(M, 2 * M, &ring));
        assert!(point_inside_or_on_boundary(0, 0, &ring));
        assert!(!point_inside_or_on_boundary(3 * M, 3 * M, &ring));
    }

    // L-shape: non-convex polygon — the original convex algorithm failed here
    fn l_shape() -> Vec<[i64; 2]> {
        vec![
            [0, 0], [60 * M, 0], [60 * M, 40 * M], [30 * M, 40 * M],
            [30 * M, 80 * M], [0, 80 * M],
        ]
    }

    #[test]
    fn point_inside_or_on_boundary_non_convex_l_shape_interior() {
        let ring = l_shape();
        // Bottom-left interior
        assert!(point_inside_or_on_boundary(15 * M, 20 * M, &ring));
        // Upper-left interior (above the step)
        assert!(point_inside_or_on_boundary(15 * M, 60 * M, &ring));
        // Outside: inside the "missing" rectangle top-right
        assert!(!point_inside_or_on_boundary(45 * M, 60 * M, &ring));
    }

    #[test]
    fn ray_cast_non_convex_star_interior() {
        // Simple concave shape: arrow-like [[0,30],[60,30],[60,0],[100,50],[60,100],[60,70],[0,70]]
        let arrow: Vec<[i64; 2]> = vec![
            [0, 30 * M], [60 * M, 30 * M], [60 * M, 0],
            [100 * M, 50 * M], [60 * M, 100 * M], [60 * M, 70 * M], [0, 70 * M],
        ];
        // Interior of the shaft
        assert!(point_inside_or_on_boundary(30 * M, 50 * M, &arrow));
        // Outside (above shaft, left of arrowhead)
        assert!(!point_inside_or_on_boundary(30 * M, 90 * M, &arrow));
    }

    #[test]
    fn collinear_overlap_same_side_returns_true() {
        // Two squares overlapping via collinear edges
        // Polygon A: bottom row (0,0)→(2M,0)→(2M,M)→(0,M)
        // Polygon B: same bottom edge but offset right: (M,0)→(3M,0)→(3M,M)→(M,M)
        // Edge A: (0,0)→(2M,0), Edge B: (M,0)→(3M,0) — same horizontal line, overlap at M..2M
        // Both polygons are ABOVE the shared edge line (same side)
        let a_ring = vec![[0, 0], [2 * M, 0], [2 * M, M], [0, M]];
        let b_ring = vec![[M, 0], [3 * M, 0], [3 * M, M], [M, M]];

        let result = collinear_segments_overlap_area(
            0,
            0,
            2 * M,
            0, // edge from A
            M,
            0,
            3 * M,
            0, // edge from B
            &a_ring,
            &b_ring,
        );
        // Both rectangles are above y=0 — same side — area overlap → true
        assert!(result);
    }

    #[test]
    fn adjacent_polygons_opposite_sides_no_overlap() {
        // Two rectangles sharing the x-axis edge: A above, B below
        let a_ring = vec![[0, 0], [2 * M, 0], [2 * M, M], [0, M]]; // above y=0
        let b_ring = vec![[0, 0], [2 * M, 0], [2 * M, -M], [0, -M]]; // below y=0

        let result =
            collinear_segments_overlap_area(0, 0, 2 * M, 0, 0, 0, 2 * M, 0, &a_ring, &b_ring);
        // Opposite sides → adjacent parcels → no area overlap → false
        assert!(!result);
    }

    #[test]
    fn non_collinear_segments_return_false() {
        // Perpendicular edges — not collinear
        let a_ring = vec![[0, 0], [M, 0], [M, M], [0, M]]; // square A
        let b_ring = vec![[2 * M, 0], [3 * M, 0], [3 * M, M], [2 * M, M]]; // square B far away

        let result = collinear_segments_overlap_area(
            0,
            0,
            M,
            0, // horizontal edge from A
            2 * M,
            0,
            2 * M,
            M, // vertical edge from B — NOT collinear
            &a_ring,
            &b_ring,
        );
        assert!(!result);
    }
}
