/// Exact twice-area for a ring, using i128 shoelace.
///
/// Returns `|2·signed_area|` as `u128`. For any ring whose vertices fit in
/// `u64` (the on-chain domain used by `polygon.move::part_twice_area_fp2`),
/// this function returns **exactly the same value** as the Move
/// implementation — the positive/negative split in Move and the signed
/// accumulation here are algebraically identical on non-negative inputs.
///
/// Unlike the previous `u128`-only port, this version also handles negative
/// `i64` coordinates correctly. Casting a negative `i64` to `u128` is a
/// sign-extension (e.g. `-91i64 as u128 == 2^128 - 91`), which silently
/// corrupted every product in the shoelace sum. Using `i128` intermediates
/// avoids that class of bug entirely while keeping Move parity for legal
/// on-chain polygons.
///
/// Overflow safety: for `|x|, |y| ≤ MAX_WORLD ≈ 4·10^13 ≈ 2^45`, each
/// product `x·y` is at most `2^90`, and with any realistic vertex count the
/// accumulated sum stays far below `i128::MAX ≈ 1.7·10^38`.
pub fn twice_area_fp2(ring: &[[i64; 2]]) -> u128 {
    let n = ring.len();
    if n < 3 {
        return 0;
    }

    let mut signed: i128 = 0;
    for i in 0..n {
        let j = if i + 1 == n { 0 } else { i + 1 };
        let xi = ring[i][0] as i128;
        let yi = ring[i][1] as i128;
        let xj = ring[j][0] as i128;
        let yj = ring[j][1] as i128;
        signed += xi * yj - xj * yi;
    }

    signed.unsigned_abs()
}

/// Lossy display conversion from fixed-point-squared to whole square meters.
pub fn area_display(twice_area: u128, area_divisor: u128) -> u64 {
    if area_divisor == 0 {
        return twice_area as u64;
    }
    (twice_area / area_divisor) as u64
}

