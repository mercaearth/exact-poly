type Listener = () => void;

let polygon: [number, number][] = [];
let notifying = false;
const listeners: Set<Listener> = new Set();

export function getPolygon(): [number, number][] {
  return polygon;
}

export function setPolygon(pts: [number, number][]) {
  polygon = pts;
  if (notifying) return;
  notifying = true;
  for (const fn of listeners) fn();
  notifying = false;
}

export function onPolygonChange(fn: Listener): () => void {
  listeners.add(fn);
  return () => listeners.delete(fn);
}
