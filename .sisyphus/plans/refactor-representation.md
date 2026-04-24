# Refactor: Unified Data Representation, WASM Split, API Polish

## TL;DR

> **Quick Summary**: Unify exact-poly on a single polygon representation (`&[[i64; 2]]`), split the 684-line WASM binding monolith into domain modules, and clean the public API for open-source 0.2.0 release. Test-first: add safety-net tests before any changes.
> 
> **Deliverables**:
> - All 17 dual-array `(xs, ys)` functions migrated to `&[[i64; 2]]`
> - `src/lib.rs` reduced from 684 → ~100 lines; exports split into `src/wasm/*.rs`
> - 7 dead/internal WASM exports removed; 14 `any` TypeScript types fixed
> - Dead types (`Point`, `Part`) removed; `split_xy` helpers eliminated
> - `ProtocolConfig::default()` → `permissive()` for library-appropriate defaults
> - Demo updated for all API changes
> - Version bumped to 0.2.0
> 
> **Estimated Effort**: Medium-Large (3-5 focused sessions)
> **Parallel Execution**: YES - 6 waves
> **Critical Path**: Wave 0 → Wave 1 (tests) → Wave 2 (migration) → Wave 3 (split) → Wave 4 (polish) → Wave 5 (demo) → FINAL

---

## Context

### Original Request
Refactor exact-poly codebase to be open-source-worthy. Three areas identified in the audit:
1. Dual data representation (`[i64; 2]` vs `(xs, ys)`) creates 40+ unnecessary conversions
2. Monolithic `lib.rs` mixes WASM glue with domain exports
3. Public API has dead exports, `any` types, inconsistent naming

### Interview Summary
**Key Decisions**:
- Canonical representation: `[i64; 2]` arrays. Remove dead `Point` and `Part` types.
- Breaking change accepted → version 0.2.0
- Remove from WASM API: `add_i64`, `sub_u64`, `sign_i128`, `is_left_turn`, `is_right_turn`, `is_collinear`, `cross_sign`
- Test-first: cover untested functions before refactoring
- `ProtocolConfig::default()` → `permissive()` (was `merca()`)
- Drop `_ring` suffix from WASM exports after migration (since ALL functions will take rings)
- Fix TypeScript `any` types via `#[wasm_bindgen(typescript_custom_section)]`
- Demo update is IN-SCOPE (it uses 6 of 7 removed exports)

**Research Findings**:
- 17 functions use `(xs, ys)` across 8 files; NONE fundamentally need it
- Migration is mechanical: `xs[i]` → `ring[i][0]`, `ys[i]` → `ring[i][1]`
- Dependency order: primitives → area/validation/aabb → sat/spatial/shared_edge → overlap/topology → containment/validate_onchain/decompose
- 59 WASM exports total; 14 return `any`, 9 take `any` params = 23 total `any` usages
- `topology.rs` has duplicate implementations of `normalize_edge`, `perimeter_l1`, `point_on_segment`
- `signed.rs::cross_sign` is byte-identical to `primitives.rs::cross2d`
- `bayazit.rs` has hardcoded `ProtocolConfig::merca()` at lines 41 AND 533

### Metis Review
**Identified Gaps** (addressed):
- Demo uses 6 of 7 removed exports → added Wave 5 for demo update
- `ProtocolConfig::default()` returns `merca()` → changing to `permissive()`
- `_ring` suffix naming inconsistency → dropping suffix in 0.2.0
- TypeScript type strategy must be decided upfront → chose `typescript_custom_section`
- `move_style_twice_area_u64` in area.rs also needs migration or removal
- `WasmDecomposeResult`/`WasmIndexPair` types need to move with bindings
- bayazit.rs has TWO hardcoded config sites, not one
- `twice_area_fp2_ring` wrapper must be eliminated, not left as call-through

---

## Work Objectives

### Core Objective
Unify the codebase on `&[[i64; 2]]` as the single polygon representation, split the WASM boundary into maintainable domain modules, and clean the public API for a credible 0.2.0 open-source release.

### Concrete Deliverables
- All `.rs` files in `src/` use `&[[i64; 2]]` for polygon data (zero `(xs, ys)` signatures outside tests)
- `src/wasm/` directory with domain-grouped binding modules
- `src/lib.rs` contains only module declarations, re-exports, and crate-level docs
- `pkg/exact_poly.d.ts` contains zero `any` types
- `Cargo.toml` version = "0.2.0"
- Demo builds and runs with the new API

### Definition of Done
- [ ] `cargo test` passes with ≥222 tests, 0 failures
- [ ] `wasm-pack build --target bundler --release` succeeds
- [ ] `grep -rn 'xs: &\[i64\].*ys: &\[i64\]' src/ --include='*.rs' | grep -v test | grep -v '//' | wc -l` = 0
- [ ] `grep 'any' pkg/exact_poly.d.ts | wc -l` = 0
- [ ] `grep 'split_xy' src/lib.rs | wc -l` = 0
- [ ] `wc -l src/lib.rs` < 120
- [ ] `cd demo && npm run build` succeeds

### Must Have
- Zero `(xs, ys)` function signatures in non-test code
- All WASM exports produce typed TypeScript declarations
- Demo compiles and runs
- All existing tests pass (count ≥ 222)
- `ProtocolConfig::default()` returns `permissive()`

### Must NOT Have (Guardrails)
- NO algorithm logic changes during data migration — ONLY change how data is passed
- NO new shared utility modules — use imports from existing canonical locations
- NO new type hierarchy — only remove dead types
- NO documentation additions beyond cleaning Merca-specific comments
- NO error type changes or error handling pattern changes
- NO benchmark additions (deferred to later)
- NO decompose cascade restructuring
- NO changes to `DecompError` or `TopologyError` enums

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (cargo test, 222 tests)
- **Automated tests**: Tests-after (add safety-net tests before refactoring, verify after each wave)
- **Framework**: `cargo test` (Rust), `wasm-pack build` (WASM)

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

**Evidence Setup**: Task 1 creates `.sisyphus/evidence/` directory. All subsequent QA scenarios MUST redirect command output to their declared evidence file (e.g., `cargo test 2>&1 | tee .sisyphus/evidence/task-N-name.txt`).

**Demo Prerequisite**: Any task running `npm run build` in `demo/` MUST first run `npm install` (since `node_modules/` is gitignored and may not exist).

- **Core modules**: Use Bash (`cargo test`) — run tests, assert 0 failures
- **WASM layer**: Use Bash (`wasm-pack build`) — verify build succeeds, check `.d.ts`
- **Demo**: Use Bash (`cd demo && npm install && npm run build`) — verify TypeScript compiles

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 0 (Baseline — 1 task):
└── Task 1: Capture baseline + verify assumptions [quick]

Wave 1 (Safety-Net Tests — 6 parallel tasks):
├── Task 2: Tests for area.rs + validation.rs migrations [quick]
├── Task 3: Tests for sat.rs + aabb.rs migrations [quick]
├── Task 4: Tests for spatial.rs + shared_edge.rs migrations [quick]
├── Task 5: Tests for overlap.rs + containment.rs migrations [quick]
├── Task 6: Tests for topology.rs + validate_onchain.rs [quick]
└── Task 7: Tests for decompose pipeline (bayazit, exact_partition, hertel_mehlhorn) [quick]

Wave 2 (Core Representation Migration — 3 sequential layers, parallel within):
├── Layer A (depends: Wave 1):
│   ├── Task 8: Migrate area.rs to &[[i64; 2]] [deep]
│   ├── Task 9: Migrate validation.rs to &[[i64; 2]] [deep]
│   └── Task 10: Migrate aabb.rs to &[[i64; 2]] [quick]
├── Layer B (depends: Layer A):
│   ├── Task 11: Migrate sat.rs to &[[i64; 2]] [deep]
│   ├── Task 12: Migrate spatial.rs to &[[i64; 2]] [deep]
│   └── Task 13: Migrate shared_edge.rs to &[[i64; 2]] [deep]
└── Layer C (depends: Layer B):
    ├── Task 14: Migrate overlap.rs + containment.rs [deep]
    ├── Task 15: Migrate topology.rs (+ deduplicate) [deep]
    └── Task 16: Pipeline cleanup: eliminate conversions in decompose/bayazit/exact_partition/hertel_mehlhorn/validate_onchain + remove split_xy + remove dead types [deep]

