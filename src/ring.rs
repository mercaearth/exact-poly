//! Ring (closed polygon) utility functions.
//!
//! A ring is a Vec<[i64; 2]> of vertices in order (first != last, closing edge implied).
//! All operations use exact integer arithmetic — no epsilon, no floating point.
//!
//! Input contract for decompose():
//! - Rings must be CCW (counter-clockwise)
//! - Rings must be simple (no self-intersections)
//! - Collinear vertices should be removed for cleaner decomposition

use crate::primitives::{cross2d, segments_properly_intersect};

/// Twice the signed area of the ring (positive = CCW, negative = CW).
/// Uses the shoelace formula with i128 intermediate arithmetic.
/// NOT the same as twice_area_fp2 in area.rs — this preserves sign for orientation detection.
pub fn signed_area_2x(ring: &[[i64; 2]]) -> i128 {
    let n = ring.len();
    if n < 3 {
        return 0;
    }
    let mut sum: i128 = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        let xi = ring[i][0] as i128;
        let yi = ring[i][1] as i128;
        let xj = ring[j][0] as i128;
        let yj = ring[j][1] as i128;
        sum += xi * yj - xj * yi;
    }
    sum
}

/// True if the ring is counter-clockwise (positive signed area).
pub fn is_ccw(ring: &[[i64; 2]]) -> bool {
    signed_area_2x(ring) > 0
}

/// Ensure the ring is CCW. Reverses in place if CW.
pub fn ensure_ccw(ring: &mut Vec<[i64; 2]>) {
    if !is_ccw(ring) {
        ring.reverse();
    }
}

/// Remove collinear vertices from a ring.
/// A vertex is collinear if cross(prev, curr, next) == 0 (exact integer test).
/// Returns a new ring with collinear vertices removed.
/// The result may have fewer vertices, but preserves the polygon shape exactly.
pub fn remove_collinear(ring: &[[i64; 2]]) -> Vec<[i64; 2]> {
    let n = ring.len();
    if n < 3 {
        return ring.to_vec();
    }

    let mut result: Vec<[i64; 2]> = Vec::with_capacity(n);

    for i in 0..n {
        let prev = ring[(i + n - 1) % n];
        let curr = ring[i];
        let next = ring[(i + 1) % n];

        let cross = cross2d(prev[0], prev[1], curr[0], curr[1], next[0], next[1]);

        if cross != 0 {
            result.push(curr);
        }
    }

    result
}

/// True if the ring has no self-intersections (is simple).
/// O(n²) check — all non-adjacent edge pairs.
/// A ring with n < 3 vertices is not simple.
pub fn is_simple(ring: &[[i64; 2]]) -> bool {
    let n = ring.len();
    if n < 3 {
        return false;
    }

    for i in 0..n {
        let i1 = i;
        let i2 = (i + 1) % n;
        let a1 = ring[i1];
        let a2 = ring[i2];

        for j in (i + 2)..n {
            // skip adjacent edges: edge 0 and edge n-1 share a vertex
            if i == 0 && j == n - 1 {
                continue;
            }

            let j1 = j;
            let j2 = (j + 1) % n;
            let b1 = ring[j1];
            let b2 = ring[j2];

            if segments_properly_intersect(a1[0], a1[1], a2[0], a2[1], b1[0], b1[1], b2[0], b2[1]) {
                return false;
            }
        }
    }

    true
}

/// Normalize a ring: remove collinear vertices, ensure CCW winding.
/// Does NOT check for simplicity (caller must ensure input is simple).
/// Returns None if the result has fewer than 3 vertices (degenerate).
pub fn normalize_ring(ring: &[[i64; 2]]) -> Option<Vec<[i64; 2]>> {
    let mut result = remove_collinear(ring);
    if result.len() < 3 {
        return None;
    }
    ensure_ccw(&mut result);
    Some(result)
}

