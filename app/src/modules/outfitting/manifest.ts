import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "outfitting",
  group: "galaxy",
  title: "Outfitting",
  summary: "The nearest stations selling a module, by class, rating and mount",
  order: 10,
  api: ["stations"],
  load: () => import("./View.svelte"),
  palette: async () => (await import("./palette")).default(),
};

export default manifest;
