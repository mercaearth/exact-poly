import type { Tab } from "../main";
import {
  createRenderCtx,
  fitToPolygons,
  clear,
  drawGrid,
  drawPolygon,
  drawParts,
  drawPoint,
  drawLabel,
  partColor,
  type RenderCtx,
} from "../canvas/renderer";
import { createDrawingTool, type DrawingTool } from "../canvas/drawing";
import { PRESETS, INVALID_PRESETS } from "../canvas/presets";
import {
  decompose_polygon,
  bayazit_decompose_polygon,
  exact_vertex_partition_polygon,
  ear_clip_triangulate_polygon,
  optimize_partition,
  toFlat,
  fromFlat,
  twice_area_ring,
  is_ccw_ring,
  ensure_ccw_ring,
  is_simple_ring,
  is_convex_ring,
  validate_part_ring,
  perimeter_l1_ring,
  validate_compactness_values,
  validate_edge_lengths_ring,
  remove_collinear_ring,
  validate_decomposition,
  type DecomposeResult,
  type DecomposeTraceEntry,
  type ValidationReport,
} from "../wasm";
import { getPolygon, setPolygon, onPolygonChange } from "../state";
import { getConfigForWasm, getConfig, onConfigChange } from "../config";

type Algorithm = "decompose" | "bayazit" | "exact_partition" | "ear_clip" | "hertel_mehlhorn";

function formatStrategy(strategy: string | { Rotation: { offset: number; inner: string | object } } | undefined): string {
  if (!strategy) return "—";
  if (typeof strategy === "string") return strategy;
  if ("Rotation" in strategy) {
    const inner = typeof strategy.Rotation.inner === "string"
      ? strategy.Rotation.inner
      : JSON.stringify(strategy.Rotation.inner);
    return `Rotation(${strategy.Rotation.offset}, ${inner})`;
  }
  return JSON.stringify(strategy);
}

function outcomeClass(outcome: DecomposeTraceEntry["outcome"]): string {
  if ("Success" in outcome) return "ok";
  if ("TooManyParts" in outcome) return "warn";
  return "error";
}

function formatOutcome(outcome: DecomposeTraceEntry["outcome"]): string {
  if ("Success" in outcome) return `Success: ${outcome.Success.part_count} parts`;
  if ("TooManyParts" in outcome) return `TooManyParts: ${outcome.TooManyParts.count}`;
  if ("ValidationFailed" in outcome) return `ValidationFailed: ${outcome.ValidationFailed.errors.join(", ")}`;
  if ("AlgorithmFailed" in outcome) return `AlgorithmFailed: "${outcome.AlgorithmFailed.error}"`;
  return JSON.stringify(outcome);
}

