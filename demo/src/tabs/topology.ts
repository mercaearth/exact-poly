import type { Tab } from "../main";
import {
  createRenderCtx,
  fitToPolygons,
  clear,
  drawGrid,
  drawPolygon,
  drawSegment,
  partColor,
  type RenderCtx,
} from "../canvas/renderer";
import { createDrawingTool, type DrawingTool } from "../canvas/drawing";
import { PRESETS, TOPOLOGY_PRESETS } from "../canvas/presets";
import { getPolygon, setPolygon, onPolygonChange } from "../state";
import { getConfigForWasm, onConfigChange } from "../config";
import {
  decompose_polygon,
  validate_multipart_topology,
  has_exact_shared_edge,
  toFlat,
  fromFlat,
  type DecomposeResult,
  type TopologyError,
} from "../wasm";

interface EdgeInfo {
  partA: number;
  partB: number;
  shared: boolean;
}

function parseTopologyError(raw: unknown): TopologyError | null {
  if (raw === null || raw === undefined) return null;
  if (typeof raw === "string") {
    return { HasHoles: { boundary_components: 0 } } as TopologyError;
  }
  return raw as TopologyError;
}

function formatTopologyError(err: TopologyError): string {
  if ("NotConnected" in err) {
    return `Parts not connected. Disconnected: [${err.NotConnected.disconnected_parts.join(", ")}]`;
  }
  if ("HasHoles" in err) {
    return `Polygon has holes. ${err.HasHoles.boundary_components} boundary components found.`;
  }
  if ("TooManyParts" in err) {
    return `Too many parts: ${err.TooManyParts.count} (max ${err.TooManyParts.max})`;
  }
  if ("NotCompact" in err) {
    return `Not compact enough: ${err.NotCompact.compactness_ppm} ppm (min ${err.NotCompact.min_ppm})`;
  }
  if ("VertexOnlyContact" in err) {
    return `Parts ${err.VertexOnlyContact.part_a} and ${err.VertexOnlyContact.part_b} have only vertex contact`;
  }
  if ("UnsupportedContact" in err) {
    return `Parts ${err.UnsupportedContact.part_a} and ${err.UnsupportedContact.part_b}: ${err.UnsupportedContact.reason}`;
  }
  return JSON.stringify(err);
}

function topologyErrorType(err: TopologyError): string {
  if ("NotConnected" in err) return "NotConnected";
  if ("HasHoles" in err) return "HasHoles";
  if ("TooManyParts" in err) return "TooManyParts";
  if ("NotCompact" in err) return "NotCompact";
  if ("VertexOnlyContact" in err) return "VertexOnlyContact";
  if ("UnsupportedContact" in err) return "UnsupportedContact";
  return "Unknown";
}

function disconnectedParts(err: TopologyError | null): Set<number> {
  if (err && "NotConnected" in err) {
    return new Set(err.NotConnected.disconnected_parts);
  }
  return new Set();
}

