//! Core types for exact-poly integer polygon geometry.
//! All coordinates are i64 (fixed-point, SCALE=1_000_000 units = 1 meter).
//! No float arithmetic anywhere in this library.

use crate::aabb::Aabb;

/// Protocol-specific configuration for polygon validation.
/// Passed to validation and decomposition functions.
/// Use `ProtocolConfig::merca()` for on-chain defaults.
/// Use `ProtocolConfig::permissive()` for demos/testing with no validation limits.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProtocolConfig {
    /// Maximum number of convex parts per polygon.
    pub max_parts: usize,
    /// Maximum vertices per convex part.
    pub max_vertices_per_part: usize,
    /// Minimum edge length squared (avoids sqrt). Compare: dx²+dy² >= this value.
    pub min_edge_length_squared: u128,
    /// Minimum compactness ratio in parts-per-million.
    /// Formula: 8_000_000 * twice_area >= min_compactness_ppm * L1_perimeter²
    pub min_compactness_ppm: u128,
    /// Divisor to convert twice_area_fp2 to display units.
    pub area_divisor: u128,
}

impl ProtocolConfig {
    /// Merca on-chain protocol defaults (matching polygon.move).
    pub fn merca() -> Self {
        Self {
            max_parts: crate::constants::MAX_PARTS,
            max_vertices_per_part: crate::constants::MAX_VERTICES_PER_PART,
            min_edge_length_squared: crate::constants::MIN_EDGE_LENGTH_SQUARED,
            min_compactness_ppm: crate::constants::MIN_COMPACTNESS_PPM,
            area_divisor: crate::constants::AREA_DIVISOR,
        }
    }

    /// No validation limits — for demos, testing, visualization.
    pub fn permissive() -> Self {
        Self {
            max_parts: usize::MAX,
            max_vertices_per_part: usize::MAX,
            min_edge_length_squared: 0,
            min_compactness_ppm: 0,
            area_divisor: 1,
        }
    }
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self::merca()
    }
}

/// A 2D point in fixed-point integer coordinates.
/// x and y are in units of 1/SCALE meters (1 unit = 1 micrometer at SCALE=1_000_000).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl From<[i64; 2]> for Point {
    fn from([x, y]: [i64; 2]) -> Self {
        Self { x, y }
    }
}

impl From<Point> for [i64; 2] {
    fn from(p: Point) -> Self {
        [p.x, p.y]
    }
}

