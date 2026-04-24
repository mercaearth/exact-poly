//! Full on-chain validation pipeline for polygon decompositions.
//!
//! Replicates the exact sequence of checks that `polygon.move::prepare_geometry`
//! and `index.move::register` perform. If `validate_decomposition` returns Ok,
//! the decomposition WILL pass on-chain registration (modulo spatial overlap
//! with existing parcels, which requires on-chain state).
//!
//! Check order matches on-chain:
//! 1. Part count within limits
//! 2. Per-part validation (vertex count, convexity, edge lengths)
//! 3. Area > 0
//! 4. Area conservation (sum of parts == original)
//! 5. Internal overlap check (no parts overlap each other)
//! 6. Multipart topology (connectivity, boundary graph, boundary compactness)
//!
//! Compactness is a boundary-level invariant (step 6), not a per-part one —
//! see the `validation` module docs for the rationale that mirrors polygon.move.

use crate::area::{areas_conserved, twice_area_fp2};
use crate::overlap::convex_parts_overlap;
use crate::topology::validate_multipart_topology;
use crate::types::{ProtocolConfig, TopologyError};
use crate::validation::validate_part;
use serde::{Deserialize, Serialize};

/// A single validation check result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationCheck {
    /// Name of the check (e.g. "part_0_convex", "area_conservation").
    pub name: String,
    /// true = passed, false = failed.
    pub passed: bool,
    /// Human-readable detail (error message or "OK").
    pub detail: String,
    /// Severity: "error" = would fail on-chain, "warn" = suspicious but allowed.
    pub severity: String,
}

/// Full validation report for a polygon decomposition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    /// All individual checks, in order.
    pub checks: Vec<ValidationCheck>,
    /// true if ALL checks passed (decomposition is valid on-chain).
    pub valid: bool,
    /// Number of errors.
    pub error_count: usize,
    /// Number of warnings.
    pub warn_count: usize,
    /// Original polygon twice-area (string for u128).
    pub original_twice_area: String,
    /// Sum of part twice-areas (string for u128).
    pub parts_twice_area_sum: String,
    /// Per-part twice-areas (strings for u128).
    pub part_areas: Vec<String>,
}

fn check_ok(name: impl Into<String>, detail: impl Into<String>) -> ValidationCheck {
    ValidationCheck {
        name: name.into(),
        passed: true,
        detail: detail.into(),
        severity: "ok".into(),
    }
}

fn check_err(name: impl Into<String>, detail: impl Into<String>) -> ValidationCheck {
    ValidationCheck {
        name: name.into(),
        passed: false,
        detail: detail.into(),
        severity: "error".into(),
    }
}

fn check_warn(name: impl Into<String>, detail: impl Into<String>) -> ValidationCheck {
    ValidationCheck {
        name: name.into(),
        passed: true,
        detail: detail.into(),
        severity: "warn".into(),
    }
}

