import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "station",
  group: "logbook",
  title: "Station",
  summary: "The station you are docked at, and where you saw a module before (offline)",
  order: 20,
  api: ["history"],
  load: () => import("./View.svelte"),
};

export default manifest;
