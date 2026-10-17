import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "controller",
  group: "controls",
  title: "My controller",
  summary: "One stick as its maker drew it: every binding, or one activity with what is not bound yet",
  help: "controls.md#my-controller",
  order: 0,
  api: ["binds"],
  load: () => import("./View.svelte"),
};

export default manifest;
