import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "materials",
  group: "engineering",
  title: "Materials",
  summary: "Engineering materials as the traders show them, plus where High Grade Emissions drop and the nearest raw-material sites",
  order: 10,
  api: ["materials"],
  load: () => import("./View.svelte"),
};

export default manifest;
