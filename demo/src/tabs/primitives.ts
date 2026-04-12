import type { Tab } from "../main";
import {
  createRenderCtx,
  clear,
  drawGrid,
  drawPoint,
  drawLabel,
  drawSegment,
  toScreen,
  fromScreen,
  type RenderCtx,
} from "../canvas/renderer";
import {
  orientation,
  cross2d,
  is_left,
  is_right,
  is_reflex,
  segments_intersect,
  segments_properly_intersect,
  point_on_segment,
} from "../wasm";

type PrimMode = "orientation" | "segments";

export function createPrimitivesTab(): Tab {
  let canvas: HTMLCanvasElement;
  let rc: RenderCtx;
  let mode: PrimMode = "orientation";
  let points: [number, number][] = [];
  let infoPanel: HTMLElement;

  function analyze() {
    const rows: string[] = [];

    if (mode === "orientation") {
      if (points.length >= 3) {
        const [a, b, c] = points;
        const orient = orientation(
          BigInt(a[0]), BigInt(a[1]),
          BigInt(b[0]), BigInt(b[1]),
          BigInt(c[0]), BigInt(c[1])
        );
        const cross = cross2d(
          BigInt(a[0]), BigInt(a[1]),
          BigInt(b[0]), BigInt(b[1]),
          BigInt(c[0]), BigInt(c[1])
        );
        const left = is_left(
          BigInt(a[0]), BigInt(a[1]),
          BigInt(b[0]), BigInt(b[1]),
          BigInt(c[0]), BigInt(c[1])
        );
        const right = is_right(
          BigInt(a[0]), BigInt(a[1]),
          BigInt(b[0]), BigInt(b[1]),
          BigInt(c[0]), BigInt(c[1])
        );

        rows.push(row("A", `(${a[0]}, ${a[1]})`));
        rows.push(row("B", `(${b[0]}, ${b[1]})`));
        rows.push(row("C", `(${c[0]}, ${c[1]})`));
        rows.push(`<div style="border-top:1px solid #222;margin:8px 0;"></div>`);
        rows.push(row("Orientation", orient));
        rows.push(row("Cross2D", cross));
        rows.push(row("C is left of AB", left ? "Yes" : "No", left ? "ok" : ""));
        rows.push(row("C is right of AB", right ? "Yes" : "No", right ? "ok" : ""));
      } else {
        rows.push(row("Click", `${3 - points.length} more point(s)`));
      }
    } else {
      if (points.length >= 4) {
        const [a1, a2, b1, b2] = points;
        const intersects = segments_intersect(
          BigInt(a1[0]), BigInt(a1[1]),
          BigInt(a2[0]), BigInt(a2[1]),
          BigInt(b1[0]), BigInt(b1[1]),
          BigInt(b2[0]), BigInt(b2[1])
        );
        const properlyIntersects = segments_properly_intersect(
          BigInt(a1[0]), BigInt(a1[1]),
          BigInt(a2[0]), BigInt(a2[1]),
          BigInt(b1[0]), BigInt(b1[1]),
          BigInt(b2[0]), BigInt(b2[1])
        );

        rows.push(row("Seg A", `(${a1[0]},${a1[1]}) → (${a2[0]},${a2[1]})`));
        rows.push(row("Seg B", `(${b1[0]},${b1[1]}) → (${b2[0]},${b2[1]})`));
        rows.push(`<div style="border-top:1px solid #222;margin:8px 0;"></div>`);
        rows.push(row("Intersects", intersects ? "Yes" : "No", intersects ? "error" : "ok"));
        rows.push(row("Properly", properlyIntersects ? "Yes" : "No", properlyIntersects ? "error" : "ok"));
      } else {
        rows.push(row("Click", `${4 - points.length} more point(s)`));
        if (points.length < 2) {
          rows.push(row("", "Click 2 pts for segment A"));
        } else {
          rows.push(row("", "Click 2 pts for segment B"));
        }
      }
    }

    infoPanel.innerHTML = `<h3>Result</h3>${rows.join("")}`;
  }

  function row(label: string, value: string, cls = ""): string {
    return `<div class="info-row"><span class="info-label">${label}</span><span class="info-value ${cls}">${value}</span></div>`;
  }

  function render() {
    rc = createRenderCtx(canvas);
    rc.scale = 3;
    rc.offsetX = rc.width / 2;
    rc.offsetY = rc.height / 2;

    clear(rc);
    drawGrid(rc, 20);

    if (mode === "orientation") {
      const colors = ["#4a9eff", "#51cf66", "#fcc419"];
      const labels = ["A", "B", "C"];

      for (let i = 0; i < points.length; i++) {
        drawPoint(rc, points[i][0], points[i][1], colors[i], 6);
        drawLabel(rc, points[i][0], points[i][1], labels[i], colors[i]);
      }

      if (points.length >= 2) {
        drawSegment(rc, points[0], points[1], "#4a9eff", 2);
      }
      if (points.length >= 3) {
        drawSegment(rc, points[1], points[2], "#51cf66", 2, true);

        const orient = orientation(
          BigInt(points[0][0]), BigInt(points[0][1]),
          BigInt(points[1][0]), BigInt(points[1][1]),
          BigInt(points[2][0]), BigInt(points[2][1])
        );

        let areaColor = "#555";
        if (orient === "CounterClockwise") areaColor = "#51cf6630";
        else if (orient === "Clockwise") areaColor = "#ff6b6b30";

        const { ctx } = rc;
        const [ax, ay] = toScreen(rc, points[0][0], points[0][1]);
        const [bx, by] = toScreen(rc, points[1][0], points[1][1]);
        const [cx, cy] = toScreen(rc, points[2][0], points[2][1]);
        ctx.beginPath();
        ctx.moveTo(ax, ay);
        ctx.lineTo(bx, by);
        ctx.lineTo(cx, cy);
        ctx.closePath();
        ctx.fillStyle = areaColor;
        ctx.fill();
      }
    } else {
      if (points.length >= 1) drawPoint(rc, points[0][0], points[0][1], "#4a9eff", 6);
      if (points.length >= 2) {
        drawPoint(rc, points[1][0], points[1][1], "#4a9eff", 6);
        drawSegment(rc, points[0], points[1], "#4a9eff", 2);
        drawLabel(rc, points[0][0], points[0][1], "A1", "#4a9eff");
        drawLabel(rc, points[1][0], points[1][1], "A2", "#4a9eff");
      }
      if (points.length >= 3) drawPoint(rc, points[2][0], points[2][1], "#51cf66", 6);
      if (points.length >= 4) {
        drawPoint(rc, points[3][0], points[3][1], "#51cf66", 6);
        drawSegment(rc, points[2], points[3], "#51cf66", 2);
        drawLabel(rc, points[2][0], points[2][1], "B1", "#51cf66");
        drawLabel(rc, points[3][0], points[3][1], "B2", "#51cf66");

        const intersects = segments_intersect(
          BigInt(points[0][0]), BigInt(points[0][1]),
          BigInt(points[1][0]), BigInt(points[1][1]),
          BigInt(points[2][0]), BigInt(points[2][1]),
          BigInt(points[3][0]), BigInt(points[3][1])
        );
        if (intersects) {
          drawSegment(rc, points[0], points[1], "#ff6b6b", 3);
          drawSegment(rc, points[2], points[3], "#ff6b6b", 3);
        }
      }
    }
  }

  function handleClick(e: MouseEvent) {
    const rect = canvas.getBoundingClientRect();
    const [wx, wy] = fromScreen(rc, e.clientX - rect.left, e.clientY - rect.top);

    const maxPts = mode === "orientation" ? 3 : 4;
    if (points.length >= maxPts) {
      points = [];
    }

    points.push([wx, wy]);
    analyze();
    render();
  }

  return {
    id: "primitives",
    label: "Primitives",

    create() {
      const el = document.createElement("div");
      el.id = "tab-primitives";
      el.innerHTML = `
        <div class="toolbar">
          <button class="btn btn-primary" id="prim-mode-orient">Orientation (3 pts)</button>
          <button class="btn" id="prim-mode-seg">Segments (4 pts)</button>
          <div class="sep"></div>
          <button class="btn btn-danger" id="prim-clear">Clear</button>
          <span class="status-text" id="prim-status">Click to place points.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="prim-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="prim-info">
              <h3>Result</h3>
              <div class="info-row"><span class="info-label">Click to place points</span></div>
            </div>
            <div class="help-text">
              <b>Orientation:</b> 3 pts (A,B,C) — C left/right of AB.<br>
              <b>Segments:</b> 4 pts — intersection test. Red = intersecting.
            </div>
          </div>
        </div>
      `;
      return el;
    },

    activate() {
      canvas = document.getElementById("prim-canvas") as HTMLCanvasElement;
      infoPanel = document.getElementById("prim-info")!;

      rc = createRenderCtx(canvas);
      canvas.addEventListener("click", handleClick);

      function setMode(m: PrimMode) {
        mode = m;
        points = [];
        const orientBtn = document.getElementById("prim-mode-orient")!;
        const segBtn = document.getElementById("prim-mode-seg")!;
        const status = document.getElementById("prim-status")!;

        if (m === "orientation") {
          orientBtn.classList.add("btn-primary");
          segBtn.classList.remove("btn-primary");
          status.textContent = "Click 3 points: A, B, C. Tests orientation of C relative to AB.";
        } else {
          segBtn.classList.add("btn-primary");
          orientBtn.classList.remove("btn-primary");
          status.textContent = "Click 4 points: A1, A2 (segment A), B1, B2 (segment B). Tests intersection.";
        }

        analyze();
        render();
      }

      document.getElementById("prim-mode-orient")!.addEventListener("click", () => setMode("orientation"));
      document.getElementById("prim-mode-seg")!.addEventListener("click", () => setMode("segments"));
      document.getElementById("prim-clear")!.addEventListener("click", () => {
        points = [];
        analyze();
        render();
      });

      render();
    },

    deactivate() {
      canvas?.removeEventListener("click", handleClick);
    },
  };
}
