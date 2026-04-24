/* tslint:disable */
/* eslint-disable */

export function area_display_from_twice_area(twice_area: string, config?: any | null): bigint;

export function areas_conserved_values(original: string, part_areas: any): boolean;

export function bayazit_decompose_polygon(ring_flat: BigInt64Array, allow_steiner: boolean): any;

/**
 * Classify contact between two polygon parts.
 *
 * Returns: `"shared_edge"`, `"partial_contact"`, or `"none"`.
 *
 * - `"shared_edge"`: at least one edge appears in both parts (valid adjacency).
 * - `"partial_contact"`: collinear overlap without exact match (T-junction —
 *   on-chain aborts `EInvalidMultipartContact`).
 * - `"none"`: no collinear contact at all.
 */
export function classify_contact(a_flat: BigInt64Array, b_flat: BigInt64Array): string;

export function collect_steiner_points(ring_flat: BigInt64Array, parts_flat: any): any;

export function collinear_segments_overlap_area_rings(a1x: bigint, a1y: bigint, a2x: bigint, a2y: bigint, b1x: bigint, b1y: bigint, b2x: bigint, b2y: bigint, a_ring_flat: BigInt64Array, b_ring_flat: BigInt64Array): boolean;

export function contains_polygon(outer_parts_flat: any, inner_parts_flat: any): boolean;

export function convex_parts_overlap(a_flat: BigInt64Array, b_flat: BigInt64Array): boolean;

export function cross2d(ax: bigint, ay: bigint, bx: bigint, by: bigint, cx: bigint, cy: bigint): string;

export function decompose_polygon(ring_flat: BigInt64Array, allow_steiner: boolean, collect_trace?: boolean | null, minimize_parts?: boolean | null, config?: any | null): any;

export function ear_clip_triangulate_polygon(ring_flat: BigInt64Array): any;

export function edge_squared_length(ax: bigint, ay: bigint, bx: bigint, by: bigint): string;

export function ensure_ccw_ring(ring_flat: BigInt64Array): any;

export function exact_partition_only_original_vertices(ring_flat: BigInt64Array, parts_flat: any): boolean;

export function exact_vertex_partition_polygon(ring_flat: BigInt64Array): any;

export function find_overlapping_parts(a_parts_flat: any, b_parts_flat: any): any;

export function has_exact_shared_edge(a_flat: BigInt64Array, b_flat: BigInt64Array): boolean;

export function is_ccw_ring(ring_flat: BigInt64Array): boolean;

export function is_collinear_pts(ax: bigint, ay: bigint, bx: bigint, by: bigint, px: bigint, py: bigint): boolean;

export function is_convex_ring(ring_flat: BigInt64Array): boolean;

export function is_left(ax: bigint, ay: bigint, bx: bigint, by: bigint, px: bigint, py: bigint): boolean;

export function is_left_or_on(ax: bigint, ay: bigint, bx: bigint, by: bigint, px: bigint, py: bigint): boolean;

export function is_reflex(prev_x: bigint, prev_y: bigint, curr_x: bigint, curr_y: bigint, next_x: bigint, next_y: bigint): boolean;

export function is_right(ax: bigint, ay: bigint, bx: bigint, by: bigint, px: bigint, py: bigint): boolean;

export function is_right_or_on(ax: bigint, ay: bigint, bx: bigint, by: bigint, px: bigint, py: bigint): boolean;

export function is_simple_ring(ring_flat: BigInt64Array): boolean;

export function merge_convex_pair(a_flat: BigInt64Array, b_flat: BigInt64Array): any;

export function normalize_polygon_ring(ring_flat: BigInt64Array): any;

export function optimize_partition(parts_flat: any): any;

export function orientation(ax: bigint, ay: bigint, bx: bigint, by: bigint, cx: bigint, cy: bigint): string;

export function parts_overlap(a_parts_flat: any, b_parts_flat: any): boolean;

export function perimeter_l1_ring(ring_flat: BigInt64Array): string;

export function point_inside_any_part(parts_flat: any, x: bigint, y: bigint): boolean;

export function point_inside_or_on_boundary_ring(px: bigint, py: bigint, ring_flat: BigInt64Array): boolean;

export function point_on_polygon_boundary_ring(px: bigint, py: bigint, ring_flat: BigInt64Array): boolean;

export function point_on_segment(px: bigint, py: bigint, ax: bigint, ay: bigint, bx: bigint, by: bigint): boolean;

export function point_strictly_inside_convex_ring(px: bigint, py: bigint, ring_flat: BigInt64Array): boolean;

export function remove_collinear_ring(ring_flat: BigInt64Array): any;

export function rotate_polygon_ring(ring_flat: BigInt64Array, start: number): any;

export function sat_overlap(a_flat: BigInt64Array, b_flat: BigInt64Array): boolean;

export function sat_overlap_with_aabb(a_flat: BigInt64Array, b_flat: BigInt64Array): boolean;

export function segments_contact(ax1: bigint, ay1: bigint, ax2: bigint, ay2: bigint, bx1: bigint, by1: bigint, bx2: bigint, by2: bigint): boolean;

export function segments_intersect(a1x: bigint, a1y: bigint, a2x: bigint, a2y: bigint, b1x: bigint, b1y: bigint, b2x: bigint, b2y: bigint): boolean;

export function segments_properly_intersect(a1x: bigint, a1y: bigint, a2x: bigint, a2y: bigint, b1x: bigint, b1y: bigint, b2x: bigint, b2y: bigint): boolean;

export function signed_area_2x_ring(ring_flat: BigInt64Array): string;

export function twice_area_ring(ring_flat: BigInt64Array): string;

/**
 * Boundary-level compactness check. Apply to a whole polygon's outer
 * boundary (single part, or the union boundary of a multipart polygon).
 * NOT intended for individual parts of a multipart polygon — that would be
 * stricter than on-chain and reject legitimate decompositions.
 */
export function validate_compactness_values(twice_area: string, perimeter: string, config?: any | null): string | undefined;

export function validate_decomposition(ring_flat: BigInt64Array, parts_flat: any, config?: any | null): any;

export function validate_edge_lengths_ring(ring_flat: BigInt64Array, config?: any | null): string | undefined;

export function validate_multipart_topology(parts_flat: any, allow_vertex_contact?: boolean | null, config?: any | null): any;

/**
 * Per-part structural validation mirroring polygon.move's `part()` entry:
 * vertex-count bounds, weak convexity, and minimum edge length.
 *
 * Breaking change (was: also enforced compactness). Compactness is a
 * boundary-level property; call `validate_multipart_topology` (or the
 * full on-chain validator) to check the assembled polygon instead.
 */
export function validate_part_ring(ring_flat: BigInt64Array, config?: any | null): string | undefined;
