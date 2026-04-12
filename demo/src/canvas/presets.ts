export interface Preset {
  name: string;
  points: [number, number][];
}

export const PRESETS: Preset[] = [
  {
    name: "Triangle",
    points: [[0, 0], [100, 0], [50, 80]],
  },
  {
    name: "Square",
    points: [[0, 0], [100, 0], [100, 100], [0, 100]],
  },
  {
    name: "Convex Pentagon",
    points: [[50, 0], [100, 35], [80, 90], [20, 90], [0, 35]],
  },
  {
    name: "L-Shape",
    points: [[0, 0], [60, 0], [60, 40], [30, 40], [30, 80], [0, 80]],
  },
  {
    name: "Star",
    points: [
      [50, 0], [62, 35], [100, 35], [70, 57],
      [80, 95], [50, 72], [20, 95], [30, 57],
      [0, 35], [38, 35],
    ],
  },
  {
    name: "Arrow",
    points: [[0, 30], [60, 30], [60, 0], [100, 50], [60, 100], [60, 70], [0, 70]],
  },
  {
    name: "U-Shape",
    points: [[0, 0], [100, 0], [100, 80], [70, 80], [70, 30], [30, 30], [30, 80], [0, 80]],
  },
  {
    name: "Zigzag",
    points: [[0, 0], [30, 50], [60, 0], [90, 50], [120, 0], [120, 40], [0, 40]],
  },
  {
    name: "Diamond",
    points: [[50, 0], [100, 50], [50, 100], [0, 50]],
  },
  {
    name: "Concave Hexagon",
    points: [[20, 0], [80, 0], [100, 50], [80, 100], [20, 100], [40, 50]],
  },
];

// Shapes that fail on-chain validation (for testing debug panel diagnostics)
export const INVALID_PRESETS: Preset[] = [
  {
    // 100m x 1m needle — fails boundary compactness (isoperimetric ratio too low)
    name: "Needle (compactness)",
    points: [[0, 0], [100, 0], [100, 1], [0, 1]],
  },
  {
    // All edges < 1m — fails MIN_EDGE_LENGTH_SQUARED
    name: "Tiny (edge length)",
    points: [[0, 0], [0.5, 0], [0.5, 0.5], [0, 0.5]],
  },
  {
    // Very thin spike — fails compactness + produces problematic decomposition
    name: "Thin Spike (compactness)",
    points: [[0, 0], [80, 0], [80, 5], [40, 2], [0, 5]],
  },
  {
    // 200m x 0.5m sliver — extreme aspect ratio, fails compactness
    name: "Sliver (compactness)",
    points: [[0, 0], [200, 0], [200, 0.5], [0, 0.5]],
  },
  {
    // Complex shape with many narrow corridors — decomposition may exceed MAX_PARTS
    name: "Many Corridors (parts)",
    points: [
      [0, 0], [100, 0], [100, 10], [20, 10], [20, 20], [100, 20],
      [100, 30], [20, 30], [20, 40], [100, 40], [100, 50], [20, 50],
      [20, 60], [100, 60], [100, 70], [0, 70], [0, 60], [10, 60],
      [10, 50], [0, 50], [0, 40], [10, 40], [10, 30], [0, 30],
      [0, 20], [10, 20], [10, 10], [0, 10],
    ],
  },
];

// Topology presets with pre-defined parts (bypass decomposer to demonstrate
// specific topology validation failures that the decomposer would never produce).
export interface TopologyPreset {
  name: string;
  parts: [number, number][][];
}

export const TOPOLOGY_PRESETS: TopologyPreset[] = [
  {
    // Rectangle base + triangle roof — triangle bottom edge is a sub-segment
    // of rectangle top edge. On-chain aborts EInvalidMultipartContact.
    name: "T-junction (partial edge)",
    parts: [
      [[0, 0], [40, 0], [40, 10], [0, 10]],
      [[10, 10], [30, 10], [20, 20]],
    ],
  },
  {
    // Two squares touching only at a corner — vertex-only contact.
    name: "Vertex-only contact",
    parts: [
      [[0, 0], [10, 0], [10, 10], [0, 10]],
      [[10, 10], [20, 10], [20, 20], [10, 20]],
    ],
  },
  {
    // Two squares with no contact at all.
    name: "Disconnected parts",
    parts: [
      [[0, 0], [10, 0], [10, 10], [0, 10]],
      [[30, 0], [40, 0], [40, 10], [30, 10]],
    ],
  },
  {
    // Valid L-shape with exact shared edge (vertex at junction point).
    name: "Valid L-shape (shared edge)",
    parts: [
      [[0, 0], [20, 0], [20, 10], [10, 10], [0, 10]],
      [[0, 10], [10, 10], [10, 20], [0, 20]],
    ],
  },
];

export const OVERLAP_PRESETS: { name: string; a: [number, number][]; b: [number, number][] }[] = [
  {
    name: "Two Squares (overlapping)",
    a: [[0, 0], [60, 0], [60, 60], [0, 60]],
    b: [[30, 30], [90, 30], [90, 90], [30, 90]],
  },
  {
    name: "Two Squares (separate)",
    a: [[0, 0], [40, 0], [40, 40], [0, 40]],
    b: [[60, 0], [100, 0], [100, 40], [60, 40]],
  },
  {
    name: "Triangle + Square",
    a: [[0, 0], [80, 0], [80, 80], [0, 80]],
    b: [[40, 20], [120, 50], [40, 80]],
  },
  {
    name: "Touching edges",
    a: [[0, 0], [50, 0], [50, 50], [0, 50]],
    b: [[50, 0], [100, 0], [100, 50], [50, 50]],
  },
];
