use crate::aabb::Aabb;

fn validate_polygon(ring: &[[i64; 2]]) -> bool {
    ring.len() >= 3
}

fn project_onto_axis(ring: &[[i64; 2]], ax: i64, ay: i64) -> (i128, i128) {
    ring.iter()
        .map(|pt| pt[0] as i128 * ax as i128 + pt[1] as i128 * ay as i128)
        .fold((i128::MAX, i128::MIN), |(min, max), v| {
            (min.min(v), max.max(v))
        })
}

fn projections_overlap(min_a: i128, max_a: i128, min_b: i128, max_b: i128) -> bool {
    max_a > min_b && max_b > min_a
}

fn has_separating_axis(edge_ring: &[[i64; 2]], a: &[[i64; 2]], b: &[[i64; 2]]) -> bool {
    let n = edge_ring.len();

    for i in 0..n {
        let next = if i + 1 < n { i + 1 } else { 0 };
        let dx = edge_ring[next][0] - edge_ring[i][0];
        let dy = edge_ring[next][1] - edge_ring[i][1];
        let axis_x = dy;
        let axis_y = -dx;

        if axis_x == 0 && axis_y == 0 {
            continue;
        }

        let (min_a, max_a) = project_onto_axis(a, axis_x, axis_y);
        let (min_b, max_b) = project_onto_axis(b, axis_x, axis_y);

        if !projections_overlap(min_a, max_a, min_b, max_b) {
            return true;
        }
    }

    false
}

pub fn sat_overlaps(a: &[[i64; 2]], b: &[[i64; 2]]) -> bool {
    if !validate_polygon(a) || !validate_polygon(b) {
        return false;
    }

    if has_separating_axis(a, a, b) {
        return false;
    }

    if has_separating_axis(b, a, b) {
        return false;
    }

    true
}

pub fn sat_overlaps_with_aabb(a: &[[i64; 2]], b: &[[i64; 2]]) -> bool {
    if !validate_polygon(a) || !validate_polygon(b) {
        return false;
    }

    let aabb_a = Aabb::from_ring(a);
    let aabb_b = Aabb::from_ring(b);

    if !aabb_a.intersects(&aabb_b) {
        return false;
    }

    sat_overlaps(a, b)
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
    fn sat_gap_detected() {
        let a = square(0, 0, M);
        let b = square(2 * M, 0, M);

        assert!(!sat_overlaps(&a, &b));
    }

    #[test]
    fn sat_touching_edges_do_not_overlap() {
        let a = square(0, 0, M);
        let b = square(M, 0, M);

        assert!(!sat_overlaps(&a, &b));
    }

    #[test]
    fn sat_overlap_detected() {
        let a = square(0, 0, 2 * M);
        let b = square(M, M, 2 * M);

        assert!(sat_overlaps(&a, &b));
    }

    #[test]
    fn sat_micro_overlap_detected() {
        let a = square(0, 0, M);
        let b = vec![[M - 1, 0], [2 * M - 1, 0], [2 * M - 1, M], [M - 1, M]];

        assert!(sat_overlaps(&a, &b));
    }

    #[test]
    fn sat_corner_touching_does_not_overlap() {
        let a = square(0, 0, M);
        let b = square(M, M, M);

        assert!(!sat_overlaps(&a, &b));
    }

    #[test]
    fn sat_triangle_touching_does_not_overlap() {
        let a = vec![[0, 0], [2 * M, 0], [M, 2 * M]];
        let b = vec![[2 * M, 0], [3 * M, 0], [2 * M + M / 2, M]];

        assert!(!sat_overlaps(&a, &b));
    }

    #[test]
    fn contained_square_overlaps() {
        let outer = square(0, 0, 10 * M);
        let inner = square(2 * M, 2 * M, 3 * M);

        assert!(sat_overlaps(&outer, &inner));
    }

    #[test]
    fn identical_squares_overlap() {
        let a = square(0, 0, M);
        let b = square(0, 0, M);

        assert!(sat_overlaps(&a, &b));
    }

    #[test]
    fn sat_is_symmetric() {
        let a = square(0, 0, 3 * M);
        let b = square(2 * M, 2 * M, 3 * M);

        assert_eq!(sat_overlaps(&a, &b), sat_overlaps(&b, &a),);
    }

    #[test]
    fn aabb_pipeline_rejects_far_polygons() {
        let a = square(0, 0, M);
        let b = square(3 * M, 0, M);

        assert!(!sat_overlaps_with_aabb(&a, &b));
    }

    #[test]
    fn aabb_prefilter_allows_sat_for_overlapping() {
        let a = square(0, 0, 3 * M);
        let b = square(2 * M, 2 * M, 3 * M);

        assert!(sat_overlaps_with_aabb(&a, &b));
    }

    #[test]
    fn triangles_overlap_correctly() {
        let a = vec![[0, 0], [4 * M, 0], [2 * M, 4 * M]];
        let b = vec![[M, M], [3 * M, M], [2 * M, 3 * M]];

        assert!(sat_overlaps(&a, &b));
    }

    #[test]
    fn right_triangles_sharing_hypotenuse_do_not_overlap() {
        let a = vec![[0, 0], [2 * M, 0], [2 * M, 2 * M]];
        let b = vec![[0, 0], [0, 2 * M], [2 * M, 2 * M]];

        assert!(!sat_overlaps(&a, &b));
    }

    #[test]
    fn triangle_fully_inside_another_overlaps() {
        let outer = vec![[0, 0], [4 * M, 0], [0, 4 * M]];
        let inner = vec![[M, M], [2 * M, M], [M, 2 * M]];

        assert!(sat_overlaps(&outer, &inner));
    }

    #[test]
    fn sat_handles_world_extreme_coordinates() {
        const MAX_WORLD: i64 = 40_075_017_000_000;

        let a = vec![
            [-MAX_WORLD, -MAX_WORLD],
            [-MAX_WORLD + M, -MAX_WORLD],
            [-MAX_WORLD + M, -MAX_WORLD + M],
            [-MAX_WORLD, -MAX_WORLD + M],
        ];
        let b = vec![
            [MAX_WORLD, MAX_WORLD],
            [MAX_WORLD + M, MAX_WORLD],
            [MAX_WORLD + M, MAX_WORLD + M],
            [MAX_WORLD, MAX_WORLD + M],
        ];

        assert!(!sat_overlaps(&a, &b));
    }

    #[test]
    fn sat_with_aabb_rejects_far_apart_polygons() {
        let a = square(0, 0, M);
        let b = square(100 * M, 100 * M, M);

        assert!(!sat_overlaps_with_aabb(&a, &b));
    }
}