Wave 3 (lib.rs Split — 3 tasks, partially parallel):
├── Task 17: Create src/wasm/ structure + move shared helpers [unspecified-high]
├── Task 18: Move domain exports to wasm/ modules (depends: 17) [unspecified-high]
└── Task 19: Clean lib.rs to module-only shell (depends: 18) [quick]

Wave 4 (API Polish — 4 parallel tasks):
├── Task 20: Remove dead exports + dead Rust code [quick]
├── Task 21: Fix TypeScript types via typescript_custom_section [deep]
├── Task 22: Fix ProtocolConfig defaults + bayazit hardcoded config + clean comments [quick]
└── Task 23: Rename WASM exports (drop _ring suffix) + bump version [quick]

Wave 5 (Demo Update — 1 task):
└── Task 24: Update demo for all API changes [unspecified-high]

Wave FINAL (After ALL tasks — 4 parallel reviews):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)
-> Present results -> Get explicit user okay
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | — | 2-7 | 0 |
| 2-7 | 1 | 8-16 | 1 |
| 8-10 | 2-7 | 11-13 | 2A |
| 11-13 | 8-10 | 14-16 | 2B |
| 14-16 | 11-13 | 17 | 2C |
| 17 | 14-16 | 18 | 3 |
| 18 | 17 | 19 | 3 |
| 19 | 18 | 20-23 | 3 |
| 20-23 | 19 | 24 | 4 |
| 24 | 20-23 | F1-F4 | 5 |
| F1-F4 | 24 | — | FINAL |

### Agent Dispatch Summary

| Wave | Tasks | Categories |
|------|-------|------------|
| 0 | 1 | T1 → `quick` |
| 1 | 6 | T2-T7 → `quick` |
| 2A | 3 | T8-T9 → `deep`, T10 → `quick` |
| 2B | 3 | T11-T13 → `deep` |
| 2C | 3 | T14-T16 → `deep` |
| 3 | 3 | T17-T18 → `unspecified-high`, T19 → `quick` |
| 4 | 4 | T20 → `quick`, T21 → `deep`, T22-T23 → `quick` |
| 5 | 1 | T24 → `unspecified-high` |
| FINAL | 4 | F1 → `oracle`, F2-F3 → `unspecified-high`, F4 → `deep` |

---

## TODOs

- [x] 1. Capture Baseline + Verify Assumptions

  **What to do**:
  - Create evidence directory: `mkdir -p .sisyphus/evidence`
  - Run `cargo test` and record exact test count (should be 222)
  - Run `wasm-pack build --target bundler --release` and verify success
  - Run `cd demo && npm install && npm run build` and verify success (npm install required — node_modules is gitignored)
  - Save `pkg/exact_poly.d.ts` as `pkg/exact_poly.d.ts.baseline` for later diff
  - Verify `bayazit.rs` hardcoded config at lines 41 AND 533
  - Verify `move_style_twice_area_u64` exists in area.rs and determine if used or dead

  **Must NOT do**:
  - Don't modify source code files
  - Don't change any Rust or TypeScript code

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 0 (solo)
  - **Blocks**: Tasks 2-7
  - **Blocked By**: None

  **References**:
  - `src/bayazit.rs:41` — hardcoded `ProtocolConfig::merca()` in production code
  - `src/bayazit.rs:533` — hardcoded config in test code (acceptable)
  - `src/area.rs` — search for `move_style_twice_area_u64` function
  - `src/types.rs:50-54` — `impl Default for ProtocolConfig` returns `merca()`

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Baseline test suite passes
    Tool: Bash
    Steps:
      1. Run `cargo test 2>&1 | grep "test result"`
      2. Assert output contains "ok" and "0 failed"
      3. Record exact passed count (should be 222)
    Expected Result: "test result: ok. 222 passed; 0 failed; 0 ignored"
    Evidence: .sisyphus/evidence/task-1-baseline-tests.txt

  Scenario: WASM build succeeds
    Tool: Bash
    Steps:
      1. Run `wasm-pack build --target bundler --release 2>&1 | tail -3`
      2. Assert output contains "Your wasm pkg is ready"
      3. Copy `pkg/exact_poly.d.ts` to `pkg/exact_poly.d.ts.baseline`
    Expected Result: Build succeeds, baseline .d.ts saved
    Evidence: .sisyphus/evidence/task-1-wasm-build.txt

  Scenario: Demo builds
    Tool: Bash
    Steps:
      1. Run `cd demo && npm install && npm run build 2>&1 | tail -5`
      2. Assert output contains "built in"
    Expected Result: Demo TypeScript compiles and Vite builds
    Evidence: .sisyphus/evidence/task-1-demo-build.txt
  ```

  **Commit**: NO

---

- [x] 2. Safety-Net Tests: area.rs + validation.rs

  **What to do**:
  - Add direct tests for `twice_area_fp2(xs, ys)` function with various polygon shapes (triangle, square, L-shape, degenerate)
  - Add direct tests for `is_convex(xs, ys)` covering: weakly convex (collinear edges), single triangle, degenerate (2 vertices)
  - Add direct tests for `validate_edge_lengths` with exact-threshold values and negative coords
  - Add direct tests for `perimeter_l1` with negative coordinates and large coordinates
  - Add direct tests for `validate_part` with configs: permissive vs merca
  - Add tests for `validate_compactness` with edge cases: zero area, zero perimeter, overflow-prone values
  - Add test for `area_display` with various divisor values
  - All tests use the CURRENT (xs, ys) API — they verify behavior before migration

  **Must NOT do**:
  - Don't change any function signatures
  - Don't modify existing tests

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 3-7)
  - **Blocks**: Tasks 8, 9
  - **Blocked By**: Task 1

  **References**:
  - `src/area.rs:1-50` — `twice_area_fp2(xs, ys)` and `twice_area_fp2_ring(ring)` signatures
  - `src/area.rs:150-230` — existing test block for patterns to follow
  - `src/validation.rs:1-100` — `is_convex`, `validate_edge_lengths`, `perimeter_l1`, `validate_part` signatures
  - `src/validation.rs:227+` — existing test block (starts at `#[cfg(test)]`)

  **Acceptance Criteria**:
  - [ ] `cargo test` passes with more tests than baseline
  - [ ] New tests exercise CURRENT (xs, ys) API with diverse inputs

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: New tests pass alongside existing
    Tool: Bash
    Steps:
      1. Run `cargo test area::tests 2>&1 | grep "test result"`
      2. Assert "0 failed" and count > previous
      3. Run `cargo test validation::tests 2>&1 | grep "test result"`
      4. Assert "0 failed" and count > previous
    Expected Result: All area + validation tests pass, count increased
    Evidence: .sisyphus/evidence/task-2-area-validation-tests.txt
  ```

  **Commit**: YES (groups with 3-7)
  - Message: `test: add safety-net tests for representation migration`

---

- [x] 3. Safety-Net Tests: sat.rs + aabb.rs

  **What to do**:
  - Add direct tests for `sat_overlaps(a_xs, a_ys, b_xs, b_ys)` with: triangles, non-convex rejection, MAX_WORLD coordinates
  - Add direct tests for `project_onto_axis(xs, ys, ax, ay)` with edge normals
  - Add direct tests for `Aabb::from_vertices(xs, ys)` with: single vertex, negative coords, large range
  - Add tests for `Aabb::merge` with disjoint and nested boxes
  - All tests use CURRENT (xs, ys) API

  **Must NOT do**:
  - Don't change function signatures
  - Don't modify existing tests

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 4-7)
  - **Blocks**: Tasks 10, 11
  - **Blocked By**: Task 1

  **References**:
  - `src/sat.rs:1-80` — `sat_overlaps`, `project_onto_axis`, `validate_polygon` signatures
  - `src/sat.rs:90-180` — existing test block
  - `src/aabb.rs:1-80` — `Aabb` struct and `from_vertices(xs, ys)` method
  - `src/aabb.rs:85-160` — existing test block

  **Acceptance Criteria**:
  - [ ] `cargo test sat::tests` and `cargo test aabb::tests` pass with increased count

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: SAT and AABB tests pass
    Tool: Bash
    Steps:
      1. Run `cargo test sat::tests 2>&1 | grep "test result"`
      2. Run `cargo test aabb::tests 2>&1 | grep "test result"`
      3. Assert both show "0 failed"
    Expected Result: All tests pass, count increased
    Evidence: .sisyphus/evidence/task-3-sat-aabb-tests.txt
  ```

  **Commit**: YES (groups with 2, 4-7)

---

