import type { Tab } from "../main";
import {
  createRenderCtx,
  fitToPolygons,
  clear,
  drawGrid,
  drawPolygon,
  drawPoint,
  drawLabel,
  fromScreen,
  type RenderCtx,
} from "../canvas/renderer";
import { createDrawingTool, type DrawingTool } from "../canvas/drawing";
import { PRESETS } from "../canvas/presets";
import { getPolygon, setPolygon, onPolygonChange } from "../state";
import {
  point_inside_or_on_boundary_ring,
  point_on_polygon_boundary_ring,
  point_strictly_inside_convex_ring,
  is_convex_ring,
  toFlat,
} from "../wasm";

interface TestPoint {
  x: number;
  y: number;
  inside: boolean;
  boundary: boolean;
  strictlyInside: boolean;
}

export function createSpatialTab(): Tab {
  let canvas: HTMLCanvasElement;
  let rc: RenderCtx;
  let drawingTool: DrawingTool;
  let polygon: [number, number][] = [];
  let testPoints: TestPoint[] = [];
  let isConvex = false;
  let mode: "draw" | "test" = "draw";
  let infoPanel: HTMLElement;
  let errorEl: HTMLElement;
  let unsubscribe: (() => void) | null = null;
  let setMode: ((m: "draw" | "test") => void) | null = null;

  function testPoint(x: number, y: number) {
    if (polygon.length < 3) return;

    try {
      const SCALE = 1_000_000;
      const flat = toFlat(polygon);
      const px = BigInt(Math.round(x * SCALE));
      const py = BigInt(Math.round(y * SCALE));
      const inside = point_inside_or_on_boundary_ring(px, py, flat);
      const boundary = point_on_polygon_boundary_ring(px, py, flat);
      let strictlyInside = false;

      if (isConvex) {
        strictlyInside = point_strictly_inside_convex_ring(px, py, flat);
      }

      testPoints.push({ x, y, inside, boundary, strictlyInside });
      updateInfo();
      render();
    } catch (e) {
      errorEl.textContent = String(e);
    }
  }

  function updateInfo() {
    const rows: string[] = [];
    rows.push(row("Vertices", String(polygon.length)));
    rows.push(row("Convex", isConvex ? "Yes" : "No", isConvex ? "ok" : "warn"));
    rows.push(row("Test points", String(testPoints.length)));

    if (testPoints.length > 0) {
      const last = testPoints[testPoints.length - 1];
      rows.push(`<div style="border-top:1px solid #222;margin:8px 0;"></div>`);
      rows.push(row("Last point", `(${last.x}, ${last.y})`));
      rows.push(row("Inside/On", last.inside ? "Yes" : "No", last.inside ? "ok" : "error"));
      rows.push(row("On boundary", last.boundary ? "Yes" : "No", last.boundary ? "warn" : ""));
      if (isConvex) {
        rows.push(row("Strictly inside", last.strictlyInside ? "Yes" : "No", last.strictlyInside ? "ok" : ""));
      }
    }

    infoPanel.innerHTML = `<h3>Spatial Query</h3>${rows.join("")}`;
  }

  function row(label: string, value: string, cls = ""): string {
    return `<div class="info-row"><span class="info-label">${label}</span><span class="info-value ${cls}">${value}</span></div>`;
  }

  function render() {
    rc = createRenderCtx(canvas);
    if (polygon.length > 0) fitToPolygons(rc, [polygon], 60);

    clear(rc);
    drawGrid(rc);

    if (polygon.length >= 2) {
      drawPolygon(rc, polygon, { fill: "#4a9eff" });
    }

    for (const pt of testPoints) {
      let color: string;
      if (pt.boundary) {
        color = "#fcc419";
      } else if (pt.inside) {
        color = "#51cf66";
      } else {
        color = "#ff6b6b";
      }
      drawPoint(rc, pt.x, pt.y, color, 5);
    }

    if (drawingTool.isDrawing && drawingTool.points.length > 0) {
      drawPolygon(rc, drawingTool.points, { stroke: "#666", closed: false, dashed: true });
    }
  }

  function handleTestClick(e: MouseEvent) {
    if (mode !== "test" || polygon.length < 3) return;
    const rect = canvas.getBoundingClientRect();
    const [wx, wy] = fromScreen(rc, e.clientX - rect.left, e.clientY - rect.top);
    testPoint(wx, wy);
  }

  return {
    id: "spatial",
    label: "Spatial Queries",

    create() {
      const el = document.createElement("div");
      el.id = "tab-spatial";
      el.innerHTML = `
        <div class="toolbar">
          <select id="spatial-preset">
            <option value="">— Preset —</option>
            ${PRESETS.map((p) => `<option value="${p.name}">${p.name}</option>`).join("")}
          </select>
          <div class="sep"></div>
          <button class="btn btn-primary" id="spatial-mode-draw">Draw</button>
          <button class="btn" id="spatial-mode-test">Test</button>
          <div class="sep"></div>
          <button class="btn" id="spatial-clear-pts">Clear Points</button>
          <button class="btn btn-danger" id="spatial-clear">Clear All</button>
          <span class="status-text" id="spatial-status">Draw polygon, then switch to Test.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="spatial-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="spatial-info">
              <h3>Spatial Query</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="spatial-error" style="color:#ff4a4a;font-size:12px;"></div>
            <div class="help-text">Green = inside, Yellow = boundary, Red = outside</div>
          </div>
        </div>
      `;
      return el;
    },

    activate() {
      canvas = document.getElementById("spatial-canvas") as HTMLCanvasElement;
      infoPanel = document.getElementById("spatial-info")!;
      errorEl = document.getElementById("spatial-error")!;

      rc = createRenderCtx(canvas);
      drawingTool = createDrawingTool(canvas, () => rc);

      polygon = getPolygon();
      unsubscribe = onPolygonChange(() => {
        polygon = getPolygon();
        testPoints = [];
        try {
          isConvex = polygon.length >= 3 ? is_convex_ring(toFlat(polygon)) : false;
        } catch (_) {
          isConvex = false;
        }
        if (polygon.length >= 3 && setMode) setMode("test");
        updateInfo();
        render();
      });

      drawingTool.setOnChange(() => render());
      drawingTool.setOnComplete((pts) => {
        drawingTool.clear();
        setPolygon(pts);
      });

      setMode = function _setMode(m: "draw" | "test") {
        mode = m;
        const drawBtn = document.getElementById("spatial-mode-draw")!;
        const testBtn = document.getElementById("spatial-mode-test")!;
        const status = document.getElementById("spatial-status")!;

        if (m === "draw") {
          drawBtn.classList.add("btn-primary");
          testBtn.classList.remove("btn-primary");
          drawingTool.enable();
          canvas.removeEventListener("click", handleTestClick);
          status.textContent = "Click to draw polygon vertices. Right-click to close.";
        } else {
          testBtn.classList.add("btn-primary");
          drawBtn.classList.remove("btn-primary");
          drawingTool.disable();
          canvas.addEventListener("click", handleTestClick);
          status.textContent = "Click anywhere to test point inclusion.";
        }
      }

      if (polygon.length >= 3) {
        try { isConvex = is_convex_ring(toFlat(polygon)); } catch (_) { isConvex = false; }
        setMode("test");
        updateInfo();
      } else {
        setMode("draw");
      }

      document.getElementById("spatial-preset")!.addEventListener("change", (e) => {
        const name = (e.target as HTMLSelectElement).value;
        const preset = PRESETS.find((p) => p.name === name);
        if (preset) {
          drawingTool.clear();
          testPoints = [];
          setPolygon(preset.points.slice());
        }
      });

      document.getElementById("spatial-clear")!.addEventListener("click", () => {
        testPoints = [];
        drawingTool.clear();
        errorEl.textContent = "";
        if (setMode) setMode("draw");
        (document.getElementById("spatial-preset") as HTMLSelectElement).value = "";
        setPolygon([]);
      });

      document.getElementById("spatial-clear-pts")!.addEventListener("click", () => {
        testPoints = [];
        updateInfo();
        render();
      });

      document.getElementById("spatial-mode-draw")!.addEventListener("click", () => setMode?.("draw"));
      document.getElementById("spatial-mode-test")!.addEventListener("click", () => setMode?.("test"));

      render();
    },

    deactivate() {
      drawingTool?.disable();
      canvas?.removeEventListener("click", handleTestClick);
      unsubscribe?.();
      unsubscribe = null;
    },
  };
}
