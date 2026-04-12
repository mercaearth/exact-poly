use crate::area::twice_area_fp2_ring;
use crate::shared_edge::{classify_contact, ContactKind};
use crate::types::{ProtocolConfig, TopologyError};
use crate::validation::check_compactness;
use std::collections::{HashMap, HashSet, VecDeque};

type Vertex = [i64; 2];
type Edge = (Vertex, Vertex);

pub fn validate_multipart_topology(
    parts: &[Vec<Vertex>],
    allow_vertex_contact: bool,
    config: &ProtocolConfig,
) -> Result<(), TopologyError> {
    let n = parts.len();

    if n == 0 {
        return Err(TopologyError::NotConnected {
            disconnected_parts: vec![],
        });
    }

    if n > config.max_parts {
        return Err(TopologyError::TooManyParts {
            count: n,
            max: config.max_parts,
        });
    }

    if n == 1 {
        return validate_single_part(parts[0].as_slice(), config);
    }

    let mut edge_adjacent = vec![vec![false; n]; n];

    for i in 0..n {
        let (ai_xs, ai_ys) = split_coords(&parts[i]);
        let ai_vertices: HashSet<Vertex> = parts[i].iter().copied().collect();

        for j in (i + 1)..n {
            let (aj_xs, aj_ys) = split_coords(&parts[j]);

            match classify_contact(&ai_xs, &ai_ys, &aj_xs, &aj_ys) {
                ContactKind::SharedEdge => {
                    edge_adjacent[i][j] = true;
                    edge_adjacent[j][i] = true;
                    continue;
                }
                ContactKind::PartialContact => {
                    return Err(TopologyError::UnsupportedContact {
                        part_a: i,
                        part_b: j,
                        reason: "collinear overlap without exact shared edge (T-junction)"
                            .into(),
                    });
                }
                ContactKind::None => {}
            }
            if n == 2 && !allow_vertex_contact {
                let has_shared_vertex = parts[j].iter().any(|vertex| ai_vertices.contains(vertex));
                if has_shared_vertex {
                    return Err(TopologyError::VertexOnlyContact {
                        part_a: i,
                        part_b: j,
                    });
                }
            }
        }
    }

    let connectivity = if allow_vertex_contact {
        build_vertex_adjacency(parts)
    } else {
        edge_adjacent.clone()
    };

    let visited = bfs_visited(&connectivity);
    if visited.iter().any(|&v| !v) {
        let disconnected: Vec<usize> = (0..n).filter(|&i| !visited[i]).collect();
        return Err(TopologyError::NotConnected {
            disconnected_parts: disconnected,
        });
    }

    let boundary_perimeter = validate_boundary_graph(parts, allow_vertex_contact)?;
    let twice_area = parts
        .iter()
        .map(|part| twice_area_fp2_ring(part))
        .sum::<u128>();

    validate_compactness_topology(twice_area, boundary_perimeter, config)?;

    Ok(())
}

fn validate_single_part(part: &[Vertex], config: &ProtocolConfig) -> Result<(), TopologyError> {
    let xs: Vec<i64> = part.iter().map(|v| v[0]).collect();
    let ys: Vec<i64> = part.iter().map(|v| v[1]).collect();
    let twice_area = twice_area_fp2_ring(part);
    let perimeter = perimeter_l1(&xs, &ys);

    validate_compactness_topology(twice_area, perimeter, config)
}

/// Thin wrapper around `validation::check_compactness` that maps the structured
/// outcome to `TopologyError::NotCompact`. Keeps `topology.rs` and
/// `validation.rs` byte-identical in their compactness semantics — both
/// delegate to the same canonical implementation.
fn validate_compactness_topology(
    twice_area: u128,
    perimeter: u128,
    config: &ProtocolConfig,
) -> Result<(), TopologyError> {
    let outcome = check_compactness(twice_area, perimeter, config);
    if outcome.passes {
        Ok(())
    } else {
        Err(TopologyError::NotCompact {
            compactness_ppm: outcome.ratio_ppm,
            min_ppm: outcome.min_ppm,
        })
    }
}

fn split_coords(part: &[Vertex]) -> (Vec<i64>, Vec<i64>) {
    let xs = part.iter().map(|v| v[0]).collect();
    let ys = part.iter().map(|v| v[1]).collect();
    (xs, ys)
}

