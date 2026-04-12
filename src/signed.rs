//! Signed arithmetic for geometry computations.
//!
//! Move lacks native signed integers, so it uses (magnitude: u128, negative: bool) pairs.
//! Rust has native i128, so we use it directly — behavior matches signed.move exactly.
//!
//! Key function: cross_sign(ax,ay, bx,by, cx,cy) → i128
//! Computes (B−A) × (C−A), the 2D cross product used for convexity and orientation.
//! Always cast i64 → i128 BEFORE multiplying to prevent overflow.

/// Compute (B−A) × (C−A) cross product for three 2D points A, B, C.
///
/// Returns:
/// - Positive: left turn (CCW)  
/// - Negative: right turn (CW)
/// - Zero: collinear
///
/// All coordinates are i64 (fixed-point, SCALE=1_000_000).
/// Uses i128 internally — safe: max cross product ≈ (4×10¹³)² = 1.6×10²⁷, i128 max ≈ 1.7×10³⁸.
///
/// Matches behavior of signed::cross_sign() in signed.move:149-184.
pub fn cross_sign(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> i128 {
    let dx1 = (bx as i128) - (ax as i128);
    let dy1 = (by as i128) - (ay as i128);
    let dx2 = (cx as i128) - (ax as i128);
    let dy2 = (cy as i128) - (ay as i128);
    dx1 * dy2 - dy1 * dx2
}

/// Signed subtraction of two u64 values, returning i128.
/// Equivalent to signed::sub_u64() in signed.move:56-62.
pub fn sub_u64(a: u64, b: u64) -> i128 {
    (a as i128) - (b as i128)
}

/// Sign of a value: +1, 0, or -1.
pub fn sign(v: i128) -> i32 {
    v.cmp(&0) as i32
}

/// True if cross product indicates a left turn (strictly positive).
pub fn is_left_turn(cross: i128) -> bool {
    cross > 0
}

/// True if cross product indicates a right turn (strictly negative).
pub fn is_right_turn(cross: i128) -> bool {
    cross < 0
}

/// True if cross product indicates collinearity (zero).
pub fn is_collinear(cross: i128) -> bool {
    cross == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_sign_left_turn_is_positive() {
        // A=(0,0), B=(2,0), C=(2,2): left turn (CCW)
        let result = cross_sign(0, 0, 2_000_000, 0, 2_000_000, 2_000_000);
        assert!(result > 0, "left turn should be positive, got {result}");
    }

    #[test]
    fn cross_sign_right_turn_is_negative() {
        // A=(0,0), B=(2,2), C=(4,0): right turn (CW)
        let result = cross_sign(0, 0, 2_000_000, 2_000_000, 4_000_000, 0);
        assert!(result < 0, "right turn should be negative, got {result}");
    }

    #[test]
    fn cross_sign_collinear_is_zero() {
        // A=(0,0), B=(1,0), C=(2,0): collinear
        let result = cross_sign(0, 0, 1_000_000, 0, 2_000_000, 0);
        assert_eq!(result, 0, "collinear should be zero");
    }

    #[test]
    fn cross_sign_matches_move_test_vectors() {
        // From signed.move test: cross_sign(0,2, 1,1, 2,0) → zero (collinear diagonal)
        let zero = cross_sign(0, 2, 1, 1, 2, 0);
        assert_eq!(zero, 0);

        // From signed.move test: cross_sign(0,0, 2,0, 2,2) → positive (left turn), magnitude=4
        let left = cross_sign(0, 0, 2, 0, 2, 2);
        assert_eq!(left, 4);

        // From signed.move test: cross_sign(0,0, 2,2, 4,0) → negative (right turn)
        let right = cross_sign(0, 0, 2, 2, 4, 0);
        assert!(right < 0);
    }

    #[test]
    fn cross_sign_no_overflow_with_max_world_coordinates() {
        // MAX_WORLD = 40_075_017_000_000 ≈ 4×10¹³
        // Worst case cross product ≈ (4×10¹³)² = 1.6×10²⁷, i128 max ≈ 1.7×10³⁸ — safe
        const MAX_WORLD: i64 = 40_075_017_000_000;
        let result = cross_sign(0, 0, MAX_WORLD, 0, 0, MAX_WORLD);
        // Should be MAX_WORLD^2 = 1.605e27
        let expected = (MAX_WORLD as i128) * (MAX_WORLD as i128);
        assert_eq!(result, expected);
    }

    #[test]
    fn cross_sign_large_negative_no_overflow() {
        const MAX_WORLD: i64 = 40_075_017_000_000;
        // Opposite orientation
        let result = cross_sign(0, MAX_WORLD, MAX_WORLD, 0, 0, 0);
        assert!(result < 0);
    }

    #[test]
    fn sub_u64_positive_and_negative() {
        assert_eq!(sub_u64(10, 5), 5_i128);
        assert_eq!(sub_u64(5, 10), -5_i128);
        assert_eq!(sub_u64(7, 7), 0_i128);
    }

    #[test]
    fn sign_returns_correct_values() {
        assert_eq!(sign(42), 1);
        assert_eq!(sign(-42), -1);
        assert_eq!(sign(0), 0);
    }
}