- [x] 4. Safety-Net Tests: spatial.rs + shared_edge.rs

  **What to do**:
  - Add direct tests for `point_strictly_inside_convex(px, py, xs, ys)` with: point at centroid, near edge, at vertex of non-square convex polygon
  - Add direct tests for `point_on_polygon_boundary(px, py, xs, ys)` with: midpoint of edge, near-miss (1 unit off)
  - Add direct tests for `point_inside_or_on_boundary` covering all three cases (interior, boundary, outside)
  - Add direct tests for `has_exact_shared_edge(a_xs, a_ys, b_xs, b_ys)` with: partial edge overlap (should be false), collinear non-overlapping edges
  - Add direct tests for `classify_contact` with: vertex-only contact (returns "none"), T-junction geometry
  - All tests use CURRENT (xs, ys) API

  **Must NOT do**:
  - Don't change function signatures

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2-3, 5-7)
  - **Blocks**: Tasks 12, 13
  - **Blocked By**: Task 1

  **References**:
  - `src/spatial.rs:1-80` — `point_strictly_inside_convex`, `point_on_polygon_boundary`, `point_inside_or_on_boundary` signatures
  - `src/spatial.rs:100-180` — existing tests
  - `src/shared_edge.rs:1-100` — `has_exact_shared_edge`, `classify_contact` signatures
  - `src/shared_edge.rs:110-280` — existing tests

  **Acceptance Criteria**:
  - [ ] `cargo test spatial::tests` and `cargo test shared_edge::tests` pass with increased count

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Spatial and shared_edge tests pass
    Tool: Bash
    Steps:
      1. Run `cargo test spatial::tests 2>&1 | grep "test result"`
      2. Run `cargo test shared_edge::tests 2>&1 | grep "test result"`
      3. Assert both show "0 failed"
    Expected Result: All pass, count increased
    Evidence: .sisyphus/evidence/task-4-spatial-shared-edge-tests.txt
  ```

  **Commit**: YES (groups with 2-3, 5-7)

---

- [x] 5. Safety-Net Tests: overlap.rs + containment.rs

  **What to do**:
  - Add direct tests for `convex_parts_overlap(a_xs, a_ys, b_xs, b_ys)` with: touching edges (should be false), micro-overlap
  - Add direct tests for `find_overlapping_parts` with: 3+ parts, no overlaps, all overlapping
  - Add direct tests for `contains_polygon` with: concave outer boundary (multipart), inner touching outer boundary
  - All tests use CURRENT API (some functions take `&[Vec<[i64; 2]>]`, some take `(xs, ys)`)

  **Must NOT do**:
  - Don't change function signatures

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2-4, 6-7)
  - **Blocks**: Task 14
  - **Blocked By**: Task 1

  **References**:
  - `src/overlap.rs:1-110` — `convex_parts_overlap`, `find_overlapping_parts`, `parts_overlap` signatures
  - `src/overlap.rs:120-200` — existing tests
  - `src/containment.rs:1-130` — `contains_polygon`, `point_inside_any_part` signatures
  - `src/containment.rs:140-240` — existing tests

  **Acceptance Criteria**:
  - [ ] `cargo test overlap::tests` and `cargo test containment::tests` pass with increased count

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Overlap and containment tests pass
    Tool: Bash
    Steps:
      1. Run `cargo test overlap::tests 2>&1 | grep "test result"`
      2. Run `cargo test containment::tests 2>&1 | grep "test result"`
      3. Assert both show "0 failed"
    Expected Result: All pass, count increased
    Evidence: .sisyphus/evidence/task-5-overlap-containment-tests.txt
  ```

  **Commit**: YES (groups with 2-4, 6-7)

---

- [x] 6. Safety-Net Tests: topology.rs + validate_onchain.rs

  **What to do**:
  - Add direct tests for `validate_multipart_topology` with: single part (should pass), 3-part connected L-shape, parts exceeding max_parts
  - Add direct tests for boundary graph validation: polygon with hole geometry, vertex-only contact with allow_vertex_contact=true vs false
  - Add direct tests for `validate_decomposition` with: valid L-shape decomposition, parts with area mismatch, overlapping parts
  - Test with both `ProtocolConfig::merca()` and `ProtocolConfig::permissive()` to catch config-dependent behavior

  **Must NOT do**:
  - Don't change function signatures

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2-5, 7)
  - **Blocks**: Task 15
  - **Blocked By**: Task 1

  **References**:
  - `src/topology.rs:1-50` — `validate_multipart_topology` signature and doc comments
  - `src/topology.rs:200-350` — existing tests
  - `src/validate_onchain.rs:1-50` — `validate_decomposition` signature
  - `src/validate_onchain.rs:313+` — existing test block (starts at `#[cfg(test)]`)

  **Acceptance Criteria**:
  - [ ] `cargo test topology::tests` and `cargo test validate_onchain::tests` pass with increased count

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Topology and onchain validation tests pass
    Tool: Bash
    Steps:
      1. Run `cargo test topology::tests 2>&1 | grep "test result"`
      2. Run `cargo test validate_onchain::tests 2>&1 | grep "test result"`
      3. Assert both show "0 failed"
    Expected Result: All pass, count increased
    Evidence: .sisyphus/evidence/task-6-topology-onchain-tests.txt
  ```

  **Commit**: YES (groups with 2-5, 7)

---

- [x] 7. Safety-Net Tests: Decompose Pipeline

  **What to do**:
  - Add direct tests for `collect_steiner_points` (currently UNTESTED)
  - Add direct tests for `only_original_vertices` in exact_partition.rs (currently UNTESTED)
  - Add direct tests for `find_steiner_points` in bayazit.rs (currently UNTESTED)
  - Add tests for `bayazit_decompose` with `allow_steiner=false` and difficult geometries
  - Add tests for `merge_convex_pair` with non-adjacent polygons (should return None)
  - Add tests for decompose with `minimize_parts=true` covering the full candidate ranking

  **Must NOT do**:
  - Don't change function signatures

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2-6)
  - **Blocks**: Task 16
  - **Blocked By**: Task 1

  **References**:
  - `src/decompose.rs:27-41` — `decompose` signature; `collect_steiner_points` is at line 651
  - `src/decompose.rs:600-1382` — existing tests (extensive)
  - `src/bayazit.rs:11-47` — `bayazit_decompose` and `find_steiner_points` signatures
  - `src/exact_partition.rs:1-60` — `exact_vertex_partition` and `only_original_vertices` signatures
  - `src/hertel_mehlhorn.rs:1-50` — `optimize_partition` and `merge_convex_pair` signatures

  **Acceptance Criteria**:
  - [ ] Previously untested public functions now have direct tests
  - [ ] `cargo test decompose::tests bayazit::tests exact_partition::tests hertel_mehlhorn::tests` all pass

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Pipeline tests pass
    Tool: Bash
    Steps:
      1. Run `cargo test 2>&1 | grep "test result"`
      2. Assert "0 failed" and total count > 222
    Expected Result: All tests pass, new count recorded
    Evidence: .sisyphus/evidence/task-7-pipeline-tests.txt
  ```

  **Commit**: YES (groups with 2-6)
  - Message: `test: add safety-net tests for representation migration`
  - Pre-commit: `cargo test`

---

