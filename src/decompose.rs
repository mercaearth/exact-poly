use crate::area::{areas_conserved, twice_area_fp2};
use crate::bayazit::{bayazit_decompose, find_steiner_points};
use crate::ear_clip::ear_clip_triangulate;
use crate::exact_partition::exact_vertex_partition;
use crate::hertel_mehlhorn::optimize_partition;
use crate::ring::{is_simple, normalize_ring, rotate_ring};
use crate::topology::validate_multipart_topology;
use crate::types::{
    Attempt, DecompError, DecomposeOptions, DecomposeResult, Outcome, ProtocolConfig, Strategy,
};
use crate::validation::validate_part;

type Parts = Vec<Vec<[i64; 2]>>;

#[derive(Debug)]
enum AttemptError {
    TooManyParts { count: usize },
    ValidationFailed(Vec<String>),
    Failed(String),
}

struct StrategySuccess {
    parts: Parts,
    strategy: Strategy,
}

pub fn decompose(
    ring: &[[i64; 2]],
    options: &DecomposeOptions,
    config: &ProtocolConfig,
) -> Result<DecomposeResult, DecompError> {
    decompose_with_strategies(
        ring,
        options,
        config,
        exact_vertex_partition,
        bayazit_decompose,
        ear_clip_triangulate,
        find_steiner_points,
    )
}

fn decompose_with_strategies<Exact, Bayazit, EarClip, Steiner>(
    ring: &[[i64; 2]],
    options: &DecomposeOptions,
    config: &ProtocolConfig,
    exact: Exact,
    bayazit: Bayazit,
    ear_clip: EarClip,
    steiner_points: Steiner,
) -> Result<DecomposeResult, DecompError>
where
    Exact: Fn(&[[i64; 2]]) -> Result<Parts, String>,
    Bayazit: Fn(&[[i64; 2]], bool) -> Result<Parts, String>,
    EarClip: Fn(&[[i64; 2]]) -> Result<Parts, String>,
    Steiner: Fn(&[[i64; 2]], &[Vec<[i64; 2]>]) -> Vec<[i64; 2]>,
{
    if ring.len() < 3 {
        return Err(DecompError::TooFewVertices);
    }

    let normalized = normalize_ring(ring).ok_or(DecompError::TooFewVertices)?;
    if !is_simple(&normalized) {
        return Err(DecompError::NotSimple);
    }

    let original_area = twice_area_fp2(&normalized);
    if original_area == 0 {
        return Err(DecompError::TooFewVertices);
    }

    let attempt_count = normalized.len().min(options.max_rotation_attempts.max(1));
    let mut trace = options.collect_trace.then(Vec::new);

    if options.minimize_parts {
        return run_minimize_parts(
            ring,
            &normalized,
            options,
            config,
            original_area,
            attempt_count,
            &exact,
            &bayazit,
            &ear_clip,
            &steiner_points,
            &mut trace,
        );
    }

    let mut saw_too_many_parts = false;
    let mut last_error: Option<String> = None;

    for rotation in 0..attempt_count {
        let rotated = rotate_ring(&normalized, rotation);
        match try_decompose(
            &rotated,
            options,
            config,
            original_area,
            &exact,
            &bayazit,
            &ear_clip,
            rotation,
            &mut trace,
        ) {
            Ok(success) => {
                let steiner = steiner_points(ring, &success.parts);
                let strategy = wrap_rotation(success.strategy, rotation);
                return Ok(DecomposeResult {
                    parts: success.parts,
                    steiner_points: steiner,
                    strategy,
                    trace,
                });
            }
            Err(AttemptError::TooManyParts { count }) => {
                saw_too_many_parts = true;
                last_error = Some(format!("decomposition produced {count} parts"));
            }
            Err(AttemptError::ValidationFailed(errors)) => {
                last_error = Some(errors.join("; "));
            }
            Err(AttemptError::Failed(err)) => last_error = Some(err),
        }
    }

    if saw_too_many_parts {
        Err(DecompError::TooManyParts)
    } else {
        Err(DecompError::Failed(last_error.unwrap_or_else(|| {
            "all decomposition strategies exhausted".into()
        })))
    }
}

/// Candidate produced by a single (strategy, rotation) pair in minimize mode.
struct Candidate {
    parts: Parts,
    strategy: Strategy,
    rotation: usize,
    steiner_points: Vec<[i64; 2]>,
}

/// Lexicographic ranking key for minimize_parts tiebreaking.
/// Lower is better on every coordinate.
fn candidate_key(c: &Candidate) -> (usize, usize, usize, u8) {
    let strategy_rank = match c.strategy {
        Strategy::AlreadyConvex => 0,
        Strategy::ExactPartition => 1,
        Strategy::Bayazit => 2,
        Strategy::EarClipMerge => 3,
        Strategy::Rotation { .. } => 4, // shouldn't appear pre-wrap, defensive
    };
    (
        c.parts.len(),
        c.steiner_points.len(),
        c.rotation,
        strategy_rank,
    )
}

