import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "datasharing",
  group: "settings",
  title: "Data sharing",
  summary: "Share what you see in game with EDDN, and your journal with EDSM",
  order: 5,
  api: ["core"],
  load: () => import("./View.svelte"),
};

export default manifest;
