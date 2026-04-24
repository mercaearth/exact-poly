use crate::aabb::Aabb;

const E_BAD_VERTICES: &str = "SAT requires at least 3 vertices";
const E_MISMATCH: &str = "SAT xs and ys must have same length";
const E_ZERO_AXIS: &str = "SAT encountered degenerate zero-length edge";

fn validate_polygon(xs: &[i64], ys: &[i64]) {
    assert!(xs.len() >= 3, "{E_BAD_VERTICES}");
    assert_eq!(xs.len(), ys.len(), "{E_MISMATCH}");
}

fn project_onto_axis(xs: &[i64], ys: &[i64], ax: i64, ay: i64) -> (i128, i128) {
    let ax = ax as i128;
    let ay = ay as i128;

    let first = (xs[0] as i128) * ax + (ys[0] as i128) * ay;
    let mut min_proj = first;
    let mut max_proj = first;

    for (&x, &y) in xs.iter().zip(ys.iter()).skip(1) {
        let proj = (x as i128) * ax + (y as i128) * ay;
        if proj < min_proj {
            min_proj = proj;
        }
        if proj > max_proj {
            max_proj = proj;
        }
    }

    (min_proj, max_proj)
}

fn projections_overlap(min_a: i128, max_a: i128, min_b: i128, max_b: i128) -> bool {
    max_a > min_b && max_b > min_a
}

fn has_separating_axis(
    edge_xs: &[i64],
    edge_ys: &[i64],
    a_xs: &[i64],
    a_ys: &[i64],
    b_xs: &[i64],
    b_ys: &[i64],
) -> bool {
    let n = edge_xs.len();

    for i in 0..n {
        let next = if i + 1 < n { i + 1 } else { 0 };
        let dx = edge_xs[next] - edge_xs[i];
        let dy = edge_ys[next] - edge_ys[i];
        let axis_x = dy;
        let axis_y = -dx;

        assert!(axis_x != 0 || axis_y != 0, "{E_ZERO_AXIS}");

        let (min_a, max_a) = project_onto_axis(a_xs, a_ys, axis_x, axis_y);
        let (min_b, max_b) = project_onto_axis(b_xs, b_ys, axis_x, axis_y);

        if !projections_overlap(min_a, max_a, min_b, max_b) {
            return true;
        }
    }

    false
}

pub fn sat_overlaps(a_xs: &[i64], a_ys: &[i64], b_xs: &[i64], b_ys: &[i64]) -> bool {
    validate_polygon(a_xs, a_ys);
    validate_polygon(b_xs, b_ys);

    if has_separating_axis(a_xs, a_ys, a_xs, a_ys, b_xs, b_ys) {
        return false;
    }

    if has_separating_axis(b_xs, b_ys, a_xs, a_ys, b_xs, b_ys) {
        return false;
    }

    true
}