/// Run the full on-chain validation pipeline on a polygon and its decomposition.
///
/// `ring` — the original polygon vertices (before decomposition).
/// `parts` — the convex parts from decomposition.
/// `config` — protocol config (use `ProtocolConfig::merca()` for on-chain defaults).
///
/// Returns a detailed report with every check and its result.
pub fn validate_decomposition(
    ring: &[[i64; 2]],
    parts: &[Vec<[i64; 2]>],
    config: &ProtocolConfig,
) -> ValidationReport {
    let mut checks = Vec::new();
    let mut error_count = 0usize;
    let mut warn_count = 0usize;

    let mut push = |c: ValidationCheck| {
        if c.severity == "error" && !c.passed {
            error_count += 1;
        } else if c.severity == "warn" {
            warn_count += 1;
        }
        checks.push(c);
    };

    // 1. Part count
    let n = parts.len();
    if n == 0 {
        push(check_err("part_count", "no parts"));
    } else if n > config.max_parts {
        push(check_err(
            "part_count",
            format!("{n} parts exceeds max {}", config.max_parts),
        ));
    } else {
        push(check_ok(
            "part_count",
            format!("{n} (max {})", config.max_parts),
        ));
    }

    // 2. Per-part validation
    let mut part_areas: Vec<u128> = Vec::with_capacity(n);
    let mut total_vertices = 0usize;

    for (i, part) in parts.iter().enumerate() {
        total_vertices += part.len();

        // Vertex count
        if part.len() < 3 {
            push(check_err(
                format!("part_{i}_vertices"),
                format!("{} vertices (min 3)", part.len()),
            ));
        } else if part.len() > config.max_vertices_per_part {
            push(check_err(
                format!("part_{i}_vertices"),
                format!(
                    "{} vertices (max {})",
                    part.len(),
                    config.max_vertices_per_part
                ),
            ));
        } else {
            push(check_ok(
                format!("part_{i}_vertices"),
                format!("{} (max {})", part.len(), config.max_vertices_per_part),
            ));
        }

        // Per-part structural validation (convexity, edges).
        // Boundary compactness is checked once at step 6 below.
        match validate_part(part, config) {
            None => {
                push(check_ok(format!("part_{i}_valid"), "OK"));
            }
            Some(err) => {
                push(check_err(format!("part_{i}_valid"), err));
            }
        }

        // Area
        let area = twice_area_fp2(part);
        part_areas.push(area);
        if area == 0 {
            push(check_err(format!("part_{i}_area"), "zero area"));
        } else {
            push(check_ok(format!("part_{i}_area"), format!("2A = {area}")));
        }

        // Coordinate range (MAX_WORLD = 40_075_017_000_000)
        let max_world = crate::constants::MAX_WORLD as i64;
        let out_of_range = part
            .iter()
            .flat_map(|vertex| vertex.iter())
            .any(|&c| c < 0 || c > max_world);
        if out_of_range {
            // On-chain uses u64, so negatives are impossible there.
            // In WASM we use i64, so negative coords are valid for demos but won't pass on-chain.
            push(check_warn(
                format!("part_{i}_coords"),
                "coordinates outside [0, MAX_WORLD] — will fail on-chain",
            ));
        }
    }

    // Total vertex count
    let max_total = config.max_vertices_per_part.saturating_mul(n.max(1));
    if total_vertices > max_total {
        push(check_err(
            "total_vertices",
            format!("{total_vertices} > max {max_total}"),
        ));
    } else {
        push(check_ok(
            "total_vertices",
            format!("{total_vertices} (max {max_total})"),
        ));
    }

    // 3. Total area > 0
    let parts_area_sum: u128 = part_areas.iter().sum();
    if parts_area_sum == 0 {
        push(check_err("total_area", "zero total area"));
    } else {
        push(check_ok("total_area", format!("2A = {parts_area_sum}")));
    }

    // 4. Area conservation
    let original_area = twice_area_fp2(ring);
    if areas_conserved(original_area, &part_areas) {
        push(check_ok(
            "area_conservation",
            format!("original={original_area}, sum={parts_area_sum}"),
        ));
    } else {
        let diff = if original_area > parts_area_sum {
            original_area - parts_area_sum
        } else {
            parts_area_sum - original_area
        };
        push(check_err(
            "area_conservation",
            format!("MISMATCH: original={original_area}, sum={parts_area_sum}, diff={diff}"),
        ));
    }

    // 5. Internal overlap check (pairwise)
    if n > 1 {
        let mut has_overlap = false;
        for i in 0..n {
            for j in (i + 1)..n {
                if convex_parts_overlap(&parts[i], &parts[j]) {
                    push(check_err(
                        format!("parts_overlap_{i}_{j}"),
                        format!("parts {i} and {j} overlap"),
                    ));
                    has_overlap = true;
                }
            }
        }
        if !has_overlap {
            push(check_ok("parts_no_overlap", "no pairwise overlap"));
        }
    }

    // 6. Multipart topology
    if n > 0 {
        match validate_multipart_topology(parts, false, config) {
            Ok(()) => {
                push(check_ok("topology", "valid"));
            }
            Err(topo_err) => {
                push(check_err("topology", format_topology_error(&topo_err)));
            }
        }
    }

    let valid = error_count == 0;
    ValidationReport {
        checks,
        valid,
        error_count,
        warn_count,
        original_twice_area: original_area.to_string(),
        parts_twice_area_sum: parts_area_sum.to_string(),
        part_areas: part_areas.iter().map(|a| a.to_string()).collect(),
    }
}

