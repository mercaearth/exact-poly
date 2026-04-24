use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &str = r#"
export interface ProtocolConfig {
  max_parts: number;
  max_vertices_per_part: number;
  min_edge_length_squared: bigint;
  min_compactness_ppm: bigint;
  area_divisor: bigint;
}

export type PolygonRing = bigint[];
export type PolygonParts = PolygonRing[];

export type DecomposeStrategy =
  | "AlreadyConvex"
  | "ExactPartition"
  | "Bayazit"
  | "EarClipMerge"
  | { Rotation: { offset: number; inner: DecomposeStrategy } };

export type DecomposeOutcome =
  | { Success: { part_count: number } }
  | { "TooManyParts": { count: number } }
  | { ValidationFailed: { errors: string[] } }
  | { AlgorithmFailed: { error: string } };

export interface DecomposeAttempt {
  strategy: DecomposeStrategy;
  rotation: number;
  outcome: DecomposeOutcome;
}

export interface DecomposeResult {
  parts: PolygonParts;
  steiner_points: PolygonRing;
  strategy: DecomposeStrategy;
  trace?: DecomposeAttempt[];
}

export interface ValidationCheck {
  name: string;
  passed: boolean;
  detail: string;
  severity: "ok" | "warn" | "error";
}

export interface ValidationReport {
  checks: ValidationCheck[];
  valid: boolean;
  error_count: number;
  warn_count: number;
  original_twice_area: string;
  parts_twice_area_sum: string;
  part_areas: string[];
}

export interface IndexPair {
  a_index: number;
  b_index: number;
}

export type TopologyError =
  | { NotConnected: { disconnected_parts: number[] } }
  | { HasHoles: { boundary_components: number } }
  | { VertexOnlyContact: { part_a: number; part_b: number } }
  | { UnsupportedContact: { part_a: number; part_b: number; reason: string } }
  | { "TooManyParts": { count: number; max: number } }
  | { NotCompact: { compactness_ppm: bigint; min_ppm: bigint } };

export function area_display_from_twice_area(twice_area: string, config?: ProtocolConfig | null): bigint;
export function areas_conserved_values(original: string, part_areas: string[]): boolean;
export function bayazit_decompose_polygon(ring_flat: BigInt64Array, allow_steiner: boolean): PolygonParts;
export function collect_steiner_points(ring_flat: BigInt64Array, parts_flat: PolygonParts): PolygonRing;
export function contains_polygon(outer_parts_flat: PolygonParts, inner_parts_flat: PolygonParts): boolean;
export function decompose_polygon(ring_flat: BigInt64Array, allow_steiner: boolean, collect_trace?: boolean | null, minimize_parts?: boolean | null, config?: ProtocolConfig | null): DecomposeResult;
export function ear_clip_triangulate_polygon(ring_flat: BigInt64Array): PolygonParts;
export function ensure_ccw(ring_flat: BigInt64Array): PolygonRing;
export function exact_partition_only_original_vertices(ring_flat: BigInt64Array, parts_flat: PolygonParts): boolean;
export function exact_vertex_partition_polygon(ring_flat: BigInt64Array): PolygonParts;
export function find_overlapping_parts(a_parts_flat: PolygonParts, b_parts_flat: PolygonParts): IndexPair[];
export function merge_convex_pair(a_flat: BigInt64Array, b_flat: BigInt64Array): PolygonRing | undefined;
export function normalize_polygon(ring_flat: BigInt64Array): PolygonRing | undefined;
export function optimize_partition(parts_flat: PolygonParts): PolygonParts;
export function parts_overlap(a_parts_flat: PolygonParts, b_parts_flat: PolygonParts): boolean;
export function point_inside_any_part(parts_flat: PolygonParts, x: bigint, y: bigint): boolean;
export function remove_collinear(ring_flat: BigInt64Array): PolygonRing;
export function rotate_polygon(ring_flat: BigInt64Array, start: number): PolygonRing;
export function validate_compactness(twice_area: string, perimeter: string, config?: ProtocolConfig | null): string | undefined;
export function validate_decomposition(ring_flat: BigInt64Array, parts_flat: PolygonParts, config?: ProtocolConfig | null): ValidationReport;
export function validate_edge_lengths(ring_flat: BigInt64Array, config?: ProtocolConfig | null): string | undefined;
export function validate_multipart_topology(parts_flat: PolygonParts, allow_vertex_contact?: boolean | null, config?: ProtocolConfig | null): TopologyError | undefined;
export function validate_part(ring_flat: BigInt64Array, config?: ProtocolConfig | null): string | undefined;
"#;

#[derive(Serialize, Deserialize)]
pub struct WasmDecomposeResult {
    pub parts: Vec<Vec<i64>>,
    pub steiner_points: Vec<i64>,
    pub strategy: crate::types::Strategy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Vec<crate::types::Attempt>>,
}

#[derive(Serialize, Deserialize)]
pub struct WasmIndexPair {
    pub a_index: usize,
    pub b_index: usize,
}
