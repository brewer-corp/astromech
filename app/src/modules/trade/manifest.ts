import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "trade",
  group: "trade",
  title: "Commodities",
  summary: "Where to sell or buy a commodity near you or anywhere, best price first (Ardent Insight), and trade routes (Spansh)",
  help: "trade.md",
  order: 10,
  api: ["trade"],
  load: () => import("./View.svelte"),
};

export default manifest;