/// Rotate ring so that index `start` becomes index 0. Preserves winding.
pub fn rotate_ring(ring: &[[i64; 2]], start: usize) -> Vec<[i64; 2]> {
    let n = ring.len();
    if n == 0 || start == 0 {
        return ring.to_vec();
    }
    let start = start % n;
    let mut result = Vec::with_capacity(n);
    result.extend_from_slice(&ring[start..]);
    result.extend_from_slice(&ring[..start]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000; // 1 meter

    fn square_ccw() -> Vec<[i64; 2]> {
        vec![[0, 0], [M, 0], [M, M], [0, M]]
    }

    fn square_cw() -> Vec<[i64; 2]> {
        vec![[0, 0], [0, M], [M, M], [M, 0]]
    }

    #[test]
    fn signed_area_ccw_is_positive() {
        let ring = square_ccw();
        assert!(signed_area_2x(&ring) > 0);
    }

    #[test]
    fn signed_area_cw_is_negative() {
        let ring = square_cw();
        assert!(signed_area_2x(&ring) < 0);
    }

    #[test]
    fn signed_area_magnitude_is_twice_area() {
        let ring = square_ccw();
        let area_2x = signed_area_2x(&ring).unsigned_abs();
        // Square 1M x 1M = 1e12, twice = 2e12
        assert_eq!(area_2x, 2 * (M as u128) * (M as u128));
    }

    #[test]
    fn is_ccw_detects_winding() {
        assert!(is_ccw(&square_ccw()));
        assert!(!is_ccw(&square_cw()));
    }

    #[test]
    fn ensure_ccw_reverses_cw_ring() {
        let mut ring = square_cw();
        ensure_ccw(&mut ring);
        assert!(is_ccw(&ring));
    }

    #[test]
    fn ensure_ccw_leaves_ccw_ring_unchanged() {
        let ccw = square_ccw();
        let mut ring = ccw.clone();
        ensure_ccw(&mut ring);
        assert_eq!(ring, ccw);
    }

    #[test]
    fn remove_collinear_removes_midpoint_on_edge() {
        // Square with extra vertex at midpoint of bottom edge
        let ring = vec![[0, 0], [M / 2, 0], [M, 0], [M, M], [0, M]];
        let result = remove_collinear(&ring);
        // Midpoint at (M/2, 0) should be removed — collinear with (0,0) and (M,0)
        assert_eq!(result.len(), 4);
        assert!(!result.contains(&[M / 2, 0]));
    }

    #[test]
    fn remove_collinear_preserves_non_collinear() {
        let ring = square_ccw();
        let result = remove_collinear(&ring);
        assert_eq!(result, ring); // all 4 corners are non-collinear
    }

    #[test]
    fn remove_collinear_exact_zero_check() {
        // Nearly-collinear (but not exactly) should NOT be removed
        let ring = vec![[0, 0], [M, 1], [2 * M, 0], [2 * M, M], [0, M]];
        let result = remove_collinear(&ring);
        // (M, 1) is not exactly collinear with (0,0) and (2M,0) — should be kept
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn is_simple_convex_ring() {
        assert!(is_simple(&square_ccw()));
    }

    #[test]
    fn is_simple_self_intersecting() {
        // Figure-8: self-intersecting
        let ring = vec![[0, 0], [2 * M, 2 * M], [2 * M, 0], [0, 2 * M]];
        assert!(!is_simple(&ring));
    }

    #[test]
    fn normalize_ring_removes_collinear_and_ensures_ccw() {
        // CW ring with collinear vertex
        let ring = vec![[0, 0], [0, M], [0, 2 * M], [M, 2 * M], [M, 0]]; // CW, has collinear
        let result = normalize_ring(&ring);
        assert!(result.is_some());
        let normalized = result.unwrap();
        assert!(is_ccw(&normalized));
        // Collinear vertex [0, M] should be removed (between [0,0] and [0,2M])
        assert_eq!(normalized.len(), 4);
    }

    #[test]
    fn rotate_ring_shifts_start() {
        let ring = square_ccw();
        let rotated = rotate_ring(&ring, 2);
        assert_eq!(rotated[0], [M, M]);
        assert_eq!(rotated.len(), 4);
    }

    #[test]
    fn rotate_ring_zero_start_unchanged() {
        let ring = square_ccw();
        let rotated = rotate_ring(&ring, 0);
        assert_eq!(rotated, ring);
    }
}
