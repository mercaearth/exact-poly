//! On-chain protocol constants matching polygon.move:31-42.
//! These are HARDCODED — no Config struct, no runtime parameters.

/// Scaling factor: 1 meter in fixed-point space (= 1_000_000 units).
pub const SCALE: u64 = 1_000_000;

/// Maximum world coordinate (Earth circumference in Web Mercator, scaled).
pub const MAX_WORLD: u64 = 40_075_017_000_000;

/// Maximum number of convex parts per polygon.
pub const MAX_PARTS: usize = 10;

/// Maximum vertices per convex part (admin-configurable on-chain, here hardcoded to protocol default).
pub const MAX_VERTICES_PER_PART: usize = 64;

/// Minimum edge length in fixed-point units (1 meter).
pub const MIN_EDGE_LENGTH: u64 = 1_000_000;

/// Minimum edge length squared (used to avoid sqrt). 1_000_000^2.
pub const MIN_EDGE_LENGTH_SQUARED: u128 = 1_000_000_000_000;

/// Minimum compactness ratio in parts-per-million for isoperimetric validation.
/// Formula: 8_000_000 * twice_area >= MIN_COMPACTNESS_PPM * L1_perimeter^2
pub const MIN_COMPACTNESS_PPM: u128 = 150_000;

/// Divisor to convert twice_area_fp2 to whole square meters: 2 * SCALE^2.
pub const AREA_DIVISOR: u128 = 2_000_000_000_000;
