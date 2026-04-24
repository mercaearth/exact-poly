import type { Tab } from "../main";
import {
  createRenderCtx,
  fitToPolygons,
  clear,
  drawGrid,
  drawPolygon,
  drawParts,
  drawLabel,
  partColor,
  type RenderCtx,
} from "../canvas/renderer";
import { createDrawingTool, type DrawingTool } from "../canvas/drawing";
import { PRESETS } from "../canvas/presets";
import { getPolygon, setPolygon, onPolygonChange } from "../state";
import { getConfigForWasm, onConfigChange } from "../config";
import {
  twice_area,
  signed_area_2x,
  area_display_from_twice_area,
  perimeter_l1,
  decompose_polygon,
  areas_conserved_values,
  toFlat,
  fromFlat,
  type DecomposeResult,
} from "../wasm";

export function createAreaTab(): Tab {
  let canvas: HTMLCanvasElement;
  let rc: RenderCtx;
  let drawingTool: DrawingTool;
  let polygon: [number, number][] = [];
  let parts: [number, number][][] = [];
  let infoPanel: HTMLElement;
  let errorEl: HTMLElement;
  let unsubscribe: (() => void) | null = null;
  let unsubConfig: (() => void) | null = null;

  function compute() {
    parts = [];
    errorEl.textContent = "";
    if (polygon.length < 3) {
      updateInfo(null);
      render();
      return;
    }

    try {
      const flat = toFlat(polygon);
      const twiceArea = twice_area(flat);
      const signedArea = signed_area_2x(flat);
      const displayArea = area_display_from_twice_area(twiceArea, getConfigForWasm());
      const perimeter = perimeter_l1(flat);

      let conserved: boolean | null = null;
      const partAreas: string[] = [];

      try {
        const result = decompose_polygon(flat, true, undefined, undefined, getConfigForWasm()) as DecomposeResult;
        parts = result.parts.map((p) => fromFlat(p));

        for (const part of parts) {
          const pFlat = toFlat(part);
          partAreas.push(twice_area(pFlat));
        }
        conserved = areas_conserved_values(twiceArea, partAreas);
      } catch (_) { /* decomposition optional */ }

      updateInfo({
        twiceArea,
        signedArea,
        displayArea: displayArea.toString(),
        perimeter,
        partsCount: parts.length,
        partAreas,
        conserved,
      });
    } catch (e) {
      errorEl.textContent = String(e);
    }

    render();
  }

  interface AreaData {
    twiceArea: string;
    signedArea: string;
    displayArea: string;
    perimeter: string;
    partsCount: number;
    partAreas: string[];
    conserved: boolean | null;
  }

  function updateInfo(data: AreaData | null) {
    if (!data) {
      infoPanel.innerHTML = `<h3>Metrics</h3><div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>`;
      return;
    }

    const rows: string[] = [];
    rows.push(row("2x Area (unsigned)", data.twiceArea));
    rows.push(row("2x Area (signed)", data.signedArea));
    rows.push(row("Display Area", data.displayArea));
    rows.push(row("L1 Perimeter", data.perimeter));

    if (data.partsCount > 0) {
      rows.push(`<div style="border-top:1px solid #222;margin:8px 0;"></div>`);
      rows.push(row("Parts", String(data.partsCount)));
      for (let i = 0; i < data.partAreas.length; i++) {
        rows.push(row(`Part ${i} 2xArea`, data.partAreas[i]));
      }
      if (data.conserved !== null) {
        rows.push(
          row(
            "Area Conserved",
            data.conserved ? "Yes" : "No",
            data.conserved ? "ok" : "error"
          )
        );
      }
    }

    infoPanel.innerHTML = `<h3>Metrics</h3>${rows.join("")}`;
  }

  function row(label: string, value: string, cls = ""): string {
    return `<div class="info-row"><span class="info-label">${label}</span><span class="info-value ${cls}">${value}</span></div>`;
  }

  function render() {
    rc = createRenderCtx(canvas);
    const allPolygons = parts.length > 0 ? [polygon, ...parts] : polygon.length > 0 ? [polygon] : [];
    if (allPolygons.length > 0) fitToPolygons(rc, allPolygons);

    clear(rc);
    drawGrid(rc);

    if (parts.length > 0) {
      drawPolygon(rc, polygon, { stroke: "#555", showVertices: false, dashed: true, lineWidth: 1 });
      drawParts(rc, parts, { showVertices: false });

      for (let i = 0; i < parts.length; i++) {
        const cx = parts[i].reduce((s, p) => s + p[0], 0) / parts[i].length;
        const cy = parts[i].reduce((s, p) => s + p[1], 0) / parts[i].length;
        drawLabel(rc, cx, cy, `P${i}`, partColor(i), [-6, 4]);
      }
    } else if (polygon.length > 0) {
      drawPolygon(rc, polygon);
    }

    if (drawingTool.isDrawing && drawingTool.points.length > 0) {
      drawPolygon(rc, drawingTool.points, { stroke: "#666", closed: false, dashed: true });
    }
  }

  return {
    id: "area",
    label: "Area & Metrics",

    create() {
      const el = document.createElement("div");
      el.id = "tab-area";
      el.innerHTML = `
        <div class="toolbar">
          <select id="area-preset">
            <option value="">— Preset —</option>
            ${PRESETS.map((p) => `<option value="${p.name}">${p.name}</option>`).join("")}
          </select>
          <button class="btn btn-danger" id="area-clear">Clear</button>
          <span class="status-text">Auto-decomposes to show area conservation.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="area-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="area-info">
              <h3>Metrics</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="area-error" style="color:#ff4a4a;font-size:12px;"></div>
          </div>
        </div>
      `;
      return el;
    },

    activate() {
      canvas = document.getElementById("area-canvas") as HTMLCanvasElement;
      infoPanel = document.getElementById("area-info")!;
      errorEl = document.getElementById("area-error")!;

      rc = createRenderCtx(canvas);
      drawingTool = createDrawingTool(canvas, () => rc);

      polygon = getPolygon();
      unsubscribe = onPolygonChange(() => {
        polygon = getPolygon();
        parts = [];
        compute();
      });

      drawingTool.setOnChange(() => render());
      drawingTool.setOnComplete((pts) => {
        drawingTool.clear();
        setPolygon(pts);
      });
      drawingTool.enable();

      document.getElementById("area-preset")!.addEventListener("change", (e) => {
        const name = (e.target as HTMLSelectElement).value;
        const preset = PRESETS.find((p) => p.name === name);
        if (preset) {
          drawingTool.clear();
          setPolygon(preset.points.slice());
        }
      });

      document.getElementById("area-clear")!.addEventListener("click", () => {
        parts = [];
        drawingTool.clear();
        errorEl.textContent = "";
        (document.getElementById("area-preset") as HTMLSelectElement).value = "";
        setPolygon([]);
      });

      unsubConfig = onConfigChange(() => {
        if (polygon.length >= 3) compute();
      });

      if (polygon.length >= 3) compute();
      else render();
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
