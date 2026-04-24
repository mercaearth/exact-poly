//! Exact-integer Bayazit convex decomposition.

use crate::area::twice_area_fp2;
use crate::primitives::{cross2d, point_on_segment, segments_intersect};
use crate::types::ProtocolConfig;
use crate::validation::validate_part;
use std::collections::HashSet;

const MAX_DEPTH: usize = 64;

/// Decompose a simple CCW polygon into convex parts.
pub fn bayazit_decompose(
    ring: &[[i64; 2]],
    allow_steiner: bool,
    config: &ProtocolConfig,
) -> Result<Vec<Vec<[i64; 2]>>, String> {
    if ring.len() < 3 {
        return Err("polygon has fewer than 3 vertices".into());
    }

    if twice_area_fp2(ring) == 0 {
        return Err("polygon area is zero".into());
    }

    let mut result = Vec::new();
    decompose_recursive(ring.to_vec(), allow_steiner, 0, &mut result)?;

    if result.is_empty() {
        return Err("decomposition produced no parts".into());
    }

    let original_area = twice_area_fp2(ring);
    let parts_area: u128 = result.iter().map(|part| twice_area_fp2(part)).sum();
    if parts_area != original_area {
        return Err(format!(
            "area not conserved: original={original_area}, parts sum={parts_area}"
        ));
    }

    for (idx, part) in result.iter().enumerate() {
        if let Some(err) = validate_part(part, config) {
            return Err(format!("invalid output part {idx}: {err}"));
        }
    }

    Ok(result)
}

fn decompose_recursive(
    poly: Vec<[i64; 2]>,
    allow_steiner: bool,
    depth: usize,
    result: &mut Vec<Vec<[i64; 2]>>,
) -> Result<(), String> {
    if poly.len() < 3 {
        return Err("sub-polygon has fewer than 3 vertices".into());
    }
    if depth > MAX_DEPTH {
        return Err("bayazit recursion limit reached".into());
    }

    let reflex = find_reflex_vertices(&poly);
    if reflex.is_empty() {
        result.push(poly);
        return Ok(());
    }

    for r_idx in reflex {
        if let Some((lower, upper)) = find_best_vertex_split(&poly, r_idx) {
            decompose_recursive(lower, allow_steiner, depth + 1, result)?;
            decompose_recursive(upper, allow_steiner, depth + 1, result)?;
            return Ok(());
        }

        if allow_steiner {
            if let Some((lower, upper)) = find_steiner_split(&poly, r_idx) {
                decompose_recursive(lower, allow_steiner, depth + 1, result)?;
                decompose_recursive(upper, allow_steiner, depth + 1, result)?;
                return Ok(());
            }
        }
    }

    Err("no valid Bayazit split found".into())
}

fn find_reflex_vertices(poly: &[[i64; 2]]) -> Vec<usize> {
    let n = poly.len();
    let mut reflex = Vec::new();
    for i in 0..n {
        if is_reflex(poly, i) {
            reflex.push(i);
        }
    }
    reflex
}

fn is_reflex(poly: &[[i64; 2]], i: usize) -> bool {
    let n = poly.len();
    let prev = poly[(i + n - 1) % n];
    let curr = poly[i];
    let next = poly[(i + 1) % n];
    cross2d(prev[0], prev[1], curr[0], curr[1], next[0], next[1]) < 0
}

fn find_best_vertex_split(
    poly: &[[i64; 2]],
    r_idx: usize,
) -> Option<(Vec<[i64; 2]>, Vec<[i64; 2]>)> {
    let n = poly.len();
    let reflexes = find_reflex_vertices(poly);

    let mut best: Option<(Vec<[i64; 2]>, Vec<[i64; 2]>)> = None;
    let mut best_score: Option<(usize, usize)> = None;

    for &prefer_reflex in &[true, false] {
        for j in 0..n {
            if j == r_idx || is_adjacent(n, r_idx, j) {
                continue;
            }
            if prefer_reflex && !reflexes.contains(&j) {
                continue;
            }
            if !is_valid_diagonal(poly, r_idx, j) {
                continue;
            }

            let (left, right) = split_polygon(poly, r_idx, j);
            if !is_valid_subpolygon(&left) || !is_valid_subpolygon(&right) {
                continue;
            }

            let score = split_score(&left, &right);
            if best_score.is_none_or(|current| score < current) {
                best_score = Some(score);
                best = Some((left, right));
            }
        }

        if best.is_some() {
            return best;
        }
    }

    None
}

