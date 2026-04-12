/**
 * @param {bigint} a
 * @param {bigint} b
 * @returns {bigint}
 */
export function add_i64(a, b) {
    const ret = wasm.add_i64(a, b);
    return ret;
}

/**
 * @param {string} twice_area
 * @param {any | null} [config]
 * @returns {bigint}
 */
export function area_display_from_twice_area(twice_area, config) {
    const ptr0 = passStringToWasm0(twice_area, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.area_display_from_twice_area(ptr0, len0, isLikeNone(config) ? 0 : addToExternrefTable0(config));
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return BigInt.asUintN(64, ret[0]);
}

/**
 * @param {string} original
 * @param {any} part_areas
 * @returns {boolean}
 */
export function areas_conserved_values(original, part_areas) {
    const ptr0 = passStringToWasm0(original, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.areas_conserved_values(ptr0, len0, part_areas);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} ring_flat
 * @param {boolean} allow_steiner
 * @returns {any}
 */
export function bayazit_decompose_polygon(ring_flat, allow_steiner) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.bayazit_decompose_polygon(ptr0, len0, allow_steiner);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * Classify contact between two polygon parts.
 *
 * Returns: `"shared_edge"`, `"partial_contact"`, or `"none"`.
 *
 * - `"shared_edge"`: at least one edge appears in both parts (valid adjacency).
 * - `"partial_contact"`: collinear overlap without exact match (T-junction —
 *   on-chain aborts `EInvalidMultipartContact`).
 * - `"none"`: no collinear contact at all.
 * @param {BigInt64Array} a_flat
 * @param {BigInt64Array} b_flat
 * @returns {string}
 */
export function classify_contact(a_flat, b_flat) {
    let deferred4_0;
    let deferred4_1;
    try {
        const ptr0 = passArray64ToWasm0(a_flat, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray64ToWasm0(b_flat, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.classify_contact(ptr0, len0, ptr1, len1);
        var ptr3 = ret[0];
        var len3 = ret[1];
        if (ret[3]) {
            ptr3 = 0; len3 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred4_0 = ptr3;
        deferred4_1 = len3;
        return getStringFromWasm0(ptr3, len3);
    } finally {
        wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
    }
}

/**
 * @param {BigInt64Array} ring_flat
 * @param {any} parts_flat
 * @returns {any}
 */
export function collect_steiner_points(ring_flat, parts_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.collect_steiner_points(ptr0, len0, parts_flat);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {bigint} a1x
 * @param {bigint} a1y
 * @param {bigint} a2x
 * @param {bigint} a2y
 * @param {bigint} b1x
 * @param {bigint} b1y
 * @param {bigint} b2x
 * @param {bigint} b2y
 * @param {BigInt64Array} a_ring_flat
 * @param {BigInt64Array} b_ring_flat
 * @returns {boolean}
 */
export function collinear_segments_overlap_area_rings(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y, a_ring_flat, b_ring_flat) {
    const ptr0 = passArray64ToWasm0(a_ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray64ToWasm0(b_ring_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.collinear_segments_overlap_area_rings(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y, ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {any} outer_parts_flat
 * @param {any} inner_parts_flat
 * @returns {boolean}
 */
export function contains_polygon(outer_parts_flat, inner_parts_flat) {
    const ret = wasm.contains_polygon(outer_parts_flat, inner_parts_flat);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} a_flat
 * @param {BigInt64Array} b_flat
 * @returns {boolean}
 */
export function convex_parts_overlap(a_flat, b_flat) {
    const ptr0 = passArray64ToWasm0(a_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray64ToWasm0(b_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.convex_parts_overlap(ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @param {bigint} cx
 * @param {bigint} cy
 * @returns {string}
 */
export function cross2d(ax, ay, bx, by, cx, cy) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.cross2d(ax, ay, bx, by, cx, cy);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @param {bigint} cx
 * @param {bigint} cy
 * @returns {string}
 */
export function cross_sign(ax, ay, bx, by, cx, cy) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.cross_sign(ax, ay, bx, by, cx, cy);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {BigInt64Array} ring_flat
 * @param {boolean} allow_steiner
 * @param {boolean | null} [collect_trace]
 * @param {boolean | null} [minimize_parts]
 * @param {any | null} [config]
 * @returns {any}
 */
export function decompose_polygon(ring_flat, allow_steiner, collect_trace, minimize_parts, config) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.decompose_polygon(ptr0, len0, allow_steiner, isLikeNone(collect_trace) ? 0xFFFFFF : collect_trace ? 1 : 0, isLikeNone(minimize_parts) ? 0xFFFFFF : minimize_parts ? 1 : 0, isLikeNone(config) ? 0 : addToExternrefTable0(config));
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {any}
 */
export function ear_clip_triangulate_polygon(ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ear_clip_triangulate_polygon(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @returns {string}
 */
export function edge_squared_length(ax, ay, bx, by) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.edge_squared_length(ax, ay, bx, by);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {any}
 */
export function ensure_ccw_ring(ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ensure_ccw_ring(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {BigInt64Array} ring_flat
 * @param {any} parts_flat
 * @returns {boolean}
 */
export function exact_partition_only_original_vertices(ring_flat, parts_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.exact_partition_only_original_vertices(ptr0, len0, parts_flat);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {any}
 */
export function exact_vertex_partition_polygon(ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.exact_vertex_partition_polygon(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {any} a_parts_flat
 * @param {any} b_parts_flat
 * @returns {any}
 */
export function find_overlapping_parts(a_parts_flat, b_parts_flat) {
    const ret = wasm.find_overlapping_parts(a_parts_flat, b_parts_flat);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {BigInt64Array} a_flat
 * @param {BigInt64Array} b_flat
 * @returns {boolean}
 */
export function has_exact_shared_edge(a_flat, b_flat) {
    const ptr0 = passArray64ToWasm0(a_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray64ToWasm0(b_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.has_exact_shared_edge(ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {boolean}
 */
export function is_ccw_ring(ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.is_ccw_ring(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {string} cross
 * @returns {boolean}
 */
export function is_collinear(cross) {
    const ptr0 = passStringToWasm0(cross, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.is_collinear(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @param {bigint} px
 * @param {bigint} py
 * @returns {boolean}
 */
export function is_collinear_pts(ax, ay, bx, by, px, py) {
    const ret = wasm.is_collinear_pts(ax, ay, bx, by, px, py);
    return ret !== 0;
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {boolean}
 */
export function is_convex_ring(ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.is_convex_ring(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @param {bigint} px
 * @param {bigint} py
 * @returns {boolean}
 */
export function is_left(ax, ay, bx, by, px, py) {
    const ret = wasm.is_left(ax, ay, bx, by, px, py);
    return ret !== 0;
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @param {bigint} px
 * @param {bigint} py
 * @returns {boolean}
 */
export function is_left_or_on(ax, ay, bx, by, px, py) {
    const ret = wasm.is_left_or_on(ax, ay, bx, by, px, py);
    return ret !== 0;
}

/**
 * @param {string} cross
 * @returns {boolean}
 */
export function is_left_turn(cross) {
    const ptr0 = passStringToWasm0(cross, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.is_left_turn(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {bigint} prev_x
 * @param {bigint} prev_y
 * @param {bigint} curr_x
 * @param {bigint} curr_y
 * @param {bigint} next_x
 * @param {bigint} next_y
 * @returns {boolean}
 */
export function is_reflex(prev_x, prev_y, curr_x, curr_y, next_x, next_y) {
    const ret = wasm.is_reflex(prev_x, prev_y, curr_x, curr_y, next_x, next_y);
    return ret !== 0;
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @param {bigint} px
 * @param {bigint} py
 * @returns {boolean}
 */
export function is_right(ax, ay, bx, by, px, py) {
    const ret = wasm.is_right(ax, ay, bx, by, px, py);
    return ret !== 0;
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @param {bigint} px
 * @param {bigint} py
 * @returns {boolean}
 */
export function is_right_or_on(ax, ay, bx, by, px, py) {
    const ret = wasm.is_right_or_on(ax, ay, bx, by, px, py);
    return ret !== 0;
}

/**
 * @param {string} cross
 * @returns {boolean}
 */
export function is_right_turn(cross) {
    const ptr0 = passStringToWasm0(cross, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.is_right_turn(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {boolean}
 */
export function is_simple_ring(ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.is_simple_ring(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} a_flat
 * @param {BigInt64Array} b_flat
 * @returns {any}
 */
export function merge_convex_pair(a_flat, b_flat) {
    const ptr0 = passArray64ToWasm0(a_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray64ToWasm0(b_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.merge_convex_pair(ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {any}
 */
export function normalize_polygon_ring(ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.normalize_polygon_ring(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {any} parts_flat
 * @returns {any}
 */
export function optimize_partition(parts_flat) {
    const ret = wasm.optimize_partition(parts_flat);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @param {bigint} cx
 * @param {bigint} cy
 * @returns {string}
 */
export function orientation(ax, ay, bx, by, cx, cy) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.orientation(ax, ay, bx, by, cx, cy);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {any} a_parts_flat
 * @param {any} b_parts_flat
 * @returns {boolean}
 */
export function parts_overlap(a_parts_flat, b_parts_flat) {
    const ret = wasm.parts_overlap(a_parts_flat, b_parts_flat);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {string}
 */
export function perimeter_l1_ring(ring_flat) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.perimeter_l1_ring(ptr0, len0);
        var ptr2 = ret[0];
        var len2 = ret[1];
        if (ret[3]) {
            ptr2 = 0; len2 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * @param {any} parts_flat
 * @param {bigint} x
 * @param {bigint} y
 * @returns {boolean}
 */
export function point_inside_any_part(parts_flat, x, y) {
    const ret = wasm.point_inside_any_part(parts_flat, x, y);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {bigint} px
 * @param {bigint} py
 * @param {BigInt64Array} ring_flat
 * @returns {boolean}
 */
export function point_inside_or_on_boundary_ring(px, py, ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.point_inside_or_on_boundary_ring(px, py, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {bigint} px
 * @param {bigint} py
 * @param {BigInt64Array} ring_flat
 * @returns {boolean}
 */
export function point_on_polygon_boundary_ring(px, py, ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.point_on_polygon_boundary_ring(px, py, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {bigint} px
 * @param {bigint} py
 * @param {bigint} ax
 * @param {bigint} ay
 * @param {bigint} bx
 * @param {bigint} by
 * @returns {boolean}
 */
export function point_on_segment(px, py, ax, ay, bx, by) {
    const ret = wasm.point_on_segment(px, py, ax, ay, bx, by);
    return ret !== 0;
}

/**
 * @param {bigint} px
 * @param {bigint} py
 * @param {BigInt64Array} ring_flat
 * @returns {boolean}
 */
export function point_strictly_inside_convex_ring(px, py, ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.point_strictly_inside_convex_ring(px, py, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {any}
 */
export function remove_collinear_ring(ring_flat) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.remove_collinear_ring(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {BigInt64Array} ring_flat
 * @param {number} start
 * @returns {any}
 */
export function rotate_polygon_ring(ring_flat, start) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.rotate_polygon_ring(ptr0, len0, start);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {BigInt64Array} a_flat
 * @param {BigInt64Array} b_flat
 * @returns {boolean}
 */
export function sat_overlap(a_flat, b_flat) {
    const ptr0 = passArray64ToWasm0(a_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray64ToWasm0(b_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.sat_overlap(ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {BigInt64Array} a_flat
 * @param {BigInt64Array} b_flat
 * @returns {boolean}
 */
export function sat_overlap_with_aabb(a_flat, b_flat) {
    const ptr0 = passArray64ToWasm0(a_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray64ToWasm0(b_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.sat_overlap_with_aabb(ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] !== 0;
}

/**
 * @param {bigint} ax1
 * @param {bigint} ay1
 * @param {bigint} ax2
 * @param {bigint} ay2
 * @param {bigint} bx1
 * @param {bigint} by1
 * @param {bigint} bx2
 * @param {bigint} by2
 * @returns {boolean}
 */
export function segments_contact(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) {
    const ret = wasm.segments_contact(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
    return ret !== 0;
}

/**
 * @param {bigint} a1x
 * @param {bigint} a1y
 * @param {bigint} a2x
 * @param {bigint} a2y
 * @param {bigint} b1x
 * @param {bigint} b1y
 * @param {bigint} b2x
 * @param {bigint} b2y
 * @returns {boolean}
 */
export function segments_intersect(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y) {
    const ret = wasm.segments_intersect(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y);
    return ret !== 0;
}

/**
 * @param {bigint} a1x
 * @param {bigint} a1y
 * @param {bigint} a2x
 * @param {bigint} a2y
 * @param {bigint} b1x
 * @param {bigint} b1y
 * @param {bigint} b2x
 * @param {bigint} b2y
 * @returns {boolean}
 */
export function segments_properly_intersect(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y) {
    const ret = wasm.segments_properly_intersect(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y);
    return ret !== 0;
}

/**
 * @param {string} value
 * @returns {number}
 */
export function sign_i128(value) {
    const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.sign_i128(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {string}
 */
export function signed_area_2x_ring(ring_flat) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.signed_area_2x_ring(ptr0, len0);
        var ptr2 = ret[0];
        var len2 = ret[1];
        if (ret[3]) {
            ptr2 = 0; len2 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * @param {bigint} a
 * @param {bigint} b
 * @returns {string}
 */
export function sub_u64(a, b) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.sub_u64(a, b);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {string}
 */
export function twice_area(ring_flat) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.twice_area(ptr0, len0);
        var ptr2 = ret[0];
        var len2 = ret[1];
        if (ret[3]) {
            ptr2 = 0; len2 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * @param {BigInt64Array} ring_flat
 * @returns {string}
 */
export function twice_area_ring(ring_flat) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.twice_area_ring(ptr0, len0);
        var ptr2 = ret[0];
        var len2 = ret[1];
        if (ret[3]) {
            ptr2 = 0; len2 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * Boundary-level compactness check. Apply to a whole polygon's outer
 * boundary (single part, or the union boundary of a multipart polygon).
 * NOT intended for individual parts of a multipart polygon — that would be
 * stricter than on-chain and reject legitimate decompositions.
 * @param {string} twice_area
 * @param {string} perimeter
 * @param {any | null} [config]
 * @returns {string | undefined}
 */
export function validate_compactness_values(twice_area, perimeter, config) {
    const ptr0 = passStringToWasm0(twice_area, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(perimeter, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.validate_compactness_values(ptr0, len0, ptr1, len1, isLikeNone(config) ? 0 : addToExternrefTable0(config));
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    let v3;
    if (ret[0] !== 0) {
        v3 = getStringFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v3;
}

/**
 * @param {BigInt64Array} ring_flat
 * @param {any} parts_flat
 * @param {any | null} [config]
 * @returns {any}
 */
export function validate_decomposition(ring_flat, parts_flat, config) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.validate_decomposition(ptr0, len0, parts_flat, isLikeNone(config) ? 0 : addToExternrefTable0(config));
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {BigInt64Array} ring_flat
 * @param {any | null} [config]
 * @returns {string | undefined}
 */
export function validate_edge_lengths_ring(ring_flat, config) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.validate_edge_lengths_ring(ptr0, len0, isLikeNone(config) ? 0 : addToExternrefTable0(config));
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    let v2;
    if (ret[0] !== 0) {
        v2 = getStringFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {any} parts_flat
 * @param {boolean | null} [allow_vertex_contact]
 * @param {any | null} [config]
 * @returns {any}
 */
export function validate_multipart_topology(parts_flat, allow_vertex_contact, config) {
    const ret = wasm.validate_multipart_topology(parts_flat, isLikeNone(allow_vertex_contact) ? 0xFFFFFF : allow_vertex_contact ? 1 : 0, isLikeNone(config) ? 0 : addToExternrefTable0(config));
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * Per-part structural validation mirroring polygon.move's `part()` entry:
 * vertex-count bounds, weak convexity, and minimum edge length.
 *
 * Breaking change (was: also enforced compactness). Compactness is a
 * boundary-level property; call `validate_multipart_topology` (or the
 * full on-chain validator) to check the assembled polygon instead.
 * @param {BigInt64Array} ring_flat
 * @param {any | null} [config]
 * @returns {string | undefined}
 */
export function validate_part_ring(ring_flat, config) {
    const ptr0 = passArray64ToWasm0(ring_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.validate_part_ring(ptr0, len0, isLikeNone(config) ? 0 : addToExternrefTable0(config));
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    let v2;
    if (ret[0] !== 0) {
        v2 = getStringFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}
export function __wbg_Error_960c155d3d49e4c2(arg0, arg1) {
    const ret = Error(getStringFromWasm0(arg0, arg1));
    return ret;
}
export function __wbg_Number_32bf70a599af1d4b(arg0) {
    const ret = Number(arg0);
    return ret;
}
export function __wbg_String_8564e559799eccda(arg0, arg1) {
    const ret = String(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_bigint_get_as_i64_3d3aba5d616c6a51(arg0, arg1) {
    const v = arg1;
    const ret = typeof(v) === 'bigint' ? v : undefined;
    getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
}
export function __wbg___wbindgen_boolean_get_6ea149f0a8dcc5ff(arg0) {
    const v = arg0;
    const ret = typeof(v) === 'boolean' ? v : undefined;
    return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
}
export function __wbg___wbindgen_debug_string_ab4b34d23d6778bd(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_in_a5d8b22e52b24dd1(arg0, arg1) {
    const ret = arg0 in arg1;
    return ret;
}
export function __wbg___wbindgen_is_bigint_ec25c7f91b4d9e93(arg0) {
    const ret = typeof(arg0) === 'bigint';
    return ret;
}
export function __wbg___wbindgen_is_function_3baa9db1a987f47d(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
}
export function __wbg___wbindgen_is_object_63322ec0cd6ea4ef(arg0) {
    const val = arg0;
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
}
export function __wbg___wbindgen_is_undefined_29a43b4d42920abd(arg0) {
    const ret = arg0 === undefined;
    return ret;
}
export function __wbg___wbindgen_jsval_eq_d3465d8a07697228(arg0, arg1) {
    const ret = arg0 === arg1;
    return ret;
}
export function __wbg___wbindgen_jsval_loose_eq_cac3565e89b4134c(arg0, arg1) {
    const ret = arg0 == arg1;
    return ret;
}
export function __wbg___wbindgen_number_get_c7f42aed0525c451(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
}
export function __wbg___wbindgen_shr_436553cbaef41a66(arg0, arg1) {
    const ret = arg0 >> arg1;
    return ret;
}
export function __wbg___wbindgen_string_get_7ed5322991caaec5(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_throw_6b64449b9b9ed33c(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_call_14b169f759b26747() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments); }
export function __wbg_done_9158f7cc8751ba32(arg0) {
    const ret = arg0.done;
    return ret;
}
export function __wbg_get_1affdbdd5573b16a() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(arg0, arg1);
    return ret;
}, arguments); }
export function __wbg_get_unchecked_17f53dad852b9588(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
}
export function __wbg_get_with_ref_key_6412cf3094599694(arg0, arg1) {
    const ret = arg0[arg1];
    return ret;
}
export function __wbg_instanceof_ArrayBuffer_7c8433c6ed14ffe3(arg0) {
    let result;
    try {
        result = arg0 instanceof ArrayBuffer;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_Uint8Array_152ba1f289edcf3f(arg0) {
    let result;
    try {
        result = arg0 instanceof Uint8Array;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_isArray_c3109d14ffc06469(arg0) {
    const ret = Array.isArray(arg0);
    return ret;
}
export function __wbg_isSafeInteger_4fc213d1989d6d2a(arg0) {
    const ret = Number.isSafeInteger(arg0);
    return ret;
}
export function __wbg_iterator_013bc09ec998c2a7() {
    const ret = Symbol.iterator;
    return ret;
}
export function __wbg_length_3d4ecd04bd8d22f1(arg0) {
    const ret = arg0.length;
    return ret;
}
export function __wbg_length_9f1775224cf1d815(arg0) {
    const ret = arg0.length;
    return ret;
}
export function __wbg_new_0c7403db6e782f19(arg0) {
    const ret = new Uint8Array(arg0);
    return ret;
}
export function __wbg_new_682678e2f47e32bc() {
    const ret = new Array();
    return ret;
}
export function __wbg_new_aa8d0fa9762c29bd() {
    const ret = new Object();
    return ret;
}
export function __wbg_next_0340c4ae324393c3() { return handleError(function (arg0) {
    const ret = arg0.next();
    return ret;
}, arguments); }
export function __wbg_next_7646edaa39458ef7(arg0) {
    const ret = arg0.next;
    return ret;
}
export function __wbg_prototypesetcall_a6b02eb00b0f4ce2(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
}
export function __wbg_set_3bf1de9fab0cd644(arg0, arg1, arg2) {
    arg0[arg1 >>> 0] = arg2;
}
export function __wbg_set_6be42768c690e380(arg0, arg1, arg2) {
    arg0[arg1] = arg2;
}
export function __wbg_value_ee3a06f4579184fa(arg0) {
    const ret = arg0.value;
    return ret;
}
export function __wbindgen_cast_0000000000000001(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return ret;
}
export function __wbindgen_cast_0000000000000002(arg0) {
    // Cast intrinsic for `I64 -> Externref`.
    const ret = arg0;
    return ret;
}
export function __wbindgen_cast_0000000000000003(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_cast_0000000000000004(arg0, arg1) {
    // Cast intrinsic for `U128 -> Externref`.
    const ret = (BigInt.asUintN(64, arg0) | (BigInt.asUintN(64, arg1) << BigInt(64)));
    return ret;
}
export function __wbindgen_cast_0000000000000005(arg0) {
    // Cast intrinsic for `U64 -> Externref`.
    const ret = BigInt.asUintN(64, arg0);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedBigUint64ArrayMemory0 = null;
function getBigUint64ArrayMemory0() {
    if (cachedBigUint64ArrayMemory0 === null || cachedBigUint64ArrayMemory0.byteLength === 0) {
        cachedBigUint64ArrayMemory0 = new BigUint64Array(wasm.memory.buffer);
    }
    return cachedBigUint64ArrayMemory0;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getBigUint64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
