import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "exploration",
  group: "exploration",
  title: "System",
  summary: "The bodies of the system you are in as you scan them, with their estimated values and which are worth mapping",
  order: 0,
  api: ["exploration"],
  load: () => import("./View.svelte"),
};

export default manifest;
