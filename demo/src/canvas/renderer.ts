const PART_COLORS = [
  "#4a9eff", "#ff6b6b", "#51cf66", "#fcc419", "#cc5de8",
  "#20c997", "#ff922b", "#a9e34b", "#e599f7", "#66d9e8",
];

const GRID_COLOR = "#1a1a1a";
const AXIS_COLOR = "#2a2a2a";
const VERTEX_RADIUS = 4;
const EDGE_WIDTH = 2;

export interface RenderCtx {
  ctx: CanvasRenderingContext2D;
  width: number;
  height: number;
  scale: number;
  offsetX: number;
  offsetY: number;
}

export function createRenderCtx(canvas: HTMLCanvasElement): RenderCtx {
  const ctx = canvas.getContext("2d")!;
  const dpr = window.devicePixelRatio || 1;
  const rect = canvas.getBoundingClientRect();
  canvas.width = rect.width * dpr;
  canvas.height = rect.height * dpr;
  ctx.scale(dpr, dpr);

  return {
    ctx,
    width: rect.width,
    height: rect.height,
    scale: 1,
    offsetX: rect.width / 2,
    offsetY: rect.height / 2,
  };
}

export function fitToPolygons(
  rc: RenderCtx,
  polygons: [number, number][][],
  padding = 40
) {
  const all = polygons.flat();
  if (all.length === 0) return;

  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
  for (const [x, y] of all) {
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
  }

  const rangeX = maxX - minX || 1;
  const rangeY = maxY - minY || 1;
  const availW = rc.width - padding * 2;
  const availH = rc.height - padding * 2;

  rc.scale = Math.min(availW / rangeX, availH / rangeY);
  rc.offsetX = padding + (availW - rangeX * rc.scale) / 2 - minX * rc.scale;
  rc.offsetY = padding + (availH - rangeY * rc.scale) / 2 + maxY * rc.scale;
}

export function toScreen(rc: RenderCtx, x: number, y: number): [number, number] {
  return [x * rc.scale + rc.offsetX, -y * rc.scale + rc.offsetY];
}

export function fromScreen(rc: RenderCtx, sx: number, sy: number): [number, number] {
  return [
    Math.round((sx - rc.offsetX) / rc.scale),
    Math.round(-(sy - rc.offsetY) / rc.scale),
  ];
}

export function clear(rc: RenderCtx) {
  rc.ctx.clearRect(0, 0, rc.width, rc.height);
}

export function drawGrid(rc: RenderCtx, step = 50) {
  const { ctx, width, height } = rc;

  ctx.strokeStyle = GRID_COLOR;
  ctx.lineWidth = 0.5;
  const worldStep = step;
  const screenStep = worldStep * rc.scale;

  if (screenStep < 10) return;

  const startX = rc.offsetX % screenStep;
  const startY = rc.offsetY % screenStep;

  ctx.beginPath();
  for (let x = startX; x < width; x += screenStep) {
    ctx.moveTo(x, 0);
    ctx.lineTo(x, height);
  }
  for (let y = startY; y < height; y += screenStep) {
    ctx.moveTo(0, y);
    ctx.lineTo(width, y);
  }
  ctx.stroke();

  ctx.strokeStyle = AXIS_COLOR;
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(rc.offsetX, 0);
  ctx.lineTo(rc.offsetX, height);
  ctx.moveTo(0, rc.offsetY);
  ctx.lineTo(width, rc.offsetY);
  ctx.stroke();
}