fn find_steiner_split(poly: &[[i64; 2]], r_idx: usize) -> Option<(Vec<[i64; 2]>, Vec<[i64; 2]>)> {
    let n = poly.len();
    let prev_idx = (r_idx + n - 1) % n;
    let next_idx = (r_idx + 1) % n;
    let reflex = poly[r_idx];
    let prev = poly[prev_idx];
    let next = poly[next_idx];

    let mut best: Option<(Vec<[i64; 2]>, Vec<[i64; 2]>)> = None;
    let mut best_score: Option<(usize, usize)> = None;

    for edge_start_idx in 0..n {
        let edge_end_idx = (edge_start_idx + 1) % n;
        if edge_start_idx == prev_idx
            || edge_start_idx == r_idx
            || edge_start_idx == next_idx
            || edge_end_idx == prev_idx
            || edge_end_idx == r_idx
            || edge_end_idx == next_idx
        {
            continue;
        }

        let a = poly[edge_start_idx];
        let b = poly[edge_end_idx];

        let lower_hit = line_segment_intersection(prev, reflex, a, b)?;
        let upper_hit = line_segment_intersection(next, reflex, a, b)?;

        for mut steiner in [round_point(lower_hit), round_point(upper_hit)] {
            if steiner == reflex || steiner == a || steiner == b {
                continue;
            }
            if !point_on_segment(steiner[0], steiner[1], a[0], a[1], b[0], b[1]) {
                continue;
            }
            if steiner == poly[edge_start_idx] || steiner == poly[edge_end_idx] {
                continue;
            }

            steiner = snap_away_from_endpoints(steiner, a, b)?;
            if !point_on_segment(steiner[0], steiner[1], a[0], a[1], b[0], b[1]) {
                continue;
            }

            let inserted = insert_point_between(poly, edge_start_idx, steiner);
            let steiner_idx = (edge_start_idx + 1) % inserted.len();
            if !is_valid_diagonal(&inserted, r_idx, steiner_idx) {
                continue;
            }

            let (left, right) = split_polygon(&inserted, r_idx, steiner_idx);
            if !is_valid_subpolygon(&left) || !is_valid_subpolygon(&right) {
                continue;
            }

            let score = split_score(&left, &right);
            if best_score.is_none_or(|current| score < current) {
                best_score = Some(score);
                best = Some((left, right));
            }
        }
    }

    best
}

fn split_score(left: &[[i64; 2]], right: &[[i64; 2]]) -> (usize, usize) {
    let len_score = left.len().max(right.len());
    let reflex_score = find_reflex_vertices(left).len() + find_reflex_vertices(right).len();
    (len_score, reflex_score)
}

fn is_adjacent(n: usize, i: usize, j: usize) -> bool {
    (i + 1) % n == j || (j + 1) % n == i
}

fn is_valid_diagonal(poly: &[[i64; 2]], i: usize, j: usize) -> bool {
    let n = poly.len();
    if n < 4 || i == j || is_adjacent(n, i, j) {
        return false;
    }

    diagonal_lies_inside(poly, i, j)
}

fn diagonal_lies_inside(poly: &[[i64; 2]], i: usize, j: usize) -> bool {
    let n = poly.len();
    let a = poly[i];
    let b = poly[j];

    for k in 0..n {
        let l = (k + 1) % n;
        if k == i || k == j || l == i || l == j {
            continue;
        }

        let c = poly[k];
        let d = poly[l];
        if segments_intersect(a[0], a[1], b[0], b[1], c[0], c[1], d[0], d[1]) {
            return false;
        }
    }

    for (k, vertex) in poly.iter().enumerate() {
        if k == i || k == j {
            continue;
        }
        if point_on_segment(vertex[0], vertex[1], a[0], a[1], b[0], b[1]) {
            return false;
        }
    }

    let midpoint = midpoint_round(a, b);
    point_in_polygon_strict(midpoint, poly)
}

fn midpoint_round(a: [i64; 2], b: [i64; 2]) -> [i64; 2] {
    [
        round_div2((a[0] as i128) + (b[0] as i128)),
        round_div2((a[1] as i128) + (b[1] as i128)),
    ]
}

fn round_div2(value: i128) -> i64 {
    if value >= 0 {
        ((value + 1) / 2) as i64
    } else {
        ((value - 1) / 2) as i64
    }
}

