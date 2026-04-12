/**
 * WASM wrapper — re-exports all exact-poly functions with typed signatures.
 * The WASM module is auto-initialized by vite-plugin-wasm + top-level-await.
 */
export {
  // Decomposition
  decompose_polygon,
  bayazit_decompose_polygon,
  exact_vertex_partition_polygon,
  ear_clip_triangulate_polygon,
  collect_steiner_points,
  exact_partition_only_original_vertices,

  // Hertel-Mehlhorn
  optimize_partition,
  merge_convex_pair,

  // Area
  twice_area,
  twice_area_ring,
  area_display_from_twice_area,
  areas_conserved_values,
  signed_area_2x_ring,

  // Ring operations
  is_ccw_ring,
  ensure_ccw_ring,
  remove_collinear_ring,
  is_simple_ring,
  is_convex_ring,
  normalize_polygon_ring,
  rotate_polygon_ring,

  // Validation
  validate_edge_lengths_ring,
  validate_compactness_values,
  validate_part_ring,
  perimeter_l1_ring,

  // Spatial
  point_strictly_inside_convex_ring,
  point_on_polygon_boundary_ring,
  point_inside_or_on_boundary_ring,
  point_inside_any_part,
  contains_polygon,
  collinear_segments_overlap_area_rings,

  // Overlap / SAT
  sat_overlap,
  sat_overlap_with_aabb,
  convex_parts_overlap,
  find_overlapping_parts,
  parts_overlap,

  // Shared edge / Topology
  has_exact_shared_edge,
  classify_contact,
  segments_contact,
  validate_multipart_topology,

  // Primitives
  cross2d,
  orientation,
  is_left,
  is_left_or_on,
  is_right,
  is_right_or_on,
  is_collinear_pts,
  is_reflex,
  edge_squared_length,
  point_on_segment,
  segments_properly_intersect,
  segments_intersect,

  // Signed
  cross_sign,
  sub_u64,
  sign_i128,
  is_left_turn,
  is_right_turn,
  is_collinear,

  // Full on-chain validation
  validate_decomposition,
} from "exact-poly";

/** Helper: convert [x0,y0,x1,y1,...] number array to BigInt64Array for WASM */
const SCALE = 1_000_000;

export function toFlat(points: [number, number][]): BigInt64Array {
  const arr = new BigInt64Array(points.length * 2);
  for (let i = 0; i < points.length; i++) {
    arr[i * 2] = BigInt(Math.round(points[i][0] * SCALE));
    arr[i * 2 + 1] = BigInt(Math.round(points[i][1] * SCALE));
  }
  return arr;
}

/** Helper: parse flat WASM result back to [x,y][] */
export function fromFlat(flat: bigint[] | number[]): [number, number][] {
  const result: [number, number][] = [];
  for (let i = 0; i < flat.length; i += 2) {
    result.push([Number(flat[i]) / SCALE, Number(flat[i + 1]) / SCALE]);
  }
  return result;
}

export interface DecomposeResult {
  parts: bigint[][];
  steiner_points: bigint[];
  strategy?: string | { Rotation: { offset: number; inner: string | object } };
  trace?: DecomposeTraceEntry[];
}

export interface DecomposeTraceEntry {
  strategy: string | { Rotation: { offset: number; inner: string | object } };
  rotation: number;
  outcome:
    | { Success: { part_count: number } }
    | { TooManyParts: { count: number } }
    | { ValidationFailed: { errors: string[] } }
    | { AlgorithmFailed: { error: string } };
}

export interface ValidationCheck {
  name: string;
  passed: boolean;
  detail: string;
  severity: string; // "ok" | "error" | "warn"
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

export type TopologyError =
  | { NotConnected: { disconnected_parts: number[] } }
  | { HasHoles: { boundary_components: number } }
  | { TooManyParts: { count: number; max: number } }
  | { NotCompact: { compactness_ppm: number; min_ppm: number } }
  | { VertexOnlyContact: { part_a: number; part_b: number } }
  | { UnsupportedContact: { part_a: number; part_b: number; reason: string } };