export function drawPolygon(
  rc: RenderCtx,
  points: [number, number][],
  options: {
    fill?: string;
    stroke?: string;
    lineWidth?: number;
    showVertices?: boolean;
    vertexColor?: string;
    closed?: boolean;
    dashed?: boolean;
  } = {}
) {
  if (points.length < 2) return;
  const { ctx } = rc;
  const {
    fill,
    stroke = "#4a9eff",
    lineWidth = EDGE_WIDTH,
    showVertices = true,
    vertexColor,
    closed = true,
    dashed = false,
  } = options;

  ctx.beginPath();
  if (dashed) ctx.setLineDash([6, 4]);
  const [sx, sy] = toScreen(rc, points[0][0], points[0][1]);
  ctx.moveTo(sx, sy);
  for (let i = 1; i < points.length; i++) {
    const [px, py] = toScreen(rc, points[i][0], points[i][1]);
    ctx.lineTo(px, py);
  }
  if (closed) ctx.closePath();

  if (fill) {
    ctx.fillStyle = fill;
    ctx.globalAlpha = 0.15;
    ctx.fill();
    ctx.globalAlpha = 1;
  }

  ctx.strokeStyle = stroke;
  ctx.lineWidth = lineWidth;
  ctx.stroke();
  ctx.setLineDash([]);

  if (showVertices) {
    for (const [x, y] of points) {
      const [px, py] = toScreen(rc, x, y);
      ctx.beginPath();
      ctx.arc(px, py, VERTEX_RADIUS, 0, Math.PI * 2);
      ctx.fillStyle = vertexColor || stroke;
      ctx.fill();
    }
  }
}

export function drawPoint(
  rc: RenderCtx,
  x: number,
  y: number,
  color = "#fff",
  radius = VERTEX_RADIUS
) {
  const [sx, sy] = toScreen(rc, x, y);
  rc.ctx.beginPath();
  rc.ctx.arc(sx, sy, radius, 0, Math.PI * 2);
  rc.ctx.fillStyle = color;
  rc.ctx.fill();
}

export function drawLabel(
  rc: RenderCtx,
  x: number,
  y: number,
  text: string,
  color = "#aaa",
  offsetPx: [number, number] = [8, -8]
) {
  const [sx, sy] = toScreen(rc, x, y);
  rc.ctx.font = "11px SF Mono, Fira Code, monospace";
  rc.ctx.fillStyle = color;
  rc.ctx.fillText(text, sx + offsetPx[0], sy + offsetPx[1]);
}

export function drawArrowHead(
  rc: RenderCtx,
  from: [number, number],
  to: [number, number],
  color = "#4a9eff",
  size = 8
) {
  const [fx, fy] = toScreen(rc, from[0], from[1]);
  const [tx, ty] = toScreen(rc, to[0], to[1]);
  const angle = Math.atan2(ty - fy, tx - fx);
  const { ctx } = rc;

  const midX = (fx + tx) / 2;
  const midY = (fy + ty) / 2;

  ctx.beginPath();
  ctx.moveTo(midX + size * Math.cos(angle), midY + size * Math.sin(angle));
  ctx.lineTo(
    midX - size * Math.cos(angle - Math.PI / 6),
    midY - size * Math.sin(angle - Math.PI / 6)
  );
  ctx.lineTo(
    midX - size * Math.cos(angle + Math.PI / 6),
    midY - size * Math.sin(angle + Math.PI / 6)
  );
  ctx.closePath();
  ctx.fillStyle = color;
  ctx.fill();
}

export function drawSegment(
  rc: RenderCtx,
  a: [number, number],
  b: [number, number],
  color = "#4a9eff",
  lineWidth = 2,
  dashed = false
) {
  const [ax, ay] = toScreen(rc, a[0], a[1]);
  const [bx, by] = toScreen(rc, b[0], b[1]);
  const { ctx } = rc;

  if (dashed) ctx.setLineDash([6, 4]);
  ctx.beginPath();
  ctx.moveTo(ax, ay);
  ctx.lineTo(bx, by);
  ctx.strokeStyle = color;
  ctx.lineWidth = lineWidth;
  ctx.stroke();
  ctx.setLineDash([]);
}

export function partColor(index: number): string {
  return PART_COLORS[index % PART_COLORS.length];
}

export function drawParts(
  rc: RenderCtx,
  parts: [number, number][][],
  options: { showVertices?: boolean } = {}
) {
  for (let i = 0; i < parts.length; i++) {
    const color = partColor(i);
    drawPolygon(rc, parts[i], {
      fill: color,
      stroke: color,
      showVertices: options.showVertices ?? true,
    });
  }
}
