import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "presets",
  title: "Presets",
  summary: "Presets",
  order: 30,
  api: [],
  load: () => import("./View.svelte"),
};

export default manifest;