fn perimeter_l1(xs: &[i64], ys: &[i64]) -> u128 {
    let mut perimeter = 0u128;
    for i in 0..xs.len() {
        let j = (i + 1) % xs.len();
        perimeter += xs[j].abs_diff(xs[i]) as u128 + ys[j].abs_diff(ys[i]) as u128;
    }
    perimeter
}

fn bfs_visited(adjacent: &[Vec<bool>]) -> Vec<bool> {
    let n = adjacent.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::from([0usize]);
    visited[0] = true;

    while let Some(current) = queue.pop_front() {
        for next in 0..n {
            if adjacent[current][next] && !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    visited
}

fn build_vertex_adjacency(parts: &[Vec<Vertex>]) -> Vec<Vec<bool>> {
    let n = parts.len();
    let mut vertex_to_parts: HashMap<Vertex, Vec<usize>> = HashMap::new();
    for (i, part) in parts.iter().enumerate() {
        for &v in part {
            vertex_to_parts.entry(v).or_default().push(i);
        }
    }

    let mut adj = vec![vec![false; n]; n];
    for part_indices in vertex_to_parts.values() {
        for a in 0..part_indices.len() {
            for b in (a + 1)..part_indices.len() {
                adj[part_indices[a]][part_indices[b]] = true;
                adj[part_indices[b]][part_indices[a]] = true;
            }
        }
    }
    adj
}

fn normalize_edge(a: Vertex, b: Vertex) -> Edge {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn validate_boundary_graph(
    parts: &[Vec<Vertex>],
    allow_vertex_contact: bool,
) -> Result<u128, TopologyError> {
    let mut edge_counts: HashMap<Edge, usize> = HashMap::new();
    let all_vertices: Vec<Vertex> = parts.iter().flat_map(|part| part.iter().copied()).collect();

    for part in parts {
        for i in 0..part.len() {
            let j = (i + 1) % part.len();
            let start = part[i];
            let end = part[j];
            let mut points: Vec<Vertex> = all_vertices
                .iter()
                .copied()
                .filter(|point| point_on_segment(start, end, *point))
                .collect();
            points.sort_unstable_by(|a, b| compare_points_on_segment(start, *a, *b));
            points.dedup();

            for pair in points.windows(2) {
                let edge = normalize_edge(pair[0], pair[1]);
                if edge.0 == edge.1 {
                    continue;
                }
                let count = edge_counts.entry(edge).or_insert(0);
                *count += 1;
                if *count > 2 {
                    return Err(TopologyError::HasHoles {
                        boundary_components: 0,
                    });
                }
            }
        }
    }

    let boundary_edges: Vec<Edge> = edge_counts
        .into_iter()
        .filter_map(|(edge, count)| (count == 1).then_some(edge))
        .collect();

    if boundary_edges.is_empty() {
        return Err(TopologyError::HasHoles {
            boundary_components: 0,
        });
    }

    let mut vertex_indices: HashMap<Vertex, usize> = HashMap::new();
    let mut degrees: Vec<usize> = Vec::new();
    let mut edge_pairs: Vec<(usize, usize)> = Vec::with_capacity(boundary_edges.len());
    let mut perimeter = 0u128;

    for &(start, end) in &boundary_edges {
        let start_idx = *vertex_indices.entry(start).or_insert_with(|| {
            degrees.push(0);
            degrees.len() - 1
        });
        let end_idx = *vertex_indices.entry(end).or_insert_with(|| {
            degrees.push(0);
            degrees.len() - 1
        });

        degrees[start_idx] += 1;
        degrees[end_idx] += 1;
        edge_pairs.push((start_idx, end_idx));
        perimeter += start[0].abs_diff(end[0]) as u128 + start[1].abs_diff(end[1]) as u128;
    }

    let degree_ok = if allow_vertex_contact {
        degrees.iter().all(|d| *d >= 2 && *d % 2 == 0)
    } else {
        degrees.iter().all(|d| *d == 2)
    };

    if !degree_ok {
        return Err(TopologyError::HasHoles {
            boundary_components: 0,
        });
    }

    let vertex_count = vertex_indices.len();
    let mut adjacency = vec![Vec::new(); vertex_count];
    for (start, end) in edge_pairs {
        adjacency[start].push(end);
        adjacency[end].push(start);
    }

    let mut component_count = 0usize;
    let mut visited = vec![false; vertex_count];
    for start in 0..vertex_count {
        if visited[start] {
            continue;
        }
        component_count += 1;
        let mut queue = VecDeque::from([start]);
        visited[start] = true;
        while let Some(current) = queue.pop_front() {
            for &next in &adjacency[current] {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
    }

    if component_count > 1 {
        return Err(TopologyError::HasHoles {
            boundary_components: component_count,
        });
    }

    Ok(perimeter)
}

fn point_on_segment(start: Vertex, end: Vertex, point: Vertex) -> bool {
    let (sx, sy) = (start[0] as i128, start[1] as i128);
    let (ex, ey) = (end[0] as i128, end[1] as i128);
    let (px, py) = (point[0] as i128, point[1] as i128);
    let cross = (ex - sx) * (py - sy) - (ey - sy) * (px - sx);

    if cross != 0 {
        return false;
    }

    px >= sx.min(ex) && px <= sx.max(ex) && py >= sy.min(ey) && py <= sy.max(ey)
}

fn compare_points_on_segment(start: Vertex, a: Vertex, b: Vertex) -> std::cmp::Ordering {
    let da = (a[0] - start[0]).abs().max((a[1] - start[1]).abs());
    let db = (b[0] - start[0]).abs().max((b[1] - start[1]).abs());
    da.cmp(&db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ProtocolConfig;

    const M: i64 = 1_000_000;

    fn merca_config() -> ProtocolConfig {
        ProtocolConfig::merca()
    }

    fn square(ox: i64, oy: i64, size: i64) -> Vec<Vertex> {
        vec![
            [ox, oy],
            [ox + size, oy],
            [ox + size, oy + size],
            [ox, oy + size],
        ]
    }

    #[test]
    fn topology_single_part_is_valid() {
        let parts = vec![square(0, 0, 10 * M)];
        assert_eq!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Ok(())
        );
    }

    #[test]
    fn topology_two_adjacent_squares_valid() {
        let parts = vec![square(0, 0, 10 * M), square(10 * M, 0, 10 * M)];
        assert_eq!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Ok(())
        );
    }

    #[test]
    fn topology_two_disconnected_squares_invalid() {
        let parts = vec![square(0, 0, 10 * M), square(30 * M, 0, 10 * M)];
        assert!(matches!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Err(TopologyError::NotConnected { .. })
        ));
    }

    #[test]
    fn topology_vertex_only_contact_rejected() {
        let parts = vec![square(0, 0, 10 * M), square(10 * M, 10 * M, 10 * M)];
        assert_eq!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Err(TopologyError::VertexOnlyContact {
                part_a: 0,
                part_b: 1,
            })
        );
    }

    #[test]
    fn topology_two_parts_forming_l_shape_valid() {
        let parts = vec![
            vec![[0, 0], [20 * M, 0], [20 * M, 10 * M], [10 * M, 10 * M], [0, 10 * M]],
            vec![[0, 10 * M], [10 * M, 10 * M], [10 * M, 20 * M], [0, 20 * M]],
        ];
        assert_eq!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Ok(())
        );
    }

    #[test]
    fn topology_hole_rejected_by_boundary_graph() {
        let parts = vec![
            square(0, 0, 10 * M),
            square(10 * M, 0, 10 * M),
            square(0, 10 * M, 10 * M),
            square(20 * M, 10 * M, 10 * M),
            square(0, 20 * M, 10 * M),
            square(10 * M, 20 * M, 10 * M),
            square(20 * M, 20 * M, 10 * M),
            square(10 * M, 30 * M, 10 * M),
        ];
        assert!(matches!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Err(TopologyError::HasHoles { .. })
        ));
    }

    #[test]
    fn topology_too_many_parts_rejected() {
        let parts: Vec<Vec<Vertex>> = (0..11).map(|i| square(i * 10 * M, 0, 10 * M)).collect();
        assert!(matches!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Err(TopologyError::TooManyParts { .. })
        ));
    }

    #[test]
    fn topology_empty_parts_invalid() {
        assert!(matches!(
            validate_multipart_topology(&[], false, &merca_config()),
            Err(TopologyError::NotConnected { .. })
        ));
    }

    #[test]
    fn not_connected_reports_disconnected_parts() {
        let parts = vec![square(0, 0, M), square(3 * M, 0, M)];
        let result = validate_multipart_topology(&parts, false, &merca_config());
        match result {
            Err(TopologyError::NotConnected { disconnected_parts }) => {
                assert!(disconnected_parts.contains(&1));
            }
            other => panic!("expected NotConnected, got {:?}", other),
        }
    }

    #[test]
    fn too_many_parts_reports_count() {
        let parts: Vec<Vec<Vertex>> = (0..11).map(|i| square(i as i64 * M, 0, M)).collect();
        let result = validate_multipart_topology(&parts, false, &merca_config());
        match result {
            Err(TopologyError::TooManyParts { count, max }) => {
                assert_eq!(count, 11);
                assert_eq!(max, 10);
            }
            other => panic!("expected TooManyParts, got {:?}", other),
        }
    }

    #[test]
    fn allow_vertex_contact_accepts_corner_touching() {
        let parts = vec![square(0, 0, M), square(M, 0, M), square(0, M, M)];
        let result = validate_multipart_topology(&parts, true, &merca_config());
        if let Err(TopologyError::VertexOnlyContact { .. }) = result {
            panic!("allow_vertex_contact=true should not reject vertex-only contact");
        }
    }

    #[test]
    fn vertex_contact_false_rejects_corner_only() {
        let parts = vec![square(0, 0, 10 * M), square(10 * M, 10 * M, 10 * M)];
        assert!(matches!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Err(TopologyError::VertexOnlyContact {
                part_a: 0,
                part_b: 1
            })
        ));
    }

    /// The connecting strip (20m × 1m) would fail per-part compactness in
    /// isolation, but the assembled dumbbell has a well-shaped outer boundary.
    /// On-chain accepts it; the Rust library must mirror that (this is the
    /// regression for Rust's historical per-part compactness check).
    #[test]
    fn dumbbell_with_thin_strip_accepted() {
        // Mirrors `polygon_accepts_dumbbell_with_thin_connecting_strip` in
        // polygon.move — same vertex coordinates, same expected decision.
        let left = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 9 * M],
            [20 * M, 10 * M],
            [20 * M, 20 * M],
            [0, 20 * M],
        ];
        let strip = vec![
            [20 * M, 9 * M],
            [50 * M, 9 * M],
            [50 * M, 10 * M],
            [20 * M, 10 * M],
        ];
        let right = vec![
            [50 * M, 0],
            [70 * M, 0],
            [70 * M, 20 * M],
            [50 * M, 20 * M],
            [50 * M, 10 * M],
            [50 * M, 9 * M],
        ];
        let parts = vec![left, strip, right];
        assert_eq!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Ok(())
        );
    }

    /// Rectangle base + triangle roof with partial collinear contact (T-junction).
    /// Mirrors polygon.move::polygon_rejects_partial_edge_contact.
    /// On-chain aborts EInvalidMultipartContact; Rust must emit UnsupportedContact.
    #[test]
    fn t_junction_partial_edge_rejected() {
        let base = vec![
            [0, 0],
            [4 * M, 0],
            [4 * M, M],
            [0, M],
        ];
        let roof = vec![
            [M, M],
            [3 * M, M],
            [2 * M, 2 * M],
        ];
        let parts = vec![base, roof];
        assert!(matches!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Err(TopologyError::UnsupportedContact { part_a: 0, part_b: 1, .. })
        ));
    }

    /// A 100m × 1m needle: a single long thin part. Aspect ratio is ~100:1 —
    /// well past the isoperimetric threshold. Boundary compactness must still
    /// reject it, proving defence-in-depth didn't regress when per-part
    /// compactness was removed.
    #[test]
    fn needle_like_single_part_rejected_by_boundary_compactness() {
        let parts = vec![vec![
            [0, 0],
            [100 * M, 0],
            [100 * M, M],
            [0, M],
        ]];
        assert!(matches!(
            validate_multipart_topology(&parts, false, &merca_config()),
            Err(TopologyError::NotCompact { .. })
        ));
    }
}
