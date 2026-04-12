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
pub fn twice_area_fp2(xs: &[i64], ys: &[i64]) -> u128 {
    let n = xs.len();
    if n < 3 || ys.len() != n {
        return 0;
    }

    let mut signed: i128 = 0;
    for i in 0..n {
        let j = if i + 1 == n { 0 } else { i + 1 };
        let xi = xs[i] as i128;
        let yi = ys[i] as i128;
        let xj = xs[j] as i128;
        let yj = ys[j] as i128;
        signed += xi * yj - xj * yi;
    }

    signed.unsigned_abs()
}

/// Convenience wrapper for [[x, y]] rings.
pub fn twice_area_fp2_ring(ring: &[[i64; 2]]) -> u128 {
    let xs: Vec<i64> = ring.iter().map(|v| v[0]).collect();
    let ys: Vec<i64> = ring.iter().map(|v| v[1]).collect();
    twice_area_fp2(&xs, &ys)
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
        let xs = vec![0, 10 * M, 10 * M, 0];
        let ys = vec![0, 0, 10 * M, 10 * M];
        let area = twice_area_fp2(&xs, &ys);
        assert_eq!(
            area, 200_000_000_000_000_u128,
            "square area mismatch: got {area}"
        );
    }

    #[test]
    fn triangle_twice_area_correct() {
        let xs = vec![0, 10 * M, 0];
        let ys = vec![0, 0, 10 * M];
        let area = twice_area_fp2(&xs, &ys);
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
        let xs = vec![0, M, M, 0];
        let ys = vec![0, 0, M, M];
        let twice_area = twice_area_fp2(&xs, &ys);
        assert_eq!(twice_area, 2_000_000_000_000_u128);
        assert_eq!(area_display(twice_area, crate::constants::AREA_DIVISOR), 1);
    }

    #[test]
    fn area_conservation_check() {
        let square_xs = vec![0, 10 * M, 10 * M, 0];
        let square_ys = vec![0, 0, 10 * M, 10 * M];
        let original = twice_area_fp2(&square_xs, &square_ys);

        let t1_xs = vec![0, 10 * M, 10 * M];
        let t1_ys = vec![0, 0, 10 * M];
        let t2_xs = vec![0, 10 * M, 0];
        let t2_ys = vec![0, 10 * M, 10 * M];

        let part_areas = vec![
            twice_area_fp2(&t1_xs, &t1_ys),
            twice_area_fp2(&t2_xs, &t2_ys),
        ];

        assert!(
            areas_conserved(original, &part_areas),
            "area not conserved: original={original}, sum={}",
            part_areas.iter().sum::<u128>()
        );
    }

    #[test]
    fn twice_area_fp2_ring_wrapper_matches() {
        let ring = vec![[0i64, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let area = twice_area_fp2_ring(&ring);
        assert_eq!(area, 200_000_000_000_000_u128);
    }

    #[test]
    fn area_is_order_independent_for_ccw_and_cw() {
        let ccw_xs = vec![0, 10 * M, 10 * M, 0];
        let ccw_ys = vec![0, 0, 10 * M, 10 * M];
        let cw_xs = vec![0, 0, 10 * M, 10 * M];
        let cw_ys = vec![0, 10 * M, 10 * M, 0];
        assert_eq!(
            twice_area_fp2(&ccw_xs, &ccw_ys),
            twice_area_fp2(&cw_xs, &cw_ys)
        );
    }

    #[test]
    fn degenerate_ring_returns_zero() {
        let xs = vec![0, M];
        let ys = vec![0, 0];
        assert_eq!(twice_area_fp2(&xs, &ys), 0);
    }

    #[test]
    fn negative_coords_are_handled_correctly() {
        // Triangle with negative coordinates. Before the i128 rewrite this
        // returned garbage (~2^128) because `-x as u128` sign-extended into
        // the high bits of every shoelace product.
        let xs = vec![-M, M, 0];
        let ys = vec![0, 0, M];
        // Same triangle translated so all coords are non-negative.
        let xs_pos = vec![0, 2 * M, M];
        let ys_pos = vec![0, 0, M];
        let expected = twice_area_fp2(&xs_pos, &ys_pos);
        assert_eq!(twice_area_fp2(&xs, &ys), expected);
        // 2·area of a (2M, M)-base-height triangle = 2·(½·2M·M) = 2·M²
        assert_eq!(expected, 2 * (M as u128) * (M as u128));
    }

    #[test]
    fn cw_and_ccw_negative_polygon_match() {
        // Concave "crown" polygon from the decomposition bug report
        // (https://.../issue): 5 vertices around the origin, drawn CW.
        let xs_cw = vec![-91 * M, -120 * M, 1 * M, 110 * M, 138 * M];
        let ys_cw = vec![-70 * M, 177 * M, 56 * M, 210 * M, -70 * M];
        let xs_ccw: Vec<i64> = xs_cw.iter().rev().copied().collect();
        let ys_ccw: Vec<i64> = ys_cw.iter().rev().copied().collect();
        let area_cw = twice_area_fp2(&xs_cw, &ys_cw);
        let area_ccw = twice_area_fp2(&xs_ccw, &ys_ccw);
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
        let xs: Vec<i64> = vec![0, 10 * M, 30 * M, 20 * M, 5 * M];
        let ys: Vec<i64> = vec![0, 15 * M, 5 * M, 25 * M, 20 * M];
        let rust = twice_area_fp2(&xs, &ys);
        let move_equivalent = move_style_twice_area_u64(&xs, &ys);
        assert_eq!(rust, move_equivalent);
    }

    /// Reference port of polygon.move::part_twice_area_fp2, for parity tests.
    /// Takes i64 solely because the rest of the test harness does; all inputs
    /// must be non-negative.
    fn move_style_twice_area_u64(xs: &[i64], ys: &[i64]) -> u128 {
        let n = xs.len();
        let mut sum_pos: u128 = 0;
        let mut sum_neg: u128 = 0;
        for i in 0..n {
            let j = (i + 1) % n;
            let xi = xs[i] as u64 as u128;
            let yi = ys[i] as u64 as u128;
            let xj = xs[j] as u64 as u128;
            let yj = ys[j] as u64 as u128;
            let t1 = xi * yj;
            let t2 = xj * yi;
            if t1 >= t2 {
                sum_pos += t1 - t2;
            } else {
                sum_neg += t2 - t1;
            }
        }
        if sum_pos >= sum_neg {
            sum_pos - sum_neg
        } else {
            sum_neg - sum_pos
        }
    }
}
