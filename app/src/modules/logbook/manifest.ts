import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "logbook",
  group: "logbook",
  title: "Logbook",
  summary: "Every system and station you have visited, how often and when, with search",
  help: "logbook.md#logbook",
  order: 0,
  api: ["history"],
  load: () => import("./View.svelte"),
};

export default manifest;