export function createDecompositionTab(): Tab {
  let canvas: HTMLCanvasElement;
  let rc: RenderCtx;
  let drawingTool: DrawingTool;
  let polygon: [number, number][] = [];
  let parts: [number, number][][] = [];
  let rawTriangles: [number, number][][] = [];
  let steinerPoints: [number, number][] = [];
  let strategy: DecomposeResult["strategy"];
  let trace: DecomposeTraceEntry[] | undefined;
  /** True if the last runDecompose() flipped the drawn polygon to CCW
   *  before handing it to the algorithms. Surfaced in the debug panel so
   *  the user can see when their winding was auto-corrected. */
  let wasNormalizedToCcw = false;
  let algorithm: Algorithm = "decompose";
  let allowSteiner = true;
  let collectTrace = false;
  let minimizeParts = false;
  let infoPanel: HTMLElement;
  let errorEl: HTMLElement;
  let tracePanel: HTMLElement;
  let debugPanel: HTMLElement = null as unknown as HTMLElement;
  let showDebug = true;
  let showVertexLabels = true;
  let unsubscribe: (() => void) | null = null;
  let unsubConfig: (() => void) | null = null;

  const SCALE = 1_000_000;

  function analyzeRing(pts: [number, number][]): Record<string, string> {
    if (pts.length < 3) return {};
    const flat = toFlat(pts);
    const info: Record<string, string> = {};
    try { info["2x area"] = twice_area_ring(flat); } catch {}
    try { info["CCW"] = String(is_ccw_ring(flat)); } catch {}
    try { info["Simple"] = String(is_simple_ring(flat)); } catch {}
    try { info["Convex"] = String(is_convex_ring(flat)); } catch {}
    try { info["Perimeter L1"] = perimeter_l1_ring(flat); } catch {}
    try {
      const edgeErr = validate_edge_lengths_ring(flat, getConfigForWasm());
      info["Edge lengths"] = edgeErr ?? "OK";
    } catch {}
    try {
      const area = twice_area_ring(flat);
      const perim = perimeter_l1_ring(flat);
      const compErr = validate_compactness_values(area, perim, getConfigForWasm());
      // Compactness is a boundary property. When this ring IS the polygon
      // boundary (whole polygon, or a single-part polygon), the result matches
      // on-chain. When the ring is an individual part of a multipart polygon
      // this row is informational only — on-chain evaluates compactness on the
      // union boundary, not on individual parts.
      info["Compactness (boundary)"] = compErr ?? "OK";
    } catch {}
    try {
      const partErr = validate_part_ring(flat, getConfigForWasm());
      info["Validate part (structural)"] = partErr ?? "OK";
    } catch {}
    try {
      const cleaned = fromFlat(remove_collinear_ring(flat) as bigint[]);
      if (cleaned.length < pts.length) {
        info["Collinear removed"] = `${pts.length} → ${cleaned.length}`;
      }
    } catch {}
    return info;
  }

  function coordsJson(pts: [number, number][]): string {
    return JSON.stringify(pts.map(([x, y]) => [x, y]));
  }

  function wasmFlatStr(pts: [number, number][]): string {
    return "[" + pts.map(([x, y]) =>
      `${Math.round(x * SCALE)}, ${Math.round(y * SCALE)}`
    ).join(", ") + "]";
  }

  function edgesInfo(pts: [number, number][]): string[] {
    const edges: string[] = [];
    for (let i = 0; i < pts.length; i++) {
      const j = (i + 1) % pts.length;
      const dx = pts[j][0] - pts[i][0];
      const dy = pts[j][1] - pts[i][1];
      const lenSq = dx * dx + dy * dy;
      const len = Math.sqrt(lenSq);
      const wasmLenSq = BigInt(Math.round(dx * SCALE)) ** 2n + BigInt(Math.round(dy * SCALE)) ** 2n;
      edges.push(`  ${i}→${j}: len=${len.toFixed(2)} lenSq_wasm=${wasmLenSq}`);
    }
    return edges;
  }

  function updateDebug() {
    if (!showDebug) {
      debugPanel.style.display = "none";
      return;
    }
    debugPanel.style.display = "block";

    if (polygon.length < 3) {
      debugPanel.innerHTML = `<details class="debug-details" open>
        <summary>Debug</summary>
        <div class="debug-content"><span class="debug-muted">No polygon</span></div>
      </details>`;
      return;
    }

    const cfg = getConfig();
    const sections: string[] = [];

    // Polygon coordinates
    sections.push(`<div class="debug-section">
      <div class="debug-section-title">Polygon (${polygon.length} vertices)</div>
      <div class="debug-copy-row">
        <span class="debug-label">Coords:</span>
        <code class="debug-code" title="Click to copy">${escHtml(coordsJson(polygon))}</code>
      </div>
      <div class="debug-copy-row">
        <span class="debug-label">WASM flat:</span>
        <code class="debug-code" title="Click to copy">${escHtml(wasmFlatStr(polygon))}</code>
      </div>
      <div class="debug-copy-row">
        <span class="debug-label">Vertices:</span>
        <code class="debug-code">${polygon.map(([x, y], i) => `${i}: (${x}, ${y})`).join("\n")}</code>
      </div>
    </div>`);

    // Polygon analysis
    const polyInfo = analyzeRing(polygon);
    if (wasNormalizedToCcw) {
      polyInfo["Normalized to CCW"] = "yes (input was CW — flipped before decompose)";
    }
    if (Object.keys(polyInfo).length > 0) {
      const rows = Object.entries(polyInfo).map(([k, v]) => {
        const cls = v === "OK" ? "debug-ok" : (v.startsWith("OK") ? "debug-ok" : (v === "true" ? "" : (v === "false" ? "debug-warn" : "")));
        return `<div class="debug-row"><span class="debug-label">${k}</span><span class="debug-value ${cls}">${escHtml(v)}</span></div>`;
      }).join("");
      sections.push(`<div class="debug-section">
        <div class="debug-section-title">Polygon Properties</div>
        ${rows}
      </div>`);
    }

    // Edges
    const edges = edgesInfo(polygon);
    sections.push(`<div class="debug-section">
      <div class="debug-section-title">Edges</div>
      <code class="debug-code">${edges.join("\n")}</code>
    </div>`);

    // Config summary
    sections.push(`<div class="debug-section">
      <div class="debug-section-title">Config</div>
      <code class="debug-code">${escHtml(JSON.stringify(cfg, null, 2))}</code>
    </div>`);

    // On-chain validation
    if (parts.length > 0) {
      try {
        const flat = toFlat(polygon);
        const partsFlat = parts.map(p => Array.from(toFlat(p)));
        const report = validate_decomposition(flat, partsFlat, getConfigForWasm()) as ValidationReport;

        const statusLabel = !report.valid
          ? "FAIL — will reject on-chain"
          : report.warn_count > 0
            ? "PASS (demo only) — has warnings, will fail on-chain with real coords"
            : "PASS — valid for on-chain";
        const statusCls = !report.valid
          ? "debug-fail"
          : report.warn_count > 0
            ? "debug-warn"
            : "debug-ok";
        const checkRows = report.checks.map(c => {
          let cls: string;
          let icon: string;
          if (c.severity === "error" && !c.passed) {
            cls = "debug-fail"; icon = "FAIL";
          } else if (c.severity === "warn") {
            cls = "debug-warn"; icon = "WARN";
          } else {
            cls = "debug-ok"; icon = "OK";
          }
          return `<div class="debug-row">
            <span class="debug-label"><span class="debug-value ${cls}">[${icon}]</span> ${c.name}</span>
            <span class="debug-value ${cls}">${escHtml(c.detail)}</span>
          </div>`;
        }).join("");

        sections.push(`<div class="debug-section">
          <div class="debug-section-title">On-chain Validation</div>
          <div class="debug-row">
            <span class="debug-label">Status</span>
            <span class="debug-value ${statusCls}">${statusLabel}</span>
          </div>
          <div class="debug-row">
            <span class="debug-label">Errors / Warnings</span>
            <span class="debug-value">${report.error_count} / ${report.warn_count}</span>
          </div>
          <div class="debug-row">
            <span class="debug-label">Original 2A</span>
            <span class="debug-value">${report.original_twice_area}</span>
          </div>
          <div class="debug-row">
            <span class="debug-label">Parts sum 2A</span>
            <span class="debug-value">${report.parts_twice_area_sum}</span>
          </div>
          ${checkRows}
        </div>`);
      } catch (e) {
        sections.push(`<div class="debug-section">
          <div class="debug-section-title">On-chain Validation</div>
          <div class="debug-value debug-warn">${escHtml(String(e))}</div>
        </div>`);
      }
    }

    // Parts details
    if (parts.length > 0) {
      const partSections = parts.map((part, i) => {
        const partInfo = analyzeRing(part);
        const rows = Object.entries(partInfo).map(([k, v]) => {
          const cls = v === "OK" ? "debug-ok" : (v.includes("fail") || v.includes("error") || v === "false" ? "debug-warn" : "");
          return `<div class="debug-row"><span class="debug-label">${k}</span><span class="debug-value ${cls}">${escHtml(v)}</span></div>`;
        }).join("");
        return `<div class="debug-part">
          <div class="debug-section-title">Part ${i} (${part.length} verts)</div>
          <div class="debug-copy-row">
            <code class="debug-code" title="Click to copy">${escHtml(coordsJson(part))}</code>
          </div>
          <div class="debug-copy-row">
            <span class="debug-label">WASM flat:</span>
            <code class="debug-code" title="Click to copy">${escHtml(wasmFlatStr(part))}</code>
          </div>
          ${rows}
        </div>`;
      }).join("");
      sections.push(`<div class="debug-section">
        <div class="debug-section-title">Parts Detail</div>
        ${partSections}
      </div>`);
    }

    debugPanel.innerHTML = `<details class="debug-details" open>
      <summary>Debug</summary>
      <div class="debug-content">${sections.join("")}</div>
    </details>`;

    // Copy on click
    debugPanel.querySelectorAll(".debug-code").forEach(el => {
      (el as HTMLElement).style.cursor = "pointer";
      el.addEventListener("click", () => {
        navigator.clipboard.writeText(el.textContent || "");
        const orig = (el as HTMLElement).style.outline;
        (el as HTMLElement).style.outline = "1px solid #51cf66";
        setTimeout(() => (el as HTMLElement).style.outline = orig, 300);
      });
    });
  }

  function escHtml(s: string): string {
    return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  }

  function runDecompose() {
    parts = [];
    rawTriangles = [];
    steinerPoints = [];
    strategy = undefined;
    trace = undefined;
    wasNormalizedToCcw = false;
    errorEl.textContent = "";

    if (polygon.length < 3) return;

    try {
      // All decomposition algorithms in exact-poly assume CCW winding.
      // On-chain polygons (u64 coords) are constructed CCW by the client,
      // but the demo lets you draw freely and the screen→world y-flip plus
      // mouse order can easily yield a CW ring. Rather than fail with
      // "area not conserved" or a silently wrong decomposition, normalize
      // here and record that we did so for the debug panel.
      //
      // Note on BigInt conversion: ensure_ccw_ring goes through
      // serde_wasm_bindgen which encodes Vec<i64> as a regular JS array of
      // Numbers, not BigInts. BigInt64Array.from() invokes spec ToBigInt on
      // each element and ToBigInt(Number) throws TypeError, so we have to
      // map through BigInt() explicitly (the `typeof` guard is for the
      // future in case serde_wasm_bindgen gets configured to emit bigints).
      const rawFlat = toFlat(polygon);
      const ccwOriginally = is_ccw_ring(rawFlat);
      wasNormalizedToCcw = !ccwOriginally;
      const flat = wasNormalizedToCcw
        ? BigInt64Array.from(
            ensure_ccw_ring(rawFlat) as Array<number | bigint>,
            (v) => (typeof v === "bigint" ? v : BigInt(v)),
          )
        : rawFlat;

      if (algorithm === "decompose") {
        const result = decompose_polygon(
          flat,
          allowSteiner,
          collectTrace || undefined,
          minimizeParts || undefined,
          getConfigForWasm(),
        ) as DecomposeResult;
        parts = result.parts.map((p) => fromFlat(p));
        steinerPoints = fromFlat(result.steiner_points);
        strategy = result.strategy;
        trace = result.trace;
      } else if (algorithm === "bayazit") {
        const result = bayazit_decompose_polygon(flat, allowSteiner) as bigint[][];
        parts = result.map((p) => fromFlat(p));
      } else if (algorithm === "exact_partition") {
        const result = exact_vertex_partition_polygon(flat) as bigint[][];
        parts = result.map((p) => fromFlat(p));
      } else if (algorithm === "ear_clip") {
        const result = ear_clip_triangulate_polygon(flat) as bigint[][];
        parts = result.map((p) => fromFlat(p));
      } else if (algorithm === "hertel_mehlhorn") {
        const triangles = ear_clip_triangulate_polygon(flat) as bigint[][];
        rawTriangles = triangles.map((p) => fromFlat(p));

        const triFlat = triangles.map((t) => Array.from(t));
        const optimized = optimize_partition(triFlat) as bigint[][];
        parts = optimized.map((p) => fromFlat(p));
      }
    } catch (e) {
      errorEl.textContent = String(e);
    }

    updateInfo();
    updateTrace();
    updateDebug();
    render();
  }

  function updateInfo() {
    const rows: string[] = [];
    rows.push(infoRow("Vertices", String(polygon.length)));
    rows.push(infoRow("Algorithm", algorithm));
    if (strategy) {
      rows.push(infoRow("Strategy", formatStrategy(strategy)));
    }
    rows.push(infoRow("Parts", String(parts.length)));
    if (algorithm === "hertel_mehlhorn" && rawTriangles.length > 0) {
      rows.push(infoRow("Triangles", String(rawTriangles.length)));
      rows.push(infoRow("Optimized", `${rawTriangles.length} → ${parts.length}`));
    }
    if (steinerPoints.length > 0) {
      rows.push(infoRow("Steiner pts", String(steinerPoints.length)));
    }
    for (let i = 0; i < parts.length; i++) {
      rows.push(infoRow(`Part ${i}`, `${parts[i].length} verts`));
    }
    infoPanel.innerHTML = `<h3>Result</h3>${rows.join("")}`;
  }

  function updateTrace() {
    if (!trace || trace.length === 0) {
      tracePanel.innerHTML = "";
      tracePanel.style.display = "none";
      return;
    }

    tracePanel.style.display = "block";
    const entries = trace.map((entry, i) => {
      const cls = outcomeClass(entry.outcome);
      return `<div class="trace-entry">
        <span class="trace-index">#${i}</span>
        <span class="trace-strategy">${formatStrategy(entry.strategy)}</span>
        <span class="trace-rotation">rot=${entry.rotation}</span>
        <span class="info-value ${cls}">${formatOutcome(entry.outcome)}</span>
      </div>`;
    }).join("");

    tracePanel.innerHTML = `
      <details class="trace-details">
        <summary>Trace (${trace.length} attempts)</summary>
        <div class="trace-list">${entries}</div>
      </details>
    `;
  }

  function infoRow(label: string, value: string, cls = ""): string {
    return `<div class="info-row"><span class="info-label">${label}</span><span class="info-value ${cls}">${value}</span></div>`;
  }

  function render() {
    rc = createRenderCtx(canvas);

    const allPolygons = parts.length > 0
      ? [...parts, ...(rawTriangles.length > 0 ? rawTriangles : [])]
      : polygon.length > 0 ? [polygon] : [];
    if (allPolygons.length > 0) fitToPolygons(rc, allPolygons);

    clear(rc);
    drawGrid(rc);

    if (parts.length > 0) {
      drawPolygon(rc, polygon, {
        stroke: "#333",
        showVertices: false,
        lineWidth: 1,
        dashed: true,
      });

      if (algorithm === "hertel_mehlhorn" && rawTriangles.length > 0) {
        for (const tri of rawTriangles) {
          drawPolygon(rc, tri, {
            stroke: "#444",
            lineWidth: 1,
            dashed: true,
            showVertices: false,
          });
        }
      }

      drawParts(rc, parts);
    } else if (polygon.length > 0) {
      drawPolygon(rc, polygon);
    }

    // Vertex index labels on source polygon
    if (showVertexLabels && polygon.length >= 3) {
      for (let i = 0; i < polygon.length; i++) {
        const [x, y] = polygon[i];
        drawLabel(rc, x, y, `v${i}`, "#888", [10, -10]);
      }
    }

    // Part vertex labels
    if (showVertexLabels && parts.length > 0) {
      for (let pi = 0; pi < parts.length; pi++) {
        const part = parts[pi];
        for (let vi = 0; vi < part.length; vi++) {
          const [x, y] = part[vi];
          drawLabel(rc, x, y, `p${pi}.${vi}`, partColor(pi), [10, 4 + pi * 12]);
        }
      }
    }

    for (const [x, y] of steinerPoints) {
      drawPoint(rc, x, y, "#ff6b6b", 6);
      drawLabel(rc, x, y, "S", "#ff6b6b");
    }

    if (drawingTool.isDrawing && drawingTool.points.length > 0) {
      drawPolygon(rc, drawingTool.points, {
        stroke: "#666",
        closed: false,
        dashed: true,
      });
    }
  }

  return {
    id: "decomposition",
    label: "Decomposition",

    create() {
      const el = document.createElement("div");
      el.id = "tab-decomposition";
      el.innerHTML = `
        <div class="toolbar">
          <select id="decomp-preset">
            <option value="">— Preset —</option>
            ${PRESETS.map((p) => `<option value="${p.name}">${p.name}</option>`).join("")}
            <optgroup label="Invalid on-chain">
              ${INVALID_PRESETS.map((p) => `<option value="${p.name}">${p.name}</option>`).join("")}
            </optgroup>
          </select>
          <div class="sep"></div>
          <select id="decomp-algo">
            <option value="decompose">Cascade</option>
            <option value="bayazit">Bayazit</option>
            <option value="exact_partition">Exact Vertex Partition</option>
            <option value="ear_clip">Ear Clip</option>
            <option value="hertel_mehlhorn">Hertel-Mehlhorn</option>
          </select>
          <div class="checkbox-row"><input type="checkbox" id="decomp-steiner" checked /><label for="decomp-steiner">Steiner</label></div>
          <div class="checkbox-row"><input type="checkbox" id="decomp-trace" /><label for="decomp-trace">Trace</label></div>
          <div class="checkbox-row"><input type="checkbox" id="decomp-minimize" /><label for="decomp-minimize" title="Best-of all strategies">Minimize</label></div>
          <div class="sep"></div>
          <button class="btn btn-primary" id="decomp-run">Decompose</button>
          <button class="btn btn-danger" id="decomp-clear">Clear</button>
          <div class="sep"></div>
          <div class="checkbox-row"><input type="checkbox" id="decomp-debug" checked /><label for="decomp-debug">Debug</label></div>
          <div class="checkbox-row"><input type="checkbox" id="decomp-vertex-labels" checked /><label for="decomp-vertex-labels">Labels</label></div>
          <span class="status-text">Click to draw. Right-click to close.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="decomp-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="decomp-info">
              <h3>Result</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="decomp-trace-panel"></div>
            <div id="decomp-error" style="color:#ff4a4a;font-size:12px;"></div>
            <div id="decomp-debug-panel"></div>
          </div>
        </div>
      `;
      return el;
    },

    activate() {
      canvas = document.getElementById("decomp-canvas") as HTMLCanvasElement;
      infoPanel = document.getElementById("decomp-info")!;
      errorEl = document.getElementById("decomp-error")!;
      tracePanel = document.getElementById("decomp-trace-panel")!;
      debugPanel = document.getElementById("decomp-debug-panel")!;

      rc = createRenderCtx(canvas);
      drawingTool = createDrawingTool(canvas, () => rc);

      polygon = getPolygon();
      unsubscribe = onPolygonChange(() => {
        polygon = getPolygon();
        parts = [];
        rawTriangles = [];
        steinerPoints = [];
        if (polygon.length >= 3) runDecompose();
        else { updateDebug(); render(); }
      });

      drawingTool.setOnChange(() => render());
      drawingTool.setOnComplete((pts) => {
        polygon = pts;
        drawingTool.clear();
        setPolygon(polygon);
      });
      drawingTool.enable();

      document.getElementById("decomp-preset")!.addEventListener("change", (e) => {
        const name = (e.target as HTMLSelectElement).value;
        const preset = PRESETS.find((p) => p.name === name) ?? INVALID_PRESETS.find((p) => p.name === name);
        if (preset) {
          drawingTool.clear();
          setPolygon(preset.points.slice());
        }
      });

      document.getElementById("decomp-clear")!.addEventListener("click", () => {
        parts = [];
        rawTriangles = [];
        steinerPoints = [];
        drawingTool.clear();
        errorEl.textContent = "";
        tracePanel.innerHTML = "";
        tracePanel.style.display = "none";
        (document.getElementById("decomp-preset") as HTMLSelectElement).value = "";
        setPolygon([]);
      });

      const minimizeInput = document.getElementById("decomp-minimize") as HTMLInputElement;
      const syncMinimizeEnabled = () => {
        minimizeInput.disabled = algorithm !== "decompose";
        minimizeInput.parentElement?.classList.toggle("disabled", minimizeInput.disabled);
      };
      syncMinimizeEnabled();

      document.getElementById("decomp-algo")!.addEventListener("change", (e) => {
        algorithm = (e.target as HTMLSelectElement).value as Algorithm;
        syncMinimizeEnabled();
        if (polygon.length >= 3) runDecompose();
      });

      document.getElementById("decomp-steiner")!.addEventListener("change", (e) => {
        allowSteiner = (e.target as HTMLInputElement).checked;
        if (polygon.length >= 3) runDecompose();
      });

      document.getElementById("decomp-trace")!.addEventListener("change", (e) => {
        collectTrace = (e.target as HTMLInputElement).checked;
        if (polygon.length >= 3) runDecompose();
      });

      document.getElementById("decomp-minimize")!.addEventListener("change", (e) => {
        minimizeParts = (e.target as HTMLInputElement).checked;
        if (polygon.length >= 3) runDecompose();
      });

      document.getElementById("decomp-run")!.addEventListener("click", runDecompose);

      document.getElementById("decomp-debug")!.addEventListener("change", (e) => {
        showDebug = (e.target as HTMLInputElement).checked;
        updateDebug();
      });

      document.getElementById("decomp-vertex-labels")!.addEventListener("change", (e) => {
        showVertexLabels = (e.target as HTMLInputElement).checked;
        render();
      });

      unsubConfig = onConfigChange(() => {
        if (polygon.length >= 3) runDecompose();
      });

      if (polygon.length >= 3) runDecompose();
      else { updateDebug(); render(); }
    },

    deactivate() {
      drawingTool?.disable();
      unsubscribe?.();
      unsubscribe = null;
      unsubConfig?.();
      unsubConfig = null;
    },
  };
}