- [x] 8. Migrate area.rs to `&[[i64; 2]]`

  **What to do**:
  - Change `twice_area_fp2(xs: &[i64], ys: &[i64])` → `twice_area_fp2(ring: &[[i64; 2]])`. Replace `xs[i]` → `ring[i][0]`, `ys[i]` → `ring[i][1]`
  - **Eliminate** `twice_area_fp2_ring` wrapper — make `twice_area_fp2` directly take `&[[i64; 2]]`. The wrapper was the migration proof-of-concept; now the real function IS the ring version
  - Keep `twice_area_fp2_ring` as a public alias that just calls `twice_area_fp2` (for backward compat during this wave)
  - Migrate or remove `move_style_twice_area_u64` — check if it's used anywhere; if dead, remove it
  - Change `area_display` if it has any (xs, ys) pattern (likely not)
  - Update `areas_conserved` if needed
  - Update ALL callers in `decompose.rs`, `bayazit.rs`, `ear_clip.rs`, `validate_onchain.rs`, `topology.rs` that currently call the old API
  - Update existing tests to use new signature

  **Must NOT do**:
  - Don't change the mathematical algorithm — only how data is accessed
  - Don't change error handling

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2A (with Tasks 9, 10)
  - **Blocks**: Tasks 11-16
  - **Blocked By**: Tasks 2-7

  **References**:
  - `src/area.rs:5-37` — current `twice_area_fp2` implementation with shoelace formula using `xs[i]`, `ys[i]`
  - `src/area.rs:39-43` — `twice_area_fp2_ring` wrapper that does the split-and-delegate pattern. THIS is what the final function should look like (but inlined)
  - `src/decompose.rs:1-10` — imports `twice_area_fp2_ring` from area
  - `src/bayazit.rs:3` — imports `twice_area_fp2_ring`
  - `src/validate_onchain.rs` — calls area functions

  **Acceptance Criteria**:
  - [ ] `grep 'fn twice_area_fp2.*xs.*ys' src/area.rs | wc -l` = 0
  - [ ] `cargo test area::tests` passes, 0 failures
  - [ ] `cargo test` full suite passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Area functions produce identical results after migration
    Tool: Bash
    Steps:
      1. Run `cargo test area::tests 2>&1 | grep "test result"`
      2. Assert "0 failed"
      3. Run full `cargo test 2>&1 | grep "test result"`
      4. Assert "0 failed"
    Expected Result: All area tests pass, no regression in other modules
    Evidence: .sisyphus/evidence/task-8-area-migration.txt

  Scenario: No (xs, ys) signatures remain in area.rs
    Tool: Bash
    Steps:
      1. Run `grep -n 'xs: &\[i64\]' src/area.rs | grep -v test | grep -v '//'`
      2. Assert empty output
    Expected Result: Zero non-test (xs, ys) signatures
    Evidence: .sisyphus/evidence/task-8-area-clean.txt
  ```

  **Commit**: YES (groups with 9, 10)
  - Message: `refactor(area,validation,aabb): migrate to ring representation`
  - Pre-commit: `cargo test`

---

- [x] 9. Migrate validation.rs to `&[[i64; 2]]`

  **What to do**:
  - Change `is_convex(xs: &[i64], ys: &[i64])` → `is_convex(ring: &[[i64; 2]])`. Replace all `xs[prev]`→`ring[prev][0]`, `ys[prev]`→`ring[prev][1]`, etc.
  - Change `validate_edge_lengths(xs, ys, config)` → `validate_edge_lengths(ring, config)`
  - Change `perimeter_l1(xs, ys)` → `perimeter_l1(ring)`
  - Change `validate_part(xs, ys, config)` → `validate_part(ring, config)` — update its calls to `is_convex` and `validate_edge_lengths` accordingly
  - Update ALL callers: `decompose.rs`, `bayazit.rs`, `exact_partition.rs`, `hertel_mehlhorn.rs`, `topology.rs`, `validate_onchain.rs`, `lib.rs`
  - Update existing tests to use new signatures

  **Must NOT do**:
  - Don't change validation logic or thresholds
  - Don't change `ProtocolConfig` (that's Task 22)

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2A (with Tasks 8, 10)
  - **Blocks**: Tasks 11-16
  - **Blocked By**: Tasks 2-7

  **References**:
  - `src/validation.rs:38-70` — `is_convex` with circular indexing: `xs[(i + n - 1) % n]`, `ys[(i + n - 1) % n]`
  - `src/validation.rs:72-90` — `validate_edge_lengths` with `xs[j] - xs[i]`, `ys[j] - ys[i]`
  - `src/validation.rs:92-105` — `perimeter_l1` with `abs_diff` on separate arrays
  - `src/validation.rs:1-36` — `validate_part` that calls `is_convex` + `validate_edge_lengths`
  - `src/bayazit.rs:39-43` — calls `validate_part` with split coords — update caller
  - `src/exact_partition.rs:24-26,53-55` — calls `is_convex` with split coords

  **Acceptance Criteria**:
  - [ ] `grep 'fn is_convex.*xs.*ys\|fn validate_edge.*xs.*ys\|fn perimeter_l1.*xs.*ys\|fn validate_part.*xs.*ys' src/validation.rs | grep -v test | wc -l` = 0
  - [ ] `cargo test validation::tests` passes
  - [ ] `cargo test` full suite passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Validation functions produce identical results
    Tool: Bash
    Steps:
      1. Run `cargo test validation::tests 2>&1 | grep "test result"`
      2. Assert "0 failed"
      3. Run `cargo test 2>&1 | grep "test result"`
      4. Assert "0 failed"
    Expected Result: All tests pass
    Evidence: .sisyphus/evidence/task-9-validation-migration.txt
  ```

  **Commit**: YES (groups with 8, 10)

---

- [x] 10. Migrate aabb.rs to `&[[i64; 2]]`

  **What to do**:
  - Change `Aabb::from_vertices(xs: &[i64], ys: &[i64])` → `Aabb::from_ring(ring: &[[i64; 2]])`
  - Replace zip pattern: `xs[1..].iter().zip(ys[1..].iter())` → iterate over `ring[1..]` directly
  - Update all callers: `sat.rs`, `overlap.rs`, `containment.rs`, `topology.rs`
  - Update existing tests

  **Must NOT do**:
  - Don't change AABB logic

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2A (with Tasks 8, 9)
  - **Blocks**: Tasks 11-16
  - **Blocked By**: Tasks 2-7

  **References**:
  - `src/aabb.rs:20-40` — `from_vertices(xs, ys)` using zip pattern
  - `src/sat.rs` — calls `Aabb::from_vertices` in `sat_overlaps_with_aabb`
  - `src/overlap.rs` — calls `Aabb::from_vertices`

  **Acceptance Criteria**:
  - [ ] `cargo test aabb::tests` passes
  - [ ] `cargo test` full suite passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: AABB functions work with ring representation
    Tool: Bash
    Steps:
      1. Run `cargo test aabb::tests 2>&1 | grep "test result"`
      2. Assert "0 failed"
    Expected Result: All AABB tests pass
    Evidence: .sisyphus/evidence/task-10-aabb-migration.txt
  ```

  **Commit**: YES (groups with 8, 9)

---

- [x] 11. Migrate sat.rs to `&[[i64; 2]]`

  **What to do**:
  - Change `sat_overlaps(a_xs, a_ys, b_xs, b_ys)` → `sat_overlaps(a: &[[i64; 2]], b: &[[i64; 2]])`
  - Change `sat_overlaps_with_aabb` similarly — now calls `Aabb::from_ring(a)` instead of `Aabb::from_vertices(a_xs, a_ys)`
  - Change `project_onto_axis(xs, ys, ax, ay)` → `project_onto_axis(ring: &[[i64; 2]], ax, ay)` — iterate over points instead of zipping
  - Change `validate_polygon(xs, ys)` → `validate_polygon(ring: &[[i64; 2]])` — fix the `assert!` to proper error handling while here
  - Update callers in `overlap.rs` and `lib.rs`
  - Update existing tests

  **Must NOT do**:
  - Don't change SAT algorithm logic
  - Don't change overlap semantics (touching = no overlap)

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2B (with Tasks 12, 13)
  - **Blocks**: Tasks 14-16
  - **Blocked By**: Tasks 8-10

  **References**:
  - `src/sat.rs:7-10` — `validate_polygon` with `assert!` (should use `Result` or at minimum keep behavior)
  - `src/sat.rs:12-50` — `sat_overlaps` implementation with edge normal computation
  - `src/sat.rs:52-60` — `project_onto_axis` with zip pattern
  - `src/overlap.rs:80-85,99-104` — callers that do `split_xy` before calling sat

  **Acceptance Criteria**:
  - [ ] `grep 'fn sat_overlaps.*a_xs.*a_ys' src/sat.rs | grep -v test | wc -l` = 0
  - [ ] `cargo test sat::tests` passes
  - [ ] `cargo test` full suite passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: SAT overlap detection works identically
    Tool: Bash
    Steps:
      1. Run `cargo test sat::tests 2>&1 | grep "test result"`
      2. Assert "0 failed"
      3. Run `cargo test 2>&1 | grep "test result"`
      4. Assert "0 failed"
    Expected Result: All SAT tests pass, no regression
    Evidence: .sisyphus/evidence/task-11-sat-migration.txt
  ```

  **Commit**: YES (groups with 12, 13)
  - Message: `refactor(sat,spatial,shared_edge): migrate to ring representation`

---

