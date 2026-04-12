export interface Tab {
  id: string;
  label: string;
  create(): HTMLElement;
  activate(): void;
  deactivate(): void;
}

import { createDecompositionTab } from "./tabs/decomposition";
import { createAreaTab } from "./tabs/area";
import { createRingTab } from "./tabs/ring";
import { createSpatialTab } from "./tabs/spatial";
import { createOverlapTab } from "./tabs/overlap";
import { createTopologyTab } from "./tabs/topology";
import { createPrimitivesTab } from "./tabs/primitives";
import {
  getConfig,
  setConfig,
  MERCA_DEFAULTS,
  PERMISSIVE,
  type ProtocolConfig,
} from "./config";

const TABS: Tab[] = [
  createDecompositionTab(),
  createAreaTab(),
  createRingTab(),
  createSpatialTab(),
  createOverlapTab(),
  createTopologyTab(),
  createPrimitivesTab(),
];

let activeTab: Tab | null = null;

function initConfigPanel() {
  const toggle = document.getElementById("config-toggle")!;
  const panel = document.getElementById("config-panel")!;
  const preset = document.getElementById("config-preset") as HTMLSelectElement;
  const maxParts = document.getElementById("config-max-parts") as HTMLInputElement;
  const maxVerts = document.getElementById("config-max-verts") as HTMLInputElement;
  const minEdge = document.getElementById("config-min-edge") as HTMLInputElement;
  const minCompact = document.getElementById("config-min-compact") as HTMLInputElement;
  const areaDiv = document.getElementById("config-area-div") as HTMLInputElement;

  function loadToInputs(cfg: ProtocolConfig) {
    maxParts.value = String(cfg.max_parts);
    maxVerts.value = String(cfg.max_vertices_per_part);
    minEdge.value = String(cfg.min_edge_length_squared);
    minCompact.value = String(cfg.min_compactness_ppm);
    areaDiv.value = String(cfg.area_divisor);
  }

  function readFromInputs(): ProtocolConfig {
    return {
      max_parts: Number(maxParts.value) || 1,
      max_vertices_per_part: Number(maxVerts.value) || 3,
      min_edge_length_squared: Number(minEdge.value) || 0,
      min_compactness_ppm: Number(minCompact.value) || 0,
      area_divisor: Number(areaDiv.value) || 1,
    };
  }

  function detectPreset(cfg: ProtocolConfig): string {
    if (JSON.stringify(cfg) === JSON.stringify(MERCA_DEFAULTS)) return "merca";
    if (JSON.stringify(cfg) === JSON.stringify(PERMISSIVE)) return "permissive";
    return "custom";
  }

  loadToInputs(getConfig());

  toggle.addEventListener("click", () => {
    const visible = panel.style.display !== "none";
    panel.style.display = visible ? "none" : "block";
    toggle.classList.toggle("active", !visible);
  });

  preset.addEventListener("change", () => {
    if (preset.value === "merca") {
      loadToInputs(MERCA_DEFAULTS);
      setConfig(MERCA_DEFAULTS);
    } else if (preset.value === "permissive") {
      loadToInputs(PERMISSIVE);
      setConfig(PERMISSIVE);
    }
  });

  for (const input of [maxParts, maxVerts, minEdge, minCompact, areaDiv]) {
    input.addEventListener("change", () => {
      const cfg = readFromInputs();
      preset.value = detectPreset(cfg);
      setConfig(cfg);
    });
  }
}

function init() {
  const tabsNav = document.getElementById("tabs")!;
  const content = document.getElementById("content")!;

  for (const tab of TABS) {
    const btn = document.createElement("button");
    btn.className = "tab-btn";
    btn.textContent = tab.label;
    btn.dataset.tabId = tab.id;
    btn.addEventListener("click", () => switchTab(tab.id));
    tabsNav.appendChild(btn);
  }

  // Config panel
  initConfigPanel();

  if (TABS.length > 0) {
    switchTab(TABS[0].id);
  }

  function switchTab(id: string) {
    const tab = TABS.find((t) => t.id === id);
    if (!tab) return;

    if (activeTab) {
      activeTab.deactivate();
    }

    tabsNav.querySelectorAll(".tab-btn").forEach((btn) => {
      btn.classList.toggle("active", (btn as HTMLElement).dataset.tabId === id);
    });

    content.innerHTML = "";
    const el = tab.create();
    content.appendChild(el);
    tab.activate();
    activeTab = tab;
  }
}

init();
