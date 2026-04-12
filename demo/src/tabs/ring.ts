import type { Tab } from "../main";
import {
  createRenderCtx,
  fitToPolygons,
  clear,
  drawGrid,
  drawPolygon,
  drawPoint,
  drawLabel,
  drawArrowHead,
  type RenderCtx,
} from "../canvas/renderer";
import { createDrawingTool, type DrawingTool } from "../canvas/drawing";
import { PRESETS } from "../canvas/presets";
import { getPolygon, setPolygon, onPolygonChange } from "../state";
import {
  is_ccw_ring,
  ensure_ccw_ring,
  remove_collinear_ring,
  is_simple_ring,
  is_convex_ring,
  normalize_polygon_ring,
  is_reflex,
  toFlat,
  fromFlat,
} from "../wasm";

export function createRingTab(): Tab {
  let canvas: HTMLCanvasElement;
  let rc: RenderCtx;
  let drawingTool: DrawingTool;
  let polygon: [number, number][] = [];
  let infoPanel: HTMLElement;
  let errorEl: HTMLElement;
  let unsubscribe: (() => void) | null = null;

  function analyze() {
    errorEl.textContent = "";
    if (polygon.length < 3) {
      updateInfo(null);
      render();
      return;
    }

    try {
      const flat = toFlat(polygon);
      const isCcw = is_ccw_ring(flat);
      const isSimple = is_simple_ring(flat);
      const isConvex = is_convex_ring(flat);

      const reflexVertices: number[] = [];
      for (let i = 0; i < polygon.length; i++) {
        const prev = polygon[(i - 1 + polygon.length) % polygon.length];
        const curr = polygon[i];
        const next = polygon[(i + 1) % polygon.length];
        if (is_reflex(
          BigInt(prev[0]), BigInt(prev[1]),
          BigInt(curr[0]), BigInt(curr[1]),
          BigInt(next[0]), BigInt(next[1])
        )) {
          reflexVertices.push(i);
        }
      }

      updateInfo({ isCcw, isSimple, isConvex, reflexCount: reflexVertices.length, reflexVertices });
      render(reflexVertices);
    } catch (e) {
      errorEl.textContent = String(e);
      render();
    }
  }

  interface RingData {
    isCcw: boolean;
    isSimple: boolean;
    isConvex: boolean;
    reflexCount: number;
    reflexVertices: number[];
  }

  function updateInfo(data: RingData | null) {
    if (!data) {
      infoPanel.innerHTML = `<h3>Properties</h3><div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>`;
      return;
    }

    const rows: string[] = [];
    rows.push(row("Vertices", String(polygon.length)));
    rows.push(row("CCW", data.isCcw ? "Yes" : "No", data.isCcw ? "ok" : "warn"));
    rows.push(row("Simple", data.isSimple ? "Yes" : "No", data.isSimple ? "ok" : "error"));
    rows.push(row("Convex", data.isConvex ? "Yes" : "No", data.isConvex ? "ok" : "warn"));
    rows.push(row("Reflex vertices", String(data.reflexCount), data.reflexCount > 0 ? "warn" : "ok"));

    infoPanel.innerHTML = `<h3>Properties</h3>${rows.join("")}`;
  }

  function row(label: string, value: string, cls = ""): string {
    return `<div class="info-row"><span class="info-label">${label}</span><span class="info-value ${cls}">${value}</span></div>`;
  }

  function render(reflexVertices: number[] = []) {
    rc = createRenderCtx(canvas);
    if (polygon.length > 0) fitToPolygons(rc, [polygon]);

    clear(rc);
    drawGrid(rc);

    if (polygon.length < 2) {
      if (drawingTool.isDrawing && drawingTool.points.length > 0) {
        drawPolygon(rc, drawingTool.points, { stroke: "#666", closed: false, dashed: true });
      }
      return;
    }

    drawPolygon(rc, polygon, { showVertices: false });

    for (let i = 0; i < polygon.length; i++) {
      const next = (i + 1) % polygon.length;
      drawArrowHead(rc, polygon[i], polygon[next]);
    }

    const reflexSet = new Set(reflexVertices);
    for (let i = 0; i < polygon.length; i++) {
      const isReflex = reflexSet.has(i);
      const color = isReflex ? "#ff6b6b" : "#4a9eff";
      drawPoint(rc, polygon[i][0], polygon[i][1], color, isReflex ? 6 : 4);
      drawLabel(rc, polygon[i][0], polygon[i][1], String(i), isReflex ? "#ff6b6b" : "#888");
    }

    if (drawingTool.isDrawing && drawingTool.points.length > 0) {
      drawPolygon(rc, drawingTool.points, { stroke: "#666", closed: false, dashed: true });
    }
  }

  function applyOp(op: string) {
    if (polygon.length < 3) return;
    errorEl.textContent = "";

    try {
      const flat = toFlat(polygon);
      let result: bigint[] | null = null;

      if (op === "ccw") {
        result = ensure_ccw_ring(flat) as bigint[];
      } else if (op === "collinear") {
        result = remove_collinear_ring(flat) as bigint[];
      } else if (op === "normalize") {
        const normalized = normalize_polygon_ring(flat);
        if (normalized) {
          result = normalized as bigint[];
        } else {
          errorEl.textContent = "Normalization returned null (degenerate ring)";
          return;
        }
      }

      if (result) {
        setPolygon(fromFlat(result));
      }
    } catch (e) {
      errorEl.textContent = String(e);
    }
  }

  return {
    id: "ring",
    label: "Ring Ops",

    create() {
      const el = document.createElement("div");
      el.id = "tab-ring";
      el.innerHTML = `
        <div class="toolbar">
          <select id="ring-preset">
            <option value="">— Preset —</option>
            ${PRESETS.map((p) => `<option value="${p.name}">${p.name}</option>`).join("")}
          </select>
          <div class="sep"></div>
          <button class="btn" id="ring-ccw">Ensure CCW</button>
          <button class="btn" id="ring-collinear">Remove Collinear</button>
          <button class="btn" id="ring-normalize">Normalize</button>
          <div class="sep"></div>
          <button class="btn btn-danger" id="ring-clear">Clear</button>
          <span class="status-text">Blue = convex, Red = reflex. Arrows = winding.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="ring-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="ring-info">
              <h3>Properties</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="ring-error" style="color:#ff4a4a;font-size:12px;"></div>
          </div>
        </div>
      `;
      return el;
    },

    activate() {
      canvas = document.getElementById("ring-canvas") as HTMLCanvasElement;
      infoPanel = document.getElementById("ring-info")!;
      errorEl = document.getElementById("ring-error")!;

      rc = createRenderCtx(canvas);
      drawingTool = createDrawingTool(canvas, () => rc);

      polygon = getPolygon();
      unsubscribe = onPolygonChange(() => {
        polygon = getPolygon();
        analyze();
      });

      drawingTool.setOnChange(() => render());
      drawingTool.setOnComplete((pts) => {
        drawingTool.clear();
        setPolygon(pts);
      });
      drawingTool.enable();

      document.getElementById("ring-preset")!.addEventListener("change", (e) => {
        const name = (e.target as HTMLSelectElement).value;
        const preset = PRESETS.find((p) => p.name === name);
        if (preset) {
          drawingTool.clear();
          setPolygon(preset.points.slice());
        }
      });

      document.getElementById("ring-clear")!.addEventListener("click", () => {
        drawingTool.clear();
        errorEl.textContent = "";
        (document.getElementById("ring-preset") as HTMLSelectElement).value = "";
        setPolygon([]);
      });

      document.getElementById("ring-ccw")!.addEventListener("click", () => applyOp("ccw"));
      document.getElementById("ring-collinear")!.addEventListener("click", () => applyOp("collinear"));
      document.getElementById("ring-normalize")!.addEventListener("click", () => applyOp("normalize"));

      if (polygon.length >= 3) analyze();
      else render();
    },

    deactivate() {
      drawingTool?.disable();
      unsubscribe?.();
      unsubscribe = null;
    },
  };
}
