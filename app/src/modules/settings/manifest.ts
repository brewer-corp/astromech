import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "settings",
  title: "Settings",
  summary: "Settings",
  order: 40,
  api: [],
  load: () => import("./View.svelte"),
};

export default manifest;
