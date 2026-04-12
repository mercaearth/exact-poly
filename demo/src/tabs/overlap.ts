import type { Tab } from "../main";
import {
  createRenderCtx,
  fitToPolygons,
  clear,
  drawGrid,
  drawPolygon,
  type RenderCtx,
} from "../canvas/renderer";
import { OVERLAP_PRESETS } from "../canvas/presets";
import {
  sat_overlap,
  sat_overlap_with_aabb,
  toFlat,
} from "../wasm";

export function createOverlapTab(): Tab {
  let canvas: HTMLCanvasElement;
  let rc: RenderCtx;
  let polyA: [number, number][] = [];
  let polyB: [number, number][] = [];
  let dragTarget: "a" | "b" | null = null;
  let dragStart: [number, number] = [0, 0];
  let dragOffset: [number, number] = [0, 0];
  let infoPanel: HTMLElement;
  let errorEl: HTMLElement;
  let useAabb = false;

  function checkOverlap(): { overlaps: boolean } | null {
    if (polyA.length < 3 || polyB.length < 3) return null;

    try {
      const flatA = toFlat(polyA);
      const flatB = toFlat(polyB);
      const overlaps = useAabb
        ? sat_overlap_with_aabb(flatA, flatB)
        : sat_overlap(flatA, flatB);
      return { overlaps };
    } catch (e) {
      errorEl.textContent = String(e);
      return null;
    }
  }

  function updateInfo() {
    errorEl.textContent = "";
    const result = checkOverlap();
    const rows: string[] = [];
    rows.push(row("Polygon A", `${polyA.length} verts`));
    rows.push(row("Polygon B", `${polyB.length} verts`));
    rows.push(row("Method", useAabb ? "SAT + AABB" : "SAT"));

    if (result) {
      rows.push(
        row("Overlaps", result.overlaps ? "Yes" : "No", result.overlaps ? "error" : "ok")
      );
    }

    infoPanel.innerHTML = `<h3>Overlap Detection</h3>${rows.join("")}`;
  }

  function row(label: string, value: string, cls = ""): string {
    return `<div class="info-row"><span class="info-label">${label}</span><span class="info-value ${cls}">${value}</span></div>`;
  }

  function render() {
    rc = createRenderCtx(canvas);
    const all = [polyA, polyB].filter((p) => p.length > 0);
    if (all.length > 0) fitToPolygons(rc, all, 50);

    clear(rc);
    drawGrid(rc);

    const result = checkOverlap();
    const overlapColor = result?.overlaps ? "#ff6b6b" : undefined;

    if (polyA.length >= 2) {
      drawPolygon(rc, polyA, {
        fill: overlapColor || "#4a9eff",
        stroke: overlapColor || "#4a9eff",
      });
    }
    if (polyB.length >= 2) {
      drawPolygon(rc, polyB, {
        fill: overlapColor || "#51cf66",
        stroke: overlapColor || "#51cf66",
      });
    }
  }

  function getCentroid(poly: [number, number][]): [number, number] {
    const cx = poly.reduce((s, p) => s + p[0], 0) / poly.length;
    const cy = poly.reduce((s, p) => s + p[1], 0) / poly.length;
    return [cx, cy];
  }

  function translatePoly(poly: [number, number][], dx: number, dy: number): [number, number][] {
    return poly.map(([x, y]) => [Math.round(x + dx), Math.round(y + dy)]);
  }

  function distSq(a: [number, number], b: [number, number]): number {
    return (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2;
  }

  function handleMouseDown(e: MouseEvent) {
    const rect = canvas.getBoundingClientRect();
    const sx = e.clientX - rect.left;
    const sy = e.clientY - rect.top;

    const wx = (sx - rc.offsetX) / rc.scale;
    const wy = -(sy - rc.offsetY) / rc.scale;

    const distA = polyA.length > 0 ? distSq([wx, wy], getCentroid(polyA)) : Infinity;
    const distB = polyB.length > 0 ? distSq([wx, wy], getCentroid(polyB)) : Infinity;

    if (distA < distB && distA < Infinity) {
      dragTarget = "a";
      dragStart = getCentroid(polyA);
    } else if (distB < Infinity) {
      dragTarget = "b";
      dragStart = getCentroid(polyB);
    }

    if (dragTarget) {
      dragOffset = [wx - dragStart[0], wy - dragStart[1]];
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (!dragTarget) return;
    const rect = canvas.getBoundingClientRect();
    const sx = e.clientX - rect.left;
    const sy = e.clientY - rect.top;

    const wx = (sx - rc.offsetX) / rc.scale;
    const wy = -(sy - rc.offsetY) / rc.scale;

    const target = dragTarget === "a" ? polyA : polyB;
    const centroid = getCentroid(target);
    const dx = wx - dragOffset[0] - centroid[0];
    const dy = wy - dragOffset[1] - centroid[1];

    if (dragTarget === "a") {
      polyA = translatePoly(polyA, dx, dy);
    } else {
      polyB = translatePoly(polyB, dx, dy);
    }

    updateInfo();
    render();
  }

  function handleMouseUp() {
    dragTarget = null;
  }

  return {
    id: "overlap",
    label: "Overlap & SAT",

    create() {
      const el = document.createElement("div");
      el.id = "tab-overlap";
      el.innerHTML = `
        <div class="toolbar">
          <select id="overlap-preset">
            ${OVERLAP_PRESETS.map((p) => `<option value="${p.name}">${p.name}</option>`).join("")}
          </select>
          <div class="checkbox-row"><input type="checkbox" id="overlap-aabb" /><label for="overlap-aabb">AABB pre-filter</label></div>
          <span class="status-text">Drag polygons. Blue=A, Green=B, Red=overlapping.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="overlap-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="overlap-info">
              <h3>Overlap Detection</h3>
            </div>
            <div id="overlap-error" style="color:#ff4a4a;font-size:12px;"></div>
          </div>
        </div>
      `;
      return el;
    },

    activate() {
      canvas = document.getElementById("overlap-canvas") as HTMLCanvasElement;
      infoPanel = document.getElementById("overlap-info")!;
      errorEl = document.getElementById("overlap-error")!;

      rc = createRenderCtx(canvas);

      const firstPreset = OVERLAP_PRESETS[0];
      polyA = firstPreset.a.slice();
      polyB = firstPreset.b.slice();

      canvas.addEventListener("mousedown", handleMouseDown);
      canvas.addEventListener("mousemove", handleMouseMove);
      canvas.addEventListener("mouseup", handleMouseUp);
      canvas.addEventListener("mouseleave", handleMouseUp);

      document.getElementById("overlap-preset")!.addEventListener("change", (e) => {
        const name = (e.target as HTMLSelectElement).value;
        const preset = OVERLAP_PRESETS.find((p) => p.name === name);
        if (preset) {
          polyA = preset.a.slice();
          polyB = preset.b.slice();
          errorEl.textContent = "";
          updateInfo();
          render();
        }
      });

      document.getElementById("overlap-aabb")!.addEventListener("change", (e) => {
        useAabb = (e.target as HTMLInputElement).checked;
        updateInfo();
        render();
      });

      updateInfo();
      render();
    },

    deactivate() {
      canvas?.removeEventListener("mousedown", handleMouseDown);
      canvas?.removeEventListener("mousemove", handleMouseMove);
      canvas?.removeEventListener("mouseup", handleMouseUp);
      canvas?.removeEventListener("mouseleave", handleMouseUp);
    },
  };
}