fn split_polygon(poly: &[[i64; 2]], i: usize, j: usize) -> (Vec<[i64; 2]>, Vec<[i64; 2]>) {
    let n = poly.len();

    let mut first = Vec::new();
    let mut cursor = i;
    loop {
        first.push(poly[cursor]);
        if cursor == j {
            break;
        }
        cursor = (cursor + 1) % n;
    }

    let mut second = Vec::new();
    let mut cursor = j;
    loop {
        second.push(poly[cursor]);
        if cursor == i {
            break;
        }
        cursor = (cursor + 1) % n;
    }

    (first, second)
}

fn is_valid_subpolygon(poly: &[[i64; 2]]) -> bool {
    if poly.len() < 3 {
        return false;
    }
    let area = twice_area_fp2(poly);
    if area == 0 {
        return false;
    }

    let mut unique = HashSet::with_capacity(poly.len());
    for &vertex in poly {
        if !unique.insert(vertex) {
            return false;
        }
    }

    true
}

fn point_in_polygon_strict(point: [i64; 2], poly: &[[i64; 2]]) -> bool {
    if poly.iter().enumerate().any(|(i, &a)| {
        let b = poly[(i + 1) % poly.len()];
        point_on_segment(point[0], point[1], a[0], a[1], b[0], b[1])
    }) {
        return false;
    }

    let mut inside = false;
    for i in 0..poly.len() {
        let j = if i == 0 { poly.len() - 1 } else { i - 1 };
        let xi = poly[i][0] as i128;
        let yi = poly[i][1] as i128;
        let xj = poly[j][0] as i128;
        let yj = poly[j][1] as i128;
        let py = point[1] as i128;
        let px = point[0] as i128;

        let intersects = if (yi > py) != (yj > py) {
            let lhs = (xj - xi) * (py - yi);
            let rhs = (px - xi) * (yj - yi);
            if yj > yi {
                lhs > rhs
            } else {
                lhs < rhs
            }
        } else {
            false
        };
        if intersects {
            inside = !inside;
        }
    }

    inside
}

fn line_segment_intersection(
    a1: [i64; 2],
    a2: [i64; 2],
    b1: [i64; 2],
    b2: [i64; 2],
) -> Option<([i128; 2], i128)> {
    let x1 = a1[0] as i128;
    let y1 = a1[1] as i128;
    let x2 = a2[0] as i128;
    let y2 = a2[1] as i128;
    let x3 = b1[0] as i128;
    let y3 = b1[1] as i128;
    let x4 = b2[0] as i128;
    let y4 = b2[1] as i128;

    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if den == 0 {
        return None;
    }

    let det1 = x1 * y2 - y1 * x2;
    let det2 = x3 * y4 - y3 * x4;
    let px_num = det1 * (x3 - x4) - (x1 - x2) * det2;
    let py_num = det1 * (y3 - y4) - (y1 - y2) * det2;

    let within_x = between_rational(px_num, den, x3, x4);
    let within_y = between_rational(py_num, den, y3, y4);
    if !within_x || !within_y {
        return None;
    }

    Some(([px_num, py_num], den))
}

fn between_rational(num: i128, den: i128, a: i128, b: i128) -> bool {
    let min = a.min(b);
    let max = a.max(b);
    if den > 0 {
        num >= min * den && num <= max * den
    } else {
        num <= min * den && num >= max * den
    }
}

fn round_point((coords, den): ([i128; 2], i128)) -> [i64; 2] {
    [
        round_rational(coords[0], den),
        round_rational(coords[1], den),
    ]
}

fn round_rational(num: i128, den: i128) -> i64 {
    let den_abs = den.abs();
    let adjusted = if (num >= 0) == (den >= 0) {
        num.abs() + den_abs / 2
    } else {
        num.abs().saturating_sub(den_abs / 2)
    };
    let quotient = adjusted / den_abs;
    if (num >= 0) == (den >= 0) {
        quotient as i64
    } else {
        -(quotient as i64)
    }
}

fn snap_away_from_endpoints(point: [i64; 2], a: [i64; 2], b: [i64; 2]) -> Option<[i64; 2]> {
    if point != a && point != b {
        return Some(point);
    }

    let dx = (b[0] as i128) - (a[0] as i128);
    let dy = (b[1] as i128) - (a[1] as i128);
    let steps = [(1_i128, 4_i128), (1, 3), (2, 3)];

    for (num, den) in steps {
        let candidate = [
            a[0].saturating_add(round_rational(dx * num, den)),
            a[1].saturating_add(round_rational(dy * num, den)),
        ];
        if candidate != a && candidate != b {
            return Some(candidate);
        }
    }

    None
}

