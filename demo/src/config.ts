/**
 * Global ProtocolConfig state for the demo.
 * Mirrors the Rust ProtocolConfig struct.
 */

type Listener = () => void;

export interface ProtocolConfig {
  max_parts: number;
  max_vertices_per_part: number;
  min_edge_length_squared: number; // u128, fits in JS number for typical values
  min_compactness_ppm: number;     // u128
  area_divisor: number;            // u128
}

export const MERCA_DEFAULTS: ProtocolConfig = {
  max_parts: 10,
  max_vertices_per_part: 64,
  min_edge_length_squared: 1_000_000_000_000,
  min_compactness_ppm: 150_000,
  area_divisor: 2_000_000_000_000,
};

export const PERMISSIVE: ProtocolConfig = {
  max_parts: 999999,
  max_vertices_per_part: 999999,
  min_edge_length_squared: 0,
  min_compactness_ppm: 0,
  area_divisor: 1,
};

let config: ProtocolConfig = { ...MERCA_DEFAULTS };
let notifying = false;
const listeners: Set<Listener> = new Set();

export function getConfig(): ProtocolConfig {
  return config;
}

/** Returns config for passing to WASM. u128 fields must be BigInt for serde-wasm-bindgen. */
export function getConfigForWasm(): object {
  return {
    max_parts: config.max_parts,
    max_vertices_per_part: config.max_vertices_per_part,
    min_edge_length_squared: BigInt(config.min_edge_length_squared),
    min_compactness_ppm: BigInt(config.min_compactness_ppm),
    area_divisor: BigInt(config.area_divisor),
  };
}

export function setConfig(newConfig: ProtocolConfig) {
  config = { ...newConfig };
  if (notifying) return;
  notifying = true;
  for (const fn of listeners) fn();
  notifying = false;
}

export function onConfigChange(fn: Listener): () => void {
  listeners.add(fn);
  return () => listeners.delete(fn);
}