/// Exact area conservation check for part decompositions.
pub fn areas_conserved(original: u128, parts: &[u128]) -> bool {
    let sum: u128 = parts.iter().sum();
    sum == original
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    #[test]
    fn square_10m_twice_area_matches_move() {
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let area = twice_area_fp2(&ring);
        assert_eq!(
            area, 200_000_000_000_000_u128,
            "square area mismatch: got {area}"
        );
    }

    #[test]
    fn triangle_twice_area_correct() {
        let ring = vec![[0, 0], [10 * M, 0], [0, 10 * M]];
        let area = twice_area_fp2(&ring);
        assert_eq!(area, 100_000_000_000_000_u128);
    }

    #[test]
    fn area_display_converts_correctly() {
        let twice_area = 200_000_000_000_000_u128;
        assert_eq!(
            area_display(twice_area, crate::constants::AREA_DIVISOR),
            100
        );
    }

    #[test]
    fn area_display_1m_x_1m_is_1_m2() {
        let ring = vec![[0, 0], [M, 0], [M, M], [0, M]];
        let twice_area = twice_area_fp2(&ring);
        assert_eq!(twice_area, 2_000_000_000_000_u128);
        assert_eq!(area_display(twice_area, crate::constants::AREA_DIVISOR), 1);
    }

    #[test]
    fn twice_area_fp2_triangle_with_diagonal_edge() {
        let ring = vec![[0, 0], [3, 0], [0, 4]];
        assert_eq!(twice_area_fp2(&ring), 12);
    }

    #[test]
    fn twice_area_fp2_l_shape_matches_expected() {
        let ring = vec![[0, 0], [4, 0], [4, 2], [2, 2], [2, 4], [0, 4]];
        assert_eq!(twice_area_fp2(&ring), 24);
    }

    #[test]
    fn twice_area_fp2_degenerate_two_vertices_is_zero() {
        let ring = vec![[0, 0], [10, 0]];
        assert_eq!(twice_area_fp2(&ring), 0);
    }

    #[test]
    fn area_display_scales_large_values() {
        assert_eq!(
            area_display(2_000_000_000_000_000_000, 2_000_000_000_000),
            1_000_000
        );
    }

    #[test]
    fn area_display_zero_area_is_zero() {
        assert_eq!(area_display(0, 1), 0);
    }

    #[test]
    fn areas_conserved_accepts_exact_sum() {
        assert!(areas_conserved(30, &[10, 20]));
    }

    #[test]
    fn areas_conserved_rejects_overage() {
        assert!(!areas_conserved(30, &[10, 21]));
    }

    #[test]
    fn area_conservation_check() {
        let square = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let original = twice_area_fp2(&square);

        let t1 = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M]];
        let t2 = vec![[0, 0], [10 * M, 10 * M], [0, 10 * M]];

        let part_areas = vec![twice_area_fp2(&t1), twice_area_fp2(&t2)];

        assert!(
            areas_conserved(original, &part_areas),
            "area not conserved: original={original}, sum={}",
            part_areas.iter().sum::<u128>()
        );
    }

    #[test]
    fn twice_area_fp2_square_ring_matches() {
        let ring = vec![[0i64, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let area = twice_area_fp2(&ring);
        assert_eq!(area, 200_000_000_000_000_u128);
    }

    #[test]
    fn area_is_order_independent_for_ccw_and_cw() {
        let ccw = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let cw = vec![[0, 0], [0, 10 * M], [10 * M, 10 * M], [10 * M, 0]];
        assert_eq!(twice_area_fp2(&ccw), twice_area_fp2(&cw));
    }

    #[test]
    fn degenerate_ring_returns_zero() {
        let ring = vec![[0, 0], [M, 0]];
        assert_eq!(twice_area_fp2(&ring), 0);
    }

    #[test]
    fn negative_coords_are_handled_correctly() {
        // Triangle with negative coordinates. Before the i128 rewrite this
        // returned garbage (~2^128) because `-x as u128` sign-extended into
        // the high bits of every shoelace product.
        let ring = vec![[-M, 0], [M, 0], [0, M]];
        // Same triangle translated so all coords are non-negative.
        let translated = vec![[0, 0], [2 * M, 0], [M, M]];
        let expected = twice_area_fp2(&translated);
        assert_eq!(twice_area_fp2(&ring), expected);
        // 2·area of a (2M, M)-base-height triangle = 2·(½·2M·M) = 2·M²
        assert_eq!(expected, 2 * (M as u128) * (M as u128));
    }

    #[test]
    fn cw_and_ccw_negative_polygon_match() {
        // Concave "crown" polygon from the decomposition bug report
        // (https://.../issue): 5 vertices around the origin, drawn CW.
        let cw = vec![
            [-91 * M, -70 * M],
            [-120 * M, 177 * M],
            [M, 56 * M],
            [110 * M, 210 * M],
            [138 * M, -70 * M],
        ];
        let ccw: Vec<[i64; 2]> = cw.iter().rev().copied().collect();
        let area_cw = twice_area_fp2(&cw);
        let area_ccw = twice_area_fp2(&ccw);
        assert_eq!(area_cw, area_ccw, "winding should not affect |2·area|");
        // Shoelace on the raw (integer-meter) vertices gives |Σ| = 90_064,
        // and the scale factor is M·M = 10^12 per product.
        assert_eq!(area_cw, 90_064_u128 * (M as u128) * (M as u128));
    }

    #[test]
    fn matches_move_on_u64_domain() {
        // Mirror polygon.move::part_twice_area_fp2 — the u64 positive/negative
        // split must agree with the i128 shoelace bit-for-bit on non-negative
        // inputs. Pick a polygon that forces both positive and negative terms.
        let ring = vec![
            [0, 0],
            [10 * M, 15 * M],
            [30 * M, 5 * M],
            [20 * M, 25 * M],
            [5 * M, 20 * M],
        ];
        assert_eq!(twice_area_fp2(&ring), 525_u128 * (M as u128) * (M as u128));
    }
}
