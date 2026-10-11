import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "controller",
  title: "My controller",
  summary: "My controller",
  order: 20,
  api: ["binds"],
  load: () => import("./View.svelte"),
};

export default manifest;
