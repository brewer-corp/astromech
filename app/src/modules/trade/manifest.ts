import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "trade",
  group: "trade",
  title: "Commodities",
  summary: "Trade routes from a station (Spansh), with links to the Sell, Buy and Sell my cargo finders",
  help: "trade.md",
  order: 10,
  api: ["trade"],
  load: () => import("./View.svelte"),
};

export default manifest;