fn insert_point_between(
    poly: &[[i64; 2]],
    edge_start_idx: usize,
    point: [i64; 2],
) -> Vec<[i64; 2]> {
    let mut out = Vec::with_capacity(poly.len() + 1);
    for (idx, &vertex) in poly.iter().enumerate() {
        out.push(vertex);
        if idx == edge_start_idx {
            out.push(point);
        }
    }
    out
}

pub fn find_steiner_points(original: &[[i64; 2]], parts: &[Vec<[i64; 2]>]) -> Vec<[i64; 2]> {
    let original_set: HashSet<[i64; 2]> = original.iter().copied().collect();
    let mut steiner = Vec::new();
    for part in parts {
        for &vertex in part {
            if !original_set.contains(&vertex) && !steiner.contains(&vertex) {
                steiner.push(vertex);
            }
        }
    }
    steiner
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::is_convex;

    const M: i64 = 1_000_000;

    fn merca_config() -> ProtocolConfig {
        ProtocolConfig::merca()
    }

    fn l_shape() -> Vec<[i64; 2]> {
        vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ]
    }

    fn square() -> Vec<[i64; 2]> {
        vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]]
    }

    fn triangle() -> Vec<[i64; 2]> {
        vec![[0, 0], [10 * M, 0], [5 * M, 8 * M]]
    }

    fn comb_shape() -> Vec<[i64; 2]> {
        vec![
            [0, 0],
            [10 * M, 0],
            [10 * M, 10 * M],
            [8 * M, 10 * M],
            [8 * M, 4 * M],
            [6 * M, 4 * M],
            [6 * M, 10 * M],
            [4 * M, 10 * M],
            [4 * M, 4 * M],
            [2 * M, 4 * M],
            [2 * M, 10 * M],
            [0, 10 * M],
        ]
    }

    #[test]
    fn bayazit_triangle_returns_single_part() {
        let parts = bayazit_decompose(&triangle(), false, &merca_config()).unwrap();
        assert_eq!(parts.len(), 1);
    }

    #[test]
    fn bayazit_square_returns_single_part() {
        let parts = bayazit_decompose(&square(), false, &merca_config()).unwrap();
        assert_eq!(parts.len(), 1);
    }

    #[test]
    fn bayazit_l_shape_decomposes_into_multiple_parts() {
        let ring = l_shape();
        let parts = bayazit_decompose(&ring, false, &merca_config()).unwrap();
        assert!(parts.len() >= 2, "expected >=2 parts, got {}", parts.len());
    }

    #[test]
    fn bayazit_parts_are_convex_and_valid() {
        let ring = l_shape();
        let parts = bayazit_decompose(&ring, false, &merca_config()).unwrap();

        for (idx, part) in parts.iter().enumerate() {
            assert!(is_convex(part), "part {idx} not convex: {part:?}");
            assert!(
                validate_part(part, &crate::types::ProtocolConfig::merca()).is_none(),
                "part {idx} invalid: {part:?}"
            );
        }
    }

    #[test]
    fn bayazit_area_is_conserved_exactly() {
        let ring = l_shape();
        let original_area = twice_area_fp2(&ring);
        let parts = bayazit_decompose(&ring, false, &merca_config()).unwrap();
        let parts_area: u128 = parts.iter().map(|part| twice_area_fp2(part)).sum();
        assert_eq!(parts_area, original_area);
    }

    #[test]
    fn bayazit_parts_have_minimum_vertex_count() {
        for part in bayazit_decompose(&l_shape(), false, &merca_config()).unwrap() {
            assert!(part.len() >= 3, "degenerate part: {part:?}");
        }
    }

    #[test]
    fn bayazit_no_steiner_points_when_disallowed() {
        let ring = l_shape();
        let parts = bayazit_decompose(&ring, false, &merca_config()).unwrap();
        let steiner = find_steiner_points(&ring, &parts);
        assert!(steiner.is_empty(), "unexpected steiner points: {steiner:?}");
    }

    #[test]
    fn bayazit_find_steiner_points_reports_new_vertices() {
        let ring = comb_shape();
        let steiner_vertex = [5 * M, 5 * M];
        let parts = vec![
            vec![ring[0], ring[1], ring[2], steiner_vertex],
            vec![steiner_vertex, ring[2], ring[3], ring[4], ring[5]],
            vec![
                ring[5], ring[6], ring[7], ring[8], ring[9], ring[10], ring[11],
            ],
        ];
        let steiner = find_steiner_points(&ring, &parts);

        assert_eq!(steiner, vec![steiner_vertex]);
        assert!(steiner.iter().all(|point| !ring.contains(point)));
    }
}
