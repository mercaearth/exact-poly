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
  area_display_from_twice_area,
  areas_conserved_values,
  signed_area_2x,

  // Ring operations
  is_ccw,
  ensure_ccw,
  remove_collinear,
  is_simple,
  is_convex,
  normalize_polygon,
  rotate_polygon,

  // Validation
  validate_edge_lengths,
  validate_compactness,
  validate_part,
  perimeter_l1,

  // Spatial
  point_strictly_inside_convex,
  point_on_polygon_boundary,
  point_inside_or_on_boundary,
  point_inside_any_part,
  contains_polygon,
  collinear_segments_overlap_area,

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

  // Full on-chain validation
  validate_decomposition,
} from "exact-poly";

export type {
  DecomposeResult,
  DecomposeAttempt,
  DecomposeStrategy,
  DecomposeOutcome,
  ValidationCheck,
  ValidationReport,
  TopologyError,
  IndexPair,
  PolygonRing,
  PolygonParts,
} from "exact-poly";

/** Backward-compat alias — prefer DecomposeAttempt */
export type { DecomposeAttempt as DecomposeTraceEntry } from "exact-poly";

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
