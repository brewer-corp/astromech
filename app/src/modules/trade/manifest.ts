import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "trade",
  group: "trade",
  title: "Commodities",
  summary: "Where to sell or buy a commodity near you, best price first (Ardent Insight)",
  order: 10,
  api: ["trade"],
  load: () => import("./View.svelte"),
};

export default manifest;
