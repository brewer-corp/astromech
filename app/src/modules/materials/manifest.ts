import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "materials",
  title: "Materials",
  summary: "Engineering materials as the traders show them, plus where High Grade Emissions drop",
  order: 25,
  api: ["materials"],
  load: () => import("./View.svelte"),
};

export default manifest;
