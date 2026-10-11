import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "materials",
  title: "Materials",
  summary: "Engineering materials as the three in-game traders show them",
  order: 25,
  api: ["materials"],
  load: () => import("./View.svelte"),
};

export default manifest;