- [x] 12. Migrate spatial.rs to `&[[i64; 2]]`

  **What to do**:
  - Change `point_strictly_inside_convex(px, py, xs, ys)` → `point_strictly_inside_convex(px, py, ring: &[[i64; 2]])`
  - Change `point_on_polygon_boundary(px, py, xs, ys)` → `point_on_polygon_boundary(px, py, ring: &[[i64; 2]])`
  - Change `point_inside_or_on_boundary(px, py, xs, ys)` → `point_inside_or_on_boundary(px, py, ring: &[[i64; 2]])`
  - Change `collinear_segments_overlap_area` — this takes both segment coords AND full polygon (xs, ys). Migrate the polygon param to `&[[i64; 2]]`
  - Update callers in `containment.rs`, `topology.rs`, `lib.rs`
  - Update existing tests

  **Must NOT do**:
  - Don't change spatial query logic

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2B (with Tasks 11, 13)
  - **Blocks**: Tasks 14-16
  - **Blocked By**: Tasks 8-10

  **References**:
  - `src/spatial.rs:1-60` — function signatures with (xs, ys) params
  - `src/spatial.rs:62-75` — `collinear_segments_overlap_area` with TWELVE parameters — migrate polygon params
  - `src/containment.rs:15-16` — caller that does split to call spatial functions

  **Acceptance Criteria**:
  - [ ] `cargo test spatial::tests` passes
  - [ ] `cargo test` full suite passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Spatial queries work identically
    Tool: Bash
    Steps:
      1. Run `cargo test spatial::tests 2>&1 | grep "test result"`
      2. Assert "0 failed"
    Expected Result: All spatial tests pass
    Evidence: .sisyphus/evidence/task-12-spatial-migration.txt
  ```

  **Commit**: YES (groups with 11, 13)

---

- [x] 13. Migrate shared_edge.rs to `&[[i64; 2]]`

  **What to do**:
  - Change `has_exact_shared_edge(a_xs, a_ys, b_xs, b_ys)` → `has_exact_shared_edge(a: &[[i64; 2]], b: &[[i64; 2]])`
  - Change `classify_contact(a_xs, a_ys, b_xs, b_ys)` → `classify_contact(a: &[[i64; 2]], b: &[[i64; 2]])`
  - Change `normalize_edge(ax, ay, bx, by)` → `normalize_edge(a: [i64; 2], b: [i64; 2])` — align with topology.rs version
  - Change `segments_contact(ax1, ay1, ..., by2)` — keep scalar params (these are segment endpoints, not polygon data)
  - Update callers in `topology.rs`, `lib.rs`
  - Update existing tests

  **Must NOT do**:
  - Don't change contact classification logic
  - Don't change `segments_contact` signature (it takes segment endpoints, not polygon data)

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2B (with Tasks 11, 12)
  - **Blocks**: Tasks 14-16
  - **Blocked By**: Tasks 8-10

  **References**:
  - `src/shared_edge.rs:15` — `normalize_edge(ax, ay, bx, by)` — different signature from topology.rs version
  - `src/shared_edge.rs:25-55` — `has_exact_shared_edge` with `a_xs[i]`, `a_ys[i]` indexing
  - `src/topology.rs:302-313` — duplicate `normalize_edge(a: [i64;2], b: [i64;2])` — after migration, shared_edge.rs should match this

  **Acceptance Criteria**:
  - [ ] `cargo test shared_edge::tests` passes
  - [ ] `cargo test` full suite passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Shared edge detection works identically
    Tool: Bash
    Steps:
      1. Run `cargo test shared_edge::tests 2>&1 | grep "test result"`
      2. Assert "0 failed"
    Expected Result: All shared_edge tests pass
    Evidence: .sisyphus/evidence/task-13-shared-edge-migration.txt
  ```

  **Commit**: YES (groups with 11, 12)

---

- [x] 14. Migrate overlap.rs + containment.rs

  **What to do**:
  - Change `convex_parts_overlap(a_xs, a_ys, b_xs, b_ys)` → `convex_parts_overlap(a: &[[i64; 2]], b: &[[i64; 2]])`
  - Eliminate all `split_xy` conversions in `find_overlapping_parts` and `parts_overlap` — these currently split `&[[i64;2]]` → `(xs,ys)` just to call the old functions. After sat.rs and aabb.rs migration, they can call directly
  - Eliminate all `split_xy` conversions in `containment.rs` — `point_inside_any_part` and `contains_polygon` currently split to call spatial functions
  - Update callers in `lib.rs`
  - Update existing tests

  **Must NOT do**:
  - Don't change overlap semantics
  - Don't change containment algorithm

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2C (with Tasks 15, 16)
  - **Blocks**: Task 17
  - **Blocked By**: Tasks 11-13

  **References**:
  - `src/overlap.rs:60-105` — `convex_parts_overlap`, `find_overlapping_parts`, `parts_overlap` — all with split conversions
  - `src/containment.rs:10-130` — `point_inside_any_part`, `contains_polygon` with split conversions at lines 15-16, 104-105
  - Now that `sat_overlaps` and `Aabb::from_ring` take `&[[i64;2]]`, the conversions in overlap.rs become dead code

  **Acceptance Criteria**:
  - [ ] Zero `let xs:` / `let ys:` conversion patterns in overlap.rs and containment.rs (outside tests)
  - [ ] `cargo test overlap::tests containment::tests` passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Overlap and containment work identically
    Tool: Bash
    Steps:
      1. Run `cargo test overlap::tests 2>&1 | grep "test result"`
      2. Run `cargo test containment::tests 2>&1 | grep "test result"`
      3. Assert both "0 failed"
    Expected Result: All tests pass
    Evidence: .sisyphus/evidence/task-14-overlap-containment-migration.txt
  ```

  **Commit**: YES (groups with 15, 16)
  - Message: `refactor(overlap,topology,pipeline): complete ring migration, remove split_xy`

---

- [x] 15. Migrate topology.rs + Deduplicate

  **What to do**:
  - Remove private `split_coords` function (topology.rs) — no longer needed since all called functions take `&[[i64;2]]`
  - Remove private duplicate `perimeter_l1` — import from `crate::validation::perimeter_l1` instead
  - Remove private duplicate `point_on_segment` — import from `crate::primitives::point_on_segment` instead
  - Remove private duplicate `normalize_edge` — import from `crate::shared_edge::normalize_edge` (which by Task 13 will take `[i64;2]` pairs)
  - Update `validate_multipart_topology` internals to pass `&[[i64;2]]` directly to called functions instead of splitting
  - Update existing tests

  **Must NOT do**:
  - Don't change topology validation logic
  - Don't create new utility modules — just import from existing canonical locations

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2C (with Tasks 14, 16)
  - **Blocks**: Task 17
  - **Blocked By**: Tasks 11-13

  **References**:
  - `src/topology.rs:97-98` — `split_coords` usage (to be eliminated)
  - `src/topology.rs:179-184` — private `point_on_segment` (duplicate of primitives.rs)
  - `src/topology.rs:302-313` — private `normalize_edge` (duplicate of shared_edge.rs)
  - `src/topology.rs` — private `perimeter_l1` (duplicate of validation.rs)
  - After Tasks 9, 12, 13: the canonical functions already take `&[[i64;2]]`

  **Acceptance Criteria**:
  - [ ] Zero `split_coords` references in topology.rs
  - [ ] Zero private duplicate functions (perimeter_l1, point_on_segment, normalize_edge)
  - [ ] `cargo test topology::tests` passes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Topology validation works identically after dedup
    Tool: Bash
    Steps:
      1. Run `cargo test topology::tests 2>&1 | grep "test result"`
      2. Assert "0 failed"
      3. Run `cargo test 2>&1 | grep "test result"`
      4. Assert "0 failed"
    Expected Result: All tests pass, no regression
    Evidence: .sisyphus/evidence/task-15-topology-migration.txt
  ```

  **Commit**: YES (groups with 14, 16)

---

