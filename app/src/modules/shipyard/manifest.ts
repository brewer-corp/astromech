import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "shipyard",
  group: "galaxy",
  title: "Shipyard",
  summary: "The nearest shipyards selling a ship, where it can land",
  order: 20,
  api: ["stations"],
  load: () => import("./View.svelte"),
};

export default manifest;