export function createTopologyTab(): Tab {
  let canvas: HTMLCanvasElement;
  let rc: RenderCtx;
  let drawingTool: DrawingTool;
  let polygon: [number, number][] = [];
  let parts: [number, number][][] = [];
  let edgeInfos: EdgeInfo[] = [];
  let topologyError: TopologyError | null = null;
  let allowVertexContact = false;
  let manualParts: [number, number][][] | null = null;
  let infoPanel: HTMLElement;
  let errorEl: HTMLElement;
  let unsubscribe: (() => void) | null = null;
  let unsubConfig: (() => void) | null = null;

  function analyze() {
    parts = [];
    edgeInfos = [];
    topologyError = null;
    errorEl.textContent = "";

    if (manualParts) {
      parts = manualParts;
    } else if (polygon.length < 3) {
      updateInfo();
      render();
      return;
    } else {
      try {
        const flat = toFlat(polygon);
        const result = decompose_polygon(flat, true, undefined, undefined, getConfigForWasm()) as DecomposeResult;
        parts = result.parts.map((p) => fromFlat(p));
      } catch (e) {
        errorEl.textContent = String(e);
        updateInfo();
        render();
        return;
      }
    }

    try {

      if (parts.length > 1) {
        for (let i = 0; i < parts.length; i++) {
          for (let j = i + 1; j < parts.length; j++) {
            try {
              const flatI = toFlat(parts[i]);
              const flatJ = toFlat(parts[j]);
              const shared = has_exact_shared_edge(flatI, flatJ);
              edgeInfos.push({ partA: i, partB: j, shared });
            } catch (_) {
              edgeInfos.push({ partA: i, partB: j, shared: false });
            }
          }
        }

        const partsForValidation = parts.map((part) => {
          const flat: bigint[] = [];
          for (const [x, y] of part) {
            flat.push(BigInt(x));
            flat.push(BigInt(y));
          }
          return flat;
        });

        const validationResult = validate_multipart_topology(
          partsForValidation,
          allowVertexContact || undefined,
          getConfigForWasm(),
        );
        topologyError = parseTopologyError(validationResult);
      }
    } catch (e) {
      errorEl.textContent = String(e);
    }

    updateInfo();
    render();
  }

  function updateInfo() {
    const rows: string[] = [];
    if (manualParts) {
      rows.push(row("Mode", "Manual parts", "warn"));
    }
    rows.push(row("Vertices", String(polygon.length)));
    rows.push(row("Parts", String(parts.length)));

    if (parts.length > 1) {
      const sharedCount = edgeInfos.filter((e) => e.shared).length;
      rows.push(row("Shared edges", `${sharedCount}/${edgeInfos.length} pairs`));

      if (topologyError) {
        const errType = topologyErrorType(topologyError);
        rows.push(row("Topology", errType, "error"));
        rows.push(`<div class="topo-error-detail">${formatTopologyError(topologyError)}</div>`);
      } else if (parts.length > 1) {
        rows.push(row("Topology", "Valid", "ok"));
      }

      rows.push(`<div style="border-top:1px solid #222;margin:8px 0;"></div>`);
      for (const e of edgeInfos) {
        rows.push(
          row(
            `P${e.partA} — P${e.partB}`,
            e.shared ? "Shared edge" : "No shared edge",
            e.shared ? "ok" : "warn"
          )
        );
      }
    }

    infoPanel.innerHTML = `<h3>Topology</h3>${rows.join("")}`;
  }

  function row(label: string, value: string, cls = ""): string {
    return `<div class="info-row"><span class="info-label">${label}</span><span class="info-value ${cls}">${value}</span></div>`;
  }

  function render() {
    rc = createRenderCtx(canvas);
    const allPolygons = parts.length > 0
      ? (polygon.length > 0 ? [polygon, ...parts] : [...parts])
      : polygon.length > 0 ? [polygon] : [];
    if (allPolygons.length > 0) fitToPolygons(rc, allPolygons);

    clear(rc);
    drawGrid(rc);

    if (parts.length > 0) {
      if (polygon.length > 0) {
        drawPolygon(rc, polygon, { stroke: "#333", showVertices: false, dashed: true, lineWidth: 1 });
      }

      const badParts = disconnectedParts(topologyError);

      for (let i = 0; i < parts.length; i++) {
        if (badParts.has(i)) {
          drawPolygon(rc, parts[i], {
            fill: "#ff4a4a",
            stroke: "#ff4a4a",
            lineWidth: 2.5,
            showVertices: true,
          });
        } else {
          const color = partColor(i);
          drawPolygon(rc, parts[i], {
            fill: color,
            stroke: color,
            showVertices: true,
          });
        }
      }

      for (const e of edgeInfos) {
        if (!e.shared) continue;
        const pa = parts[e.partA];
        const pb = parts[e.partB];

        for (let i = 0; i < pa.length; i++) {
          const a1 = pa[i];
          const a2 = pa[(i + 1) % pa.length];
          for (let j = 0; j < pb.length; j++) {
            const b1 = pb[j];
            const b2 = pb[(j + 1) % pb.length];
            if (
              (a1[0] === b2[0] && a1[1] === b2[1] && a2[0] === b1[0] && a2[1] === b1[1]) ||
              (a1[0] === b1[0] && a1[1] === b1[1] && a2[0] === b2[0] && a2[1] === b2[1])
            ) {
              drawSegment(rc, a1, a2, "#fcc419", 3);
            }
          }
        }
      }
    } else if (polygon.length > 0) {
      drawPolygon(rc, polygon);
    }

    if (drawingTool.isDrawing && drawingTool.points.length > 0) {
      drawPolygon(rc, drawingTool.points, { stroke: "#666", closed: false, dashed: true });
    }
  }

  return {
    id: "topology",
    label: "Topology",

    create() {
      const el = document.createElement("div");
      el.id = "tab-topology";
      el.innerHTML = `
        <div class="toolbar">
          <select id="topo-preset">
            <option value="">— Preset —</option>
            ${PRESETS.map((p) => `<option value="${p.name}">${p.name}</option>`).join("")}
            <optgroup label="Manual parts (validation)">
              ${TOPOLOGY_PRESETS.map((p) => `<option value="topo:${p.name}">${p.name}</option>`).join("")}
            </optgroup>
          </select>
          <div class="checkbox-row"><input type="checkbox" id="topo-vertex-contact" /><label for="topo-vertex-contact">Allow vertex contact</label></div>
          <div class="sep"></div>
          <button class="btn btn-danger" id="topo-clear">Clear</button>
          <span class="status-text">Yellow = shared edges, Red = disconnected.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="topo-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="topo-info">
              <h3>Topology</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="topo-error" style="color:#ff4a4a;font-size:12px;"></div>
          </div>
        </div>
      `;
      return el;
    },

    activate() {
      canvas = document.getElementById("topo-canvas") as HTMLCanvasElement;
      infoPanel = document.getElementById("topo-info")!;
      errorEl = document.getElementById("topo-error")!;

      rc = createRenderCtx(canvas);
      drawingTool = createDrawingTool(canvas, () => rc);

      polygon = getPolygon();
      unsubscribe = onPolygonChange(() => {
        polygon = getPolygon();
        parts = [];
        edgeInfos = [];
        topologyError = null;
        manualParts = null;
        analyze();
      });

      drawingTool.setOnChange(() => render());
      drawingTool.setOnComplete((pts) => {
        drawingTool.clear();
        setPolygon(pts);
      });
      drawingTool.enable();

      document.getElementById("topo-preset")!.addEventListener("change", (e) => {
        const name = (e.target as HTMLSelectElement).value;
        if (name.startsWith("topo:")) {
          const topoName = name.slice(5);
          const topoPreset = TOPOLOGY_PRESETS.find((p) => p.name === topoName);
          if (topoPreset) {
            drawingTool.clear();
            manualParts = topoPreset.parts.map((p) => p.slice());
            polygon = [];
            analyze();
          }
        } else {
          manualParts = null;
          const preset = PRESETS.find((p) => p.name === name);
          if (preset) {
            drawingTool.clear();
            setPolygon(preset.points.slice());
          }
        }
      });

      document.getElementById("topo-clear")!.addEventListener("click", () => {
        parts = [];
        edgeInfos = [];
        topologyError = null;
        manualParts = null;
        drawingTool.clear();
        errorEl.textContent = "";
        (document.getElementById("topo-preset") as HTMLSelectElement).value = "";
        setPolygon([]);
      });

      document.getElementById("topo-vertex-contact")!.addEventListener("change", (e) => {
        allowVertexContact = (e.target as HTMLInputElement).checked;
        if (polygon.length >= 3) analyze();
      });

      unsubConfig = onConfigChange(() => {
        if (polygon.length >= 3) analyze();
      });

      if (polygon.length >= 3) analyze();
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
