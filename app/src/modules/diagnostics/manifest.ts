import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "diagnostics",
  group: "settings",
  title: "Diagnostics",
  summary: "What astromech sees, logs, Save diagnostics",
  help: "settings.md#diagnostics",
  order: 10,
  api: ["core"],
  load: () => import("./View.svelte"),
};

export default manifest;
