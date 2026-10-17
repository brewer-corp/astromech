import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "colonisation",
  group: "trade",
  title: "Colonisation",
  summary: "Construction projects, what is left to buy and where to buy it near the site (Ardent Insight), and unclaimed systems open to colonise near you (Spansh)",
  order: 20,
  api: ["colonisation", "trade"],
  load: () => import("./View.svelte"),
};

export default manifest;
