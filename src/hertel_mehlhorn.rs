use crate::area::twice_area_fp2_ring;
use crate::validation::is_convex;
use std::collections::BTreeMap;

type Vertex = [i64; 2];
type Edge = (Vertex, Vertex);

fn normalize_edge(a: Vertex, b: Vertex) -> Edge {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn boundary_cycle_without_shared(a: &[Vertex], b: &[Vertex], shared: Edge) -> Option<Vec<Vertex>> {
    let mut next: BTreeMap<Vertex, Vertex> = BTreeMap::new();
    let mut incoming: BTreeMap<Vertex, usize> = BTreeMap::new();

    for poly in [a, b] {
        for i in 0..poly.len() {
            let start = poly[i];
            let end = poly[(i + 1) % poly.len()];
            if normalize_edge(start, end) == shared {
                continue;
            }
            if next.insert(start, end).is_some() {
                return None;
            }
            *incoming.entry(end).or_insert(0) += 1;
        }
    }

    if next.len() < 3 {
        return None;
    }

    for vertex in next.keys() {
        if incoming.get(vertex).copied().unwrap_or(0) != 1 {
            return None;
        }
    }

    let start = *next.keys().next()?;
    let mut merged = Vec::with_capacity(next.len());
    let mut current = start;

    loop {
        merged.push(current);
        current = *next.get(&current)?;
        if current == start {
            break;
        }
        if merged.len() >= next.len() {
            return None;
        }
    }

    if merged.len() != next.len() {
        return None;
    }

    Some(merged)
}

pub fn merge_convex_pair(a: &[Vertex], b: &[Vertex]) -> Option<Vec<Vertex>> {
    if a.len() < 3 || b.len() < 3 {
        return None;
    }

    let mut shared_edges = Vec::new();
    for i in 0..a.len() {
        let edge_a = normalize_edge(a[i], a[(i + 1) % a.len()]);
        for j in 0..b.len() {
            let edge_b = normalize_edge(b[j], b[(j + 1) % b.len()]);
            if edge_a == edge_b {
                shared_edges.push(edge_a);
            }
        }
    }

    shared_edges.sort_unstable();
    shared_edges.dedup();
    if shared_edges.len() != 1 {
        return None;
    }

    let merged = boundary_cycle_without_shared(a, b, shared_edges[0])?;
    if twice_area_fp2_ring(&merged) == 0 {
        return None;
    }

    let xs: Vec<i64> = merged.iter().map(|vertex| vertex[0]).collect();
    let ys: Vec<i64> = merged.iter().map(|vertex| vertex[1]).collect();
    if is_convex(&xs, &ys) {
        Some(merged)
    } else {
        None
    }
}

pub fn optimize_partition(parts: &[Vec<Vertex>]) -> Vec<Vec<Vertex>> {
    let mut optimized = parts.to_vec();

    loop {
        let mut edges: BTreeMap<Edge, Vec<usize>> = BTreeMap::new();
        for (part_index, part) in optimized.iter().enumerate() {
            for i in 0..part.len() {
                let edge = normalize_edge(part[i], part[(i + 1) % part.len()]);
                edges.entry(edge).or_default().push(part_index);
            }
        }

        let mut merged_any = false;
        for indices in edges.values() {
            if indices.len() != 2 || indices[0] == indices[1] {
                continue;
            }

            let a_idx = indices[0];
            let b_idx = indices[1];
            let Some(merged) = merge_convex_pair(&optimized[a_idx], &optimized[b_idx]) else {
                continue;
            };

            let mut next_parts = Vec::with_capacity(optimized.len() - 1);
            for (part_index, part) in optimized.iter().enumerate() {
                if part_index == a_idx {
                    next_parts.push(merged.clone());
                } else if part_index != b_idx {
                    next_parts.push(part.clone());
                }
            }

            optimized = next_parts;
            merged_any = true;
            break;
        }

        if !merged_any {
            return optimized;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::area::twice_area_fp2_ring;
    use crate::ear_clip::ear_clip_triangulate;

    const M: i64 = 1_000_000;

    #[test]
    fn hertel_mehlhorn_single_triangle_unchanged() {
        let tri = vec![vec![[0, 0], [10 * M, 0], [5 * M, 8 * M]]];
        let result = optimize_partition(&tri);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn hertel_mehlhorn_two_triangles_forming_square_merge_to_one() {
        let t1 = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M]];
        let t2 = vec![[0, 0], [10 * M, 10 * M], [0, 10 * M]];
        let result = optimize_partition(&[t1, t2]);
        assert_eq!(
            result.len(),
            1,
            "two triangles forming square should merge to 1"
        );

        let xs: Vec<i64> = result[0].iter().map(|v| v[0]).collect();
        let ys: Vec<i64> = result[0].iter().map(|v| v[1]).collect();
        assert!(is_convex(&xs, &ys));
    }

    #[test]
    fn hertel_mehlhorn_l_shape_triangles_optimize() {
        let l_shape = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];
        let triangles = ear_clip_triangulate(&l_shape).unwrap();
        assert_eq!(triangles.len(), 4);

        let optimized = optimize_partition(&triangles);
        assert!(
            optimized.len() < triangles.len(),
            "should reduce part count"
        );
        assert!(
            optimized.len() >= 2,
            "L-shape needs at least 2 convex parts"
        );

        for part in &optimized {
            let xs: Vec<i64> = part.iter().map(|v| v[0]).collect();
            let ys: Vec<i64> = part.iter().map(|v| v[1]).collect();
            assert!(is_convex(&xs, &ys));
        }
    }

    #[test]
    fn hertel_mehlhorn_area_conservation() {
        let l_shape = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];
        let triangles = ear_clip_triangulate(&l_shape).unwrap();
        let original_area: u128 = triangles.iter().map(|t| twice_area_fp2_ring(t)).sum();
        let optimized = optimize_partition(&triangles);
        let optimized_area: u128 = optimized.iter().map(|p| twice_area_fp2_ring(p)).sum();
        assert_eq!(original_area, optimized_area);
    }

    #[test]
    fn hertel_mehlhorn_merge_convex_pair_works() {
        let a = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M]];
        let b = vec![[0, 0], [10 * M, 10 * M], [0, 10 * M]];
        let merged = merge_convex_pair(&a, &b);
        assert!(merged.is_some());
        assert_eq!(merged.unwrap().len(), 4);
    }

    #[test]
    fn hertel_mehlhorn_merge_non_convex_returns_none() {
        let a = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M]];
        let b = vec![[10 * M, 10 * M], [20 * M, 10 * M], [20 * M, 20 * M]];
        let merged = merge_convex_pair(&a, &b);
        assert!(merged.is_none());
    }
}