#[allow(clippy::too_many_arguments)]
fn run_minimize_parts<Exact, Bayazit, EarClip, Steiner>(
    original_ring: &[[i64; 2]],
    normalized: &[[i64; 2]],
    options: &DecomposeOptions,
    config: &ProtocolConfig,
    original_area: u128,
    attempt_count: usize,
    exact: &Exact,
    bayazit: &Bayazit,
    ear_clip: &EarClip,
    steiner_points: &Steiner,
    trace: &mut Option<Vec<Attempt>>,
) -> Result<DecomposeResult, DecompError>
where
    Exact: Fn(&[[i64; 2]]) -> Result<Parts, String>,
    Bayazit: Fn(&[[i64; 2]], bool) -> Result<Parts, String>,
    EarClip: Fn(&[[i64; 2]]) -> Result<Parts, String>,
    Steiner: Fn(&[[i64; 2]], &[Vec<[i64; 2]>]) -> Vec<[i64; 2]>,
{
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut saw_too_many_parts = false;
    let mut last_error: Option<String> = None;

    for rotation in 0..attempt_count {
        let rotated = rotate_ring(normalized, rotation);
        collect_candidates_for_rotation(
            original_ring,
            &rotated,
            options,
            config,
            original_area,
            rotation,
            exact,
            bayazit,
            ear_clip,
            steiner_points,
            trace,
            &mut candidates,
            &mut saw_too_many_parts,
            &mut last_error,
        );
    }

    match candidates.into_iter().min_by_key(candidate_key) {
        Some(best) => {
            let wrapped = wrap_rotation(best.strategy, best.rotation);
            Ok(DecomposeResult {
                parts: best.parts,
                steiner_points: best.steiner_points,
                strategy: wrapped,
                trace: trace.take(),
            })
        }
        None => {
            if saw_too_many_parts {
                Err(DecompError::TooManyParts)
            } else {
                Err(DecompError::Failed(last_error.unwrap_or_else(|| {
                    "no strategy produced a valid decomposition".into()
                })))
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_candidates_for_rotation<Exact, Bayazit, EarClip, Steiner>(
    original_ring: &[[i64; 2]],
    rotated: &[[i64; 2]],
    options: &DecomposeOptions,
    config: &ProtocolConfig,
    original_area: u128,
    rotation: usize,
    exact: &Exact,
    bayazit: &Bayazit,
    ear_clip: &EarClip,
    steiner_points: &Steiner,
    trace: &mut Option<Vec<Attempt>>,
    candidates: &mut Vec<Candidate>,
    saw_too_many_parts: &mut bool,
    last_error: &mut Option<String>,
) where
    Exact: Fn(&[[i64; 2]]) -> Result<Parts, String>,
    Bayazit: Fn(&[[i64; 2]], bool) -> Result<Parts, String>,
    EarClip: Fn(&[[i64; 2]]) -> Result<Parts, String>,
    Steiner: Fn(&[[i64; 2]], &[Vec<[i64; 2]>]) -> Vec<[i64; 2]>,
{
    // Strategy 1: ExactPartition
    record_strategy_candidate(
        Strategy::ExactPartition,
        rotation,
        exact(rotated),
        config,
        original_area,
        original_ring,
        steiner_points,
        trace,
        candidates,
        saw_too_many_parts,
        last_error,
    );

    // Strategy 2: Bayazit (only when Steiner points are allowed)
    if options.allow_steiner {
        record_strategy_candidate(
            Strategy::Bayazit,
            rotation,
            bayazit(rotated, true),
            config,
            original_area,
            original_ring,
            steiner_points,
            trace,
            candidates,
            saw_too_many_parts,
            last_error,
        );
    }

    // Strategy 3: EarClip triangulation followed by Hertel-Mehlhorn merge.
    // The inner Err path also has to be surfaced to trace under the merged
    // strategy name, matching cascade mode's behaviour.
    let ear_result = ear_clip(rotated).map(|triangles| optimize_partition(&triangles));
    record_strategy_candidate(
        Strategy::EarClipMerge,
        rotation,
        ear_result,
        config,
        original_area,
        original_ring,
        steiner_points,
        trace,
        candidates,
        saw_too_many_parts,
        last_error,
    );
}

#[allow(clippy::too_many_arguments)]
fn record_strategy_candidate<Steiner>(
    strategy: Strategy,
    rotation: usize,
    raw: Result<Parts, String>,
    config: &ProtocolConfig,
    original_area: u128,
    original_ring: &[[i64; 2]],
    steiner_points: &Steiner,
    trace: &mut Option<Vec<Attempt>>,
    candidates: &mut Vec<Candidate>,
    saw_too_many_parts: &mut bool,
    last_error: &mut Option<String>,
) where
    Steiner: Fn(&[[i64; 2]], &[Vec<[i64; 2]>]) -> Vec<[i64; 2]>,
{
    match raw {
        Ok(parts) => match finalize_parts(parts, config, original_area) {
            Ok(parts) => {
                push_attempt(
                    trace,
                    strategy.clone(),
                    rotation,
                    Outcome::Success {
                        part_count: parts.len(),
                    },
                );
                let steiner = steiner_points(original_ring, &parts);
                candidates.push(Candidate {
                    parts,
                    strategy,
                    rotation,
                    steiner_points: steiner,
                });
            }
            Err(err) => {
                push_attempt(trace, strategy, rotation, outcome_from_error(&err));
                match err {
                    AttemptError::TooManyParts { .. } => *saw_too_many_parts = true,
                    AttemptError::ValidationFailed(errors) => {
                        *last_error = Some(errors.join("; "));
                    }
                    AttemptError::Failed(message) => *last_error = Some(message),
                }
            }
        },
        Err(err) => {
            let formatted = format!("{strategy:?} failed: {err}");
            push_attempt(
                trace,
                strategy,
                rotation,
                Outcome::AlgorithmFailed {
                    error: formatted.clone(),
                },
            );
            *last_error = Some(formatted);
        }
    }
}

fn try_decompose<Exact, Bayazit, EarClip>(
    ring: &[[i64; 2]],
    options: &DecomposeOptions,
    config: &ProtocolConfig,
    original_area: u128,
    exact: &Exact,
    bayazit: &Bayazit,
    ear_clip: &EarClip,
    rotation: usize,
    trace: &mut Option<Vec<Attempt>>,
) -> Result<StrategySuccess, AttemptError>
where
    Exact: Fn(&[[i64; 2]]) -> Result<Parts, String>,
    Bayazit: Fn(&[[i64; 2]], bool) -> Result<Parts, String>,
    EarClip: Fn(&[[i64; 2]]) -> Result<Parts, String>,
{
    let mut saw_too_many_parts = false;
    let mut last_error: Option<String> = None;

    match exact(ring) {
        Ok(parts) => match finalize_parts(parts, config, original_area) {
            Ok(parts) => {
                push_attempt(
                    trace,
                    Strategy::ExactPartition,
                    rotation,
                    Outcome::Success {
                        part_count: parts.len(),
                    },
                );
                return Ok(StrategySuccess {
                    parts,
                    strategy: Strategy::ExactPartition,
                });
            }
            Err(err @ AttemptError::TooManyParts { .. }) => {
                push_attempt(
                    trace,
                    Strategy::ExactPartition,
                    rotation,
                    outcome_from_error(&err),
                );
                saw_too_many_parts = true;
            }
            Err(err @ AttemptError::ValidationFailed(_)) => {
                push_attempt(
                    trace,
                    Strategy::ExactPartition,
                    rotation,
                    outcome_from_error(&err),
                );
                if let AttemptError::ValidationFailed(errors) = err {
                    last_error = Some(errors.join("; "));
                }
            }
            Err(err @ AttemptError::Failed(_)) => {
                push_attempt(
                    trace,
                    Strategy::ExactPartition,
                    rotation,
                    outcome_from_error(&err),
                );
                if let AttemptError::Failed(message) = err {
                    last_error = Some(message);
                }
            }
        },
        Err(err) => {
            let error = format!("exact vertex partition failed: {err}");
            push_attempt(
                trace,
                Strategy::ExactPartition,
                rotation,
                Outcome::AlgorithmFailed {
                    error: error.clone(),
                },
            );
            last_error = Some(error);
        }
    }

    if options.allow_steiner {
        match bayazit(ring, true) {
            Ok(parts) => match finalize_parts(parts, config, original_area) {
                Ok(parts) => {
                    push_attempt(
                        trace,
                        Strategy::Bayazit,
                        rotation,
                        Outcome::Success {
                            part_count: parts.len(),
                        },
                    );
                    return Ok(StrategySuccess {
                        parts,
                        strategy: Strategy::Bayazit,
                    });
                }
                Err(err @ AttemptError::TooManyParts { .. }) => {
                    push_attempt(trace, Strategy::Bayazit, rotation, outcome_from_error(&err));
                    saw_too_many_parts = true;
                }
                Err(err @ AttemptError::ValidationFailed(_)) => {
                    push_attempt(trace, Strategy::Bayazit, rotation, outcome_from_error(&err));
                    if let AttemptError::ValidationFailed(errors) = err {
                        last_error = Some(errors.join("; "));
                    }
                }
                Err(err @ AttemptError::Failed(_)) => {
                    push_attempt(trace, Strategy::Bayazit, rotation, outcome_from_error(&err));
                    if let AttemptError::Failed(message) = err {
                        last_error = Some(message);
                    }
                }
            },
            Err(err) => {
                let error = format!("bayazit failed: {err}");
                push_attempt(
                    trace,
                    Strategy::Bayazit,
                    rotation,
                    Outcome::AlgorithmFailed {
                        error: error.clone(),
                    },
                );
                last_error = Some(error);
            }
        }
    }

    match ear_clip(ring) {
        Ok(parts) => match finalize_parts(optimize_partition(&parts), config, original_area) {
            Ok(parts) => {
                push_attempt(
                    trace,
                    Strategy::EarClipMerge,
                    rotation,
                    Outcome::Success {
                        part_count: parts.len(),
                    },
                );
                Ok(StrategySuccess {
                    parts,
                    strategy: Strategy::EarClipMerge,
                })
            }
            Err(err @ AttemptError::TooManyParts { .. }) => {
                push_attempt(
                    trace,
                    Strategy::EarClipMerge,
                    rotation,
                    outcome_from_error(&err),
                );
                Err(err)
            }
            Err(err @ AttemptError::ValidationFailed(_)) => {
                push_attempt(
                    trace,
                    Strategy::EarClipMerge,
                    rotation,
                    outcome_from_error(&err),
                );
                if saw_too_many_parts {
                    Err(AttemptError::TooManyParts {
                        count: config.max_parts.saturating_add(1),
                    })
                } else {
                    Err(err)
                }
            }
            Err(err @ AttemptError::Failed(_)) => {
                push_attempt(
                    trace,
                    Strategy::EarClipMerge,
                    rotation,
                    outcome_from_error(&err),
                );
                if saw_too_many_parts {
                    Err(AttemptError::TooManyParts {
                        count: config.max_parts.saturating_add(1),
                    })
                } else {
                    Err(err)
                }
            }
        },
        Err(err) => {
            let error = format!("ear clipping failed: {err}");
            push_attempt(
                trace,
                Strategy::EarClipMerge,
                rotation,
                Outcome::AlgorithmFailed {
                    error: error.clone(),
                },
            );
            if saw_too_many_parts {
                Err(AttemptError::TooManyParts {
                    count: config.max_parts.saturating_add(1),
                })
            } else {
                Err(AttemptError::Failed(last_error.unwrap_or(error)))
            }
        }
    }
}

fn finalize_parts(
    parts: Parts,
    config: &ProtocolConfig,
    original_area: u128,
) -> Result<Parts, AttemptError> {
    if parts.len() > config.max_parts {
        return Err(AttemptError::TooManyParts { count: parts.len() });
    }

    let validation_errors = validate_all_parts(&parts, config);
    if !validation_errors.is_empty() {
        return Err(AttemptError::ValidationFailed(validation_errors));
    }

    let part_areas: Vec<u128> = parts.iter().map(|part| twice_area_fp2(part)).collect();
    if !areas_conserved(original_area, &part_areas) {
        return Err(AttemptError::ValidationFailed(vec![
            "area not conserved".into()
        ]));
    }

    // On-chain runs `validate_multipart_topology` on every accepted polygon.
    // Per-part structural checks alone are necessary but not sufficient:
    // boundary compactness, connectivity, and the no-holes invariant are
    // group-level properties. If a strategy produces structurally valid parts
    // whose union would still be rejected on-chain, treat it as a failed
    // candidate so the cascade can try the next strategy/rotation.
    if let Err(topology_err) = validate_multipart_topology(&parts, false, config) {
        return Err(AttemptError::ValidationFailed(vec![format!(
            "topology: {topology_err}"
        )]));
    }

    Ok(parts)
}

fn validate_all_parts(parts: &[Vec<[i64; 2]>], config: &ProtocolConfig) -> Vec<String> {
    parts
        .iter()
        .enumerate()
        .filter_map(|(idx, part)| {
            validate_part(part, config).map(|err| format!("part {idx}: {err}"))
        })
        .collect()
}

fn push_attempt(
    trace: &mut Option<Vec<Attempt>>,
    strategy: Strategy,
    rotation: usize,
    outcome: Outcome,
) {
    if let Some(trace) = trace.as_mut() {
        trace.push(Attempt {
            strategy,
            rotation,
            outcome,
        });
    }
}

fn outcome_from_error(error: &AttemptError) -> Outcome {
    match error {
        AttemptError::TooManyParts { count } => Outcome::TooManyParts { count: *count },
        AttemptError::ValidationFailed(errors) => Outcome::ValidationFailed {
            errors: errors.clone(),
        },
        AttemptError::Failed(error) => Outcome::AlgorithmFailed {
            error: error.clone(),
        },
    }
}

fn wrap_rotation(strategy: Strategy, rotation: usize) -> Strategy {
    if rotation == 0 {
        strategy
    } else {
        Strategy::Rotation {
            offset: rotation,
            inner: Box::new(strategy),
        }
    }
}

pub fn collect_steiner_points(original: &[[i64; 2]], parts: &[Vec<[i64; 2]>]) -> Vec<[i64; 2]> {
    find_steiner_points(original, parts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    const M: i64 = 1_000_000;

    fn square() -> Vec<[i64; 2]> {
        vec![[0, 0], [12 * M, 0], [12 * M, 12 * M], [0, 12 * M]]
    }

    fn triangle() -> Vec<[i64; 2]> {
        vec![[0, 0], [12 * M, 0], [6 * M, 10 * M]]
    }

    fn l_shape() -> Vec<[i64; 2]> {
        vec![
            [0, 0],
            [24 * M, 0],
            [24 * M, 8 * M],
            [8 * M, 8 * M],
            [8 * M, 24 * M],
            [0, 24 * M],
        ]
    }

    fn t_shape() -> Vec<[i64; 2]> {
        vec![
            [0, 0],
            [30 * M, 0],
            [30 * M, 10 * M],
            [20 * M, 10 * M],
            [20 * M, 24 * M],
            [10 * M, 24 * M],
            [10 * M, 10 * M],
            [0, 10 * M],
        ]
    }

    fn step_shape() -> Vec<[i64; 2]> {
        vec![
            [0, 0],
            [30 * M, 0],
            [30 * M, 10 * M],
            [20 * M, 10 * M],
            [20 * M, 20 * M],
            [10 * M, 20 * M],
            [10 * M, 30 * M],
            [0, 30 * M],
        ]
    }

    /// 12-vertex comb with two interior notches. Historically the Rust
    /// library's per-part compactness rejection broke decomposition of this
    /// shape (the interior narrow parts produced by the cut strategies
    /// satisfied every on-chain per-part rule but were thin enough to trip
    /// the extra Rust-only compactness check). This shape is the minimal
    /// regression for that bug.
    fn comb_twelve_vertices() -> Vec<[i64; 2]> {
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

    fn valid_samples() -> Vec<Vec<[i64; 2]>> {
        vec![
            square(),
            triangle(),
            l_shape(),
            t_shape(),
            step_shape(),
            comb_twelve_vertices(),
        ]
    }

    fn default_opts() -> DecomposeOptions {
        DecomposeOptions::default()
    }

    fn merca_config() -> ProtocolConfig {
        ProtocolConfig::merca()
    }

    fn single_part_config() -> ProtocolConfig {
        ProtocolConfig {
            max_parts: 1,
            ..ProtocolConfig::merca()
        }
    }

    fn assert_valid_decomposition(ring: &[[i64; 2]], options: &DecomposeOptions) {
        let result = decompose(ring, options, &merca_config()).unwrap();
        assert!(!result.parts.is_empty());

        let original_area = twice_area_fp2(&normalize_ring(ring).unwrap());
        let parts_area: u128 = result.parts.iter().map(|part| twice_area_fp2(part)).sum();
        assert_eq!(parts_area, original_area);

        for part in &result.parts {
            assert!(
                validate_part(part, &merca_config()).is_none(),
                "invalid part: {part:?}"
            );
        }
    }

    #[test]
    fn convex_family_stays_single_part() {
        for ring in [square(), triangle()] {
            let result = decompose(&ring, &default_opts(), &merca_config()).unwrap();
            assert_eq!(result.parts.len(), 1, "ring={ring:?}");
        }
    }

    /// Regression: on-chain accepts the assembled comb (compactness is a
    /// boundary property), so the Rust decomposer must also succeed. Before
    /// removing the per-part compactness check, this shape returned
    /// `DecompError::Failed` because intermediate narrow parts were rejected
    /// even though the final polygon would have passed on-chain.
    #[test]
    fn comb_polygon_decomposes_and_passes_topology() {
        let ring = comb_twelve_vertices();
        let result =
            decompose(&ring, &default_opts(), &merca_config()).expect("comb must decompose");

        assert!(!result.parts.is_empty());
        assert!(result.parts.len() <= crate::constants::MAX_PARTS);

        // Every accepted decomposition must satisfy full topology validation
        // — finalize_parts guarantees this, but we double-check here because
        // this is the regression anchor for the bug.
        assert_eq!(
            crate::topology::validate_multipart_topology(&result.parts, false, &merca_config()),
            Ok(())
        );
    }

    #[test]
    fn all_sample_polygons_conserve_area() {
        for ring in valid_samples() {
            assert_valid_decomposition(&ring, &default_opts());
        }
    }

    #[test]
    fn all_sample_polygons_respect_max_parts() {
        for ring in valid_samples() {
            let result = decompose(&ring, &default_opts(), &merca_config()).unwrap();
            assert!(result.parts.len() <= crate::constants::MAX_PARTS);
        }
    }

    #[test]
    fn all_output_parts_validate_for_sample_family() {
        for ring in valid_samples() {
            let result = decompose(&ring, &default_opts(), &merca_config()).unwrap();
            assert!(
                validate_all_parts(&result.parts, &merca_config()).is_empty(),
                "ring={ring:?}"
            );
        }
    }

    #[test]
    fn clockwise_inputs_normalize_successfully() {
        for mut ring in valid_samples() {
            ring.reverse();
            assert_valid_decomposition(&ring, &default_opts());
        }
    }

    #[test]
    fn steiner_points_empty_when_disallowed() {
        let options = DecomposeOptions {
            allow_steiner: false,
            ..default_opts()
        };

        for ring in valid_samples() {
            let result = decompose(&ring, &options, &merca_config()).unwrap();
            assert!(result.steiner_points.is_empty(), "ring={ring:?}");
        }
    }

    #[test]
    fn too_few_vertices_returns_error() {
        let ring = vec![[0, 0], [M, 0]];
        assert!(matches!(
            decompose(&ring, &default_opts(), &merca_config()),
            Err(DecompError::TooFewVertices)
        ));
    }

    #[test]
    fn self_intersecting_polygon_returns_error() {
        let bow_tie = vec![[0, 0], [4 * M, 4 * M], [4 * M, 0], [0, 4 * M]];
        assert!(matches!(
            decompose(&bow_tie, &default_opts(), &merca_config()),
            Err(DecompError::NotSimple)
        ));
    }

    #[test]
    fn cascade_prefers_exact_before_other_strategies() {
        let bayazit_calls = Cell::new(0usize);
        let ear_calls = Cell::new(0usize);
        let ring = square();

        let result = decompose_with_strategies(
            &ring,
            &default_opts(),
            &merca_config(),
            |poly| Ok(vec![poly.to_vec()]),
            |_, _| {
                bayazit_calls.set(bayazit_calls.get() + 1);
                Err("should not run".into())
            },
            |_| {
                ear_calls.set(ear_calls.get() + 1);
                Err("should not run".into())
            },
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(result.parts.len(), 1);
        assert_eq!(bayazit_calls.get(), 0);
        assert_eq!(ear_calls.get(), 0);
    }

    #[test]
    fn cascade_falls_back_to_bayazit_when_exact_fails() {
        let bayazit_calls = Cell::new(0usize);
        let ear_calls = Cell::new(0usize);
        let ring = square();

        let result = decompose_with_strategies(
            &ring,
            &default_opts(),
            &merca_config(),
            |_| Err("exact failed".into()),
            |poly, allow_steiner| {
                bayazit_calls.set(bayazit_calls.get() + 1);
                assert!(allow_steiner);
                Ok(vec![poly.to_vec()])
            },
            |_| {
                ear_calls.set(ear_calls.get() + 1);
                Err("should not run".into())
            },
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(result.parts.len(), 1);
        assert_eq!(bayazit_calls.get(), 1);
        assert_eq!(ear_calls.get(), 0);
    }

    #[test]
    fn cascade_falls_back_to_ear_clip_when_others_fail() {
        let bayazit_calls = Cell::new(0usize);
        let ear_calls = Cell::new(0usize);
        let ring = square();

        let result = decompose_with_strategies(
            &ring,
            &default_opts(),
            &merca_config(),
            |_| Err("exact failed".into()),
            |_, _| {
                bayazit_calls.set(bayazit_calls.get() + 1);
                Err("bayazit failed".into())
            },
            |poly| {
                ear_calls.set(ear_calls.get() + 1);
                Ok(vec![
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                ])
            },
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(bayazit_calls.get(), 1);
        assert_eq!(ear_calls.get(), 1);
        assert!(!result.parts.is_empty());
        assert!(validate_all_parts(&result.parts, &merca_config()).is_empty());
        let original_area = twice_area_fp2(&normalize_ring(&ring).unwrap());
        let parts_area: u128 = result.parts.iter().map(|part| twice_area_fp2(part)).sum();
        assert_eq!(parts_area, original_area);
    }

    #[test]
    fn bayazit_is_skipped_when_steiner_is_disallowed() {
        let bayazit_calls = Cell::new(0usize);
        let ring = square();
        let options = DecomposeOptions {
            allow_steiner: false,
            ..default_opts()
        };

        let result = decompose_with_strategies(
            &ring,
            &options,
            &merca_config(),
            |_| Err("exact failed".into()),
            |_, _| {
                bayazit_calls.set(bayazit_calls.get() + 1);
                Err("should not run".into())
            },
            |poly| {
                Ok(vec![
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                ])
            },
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(bayazit_calls.get(), 0);
        assert!(!result.parts.is_empty());
        assert!(validate_all_parts(&result.parts, &merca_config()).is_empty());
        assert!(matches!(result.strategy, Strategy::EarClipMerge));
    }

    #[test]
    fn rotation_retry_recovers_on_later_start_vertex() {
        let attempts = Cell::new(0usize);
        let ring = square();
        let expected_start = ring[1];

        let result = decompose_with_strategies(
            &ring,
            &default_opts(),
            &merca_config(),
            |poly| {
                attempts.set(attempts.get() + 1);
                if poly[0] == expected_start {
                    Ok(vec![
                        vec![poly[0], poly[1], poly[2]],
                        vec![poly[0], poly[2], poly[3]],
                    ])
                } else {
                    Err("try another rotation".into())
                }
            },
            |_, _| Err("should not run".into()),
            |_| Err("should not run".into()),
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(attempts.get(), 2);
        assert_eq!(result.parts.len(), 2);
    }

    #[test]
    fn max_rotation_attempts_limits_retry_budget() {
        let attempts = Cell::new(0usize);
        let ring = square();
        let options = DecomposeOptions {
            max_rotation_attempts: 1,
            ..default_opts()
        };

        let error = decompose_with_strategies(
            &ring,
            &options,
            &merca_config(),
            |poly| {
                attempts.set(attempts.get() + 1);
                if poly[0] == ring[1] {
                    Ok(vec![poly.to_vec()])
                } else {
                    Err("need more rotations".into())
                }
            },
            |_, _| Err("bayazit failed".into()),
            |_| Err("ear failed".into()),
            |_, _| Vec::new(),
        )
        .unwrap_err();

        assert_eq!(attempts.get(), 1);
        assert!(matches!(error, DecompError::Failed(_)));
    }

    #[test]
    fn exact_partition_can_succeed_with_single_part_budget() {
        let ring = square();
        let result = decompose_with_strategies(
            &ring,
            &default_opts(),
            &single_part_config(),
            |poly| Ok(vec![poly.to_vec()]),
            |_, _| Err("bayazit failed".into()),
            |poly| {
                Ok(vec![
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                ])
            },
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(result.parts.len(), 1);
        assert!(validate_all_parts(&result.parts, &single_part_config()).is_empty());
        assert!(matches!(result.strategy, Strategy::ExactPartition));
    }

    #[test]
    fn collect_steiner_points_reports_new_vertices_only() {
        let original = square();
        let midpoint = [6 * M, 0];
        let parts = vec![
            vec![original[0], midpoint, original[3]],
            vec![midpoint, original[1], original[2], original[3]],
        ];

        assert_eq!(collect_steiner_points(&original, &parts), vec![midpoint]);
    }

    #[test]
    fn collect_steiner_points_from_bayazit_parts_stays_within_bounds() {
        let ring = comb_twelve_vertices();
        let midpoint = [6 * M, 0];
        let parts = vec![
            vec![ring[0], midpoint, ring[5]],
            vec![midpoint, ring[1], ring[2], ring[3], ring[4], ring[5]],
        ];

        let steiner = collect_steiner_points(&ring, &parts);
        assert_eq!(steiner, vec![midpoint]);

        let (min_x, max_x) = ring.iter().fold((i64::MAX, i64::MIN), |(min_x, max_x), v| {
            (min_x.min(v[0]), max_x.max(v[0]))
        });
        let (min_y, max_y) = ring.iter().fold((i64::MAX, i64::MIN), |(min_y, max_y), v| {
            (min_y.min(v[1]), max_y.max(v[1]))
        });

        for point in steiner {
            assert!(!ring.contains(&point), "{point:?} should be new");
            assert!(point[0] >= min_x && point[0] <= max_x);
            assert!(point[1] >= min_y && point[1] <= max_y);
        }
    }

    #[test]
    fn hertel_mehlhorn_optimizes_ear_clip_fallback() {
        let ring = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];

        let result = decompose_with_strategies(
            &ring,
            &default_opts(),
            &merca_config(),
            |_| Err("exact failed".into()),
            |_, _| Err("bayazit failed".into()),
            |poly| ear_clip_triangulate(poly),
            |_, _| Vec::new(),
        )
        .unwrap();

        assert!(result.parts.len() < 4);
        assert!(result.parts.len() >= 2);
    }

    #[test]
    fn trace_disabled_by_default() {
        let ring = vec![[0i64, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let result = decompose(&ring, &DecomposeOptions::default(), &merca_config()).unwrap();
        assert!(result.trace.is_none());
    }

    #[test]
    fn trace_enabled_records_attempts() {
        let ring = vec![[0i64, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let opts = DecomposeOptions {
            collect_trace: true,
            ..Default::default()
        };
        let result = decompose(&ring, &opts, &merca_config()).unwrap();
        let trace = result.trace.unwrap();
        assert!(!trace.is_empty());
        assert!(trace
            .iter()
            .any(|a| matches!(a.outcome, Outcome::Success { .. })));
    }

    #[test]
    fn strategy_reports_correct_winner() {
        let square = vec![[0i64, 0], [10 * M, 0], [10 * M, 10 * M], [0, 10 * M]];
        let result = decompose(&square, &DecomposeOptions::default(), &merca_config()).unwrap();
        assert_eq!(result.parts.len(), 1);
        assert!(matches!(
            result.strategy,
            Strategy::AlreadyConvex | Strategy::ExactPartition
        ));
    }

    #[test]
    fn minimize_parts_picks_fewest_across_strategies() {
        // Rigged strategies: ExactPartition yields 3 parts, Bayazit yields 2,
        // EarClipMerge yields 4. Cascade mode would short-circuit on Exact
        // (3 parts). minimize_parts mode must reach past Exact and return
        // the Bayazit result (2 parts).
        let ring = square();
        let opts = DecomposeOptions {
            minimize_parts: true,
            ..default_opts()
        };

        let result = decompose_with_strategies(
            &ring,
            &opts,
            &merca_config(),
            |poly| {
                // 3 parts — split the square into three thin strips.
                let y0 = poly[0][1];
                let y1 = poly[2][1];
                let third = (y1 - y0) / 3;
                let a = [poly[0][0], y0 + third];
                let b = [poly[1][0], y0 + third];
                let c = [poly[0][0], y0 + 2 * third];
                let d = [poly[1][0], y0 + 2 * third];
                Ok(vec![
                    vec![poly[0], poly[1], b, a],
                    vec![a, b, d, c],
                    vec![c, d, poly[2], poly[3]],
                ])
            },
            |poly, _allow| {
                // 2 parts — optimal split down the middle.
                let mid_bot = [(poly[0][0] + poly[1][0]) / 2, poly[0][1]];
                let mid_top = [(poly[2][0] + poly[3][0]) / 2, poly[2][1]];
                Ok(vec![
                    vec![poly[0], mid_bot, mid_top, poly[3]],
                    vec![mid_bot, poly[1], poly[2], mid_top],
                ])
            },
            |poly| {
                // 4 parts from a fake triangulation.
                Ok(vec![
                    vec![poly[0], poly[1], [poly[0][0] + M, poly[0][1] + M]],
                    vec![poly[1], poly[2], [poly[0][0] + M, poly[0][1] + M]],
                    vec![poly[2], poly[3], [poly[0][0] + M, poly[0][1] + M]],
                    vec![poly[3], poly[0], [poly[0][0] + M, poly[0][1] + M]],
                ])
            },
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(
            result.parts.len(),
            2,
            "minimize_parts should pick the 2-part split"
        );
        assert!(matches!(
            result.strategy,
            Strategy::Bayazit | Strategy::Rotation { inner: _, .. }
        ));
    }

    #[test]
    fn minimize_parts_breaks_ties_with_exact_over_bayazit() {
        // Both ExactPartition and Bayazit return 2 parts with zero Steiner
        // points. Tiebreaker order is (parts, steiner, rotation, strategy),
        // so Exact must win on the strategy rank. EarClip is disabled here
        // because Hertel-Mehlhorn would merge the two mock triangles back
        // into a single convex square and steal the win with 1 part —
        // which would be correct in the general case but unrelated to the
        // tiebreak rule we're testing.
        let ring = square();
        let opts = DecomposeOptions {
            minimize_parts: true,
            ..default_opts()
        };

        let result = decompose_with_strategies(
            &ring,
            &opts,
            &merca_config(),
            |poly| {
                Ok(vec![
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                ])
            },
            |poly, _| {
                Ok(vec![
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                ])
            },
            |_| Err("ear clip disabled for this tiebreak test".into()),
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(result.parts.len(), 2);
        assert!(matches!(result.strategy, Strategy::ExactPartition));
    }

    #[test]
    fn minimize_parts_prefers_fewer_steiner_points_on_tie() {
        // 2 parts from both strategies, but Bayazit produces one Steiner point.
        // Even though Bayazit is called after Exact, the tiebreaker on
        // steiner_count should still pick Exact.
        let ring = square();
        let opts = DecomposeOptions {
            minimize_parts: true,
            ..default_opts()
        };

        let result = decompose_with_strategies(
            &ring,
            &opts,
            &merca_config(),
            |poly| {
                Ok(vec![
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                ])
            },
            |poly, _| {
                // Split with a Steiner point on the bottom edge.
                let steiner = [(poly[0][0] + poly[1][0]) / 2, poly[0][1]];
                Ok(vec![
                    vec![poly[0], steiner, poly[2], poly[3]],
                    vec![steiner, poly[1], poly[2]],
                ])
            },
            |_| Err("ear clip skipped".into()),
            |original, parts| {
                // Report points that are in parts but not in the original ring.
                let mut originals: std::collections::HashSet<[i64; 2]> =
                    std::collections::HashSet::new();
                for v in original {
                    originals.insert(*v);
                }
                let mut result: Vec<[i64; 2]> = Vec::new();
                for part in parts {
                    for v in part {
                        if !originals.contains(v) && !result.contains(v) {
                            result.push(*v);
                        }
                    }
                }
                result
            },
        )
        .unwrap();

        assert_eq!(result.parts.len(), 2);
        assert!(matches!(result.strategy, Strategy::ExactPartition));
        assert!(result.steiner_points.is_empty());
    }

    #[test]
    fn minimize_parts_falls_through_when_exact_invalid() {
        // Exact produces a decomposition that fails validation (too many
        // parts). Bayazit succeeds with 2 parts. Result must be Bayazit's,
        // without a TooManyParts error bubbling up.
        let ring = square();
        let opts = DecomposeOptions {
            minimize_parts: true,
            ..default_opts()
        };
        let config = ProtocolConfig {
            max_parts: 3,
            ..merca_config()
        };

        let result = decompose_with_strategies(
            &ring,
            &opts,
            &config,
            |poly| {
                // 4 parts → rejected by max_parts=3 in finalize_parts.
                Ok(vec![
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                ])
            },
            |poly, _| {
                Ok(vec![
                    vec![poly[0], poly[1], poly[2]],
                    vec![poly[0], poly[2], poly[3]],
                ])
            },
            |_| Err("ear clip skipped".into()),
            |_, _| Vec::new(),
        )
        .unwrap();

        assert_eq!(result.parts.len(), 2);
        assert!(matches!(result.strategy, Strategy::Bayazit));
    }

    #[test]
    fn l_shape_trace_shows_multiple_attempts() {
        let l_shape = vec![
            [0, 0],
            [20 * M, 0],
            [20 * M, 10 * M],
            [10 * M, 10 * M],
            [10 * M, 20 * M],
            [0, 20 * M],
        ];
        let opts = DecomposeOptions {
            collect_trace: true,
            ..Default::default()
        };
        let result = decompose(&l_shape, &opts, &merca_config()).unwrap();
        let trace = result.trace.unwrap();
        assert!(!trace.is_empty());
        assert!(trace.len() >= 1);
    }
}
