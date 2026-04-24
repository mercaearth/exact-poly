//! Shared edge detection and collinear contact classification.
//! Ported from polygon.move::has_exact_shared_edge, edges_match_exactly,
//! segments_contact, and shared_edge_relation.
//!
//! Used for topology validation (parts must connect via shared edges, not just
//! vertices, and partial collinear contact is rejected as a T-junction).

use crate::primitives::cross2d;

/// Normalize an edge as an unordered pair with (min, max) ordering.
/// Uses lexicographic comparison: (x1,y1) < (x2,y2) iff x1<x2 or (x1==x2 and y1<y2).
/// Ensures edge A→B and B→A are treated as the same edge.
///
/// Matches polygon.move::normalize_edge + point_precedes.
fn normalize_edge(x1: i64, y1: i64, x2: i64, y2: i64) -> ((i64, i64), (i64, i64)) {
    let p1 = (x1, y1);
    let p2 = (x2, y2);
    if p1 <= p2 {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

/// True if polygon A and polygon B share at least one exact edge.
/// An "exact shared edge" is an edge (v_i, v_{i+1}) that appears in both polygons,
/// possibly in reverse order.
///
/// Matches polygon.move::has_exact_shared_edge() + edges_match_exactly().
pub fn has_exact_shared_edge(a_xs: &[i64], a_ys: &[i64], b_xs: &[i64], b_ys: &[i64]) -> bool {
    let na = a_xs.len();
    let nb = b_xs.len();

    for i in 0..na {
        let j = (i + 1) % na;
        let edge_a = normalize_edge(a_xs[i], a_ys[i], a_xs[j], a_ys[j]);

        for k in 0..nb {
            let l = (k + 1) % nb;
            let edge_b = normalize_edge(b_xs[k], b_ys[k], b_xs[l], b_ys[l]);

            if edge_a == edge_b {
                return true;
            }
        }
    }

    false
}

/// True if two line segments are collinear and overlap along a non-degenerate interval.
/// Contact means: parallel, collinear, and 1D intervals strictly overlap.
/// Touching at a single endpoint is NOT contact (strict overlap required).
///
/// This detects partial edge contact (T-junctions etc.) not covered by exact edge matching.
/// Used in classify_contact to detect collinear overlaps between polygon edges.
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
    // Parallel check: cross of direction vectors must be 0
    let dax = (ax2 as i128) - (ax1 as i128);
    let day = (ay2 as i128) - (ay1 as i128);
    let dbx = (bx2 as i128) - (bx1 as i128);
    let dby = (by2 as i128) - (by1 as i128);

    if dax * dby != day * dbx {
        return false; // Not parallel
    }

    // Collinear check: b1 must lie on the line through a1→a2
    let collinear = cross2d(ax1, ay1, ax2, ay2, bx1, by1);
    if collinear != 0 {
        return false;
    }

    // 1D overlap check along dominant axis (strict: lo < hi, not <=)
    if dax.abs() > day.abs() {
        // Project onto X axis
        let (a_lo, a_hi) = (ax1.min(ax2), ax1.max(ax2));
        let (b_lo, b_hi) = (bx1.min(bx2), bx1.max(bx2));
        a_lo.max(b_lo) < a_hi.min(b_hi)
    } else {
        // Project onto Y axis
        let (a_lo, a_hi) = (ay1.min(ay2), ay1.max(ay2));
        let (b_lo, b_hi) = (by1.min(by2), by1.max(by2));
        a_lo.max(b_lo) < a_hi.min(b_hi)
    }
}

/// Classification of how two polygon parts touch along their boundaries.
///
/// Mirrors the three-way outcome of polygon.move::shared_edge_relation:
/// exact shared edge (valid adjacency), no contact (independent), or partial
/// collinear overlap (on-chain aborts `EInvalidMultipartContact`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactKind {
    /// No collinear overlap between any edge pair. Vertex-only contact is
    /// classified here as well — callers that care must check vertices
    /// separately.
    None,
    /// At least one edge appears in both parts (possibly reversed). This is
    /// the only form of adjacency on-chain accepts.
    SharedEdge,
    /// Edges overlap on a non-degenerate interval without matching exactly
    /// (T-junction). On-chain aborts `EInvalidMultipartContact`; in Rust the
    /// caller must surface this as `TopologyError::UnsupportedContact`.
    PartialContact,
}

