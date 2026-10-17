import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "exobiology",
  group: "exploration",
  title: "Biology",
  summary: "The system's biology ranked by value: genera, likely species and their worth, what others reported, and how far you are from your last sample",
  order: 10,
  api: ["exobiology"],
  load: () => import("./View.svelte"),
};

export default manifest;