pub fn sat_overlaps_with_aabb(a_xs: &[i64], a_ys: &[i64], b_xs: &[i64], b_ys: &[i64]) -> bool {
    let aabb_a = Aabb::from_vertices(a_xs, a_ys);
    let aabb_b = Aabb::from_vertices(b_xs, b_ys);

    if !aabb_a.intersects(&aabb_b) {
        return false;
    }

    sat_overlaps(a_xs, a_ys, b_xs, b_ys)
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    fn square(ox: i64, oy: i64, size: i64) -> (Vec<i64>, Vec<i64>) {
        (
            vec![ox, ox + size, ox + size, ox],
            vec![oy, oy, oy + size, oy + size],
        )
    }

    #[test]
    fn sat_gap_detected() {
        let (a_xs, a_ys) = square(0, 0, M);
        let (b_xs, b_ys) = square(2 * M, 0, M);

        assert!(!sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn sat_touching_edges_do_not_overlap() {
        let (a_xs, a_ys) = square(0, 0, M);
        let (b_xs, b_ys) = square(M, 0, M);

        assert!(!sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn sat_overlap_detected() {
        let (a_xs, a_ys) = square(0, 0, 2 * M);
        let (b_xs, b_ys) = square(M, M, 2 * M);

        assert!(sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn sat_micro_overlap_detected() {
        let (a_xs, a_ys) = square(0, 0, M);
        let (b_xs, b_ys) = (vec![M - 1, 2 * M - 1, 2 * M - 1, M - 1], vec![0, 0, M, M]);

        assert!(sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn sat_corner_touching_does_not_overlap() {
        let (a_xs, a_ys) = square(0, 0, M);
        let (b_xs, b_ys) = square(M, M, M);

        assert!(!sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn sat_triangle_touching_does_not_overlap() {
        let a_xs = vec![0, 2 * M, M];
        let a_ys = vec![0, 0, 2 * M];
        let b_xs = vec![2 * M, 3 * M, 2 * M + M / 2];
        let b_ys = vec![0, 0, M];

        assert!(!sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn contained_square_overlaps() {
        let (outer_xs, outer_ys) = square(0, 0, 10 * M);
        let (inner_xs, inner_ys) = square(2 * M, 2 * M, 3 * M);

        assert!(sat_overlaps(&outer_xs, &outer_ys, &inner_xs, &inner_ys));
    }

    #[test]
    fn identical_squares_overlap() {
        let (a_xs, a_ys) = square(0, 0, M);
        let (b_xs, b_ys) = square(0, 0, M);

        assert!(sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn sat_is_symmetric() {
        let (a_xs, a_ys) = square(0, 0, 3 * M);
        let (b_xs, b_ys) = square(2 * M, 2 * M, 3 * M);

        assert_eq!(
            sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys),
            sat_overlaps(&b_xs, &b_ys, &a_xs, &a_ys),
        );
    }

    #[test]
    fn aabb_pipeline_rejects_far_polygons() {
        let (a_xs, a_ys) = square(0, 0, M);
        let (b_xs, b_ys) = square(3 * M, 0, M);

        assert!(!sat_overlaps_with_aabb(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn aabb_prefilter_allows_sat_for_overlapping() {
        let (a_xs, a_ys) = square(0, 0, 3 * M);
        let (b_xs, b_ys) = square(2 * M, 2 * M, 3 * M);

        assert!(sat_overlaps_with_aabb(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn triangles_overlap_correctly() {
        let a_xs = vec![0, 4 * M, 2 * M];
        let a_ys = vec![0, 0, 4 * M];
        let b_xs = vec![M, 3 * M, 2 * M];
        let b_ys = vec![M, M, 3 * M];

        assert!(sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn right_triangles_sharing_hypotenuse_do_not_overlap() {
        let a_xs = vec![0, 2 * M, 2 * M];
        let a_ys = vec![0, 0, 2 * M];
        let b_xs = vec![0, 0, 2 * M];
        let b_ys = vec![0, 2 * M, 2 * M];

        assert!(!sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn triangle_fully_inside_another_overlaps() {
        let outer_xs = vec![0, 4 * M, 0];
        let outer_ys = vec![0, 0, 4 * M];
        let inner_xs = vec![M, 2 * M, M];
        let inner_ys = vec![M, M, 2 * M];

        assert!(sat_overlaps(&outer_xs, &outer_ys, &inner_xs, &inner_ys));
    }

    #[test]
    fn sat_handles_world_extreme_coordinates() {
        const MAX_WORLD: i64 = 40_075_017_000_000;

        let a_xs = vec![-MAX_WORLD, -MAX_WORLD + M, -MAX_WORLD + M, -MAX_WORLD];
        let a_ys = vec![-MAX_WORLD, -MAX_WORLD, -MAX_WORLD + M, -MAX_WORLD + M];
        let b_xs = vec![MAX_WORLD, MAX_WORLD + M, MAX_WORLD + M, MAX_WORLD];
        let b_ys = vec![MAX_WORLD, MAX_WORLD, MAX_WORLD + M, MAX_WORLD + M];

        assert!(!sat_overlaps(&a_xs, &a_ys, &b_xs, &b_ys));
    }

    #[test]
    fn sat_with_aabb_rejects_far_apart_polygons() {
        let (a_xs, a_ys) = square(0, 0, M);
        let (b_xs, b_ys) = square(100 * M, 100 * M, M);

        assert!(!sat_overlaps_with_aabb(&a_xs, &a_ys, &b_xs, &b_ys));
    }
}