/// Classify the contact between two polygon parts.
///
/// Matches polygon.move::shared_edge_relation's decision tree:
///
/// 1. If any edge pair matches exactly → `SharedEdge`.
/// 2. Else if any edge pair has a non-degenerate collinear overlap →
///    `PartialContact` (the on-chain abort case).
/// 3. Otherwise → `None`.
///
/// Note: Move also performs `aabbs_may_contact` + SAT overlap checks before
/// this point, but those are orthogonal (overlap → `EPartOverlap`). This
/// function only classifies boundary contact.
pub fn classify_contact(a_xs: &[i64], a_ys: &[i64], b_xs: &[i64], b_ys: &[i64]) -> ContactKind {
    if has_exact_shared_edge(a_xs, a_ys, b_xs, b_ys) {
        return ContactKind::SharedEdge;
    }

    let na = a_xs.len();
    let nb = b_xs.len();
    for i in 0..na {
        let j = (i + 1) % na;
        for k in 0..nb {
            let l = (k + 1) % nb;
            if segments_contact(
                a_xs[i], a_ys[i], a_xs[j], a_ys[j], b_xs[k], b_ys[k], b_xs[l], b_ys[l],
            ) {
                return ContactKind::PartialContact;
            }
        }
    }

    ContactKind::None
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    // ── normalize_edge ──────────────────────────────────────────────

    #[test]
    fn shared_edge_normalize_orders_lexicographically() {
        // (0,0)→(M,M) already sorted
        assert_eq!(normalize_edge(0, 0, M, M), ((0, 0), (M, M)));
        // Reversed input should give same result
        assert_eq!(normalize_edge(M, M, 0, 0), ((0, 0), (M, M)));
    }

    #[test]
    fn shared_edge_normalize_same_x_sorts_by_y() {
        assert_eq!(normalize_edge(M, 2 * M, M, 0), ((M, 0), (M, 2 * M)));
        assert_eq!(normalize_edge(M, 0, M, 2 * M), ((M, 0), (M, 2 * M)));
    }

    #[test]
    fn shared_edge_normalize_degenerate_point() {
        // Same point: both orderings give same result
        assert_eq!(normalize_edge(5, 5, 5, 5), ((5, 5), (5, 5)));
    }

    // ── has_exact_shared_edge ───────────────────────────────────────

    #[test]
    fn shared_edge_exact_detected() {
        // A: left square, B: right square, sharing edge x=M
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 2 * M, 2 * M, M];
        let b_ys = vec![0, 0, M, M];
        assert!(has_exact_shared_edge(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn shared_edge_exact_direction_independent() {
        // Same edge but B has reversed winding
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        // B winding: (M,M)→(2M,M)→(2M,0)→(M,0)
        // Edge (M,M)→(M,0) is reverse of A's (M,0)→(M,M)
        let b_xs = vec![M, 2 * M, 2 * M, M];
        let b_ys = vec![M, M, 0, 0];
        assert!(has_exact_shared_edge(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn shared_edge_no_match_separated_squares() {
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![3 * M, 4 * M, 4 * M, 3 * M];
        let b_ys = vec![0, 0, M, M];
        assert!(!has_exact_shared_edge(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn shared_edge_no_match_vertex_only_contact() {
        // Diagonal neighbors share only corner (M,M)
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 2 * M, 2 * M, M];
        let b_ys = vec![M, M, 2 * M, 2 * M];
        assert!(!has_exact_shared_edge(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn shared_edge_triangles_sharing_hypotenuse() {
        // Triangle A: (0,0), (M,0), (0,M) — hypotenuse (M,0)→(0,M)
        // Triangle B: (M,M), (0,M), (M,0) — hypotenuse (0,M)→(M,0) reversed
        let a_xs = vec![0, M, 0];
        let a_ys = vec![0, 0, M];
        let b_xs = vec![M, 0, M];
        let b_ys = vec![M, M, 0];
        assert!(has_exact_shared_edge(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    // ── segments_contact ────────────────────────────────────────────

    #[test]
    fn shared_edge_segments_collinear_overlap() {
        // Horizontal on y=0: [0,2M] and [M,3M] overlap on [M,2M]
        assert!(segments_contact(0, 0, 2 * M, 0, M, 0, 3 * M, 0));
    }

    #[test]
    fn shared_edge_segments_collinear_overlap_vertical() {
        // Vertical on x=0: [0,2M] and [M,3M] overlap on [M,2M]
        assert!(segments_contact(0, 0, 0, 2 * M, 0, M, 0, 3 * M));
    }

    #[test]
    fn shared_edge_segments_collinear_overlap_diagonal() {
        // Diagonal: (0,0)→(4M,4M) and (M,M)→(3M,3M) — contained overlap
        assert!(segments_contact(0, 0, 4 * M, 4 * M, M, M, 3 * M, 3 * M));
    }

    #[test]
    fn shared_edge_segments_no_overlap_gap() {
        // [0,M] and [2M,3M] on y=0 — collinear but gap
        assert!(!segments_contact(0, 0, M, 0, 2 * M, 0, 3 * M, 0));
    }

    #[test]
    fn shared_edge_segments_touching_endpoint_no_overlap() {
        // [0,M] and [M,2M] on y=0 — touching at single point, NOT contact
        assert!(!segments_contact(0, 0, M, 0, M, 0, 2 * M, 0));
    }

    #[test]
    fn shared_edge_segments_not_collinear() {
        // Perpendicular segments
        assert!(!segments_contact(0, 0, M, 0, 0, 0, 0, M));
    }

    #[test]
    fn shared_edge_segments_parallel_not_collinear() {
        // Parallel but offset — not collinear
        assert!(!segments_contact(0, 0, M, 0, 0, M, M, M));
    }

    #[test]
    fn shared_edge_segments_reversed_direction() {
        // Same overlap but segment B is reversed
        assert!(segments_contact(0, 0, 2 * M, 0, 3 * M, 0, M, 0));
    }

    #[test]
    fn shared_edge_segments_degenerate_point() {
        // Zero-length segment can't have strict overlap
        assert!(!segments_contact(M, 0, M, 0, 0, 0, 2 * M, 0));
    }

    // ── classify_contact ────────────────────────────────────────────

    #[test]
    fn classify_contact_via_exact_edge() {
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 2 * M, 2 * M, M];
        let b_ys = vec![0, 0, M, M];
        assert_eq!(
            classify_contact(&a_xs, &a_ys, &b_xs, &b_ys),
            ContactKind::SharedEdge
        );
    }

    #[test]
    fn shared_edge_no_match_partial_overlap_subsegment() {
        let a_xs = vec![M, 2 * M, 2 * M, M];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![0, 4 * M, 4 * M, 0];
        let b_ys = vec![0, 0, M, M];
        assert!(!has_exact_shared_edge(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn shared_edge_no_match_collinear_but_disjoint_edges() {
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![2 * M, 3 * M, 3 * M, 2 * M];
        let b_ys = vec![0, 0, M, M];
        assert!(!has_exact_shared_edge(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn classify_contact_none_for_separated_squares() {
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![3 * M, 4 * M, 4 * M, 3 * M];
        let b_ys = vec![0, 0, M, M];
        assert_eq!(
            classify_contact(&a_xs, &a_ys, &b_xs, &b_ys),
            ContactKind::None
        );
    }

    #[test]
    fn classify_contact_symmetric_shared_edge() {
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 2 * M, 2 * M, M];
        let b_ys = vec![0, 0, M, M];
        assert_eq!(
            classify_contact(&a_xs, &a_ys, &b_xs, &b_ys),
            classify_contact(&b_xs, &b_ys, &a_xs, &a_ys)
        );
    }

    /// Rectangle base [(0,0)-(4M,0)-(4M,M)-(0,M)] and triangle roof
    /// [(M,M),(3M,M),(2M,2M)]. Triangle's bottom edge (M,M)→(3M,M) is
    /// collinear with rectangle's top edge (4M,M)→(0,M) but not an exact
    /// match — classic T-junction.
    #[test]
    fn classify_contact_partial_edge_is_t_junction() {
        let a_xs = vec![0, 4 * M, 4 * M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 3 * M, 2 * M];
        let b_ys = vec![M, M, 2 * M];
        assert_eq!(
            classify_contact(&a_xs, &a_ys, &b_xs, &b_ys),
            ContactKind::PartialContact
        );
    }

    #[test]
    fn classify_contact_t_junction_symmetric() {
        let a_xs = vec![0, 4 * M, 4 * M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 3 * M, 2 * M];
        let b_ys = vec![M, M, 2 * M];
        assert_eq!(
            classify_contact(&a_xs, &a_ys, &b_xs, &b_ys),
            classify_contact(&b_xs, &b_ys, &a_xs, &a_ys)
        );
    }

    /// Vertex-only touch is not a "shared edge" and not a "partial contact"
    /// (no 1D overlap). Callers that want to reject vertex touching must do
    /// so via a separate vertex check.
    #[test]
    fn classify_contact_vertex_only_is_none() {
        let a_xs = vec![0, M, M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 2 * M, 2 * M, M];
        let b_ys = vec![M, M, 2 * M, 2 * M];
        assert_eq!(
            classify_contact(&a_xs, &a_ys, &b_xs, &b_ys),
            ContactKind::None
        );
    }

    #[test]
    fn classify_contact_vertex_only_sharing_one_corner_is_none() {
        let a_xs = vec![0, 2 * M, 2 * M, 0];
        let a_ys = vec![0, 0, 2 * M, 2 * M];
        let b_xs = vec![2 * M, 4 * M, 4 * M, 2 * M];
        let b_ys = vec![2 * M, 2 * M, 4 * M, 4 * M];
        assert_eq!(
            classify_contact(&a_xs, &a_ys, &b_xs, &b_ys),
            ContactKind::None
        );
    }

    #[test]
    fn classify_contact_t_junction_returns_partial_contact() {
        let a_xs = vec![0, 4 * M, 4 * M, 0];
        let a_ys = vec![0, 0, M, M];
        let b_xs = vec![M, 3 * M, 3 * M, M];
        let b_ys = vec![M, M, 2 * M, 2 * M];
        assert_eq!(
            classify_contact(&a_xs, &a_ys, &b_xs, &b_ys),
            ContactKind::PartialContact
        );
    }
}
