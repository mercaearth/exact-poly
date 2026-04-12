/* @ts-self-types="./exact_poly.d.ts" */
import * as wasm from "./exact_poly_bg.wasm";
import { __wbg_set_wasm } from "./exact_poly_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    add_i64, area_display_from_twice_area, areas_conserved_values, bayazit_decompose_polygon, classify_contact, collect_steiner_points, collinear_segments_overlap_area_rings, contains_polygon, convex_parts_overlap, cross2d, cross_sign, decompose_polygon, ear_clip_triangulate_polygon, edge_squared_length, ensure_ccw_ring, exact_partition_only_original_vertices, exact_vertex_partition_polygon, find_overlapping_parts, has_exact_shared_edge, is_ccw_ring, is_collinear, is_collinear_pts, is_convex_ring, is_left, is_left_or_on, is_left_turn, is_reflex, is_right, is_right_or_on, is_right_turn, is_simple_ring, merge_convex_pair, normalize_polygon_ring, optimize_partition, orientation, parts_overlap, perimeter_l1_ring, point_inside_any_part, point_inside_or_on_boundary_ring, point_on_polygon_boundary_ring, point_on_segment, point_strictly_inside_convex_ring, remove_collinear_ring, rotate_polygon_ring, sat_overlap, sat_overlap_with_aabb, segments_contact, segments_intersect, segments_properly_intersect, sign_i128, signed_area_2x_ring, sub_u64, twice_area, twice_area_ring, validate_compactness_values, validate_decomposition, validate_edge_lengths_ring, validate_multipart_topology, validate_part_ring
} from "./exact_poly_bg.js";