fn format_topology_error(err: &TopologyError) -> String {
    match err {
        TopologyError::NotConnected { disconnected_parts } => {
            format!("not connected (disconnected: {disconnected_parts:?})")
        }
        TopologyError::HasHoles {
            boundary_components,
        } => {
            format!("has holes ({boundary_components} boundary components)")
        }
        TopologyError::VertexOnlyContact { part_a, part_b } => {
            format!("vertex-only contact between parts {part_a} and {part_b}")
        }
        TopologyError::UnsupportedContact {
            part_a,
            part_b,
            reason,
        } => {
            format!("unsupported contact {part_a}↔{part_b}: {reason}")
        }
        TopologyError::TooManyParts { count, max } => {
            format!("too many parts ({count} > {max})")
        }
        TopologyError::NotCompact {
            compactness_ppm,
            min_ppm,
        } => {
            format!("not compact ({compactness_ppm} ppm < {min_ppm} ppm)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ProtocolConfig;

    const M: i64 = 1_000_000;

    fn merca() -> ProtocolConfig {
        ProtocolConfig::merca()
    }

    fn permissive() -> ProtocolConfig {
        ProtocolConfig::permissive()
    }

    #[test]
    fn valid_single_convex_part() {
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let parts = vec![ring.clone()];
        let report = validate_decomposition(&ring, &parts, &merca());
        assert!(
            report.valid,
            "errors: {:?}",
            report
                .checks
                .iter()
                .filter(|c| !c.passed)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_two_part_l_shape() {
        let ring = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];
        let parts = vec![
            vec![
                [0, 0],
                [20 * M, 0],
                [20 * M, 10 * M],
                [10 * M, 10 * M],
                [0, 10 * M],
            ],
            vec![[0, 10 * M], [10 * M, 10 * M], [10 * M, 20 * M], [0, 20 * M]],
        ];
        let report = validate_decomposition(&ring, &parts, &merca());
        assert!(
            report.valid,
            "errors: {:?}",
            report
                .checks
                .iter()
                .filter(|c| !c.passed)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn valid_two_part_l_shape_permissive_config() {
        let ring = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];
        let parts = vec![
            vec![
                [0, 0],
                [20 * M, 0],
                [20 * M, 10 * M],
                [10 * M, 10 * M],
                [0, 10 * M],
            ],
            vec![[0, 10 * M], [10 * M, 10 * M], [10 * M, 20 * M], [0, 20 * M]],
        ];
        let report = validate_decomposition(&ring, &parts, &permissive());
        assert!(
            report.valid,
            "errors: {:?}",
            report
                .checks
                .iter()
                .filter(|c| !c.passed)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn rejects_overlapping_parts() {
        let ring = vec![[0, 0], [20 * M, 0], [20 * M, 20 * M], [0, 20 * M]];
        // Two overlapping squares
        let parts = vec![
            vec![[0, 0], [15 * M, 0], [15 * M, 15 * M], [0, 15 * M]],
            vec![
                [5 * M, 5 * M],
                [20 * M, 5 * M],
                [20 * M, 20 * M],
                [5 * M, 20 * M],
            ],
        ];
        let report = validate_decomposition(&ring, &parts, &merca());
        assert!(!report.valid);
        assert!(report
            .checks
            .iter()
            .any(|c| c.name.starts_with("parts_overlap")));
    }

    #[test]
    fn rejects_overlapping_parts_permissive_config() {
        let ring = vec![[0, 0], [20 * M, 0], [20 * M, 20 * M], [0, 20 * M]];
        let parts = vec![
            vec![[0, 0], [15 * M, 0], [15 * M, 15 * M], [0, 15 * M]],
            vec![
                [5 * M, 5 * M],
                [20 * M, 5 * M],
                [20 * M, 20 * M],
                [5 * M, 20 * M],
            ],
        ];
        let report = validate_decomposition(&ring, &parts, &permissive());
        assert!(!report.valid);
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "parts_overlap_0_1" && !c.passed));
    }

    #[test]
    fn rejects_area_mismatch() {
        let ring = vec![[0, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        // Part is smaller than original
        let parts = vec![vec![[0, 0], [5 * M, 0], [5 * M, 5 * M], [0, 5 * M]]];
        let report = validate_decomposition(&ring, &parts, &merca());
        assert!(!report.valid);
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "area_conservation" && !c.passed));
    }

    #[test]
    fn rejects_area_mismatch_with_connected_parts() {
        let ring = vec![[0, 0], [20 * M, 0], [20 * M, 20 * M], [0, 20 * M]];
        let parts = vec![
            vec![[0, 0], [20 * M, 0], [20 * M, 10 * M], [0, 10 * M]],
            vec![[0, 10 * M], [10 * M, 10 * M], [10 * M, 20 * M], [0, 20 * M]],
        ];
        let report = validate_decomposition(&ring, &parts, &permissive());
        assert!(!report.valid);
        assert!(report
            .checks
            .iter()
            .any(|c| c.name == "area_conservation" && !c.passed));
    }

    #[test]
    fn warns_negative_coords() {
        let ring = vec![
            [-5 * M, -5 * M],
            [5 * M, -5 * M],
            [5 * M, 5 * M],
            [-5 * M, 5 * M],
        ];
        let parts = vec![ring.clone()];
        let report = validate_decomposition(&ring, &parts, &merca());
        assert!(report
            .checks
            .iter()
            .any(|c| c.name.contains("coords") && c.severity == "warn"));
    }
}