- [x] 16. Pipeline Cleanup: Eliminate Remaining Conversions + Remove Dead Code

  **What to do**:
  - Eliminate all remaining `let xs: Vec<i64> = ...map(|v| v[0]).collect()` patterns in: `decompose.rs`, `bayazit.rs`, `exact_partition.rs`, `hertel_mehlhorn.rs`, `validate_onchain.rs`
  - These files take `&[[i64;2]]` already but internally split to call (xs,ys) functions. After Wave 2A/2B, those functions now take `&[[i64;2]]` directly → remove the split-and-delegate
  - Fix `bayazit.rs:41` — change hardcoded `ProtocolConfig::merca()` to accept config parameter from caller
  - Remove `split_xy`, `split_xy_from_flat` helper functions from `lib.rs` — no longer needed
  - Remove dead types from `types.rs`: `Point` struct (lines 56-80), `Part` struct (lines 82-89)
  - Update `lib.rs` WASM exports to pass `&[[i64;2]]` directly instead of splitting
  - Run full test suite

  **Must NOT do**:
  - Don't change decomposition algorithm logic
  - Don't change cascade order or strategy selection
  - Don't restructure decompose.rs

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2C (with Tasks 14, 15)
  - **Blocks**: Task 17
  - **Blocked By**: Tasks 11-13

  **References**:
  - `src/lib.rs:57-67` — `split_xy_from_flat` and `split_xy` — DELETE after callers updated
  - `src/lib.rs:197-206` — `twice_area` and `twice_area_ring` WASM exports — update to call new API directly
  - `src/types.rs:56-89` — dead `Point` and `Part` types — DELETE
  - `src/bayazit.rs:41` — `ProtocolConfig::merca()` hardcoded — MUST accept config parameter
  - `src/bayazit.rs:533` — config in test (acceptable, leave as-is)
  - `src/exact_partition.rs:24-26,53-55` — split patterns to eliminate
  - `src/hertel_mehlhorn.rs:93-94,172-173,201-202` — split patterns to eliminate
  - `src/decompose.rs:606-607,769-770` — split patterns to eliminate

  **Acceptance Criteria**:
  - [ ] `grep -rn 'split_xy' src/ --include='*.rs' | grep -v test | grep -v '//' | wc -l` = 0
  - [ ] `grep 'pub struct Point\b\|pub struct Part\b' src/types.rs | wc -l` = 0
  - [ ] `grep -rn 'xs: &\[i64\].*ys: &\[i64\]' src/ --include='*.rs' | grep -v test | grep -v '//' | wc -l` = 0
  - [ ] `cargo test` full suite passes, 0 failures
  - [ ] `wasm-pack build --target bundler --release` succeeds

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Full codebase migrated, no (xs,ys) signatures remain
    Tool: Bash
    Steps:
      1. Run `grep -rn 'xs: &\[i64\].*ys: &\[i64\]' src/ --include='*.rs' | grep -v test | grep -v '//' | wc -l`
      2. Assert output = 0
      3. Run `grep 'split_xy' src/lib.rs | wc -l`
      4. Assert output = 0
      5. Run `cargo test 2>&1 | grep "test result"`
      6. Assert "0 failed"
      7. Run `wasm-pack build --target bundler --release 2>&1 | tail -3`
      8. Assert success
    Expected Result: Zero dual-array signatures, zero split_xy, all tests pass, WASM builds
    Evidence: .sisyphus/evidence/task-16-pipeline-cleanup.txt
  ```

  **Commit**: YES (groups with 14, 15)
  - Message: `refactor(overlap,topology,pipeline): complete ring migration, remove split_xy`
  - Pre-commit: `cargo test && wasm-pack build --target bundler --release`

---

- [x] 17. Create `src/wasm/` Structure + Move Shared Helpers

  **What to do**:
  - Create `src/wasm/mod.rs` with submodule declarations
  - Create `src/wasm/helpers.rs` — move these functions from `lib.rs`:
    - `invalid_input`, `parse_flat_ring`, `flatten_ring`, `flatten_parts`, `parse_flat_parts`, `serialize`, `parse_u128_str`, `parse_config`, `has_zero_length_edge`
  - Move `WasmDecomposeResult` and `WasmIndexPair` structs to `src/wasm/types.rs`
  - Update `src/lib.rs` to declare `pub mod wasm;` and re-export
  - Verify WASM still builds

  **Must NOT do**:
  - Don't move `#[wasm_bindgen]` exports yet (that's Task 18)
  - Don't change any function logic
  - Don't rename anything

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential: 17 → 18 → 19)
  - **Blocks**: Task 18
  - **Blocked By**: Tasks 14-16

  **References**:
  - `src/lib.rs:25-38` — `WasmDecomposeResult` and `WasmIndexPair` structs
  - `src/lib.rs:40-114` — all helper functions to move
  - wasm-bindgen documentation for module organization

  **Acceptance Criteria**:
  - [ ] `src/wasm/helpers.rs` exists with all helper functions
  - [ ] `src/wasm/types.rs` exists with WASM-specific types
  - [ ] `cargo test` passes
  - [ ] `wasm-pack build --target bundler --release` succeeds

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: WASM builds after helper extraction
    Tool: Bash
    Steps:
      1. Run `cargo test 2>&1 | grep "test result"`
      2. Assert "0 failed"
      3. Run `wasm-pack build --target bundler --release 2>&1 | tail -3`
      4. Assert success
    Expected Result: Both pass
    Evidence: .sisyphus/evidence/task-17-wasm-helpers.txt
  ```

  **Commit**: YES (groups with 18, 19)
  - Message: `refactor(wasm): split lib.rs into domain binding modules`

---

- [x] 18. Move Domain Exports to `src/wasm/` Modules

  **What to do**:
  - Create and populate these files by moving `#[wasm_bindgen]` exports from `lib.rs`:
    - `src/wasm/decompose.rs` — 5 exports: `decompose_polygon`, `collect_steiner_points`, `bayazit_decompose_polygon`, `exact_vertex_partition_polygon`, `exact_partition_only_original_vertices`, `ear_clip_triangulate_polygon`
    - `src/wasm/area.rs` — 4 exports: `twice_area_ring`, `area_display_from_twice_area`, `areas_conserved_values`, `signed_area_2x_ring`
    - `src/wasm/ring.rs` — 7 exports: `is_ccw_ring`, `ensure_ccw_ring`, `remove_collinear_ring`, `is_simple_ring`, `normalize_polygon_ring`, `rotate_polygon_ring`, `is_convex_ring`
    - `src/wasm/validation.rs` — 4 exports: `validate_edge_lengths_ring`, `perimeter_l1_ring`, `validate_compactness_values`, `validate_part_ring`
    - `src/wasm/spatial.rs` — 5 exports: `point_strictly_inside_convex_ring`, `point_on_polygon_boundary_ring`, `point_inside_or_on_boundary_ring`, `point_inside_any_part`, `contains_polygon`
    - `src/wasm/overlap.rs` — 5 exports: `sat_overlap`, `sat_overlap_with_aabb`, `convex_parts_overlap`, `find_overlapping_parts`, `parts_overlap`
    - `src/wasm/topology.rs` — 5 exports: `has_exact_shared_edge`, `segments_contact`, `classify_contact`, `validate_multipart_topology`, `validate_decomposition`
    - `src/wasm/primitives.rs` — remaining exports: `cross2d`, `orientation`, `is_left`, `is_left_or_on`, `is_right`, `is_right_or_on`, `is_collinear_pts`, `is_reflex`, `edge_squared_length`, `point_on_segment`, `segments_properly_intersect`, `segments_intersect`, `collinear_segments_overlap_area_rings`
    - `src/wasm/optimization.rs` — 2 exports: `optimize_partition`, `merge_convex_pair`
  - Each module uses `use super::helpers::*;` for shared helpers
  - Update `src/wasm/mod.rs` with all submodule declarations
  - Verify WASM output is functionally identical (compare .d.ts)

  **Must NOT do**:
  - Don't rename exports yet (that's Task 23)
  - Don't remove exports yet (that's Task 20)
  - Don't change function logic

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential: 17 → 18 → 19)
  - **Blocks**: Task 19
  - **Blocked By**: Task 17

  **References**:
  - `src/lib.rs:116-684` — ALL `#[wasm_bindgen]` exports to distribute
  - WASM export grouping analysis from research phase

  **Acceptance Criteria**:
  - [ ] `src/wasm/` directory contains 10 files: mod.rs, helpers.rs, types.rs, + 7-9 domain modules
  - [ ] `cargo test` passes
  - [ ] `wasm-pack build --target bundler --release` succeeds
  - [ ] `diff <(sort pkg/exact_poly.d.ts) <(sort pkg/exact_poly.d.ts.baseline)` shows only ordering changes

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: WASM exports identical after split
    Tool: Bash
    Steps:
      1. Run `wasm-pack build --target bundler --release`
      2. Compare new .d.ts with baseline: count of `export function` lines should be identical
      3. Run `cargo test 2>&1 | grep "test result"`
      4. Assert "0 failed"
    Expected Result: Same exports, same types, all tests pass
    Evidence: .sisyphus/evidence/task-18-wasm-split.txt
  ```

  **Commit**: YES (groups with 17, 19)

---

- [x] 19. Clean lib.rs to Module-Only Shell

  **What to do**:
  - Remove ALL remaining `#[wasm_bindgen]` exports from `lib.rs`
  - Remove ALL remaining helper functions (should be in `wasm/helpers.rs` now)
  - Remove ALL remaining WASM-specific types (should be in `wasm/types.rs` now)
  - `lib.rs` should contain ONLY: `pub mod` declarations for all modules + crate-level `//!` doc comment
  - Verify `wc -l src/lib.rs` < 120

  **Must NOT do**:
  - Don't remove domain modules (`pub mod area;`, `pub mod decompose;`, etc.)
  - Don't change module visibility

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential: 17 → 18 → 19)
  - **Blocks**: Tasks 20-23
  - **Blocked By**: Task 18

  **References**:
  - After Task 18, lib.rs should have only module declarations left

  **Acceptance Criteria**:
  - [ ] `wc -l src/lib.rs` < 120
  - [ ] `grep '#\[wasm_bindgen\]' src/lib.rs | wc -l` = 0 (or just the use import)
  - [ ] `cargo test` passes
  - [ ] `wasm-pack build --target bundler --release` succeeds

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: lib.rs is minimal module shell
    Tool: Bash
    Steps:
      1. Run `wc -l src/lib.rs`
      2. Assert < 120
      3. Run `cargo test 2>&1 | grep "test result"`
      4. Assert "0 failed"
    Expected Result: lib.rs is small, all tests pass
    Evidence: .sisyphus/evidence/task-19-lib-clean.txt
  ```

  **Commit**: YES (groups with 17, 18)
  - Message: `refactor(wasm): split lib.rs into domain binding modules`
  - Pre-commit: `cargo test && wasm-pack build --target bundler --release`

---

- [x] 20. Remove Dead Exports + Dead Rust Code

  **What to do**:
  - Remove WASM exports: `add_i64`, `sub_u64`, `sign_i128`, `is_left_turn`, `is_right_turn`, `is_collinear` (the function, not `is_collinear_pts`), `cross_sign`
  - Remove duplicate export: `twice_area` (keep only `twice_area_ring` which will be renamed in Task 23)
  - Evaluate `signed.rs` module: if NO internal callers remain after export removal, mark as candidate for full removal or make functions `pub(crate)` only
  - Clean up unused imports in all modified files

  **Must NOT do**:
  - Don't remove `signed.rs` module if any internal function uses it
  - Don't remove non-dead exports

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 21-23)
  - **Blocks**: Task 24
  - **Blocked By**: Task 19

  **References**:
  - `src/wasm/primitives.rs` (after split) — location of signed arithmetic exports
  - `src/signed.rs` — check for internal callers via `lsp_find_references`
  - `demo/src/wasm.ts:76-81` — demo imports these (will be fixed in Task 24)

  **Acceptance Criteria**:
  - [ ] `grep -c 'pub fn add_i64\|pub fn sub_u64\|pub fn sign_i128\|pub fn is_left_turn\|pub fn is_right_turn\|pub fn cross_sign' src/wasm/*.rs` = 0
  - [ ] `cargo test` passes
  - [ ] `wasm-pack build --target bundler --release` succeeds

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Dead exports removed from WASM
    Tool: Bash
    Steps:
      1. Run `wasm-pack build --target bundler --release`
      2. Run `grep 'add_i64\|sub_u64\|sign_i128\|is_left_turn\|is_right_turn\|cross_sign' pkg/exact_poly.d.ts | wc -l`
      3. Assert = 0
    Expected Result: Removed functions absent from TypeScript definitions
    Evidence: .sisyphus/evidence/task-20-dead-exports.txt
  ```

  **Commit**: YES (groups with 21-23)
  - Message: `refactor(api): polish exports, fix types, bump 0.2.0`

---

- [x] 21. Fix TypeScript Types via `typescript_custom_section`

  **What to do**:
  - Add `#[wasm_bindgen(typescript_custom_section)]` blocks to define TypeScript interfaces for all 14 `any`-typed returns:
    - `DecomposeResult` (parts, steiner_points, strategy, trace)
    - `TopologyError` (tagged union)
    - `ValidationReport` (checks, valid, error_count, etc.)
    - `IndexPair` (a_index, b_index)
    - Ring arrays (bigint[])
    - Parts arrays (bigint[][])
  - Add `#[wasm_bindgen(typescript_type = "...")]` attribute to return types where possible
  - Also fix `any`-typed PARAMETERS (9 functions): `parts_flat: any` should be typed as `bigint[][]`
  - Verify generated `.d.ts` has zero `any` occurrences

  **Must NOT do**:
  - Don't add new crate dependencies (no tsify)
  - Don't change Rust function logic
  - Don't change serialization strategy

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 20, 22, 23)
  - **Blocks**: Task 24
  - **Blocked By**: Task 19

  **References**:
  - `pkg/exact_poly.d.ts` — current generated file showing all `any` occurrences
  - `demo/src/wasm.ts:108-147` — hand-rolled TS interfaces (these should match what we generate)
  - wasm-bindgen docs on `typescript_custom_section` and `typescript_type`

  **Acceptance Criteria**:
  - [ ] `grep 'any' pkg/exact_poly.d.ts | wc -l` = 0
  - [ ] `wasm-pack build --target bundler --release` succeeds
  - [ ] Generated `.d.ts` contains interface definitions for all complex types

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Zero `any` in TypeScript definitions
    Tool: Bash
    Steps:
      1. Run `wasm-pack build --target bundler --release`
      2. Run `grep -c 'any' pkg/exact_poly.d.ts`
      3. Assert = 0
      4. Run `grep -c 'interface' pkg/exact_poly.d.ts`
      5. Assert > 0 (interfaces present)
    Expected Result: No any types, typed interfaces present
    Evidence: .sisyphus/evidence/task-21-typescript-types.txt
  ```

  **Commit**: YES (groups with 20, 22, 23)

---

- [x] 22. Fix ProtocolConfig Defaults + Hardcoded Config + Clean Comments

  **What to do**:
  - Change `src/types.rs`: `impl Default for ProtocolConfig` → return `Self::permissive()` instead of `Self::merca()`
  - Change `src/wasm/helpers.rs` (or wherever `parse_config` lives): `parse_config(None)` → return `ProtocolConfig::permissive()` (matches new default)
  - Fix `src/bayazit.rs:41` — `bayazit_decompose` must accept `&ProtocolConfig` parameter and pass it to `validate_part`. Update caller in `decompose.rs`
  - Remove internal monorepo path references from comments:
    - `src/primitives.rs:5-6` — remove `"Ground truth: deploy/..."` and `"On-chain reference: deploy/..."` lines
    - `src/constants.rs` — clean `"matching polygon.move:31-42"` reference (replace with generic description)
    - Search other files for `deploy/` references
  - Keep algorithmic references (e.g., "Bayazit convex decomposition" is fine — it's a published algorithm)

  **Must NOT do**:
  - Don't change ProtocolConfig fields or methods
  - Don't add new config constructors
  - Don't change validation thresholds

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 20, 21, 23)
  - **Blocks**: Task 24
  - **Blocked By**: Task 19

  **References**:
  - `src/types.rs:50-54` — `impl Default for ProtocolConfig` returning `merca()`
  - `src/bayazit.rs:41` — hardcoded `ProtocolConfig::merca()` in production code
  - `src/primitives.rs:5-6` — internal path references
  - `src/constants.rs:1` — Merca-specific comment

  **Acceptance Criteria**:
  - [ ] `grep 'fn default' src/types.rs` shows `permissive()` not `merca()`
  - [ ] `grep -rn 'deploy/' src/ --include='*.rs' | wc -l` = 0
  - [ ] `cargo test` passes (some tests may need ProtocolConfig::merca() explicitly now)

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Default config is permissive
    Tool: Bash
    Steps:
      1. Run `grep 'Self::permissive' src/types.rs | wc -l`
      2. Assert ≥ 1 (in Default impl)
      3. Run `grep -rn 'deploy/' src/ --include='*.rs' | wc -l`
      4. Assert = 0
      5. Run `cargo test 2>&1 | grep "test result"`
      6. Assert "0 failed"
    Expected Result: Permissive default, no internal refs, tests pass
    Evidence: .sisyphus/evidence/task-22-config-comments.txt
  ```

  **Commit**: YES (groups with 20, 21, 23)

---

- [x] 23. Rename WASM Exports (Drop `_ring` Suffix) + Bump Version

  **What to do**:
  - Rename WASM exports to drop redundant `_ring` suffix (since ALL functions now take rings):
    - `twice_area_ring` → `twice_area`
    - `signed_area_2x_ring` → `signed_area_2x`
    - `is_ccw_ring` → `is_ccw`
    - `ensure_ccw_ring` → `ensure_ccw`
    - `remove_collinear_ring` → `remove_collinear`
    - `is_simple_ring` → `is_simple`
    - `normalize_polygon_ring` → `normalize_polygon`
    - `rotate_polygon_ring` → `rotate_polygon`
    - `is_convex_ring` → `is_convex`
    - `validate_edge_lengths_ring` → `validate_edge_lengths`
    - `perimeter_l1_ring` → `perimeter_l1`
    - `validate_part_ring` → `validate_part`
    - `point_strictly_inside_convex_ring` → `point_strictly_inside_convex`
    - `point_on_polygon_boundary_ring` → `point_on_polygon_boundary`
    - `point_inside_or_on_boundary_ring` → `point_inside_or_on_boundary`
    - `collinear_segments_overlap_area_rings` → `collinear_segments_overlap_area`
  - Bump `Cargo.toml` version to `"0.2.0"`
  - Bump `package.json` version to `"0.2.0"`
  - Rebuild WASM to generate new `.d.ts`

  **Must NOT do**:
  - Don't rename internal Rust function names — only `#[wasm_bindgen]` export names
  - Don't change function logic

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 20-22)
  - **Blocks**: Task 24
  - **Blocked By**: Task 19

  **References**:
  - All `src/wasm/*.rs` files — `#[wasm_bindgen]` function names
  - `Cargo.toml:3` — version field
  - `package.json:3` — version field

  **Acceptance Criteria**:
  - [ ] `grep '_ring' pkg/exact_poly.d.ts | wc -l` = 0
  - [ ] `grep 'version.*0.2.0' Cargo.toml | wc -l` = 1
  - [ ] `wasm-pack build --target bundler --release` succeeds

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Exports renamed, version bumped
    Tool: Bash
    Steps:
      1. Run `wasm-pack build --target bundler --release`
      2. Run `grep '_ring' pkg/exact_poly.d.ts | wc -l`
      3. Assert = 0
      4. Run `grep 'version' Cargo.toml | head -1`
      5. Assert contains "0.2.0"
    Expected Result: No _ring suffixes, version 0.2.0
    Evidence: .sisyphus/evidence/task-23-rename-version.txt
  ```

  **Commit**: YES (groups with 20-22)
  - Message: `refactor(api): polish exports, fix types, bump 0.2.0`
  - Pre-commit: `cargo test && wasm-pack build --target bundler --release`

---

- [x] 24. Update Demo for 0.2.0 API Changes

  **What to do**:
  - Update `demo/src/wasm.ts`:
    - Remove imports of deleted exports: `cross_sign`, `sub_u64`, `sign_i128`, `is_left_turn`, `is_right_turn`, `is_collinear`
    - Rename all `_ring` imports to new names (e.g., `is_ccw_ring` → `is_ccw`)
    - Remove `twice_area` import (now only `twice_area` exists, was `twice_area_ring`)
    - Update hand-rolled TypeScript types (DecomposeResult, ValidationReport, TopologyError) to match generated ones, or remove them if `.d.ts` now exports proper types
  - Update ALL demo tab files (`demo/src/tabs/*.ts`) that reference renamed/removed functions
  - Update `demo/src/canvas/*.ts` if needed
  - Rebuild demo: `cd demo && npm run build`
  - Verify no TypeScript errors

  **Must NOT do**:
  - Don't add new demo features
  - Don't restyle the demo
  - Don't change demo functionality — only adapt to new API names

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 5 (solo)
  - **Blocks**: F1-F4
  - **Blocked By**: Tasks 20-23

  **References**:
  - `demo/src/wasm.ts:5-85` — ALL imports from "exact-poly"
  - `demo/src/wasm.ts:108-147` — hand-rolled TypeScript interfaces
  - `demo/src/tabs/*.ts` — 7 tab files that call WASM functions
  - `demo/src/canvas/*.ts` — canvas rendering files
  - `demo/src/config.ts` — ProtocolConfig usage (may need `merca` preset explicitly since default changed)

  **Acceptance Criteria**:
  - [ ] `cd demo && npm run build` succeeds (TypeScript compiles, Vite builds)
  - [ ] No references to old function names in demo source
  - [ ] Demo config still works (merca preset explicitly set where needed)

  **QA Scenarios (MANDATORY):**
  ```
  Scenario: Demo builds with new API
    Tool: Bash
    Steps:
      1. Run `cd demo && npm install && npm run build 2>&1 | tail -5`
      2. Assert output contains "built in"
      3. Run `grep -rn '_ring\|add_i64\|sub_u64\|sign_i128\|is_left_turn\|is_right_turn\|cross_sign' demo/src/ --include='*.ts' | wc -l`
      4. Assert = 0
    Expected Result: Demo builds clean, no old API references
    Evidence: .sisyphus/evidence/task-24-demo-update.txt

  Scenario: Demo has no TypeScript errors
    Tool: Bash
    Steps:
      1. Run `cd demo && npx tsc --noEmit 2>&1 | tail -5`
      2. Assert no errors
    Expected Result: Zero TypeScript errors
    Evidence: .sisyphus/evidence/task-24-demo-tsc.txt
  ```

  **Commit**: YES
  - Message: `fix(demo): update for 0.2.0 API changes`
  - Pre-commit: `cd demo && npm run build`

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo test`, `cargo clippy`, `wasm-pack build --target bundler --release`. Review all changed files for: `as any`, `split_xy` remnants, `(xs, ys)` signatures in non-test code, dead imports. Check that `pkg/exact_poly.d.ts` has zero `any`.
  Output: `Build [PASS/FAIL] | Tests [N pass/N fail] | Clippy [PASS/FAIL] | any-types [0/N] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state. Build WASM (`wasm-pack build`), install demo deps (`cd demo && npm install`), build demo (`npm run build`), verify demo TypeScript compiles. Run `cargo test --release`. Save evidence.
  Output: `WASM [PASS/FAIL] | Demo [PASS/FAIL] | Tests [N/N pass] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", check actual changes. Verify: all (xs,ys) signatures migrated, lib.rs is module-shell only, dead exports removed, ProtocolConfig default changed, version bumped to 0.2.0. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| Wave | Commit Message | Files |
|------|---------------|-------|
| 1 | `test: add safety-net tests for representation migration` | `src/*.rs` (test blocks only) |
| 2A | `refactor(area,validation,aabb): migrate to ring representation` | `src/area.rs`, `src/validation.rs`, `src/aabb.rs` |
| 2B | `refactor(sat,spatial,shared_edge): migrate to ring representation` | `src/sat.rs`, `src/spatial.rs`, `src/shared_edge.rs` |
| 2C | `refactor(overlap,topology,pipeline): complete ring migration, remove split_xy` | `src/overlap.rs`, `src/topology.rs`, `src/containment.rs`, `src/validate_onchain.rs`, `src/decompose.rs`, `src/bayazit.rs`, `src/exact_partition.rs`, `src/hertel_mehlhorn.rs`, `src/lib.rs`, `src/types.rs` |
| 3 | `refactor(wasm): split lib.rs into domain binding modules` | `src/lib.rs`, `src/wasm/*.rs` |
| 4 | `refactor(api): polish exports, fix types, bump 0.2.0` | `src/wasm/*.rs`, `src/types.rs`, `Cargo.toml`, `package.json` |
| 5 | `fix(demo): update for 0.2.0 API changes` | `demo/src/*.ts` |

---

## Success Criteria

### Verification Commands
```bash
cargo test                                    # Expected: ok. N passed (N≥222), 0 failed
wasm-pack build --target bundler --release    # Expected: success
cd demo && npm run build                      # Expected: success
grep -rn 'xs: &\[i64\].*ys: &\[i64\]' src/ --include='*.rs' | grep -v test | grep -v '//' | wc -l  # Expected: 0
grep 'any' pkg/exact_poly.d.ts | wc -l       # Expected: 0
grep 'split_xy' src/lib.rs | wc -l           # Expected: 0
wc -l src/lib.rs                              # Expected: < 120
grep 'version' Cargo.toml | head -1          # Expected: version = "0.2.0"
grep 'pub struct Point\b' src/types.rs | wc -l  # Expected: 0
grep 'pub struct Part\b' src/types.rs | wc -l   # Expected: 0
grep 'fn default' src/types.rs               # Expected: Self::permissive()
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] All tests pass (≥222)
- [ ] WASM builds clean
- [ ] Demo compiles and runs
- [ ] Zero `any` in TypeScript definitions
- [ ] Zero `(xs, ys)` signatures in production code
- [ ] Version is 0.2.0
