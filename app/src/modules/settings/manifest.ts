import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "settings",
  group: "settings",
  title: "General",
  summary: "Start with Windows, push-to-talk, speech models, updates",
  order: 0,
  api: [],
  load: () => import("./View.svelte"),
};

export default manifest;
