import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "station",
  group: "now",
  title: "Station",
  summary: "The station you are docked at, its market against nearby prices, and where you saw a module before (offline)",
  help: "now.md#station",
  order: 10,
  api: ["history", "trade"],
  load: () => import("./View.svelte"),
};

export default manifest;
