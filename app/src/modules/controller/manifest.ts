import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "controller",
  group: "controls",
  title: "My controller",
  summary: "One stick as its maker drew it, with what each control does",
  order: 20,
  api: ["binds"],
  load: () => import("./View.svelte"),
};

export default manifest;
