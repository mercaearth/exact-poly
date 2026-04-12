import { fromScreen, type RenderCtx } from "./renderer";

export type DrawingCallback = (points: [number, number][]) => void;

export interface DrawingTool {
  points: [number, number][];
  isDrawing: boolean;
  enable(): void;
  disable(): void;
  clear(): void;
  setOnChange(cb: DrawingCallback): void;
  setOnComplete(cb: DrawingCallback): void;
}

const SNAP_DISTANCE_PX = 12;

export function createDrawingTool(
  canvas: HTMLCanvasElement,
  getRenderCtx: () => RenderCtx
): DrawingTool {
  let points: [number, number][] = [];
  let isDrawing = false;
  let onChange: DrawingCallback | null = null;
  let onComplete: DrawingCallback | null = null;
  let enabled = false;

  function handleClick(e: MouseEvent) {
    if (!enabled) return;
    const rect = canvas.getBoundingClientRect();
    const sx = e.clientX - rect.left;
    const sy = e.clientY - rect.top;
    const rc = getRenderCtx();
    const [wx, wy] = fromScreen(rc, sx, sy);

    if (points.length >= 3) {
      const [firstSx, firstSy] = [
        points[0][0] * rc.scale + rc.offsetX,
        -points[0][1] * rc.scale + rc.offsetY,
      ];
      const dx = sx - firstSx;
      const dy = sy - firstSy;
      if (Math.sqrt(dx * dx + dy * dy) < SNAP_DISTANCE_PX) {
        isDrawing = false;
        onComplete?.(points.slice());
        return;
      }
    }

    isDrawing = true;
    points.push([wx, wy]);
    onChange?.(points.slice());
  }

  function handleRightClick(e: MouseEvent) {
    e.preventDefault();
    if (!enabled || points.length === 0) return;

    if (points.length >= 3) {
      isDrawing = false;
      onComplete?.(points.slice());
    }
  }

  const tool: DrawingTool = {
    get points() { return points; },
    get isDrawing() { return isDrawing; },

    enable() {
      if (enabled) return;
      enabled = true;
      canvas.addEventListener("click", handleClick);
      canvas.addEventListener("contextmenu", handleRightClick);
    },

    disable() {
      enabled = false;
      canvas.removeEventListener("click", handleClick);
      canvas.removeEventListener("contextmenu", handleRightClick);
    },

    clear() {
      points = [];
      isDrawing = false;
      onChange?.(points.slice());
    },

    setOnChange(cb) { onChange = cb; },
    setOnComplete(cb) { onComplete = cb; },
  };

  return tool;
}
