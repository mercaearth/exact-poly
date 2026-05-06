# exact-poly

[![build](https://github.com/mercaearth/exact-poly/actions/workflows/build.yml/badge.svg)](https://github.com/mercaearth/exact-poly/actions/workflows/build.yml)

**Live demo:** [exact-poly.merca.earth](https://exact-poly.merca.earth)

Integer polygon geometry for deterministic on-chain validation. Rust library compiled to WebAssembly.

## Used by

- [mercaearth/mercator](https://github.com/mercaearth/mercator) — reverse side, Move VM polygon validation on-chain
- [merca.earth](https://merca.earth) — production land registry frontend

All arithmetic uses `i64` coordinates in fixed-point representation (1 unit = 1 micrometer at SCALE = 1,000,000). No floats anywhere — results are bit-exact and reproducible across every platform.

## Why

Floating-point polygon math is nondeterministic across architectures. On-chain land registry needs geometry operations that produce identical results on every validator node, in every browser, and in every Move VM execution. `exact-poly` solves this by using integer-only arithmetic with 128-bit intermediates where needed.

## Features

**Convex decomposition** — split any simple polygon into convex parts:
- Cascade strategy: tries ExactPartition → Bayazit → EarClip+Hertel-Mehlhorn, picks the first valid result
- Ring rotation for difficult geometries
- Minimize-parts mode: exhaustive search across all strategies and rotations
- Steiner point tracking

**Area & perimeter** — exact computation without division:
- `twice_area` returns 2× the area (avoids the ÷2 that would lose precision)
- Signed area for winding detection
- L1 (Manhattan) perimeter
- Area conservation verification across decomposed parts

**Ring operations**:
- CCW/CW winding detection and normalization
- Simplicity check (self-intersection detection)
- Convexity check
- Collinear vertex removal
- Canonical normalization (rotation to smallest vertex)

**Spatial queries**:
- Point-in-convex-polygon (strict interior)
- Point-on-boundary
- Point-inside-or-on-boundary (inclusive)

**Overlap detection**:
- SAT (Separating Axis Theorem) for convex pairs
- AABB pre-filter for early rejection
- Multi-part overlap search

**Topology validation**:
- Shared edge detection between adjacent parts
- BFS connectivity check
- Hole detection via boundary graph analysis
- Vertex-only contact detection
- T-junction / partial overlap classification
- Compactness (isoperimetric ratio) validation

**Geometric primitives**:
- Cross product, orientation (CCW / CW / Collinear)
- Point-on-segment, segment intersection
- Reflex vertex detection

## Install

### npm (WASM)

```
npm install exact-poly
```

Package ships three wasm-bindgen targets — picked automatically via conditional `exports`:

| Runtime | Resolved entry | Notes |
|---|---|---|
| Node.js (`import`/`require`) | `pkg/node/` | Sync init, works out of the box |
| Bundlers (Vite, webpack, Rollup) | `pkg/bundler/` | Vite needs [`vite-plugin-wasm`](https://www.npmjs.com/package/vite-plugin-wasm) + [`vite-plugin-top-level-await`](https://www.npmjs.com/package/vite-plugin-top-level-await); webpack 5 supports WASM natively |
| Browser direct (no bundler) | `import "exact-poly/web"` | Returns an `init()` you must `await` before calling exports |

All functions accept `BigInt64Array` for polygon rings encoded as flat `[x0, y0, x1, y1, ...]`.

**Node / bundler:**
```js
import { decompose_polygon, twice_area, is_convex } from "exact-poly";

const ring = BigInt64Array.from([0n, 0n, 60n, 0n, 60n, 40n, 30n, 40n, 30n, 80n, 0n, 80n]);

const result = decompose_polygon(ring, true);
console.log(result.parts.length); // convex parts

const area = twice_area(ring);
console.log(area); // "7200" (2× the actual area)

console.log(is_convex(ring)); // false (L-shape)
```

**Browser direct:**
```js
import init, { twice_area } from "exact-poly/web";

await init();
const ring = BigInt64Array.from([0n, 0n, 60n, 0n, 60n, 40n, 30n, 40n, 30n, 80n, 0n, 80n]);
console.log(twice_area(ring)); // "7200"
```

### Rust (crate)

```toml
[dependencies]
exact-poly = { git = "https://github.com/mercaearth/exact-poly" }
```

```rust
use exact_poly::decompose::decompose;
use exact_poly::types::{DecomposeOptions, ProtocolConfig};

let ring = vec![[0, 0], [60, 0], [60, 40], [30, 40], [30, 80], [0, 80]];
let result = decompose(&ring, &DecomposeOptions::default(), &ProtocolConfig::default()).unwrap();
println!("{} parts", result.parts.len());
```

## Build

Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/):

```
bash build.sh
```

Builds three wasm-bindgen targets in parallel — `pkg/bundler/`, `pkg/node/`, `pkg/web/`. Run tests with:

```
cargo test
```

222 tests covering decomposition strategies, area conservation, topology validation, edge cases, and geometric primitives.

## Demo

Interactive demo app with 7 tabs visualizing every feature:

```
cd demo && npm install && npm run dev
```

Draw polygons or pick presets. See decomposition results, area metrics, reflex vertices, point-in-polygon testing, SAT overlap detection, topology validation, and primitive operations — all running the WASM module in the browser.

## Coordinate convention

| Property | Value |
|---|---|
| Type | `i64` (signed 64-bit integer) |
| Scale | 1,000,000 units = 1 meter |
| Winding | Counter-clockwise (CCW) |
| Encoding | Flat array: `[x0, y0, x1, y1, ...]` |
| JS type | `BigInt64Array` |

Areas are returned as `2 × area` (string-encoded `u128` / `i128`) to avoid precision loss from division.

## Protocol config

Validation functions accept an optional `ProtocolConfig` controlling on-chain limits:

| Parameter | Default (Merca) | Description |
|---|---|---|
| `max_parts` | 10 | Maximum convex parts per polygon |
| `max_vertices_per_part` | 64 | Maximum vertices per convex part |
| `min_edge_length_squared` | 10^12 | Minimum edge length² (1 meter²) |
| `min_compactness_ppm` | 150,000 | Isoperimetric ratio floor (ppm) |
| `area_divisor` | 2 × 10^12 | Converts 2×area to display m² |

Use `ProtocolConfig::permissive()` (Rust) or omit the config parameter (JS) for demo/testing with no validation limits.

## API reference

### Decomposition

| Function | Description |
|---|---|
| `decompose_polygon(ring, allow_steiner, trace?, minimize?, config?)` | Cascade decomposition into convex parts |
| `bayazit_decompose_polygon(ring, allow_steiner)` | Bayazit algorithm only |
| `exact_vertex_partition_polygon(ring)` | Diagonal-only partition (no Steiner points) |
| `ear_clip_triangulate_polygon(ring)` | Ear clipping triangulation |
| `optimize_partition(parts)` | Hertel-Mehlhorn merge of triangulated parts |

### Area & metrics

| Function | Returns |
|---|---|
| `twice_area_ring(ring)` | Unsigned 2×area as string |
| `signed_area_2x_ring(ring)` | Signed 2×area (positive = CCW) |
| `areas_conserved_values(original, part_areas)` | `true` if sum of parts equals original |
| `perimeter_l1_ring(ring)` | Manhattan perimeter as string |

### Ring operations

| Function | Description |
|---|---|
| `is_ccw_ring(ring)` | Winding direction check |
| `ensure_ccw_ring(ring)` | Reverse if CW → return CCW |
| `is_simple_ring(ring)` | No self-intersections |
| `is_convex_ring(ring)` | All vertices convex |
| `remove_collinear_ring(ring)` | Strip collinear vertices |
| `normalize_polygon_ring(ring)` | Canonical form (rotate to smallest vertex) |

### Spatial queries

| Function | Description |
|---|---|
| `point_strictly_inside_convex_ring(px, py, ring)` | Strict interior (convex only) |
| `point_on_polygon_boundary_ring(px, py, ring)` | On any edge |
| `point_inside_or_on_boundary_ring(px, py, ring)` | Interior or boundary |
| `point_inside_any_part(parts, x, y)` | Point in any convex part |
| `contains_polygon(outer_parts, inner_parts)` | Full polygon containment |

### Overlap

| Function | Description |
|---|---|
| `sat_overlap(a, b)` | SAT overlap for two convex polygons |
| `sat_overlap_with_aabb(a, b)` | SAT with AABB early-out |
| `convex_parts_overlap(a, b)` | Interior overlap (touching OK) |
| `parts_overlap(a_parts, b_parts)` | Any part pair overlaps |

### Topology

| Function | Description |
|---|---|
| `validate_multipart_topology(parts, allow_vertex_contact?, config?)` | Full topology validation |
| `has_exact_shared_edge(a, b)` | Shared edge between two parts |
| `classify_contact(a, b)` | `"shared_edge"` / `"partial_contact"` / `"none"` |
| `validate_decomposition(ring, parts, config?)` | End-to-end on-chain validation |

### Primitives

| Function | Description |
|---|---|
| `orientation(ax, ay, bx, by, cx, cy)` | `"CounterClockwise"` / `"Clockwise"` / `"Collinear"` |
| `cross2d(ax, ay, bx, by, cx, cy)` | Cross product as string |
| `is_left(ax, ay, bx, by, px, py)` | Point left of directed line |
| `is_reflex(prev_x, prev_y, curr_x, curr_y, next_x, next_y)` | Reflex vertex test |
| `segments_intersect(a1x, a1y, a2x, a2y, b1x, b1y, b2x, b2y)` | Segment intersection (including endpoints) |
| `segments_properly_intersect(...)` | Strict crossing (excluding endpoints) |
| `point_on_segment(px, py, ax, ay, bx, by)` | Point lies on segment |

## Architecture

```
src/
  lib.rs              WASM bindings (#[wasm_bindgen] exports)
  types.rs            Core types: Point, Part, DecomposeResult, ProtocolConfig
  constants.rs        On-chain protocol constants
  decompose.rs        Cascade decomposition engine
  exact_partition.rs  Diagonal-only convex partition
  bayazit.rs          Bayazit convex decomposition
  ear_clip.rs         Ear clipping triangulation
  hertel_mehlhorn.rs  Triangle merge optimization
  area.rs             Exact area computation (i128)
  ring.rs             Ring operations (CCW, simple, normalize)
  spatial.rs          Point-in-polygon queries
  sat.rs              Separating Axis Theorem
  aabb.rs             Axis-aligned bounding boxes
  overlap.rs          Multi-part overlap detection
  containment.rs      Polygon containment
  shared_edge.rs      Edge sharing and contact classification
  topology.rs         Multipart topology validation
  validation.rs       Per-part structural validation
  validate_onchain.rs End-to-end on-chain validation
  primitives.rs       Cross product, orientation, segment ops
  signed.rs           Signed integer helpers (i128)
```

## License

MIT — see [LICENSE](LICENSE).
