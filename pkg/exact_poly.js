/* @ts-self-types="./exact_poly.d.ts" */
import * as wasm from "./exact_poly_bg.wasm";
import { __wbg_set_wasm } from "./exact_poly_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    area_display_from_twice_area, areas_conserved_values, bayazit_decompose_polygon, classify_contact, collect_steiner_points, collinear_segments_overlap_area, contains_polygon, convex_parts_overlap, cross2d, decompose_polygon, ear_clip_triangulate_polygon, edge_squared_length, ensure_ccw, exact_partition_only_original_vertices, exact_vertex_partition_polygon, find_overlapping_parts, has_exact_shared_edge, is_ccw, is_collinear_pts, is_convex, is_left, is_left_or_on, is_reflex, is_right, is_right_or_on, is_simple, merge_convex_pair, normalize_polygon, optimize_partition, orientation, parts_overlap, perimeter_l1, point_inside_any_part, point_inside_or_on_boundary, point_on_polygon_boundary, point_on_segment, point_strictly_inside_convex, remove_collinear, rotate_polygon, sat_overlap, sat_overlap_with_aabb, segments_contact, segments_intersect, segments_properly_intersect, signed_area_2x, twice_area, validate_compactness, validate_decomposition, validate_edge_lengths, validate_multipart_topology, validate_part
} from "./exact_poly_bg.js";
