import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "diagnostics",
  title: "Diagnostics",
  summary: "What the core sees, logs, Save diagnostics",
  order: 50,
  api: ["core"],
  load: () => import("./View.svelte"),
};

export default manifest;