/// A convex part of a polygon — array of vertices + bounding box.
/// All vertices are in fixed-point integer coordinates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Part {
    pub xs: Vec<i64>,
    pub ys: Vec<i64>,
    pub aabb: Aabb,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Strategy {
    AlreadyConvex,
    ExactPartition,
    Bayazit,
    EarClipMerge,
    Rotation { offset: usize, inner: Box<Strategy> },
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Attempt {
    pub strategy: Strategy,
    pub rotation: usize,
    pub outcome: Outcome,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Outcome {
    Success { part_count: usize },
    TooManyParts { count: usize },
    ValidationFailed { errors: Vec<String> },
    AlgorithmFailed { error: String },
}

/// Options for the decompose() cascade.
#[derive(Clone, Debug)]
pub struct DecomposeOptions {
    /// If true, Bayazit may introduce Steiner points at edge intersections.
    /// If false, only original vertices are used (exact_vertex_partition preferred).
    pub allow_steiner: bool,
    /// Number of ring rotation attempts if initial decomposition fails. Default: vertex_count.
    pub max_rotation_attempts: usize,
    pub collect_trace: bool,
    /// If `false` (default) — cascade mode: try ExactPartition → Bayazit →
    /// EarClipMerge in order and return the **first** strategy that passes
    /// validation. This minimizes Steiner-point pollution and keeps the
    /// on-chain client behaviour deterministic.
    ///
    /// If `true` — run **every** strategy across **every** rotation that is
    /// attempted, collect all validated candidates, and return the one with
    /// the fewest parts. Ties are broken by: fewer Steiner points, then
    /// earlier rotation, then strategy preference (Exact > Bayazit > EarClip).
    /// Strictly more work than cascade mode; intended for demos, tooling,
    /// and research where "absolute minimum parts" matters more than
    /// cascade determinism.
    pub minimize_parts: bool,
}

impl Default for DecomposeOptions {
    fn default() -> Self {
        Self {
            allow_steiner: true,
            max_rotation_attempts: usize::MAX,
            collect_trace: false,
            minimize_parts: false,
        }
    }
}

/// Result of a successful decomposition.
#[derive(Clone, Debug)]
pub struct DecomposeResult {
    /// The convex parts. Each is a ring of [x, y] points.
    pub parts: Vec<Vec<[i64; 2]>>,
    /// Vertices introduced by decomposition (not in original ring).
    /// Empty when allow_steiner=false or exact partition succeeded.
    pub steiner_points: Vec<[i64; 2]>,
    pub strategy: Strategy,
    pub trace: Option<Vec<Attempt>>,
}

/// Decomposition error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecompError {
    /// Input ring has fewer than 3 vertices.
    TooFewVertices,
    /// Input ring is not simple (self-intersecting).
    NotSimple,
    /// Could not decompose within MAX_PARTS limit.
    TooManyParts,
    /// All strategies failed.
    Failed(String),
}

impl std::fmt::Display for DecompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecompError::TooFewVertices => write!(f, "ring has fewer than 3 vertices"),
            DecompError::NotSimple => write!(f, "ring is self-intersecting"),
            DecompError::TooManyParts => write!(f, "decomposition exceeds MAX_PARTS"),
            DecompError::Failed(s) => write!(f, "decomposition failed: {s}"),
        }
    }
}

/// Topology validation error with structured diagnostic data.
/// Serializable to JSON via serde for WASM boundary.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TopologyError {
    /// Parts are not all connected via shared edges.
    /// `disconnected_parts`: indices of parts not reachable from part 0 via BFS.
    NotConnected { disconnected_parts: Vec<usize> },

    /// Boundary graph has multiple connected components (polygon has holes).
    /// `boundary_components`: number of separate boundary loops found.
    HasHoles { boundary_components: usize },

    /// Two parts touch only at a vertex, not along a shared edge.
    VertexOnlyContact { part_a: usize, part_b: usize },

    /// Two parts have unsupported contact (T-junction, partial overlap, etc.)
    UnsupportedContact {
        part_a: usize,
        part_b: usize,
        reason: String,
    },

    /// Too many parts for the protocol.
    TooManyParts { count: usize, max: usize },

    /// Polygon boundary is not compact enough (isoperimetric ratio too low).
    NotCompact {
        compactness_ppm: u128,
        min_ppm: u128,
    },
}

impl std::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopologyError::NotConnected { disconnected_parts } => {
                write!(
                    f,
                    "parts are not all connected (disconnected: {disconnected_parts:?})"
                )
            }
            TopologyError::HasHoles {
                boundary_components,
            } => {
                write!(
                    f,
                    "multipart polygon has {boundary_components} boundary components (holes)"
                )
            }
            TopologyError::VertexOnlyContact { part_a, part_b } => {
                write!(f, "parts {part_a} and {part_b} have only vertex contact")
            }
            TopologyError::UnsupportedContact {
                part_a,
                part_b,
                reason,
            } => {
                write!(
                    f,
                    "parts {part_a} and {part_b} have unsupported contact: {reason}"
                )
            }
            TopologyError::TooManyParts { count, max } => {
                write!(f, "too many parts ({count} > max {max})")
            }
            TopologyError::NotCompact {
                compactness_ppm,
                min_ppm,
            } => {
                write!(
                    f,
                    "polygon is not compact enough ({compactness_ppm} ppm < min {min_ppm} ppm)"
                )
            }
        }
    }
}
