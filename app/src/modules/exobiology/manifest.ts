import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "exobiology",
  group: "galaxy",
  title: "Biology",
  summary: "The system's biology ranked by value: genera, likely species and their worth, what others reported, and how far you are from your last sample",
  order: 40,
  api: ["exobiology"],
  load: () => import("./View.svelte"),
};

export default manifest;
