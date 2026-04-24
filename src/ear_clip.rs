use crate::area::twice_area_fp2;
use crate::primitives::cross2d;

pub fn ear_clip_triangulate(ring: &[[i64; 2]]) -> Result<Vec<Vec<[i64; 2]>>, String> {
    let n = ring.len();
    if n < 3 {
        return Err(format!("polygon has {n} vertices, need >= 3"));
    }
    if n == 3 {
        return Ok(vec![ring.to_vec()]);
    }

    let mut poly: Vec<[i64; 2]> = ring.to_vec();
    let mut triangles: Vec<Vec<[i64; 2]>> = Vec::with_capacity(n - 2);

    let mut iterations = 0;
    let max_iterations = n * n;

    while poly.len() > 3 && iterations < max_iterations {
        iterations += 1;
        let m = poly.len();

        let Some(i) = find_ear(&poly) else {
            break;
        };

        let prev = (i + m - 1) % m;
        let next = (i + 1) % m;

        triangles.push(vec![poly[prev], poly[i], poly[next]]);
        poly.remove(i);
    }

    if poly.len() == 3 {
        triangles.push(poly);
    }

    if triangles.len() != n - 2 {
        return Err(format!(
            "ear clipping failed: expected {} triangles, got {}",
            n - 2,
            triangles.len()
        ));
    }

    let original_area = twice_area_fp2(ring);
    let parts_area: u128 = triangles
        .iter()
        .map(|triangle| twice_area_fp2(triangle))
        .sum();
    if parts_area != original_area {
        return Err(format!(
            "area not conserved: original={original_area}, sum={parts_area}"
        ));
    }

    if triangles
        .iter()
        .any(|triangle| twice_area_fp2(triangle) == 0)
    {
        return Err("ear clipping produced degenerate triangle".to_string());
    }

    Ok(triangles)
}

fn find_ear(poly: &[[i64; 2]]) -> Option<usize> {
    let n = poly.len();
    for i in 0..n {
        let prev = (i + n - 1) % n;
        let next = (i + 1) % n;

        let cross = cross2d(
            poly[prev][0],
            poly[prev][1],
            poly[i][0],
            poly[i][1],
            poly[next][0],
            poly[next][1],
        );
        if cross <= 0 {
            continue;
        }

        if no_vertex_inside_triangle(poly, prev, i, next) {
            return Some(i);
        }
    }
    None
}

fn no_vertex_inside_triangle(poly: &[[i64; 2]], a: usize, b: usize, c: usize) -> bool {
    let ta = poly[a];
    let tb = poly[b];
    let tc = poly[c];

    for (i, point) in poly.iter().enumerate() {
        if i == a || i == b || i == c {
            continue;
        }
        if point_in_triangle_non_strict(*point, ta, tb, tc) {
            return false;
        }
    }
    true
}

fn point_in_triangle_non_strict(p: [i64; 2], a: [i64; 2], b: [i64; 2], c: [i64; 2]) -> bool {
    let d1 = cross2d(a[0], a[1], b[0], b[1], p[0], p[1]);
    let d2 = cross2d(b[0], b[1], c[0], c[1], p[0], p[1]);
    let d3 = cross2d(c[0], c[1], a[0], a[1], p[0], p[1]);

    let has_neg = d1 < 0 || d2 < 0 || d3 < 0;
    let has_pos = d1 > 0 || d2 > 0 || d3 > 0;

    !(has_neg && has_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    fn square() -> Vec<[i64; 2]> {
        vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]]
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

    #[test]
    fn triangle_input_returns_one_triangle() {
        let tri = vec![[0, 0], [10 * M, 0], [5 * M, 8 * M]];
        let result = ear_clip_triangulate(&tri).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], tri);
    }

    #[test]
    fn square_produces_2_triangles() {
        let result = ear_clip_triangulate(&square()).unwrap();
        assert_eq!(result.len(), 2, "square(4 vertices) → 2 triangles");
    }

    #[test]
    fn l_shape_produces_4_triangles() {
        let result = ear_clip_triangulate(&l_shape()).unwrap();
        assert_eq!(result.len(), 4, "L-shape(6 vertices) → 4 triangles");
    }

    #[test]
    fn n_vertices_produces_n_minus_2_triangles() {
        let ring = l_shape();
        let result = ear_clip_triangulate(&ring).unwrap();
        assert_eq!(result.len(), ring.len() - 2);
    }

    #[test]
    fn all_triangles_have_positive_area() {
        let parts = ear_clip_triangulate(&l_shape()).unwrap();
        for (i, tri) in parts.iter().enumerate() {
            let area = twice_area_fp2(tri);
            assert!(area > 0, "triangle {i} has zero area");
        }
    }

    #[test]
    fn area_conservation_square() {
        let ring = square();
        let original_area = twice_area_fp2(&ring);
        let triangles = ear_clip_triangulate(&ring).unwrap();
        let parts_area: u128 = triangles.iter().map(|t| twice_area_fp2(t)).sum();
        assert_eq!(
            parts_area, original_area,
            "area not conserved: original={original_area}, sum={parts_area}"
        );
    }

    #[test]
    fn area_conservation_l_shape() {
        let ring = l_shape();
        let original_area = twice_area_fp2(&ring);
        let triangles = ear_clip_triangulate(&ring).unwrap();
        let parts_area: u128 = triangles.iter().map(|t| twice_area_fp2(t)).sum();
        assert_eq!(
            parts_area, original_area,
            "area not conserved: original={original_area}, sum={parts_area}"
        );
    }

    #[test]
    fn too_few_vertices_returns_error() {
        let tiny = vec![[0i64, 0], [M, 0]];
        assert!(ear_clip_triangulate(&tiny).is_err());
    }

    #[test]
    fn point_on_triangle_edge_is_not_treated_as_ear_interior_clearance() {
        let a = [0, 0];
        let b = [4 * M, 0];
        let c = [0, 4 * M];
        let p = [2 * M, 0];
        assert!(point_in_triangle_non_strict(p, a, b, c));
    }
}
