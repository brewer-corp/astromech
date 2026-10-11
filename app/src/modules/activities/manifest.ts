import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "activities",
  title: "Activities",
  summary: "Activities",
  order: 0,
  api: [],
  load: () => import("./View.svelte"),
};

export default manifest;
