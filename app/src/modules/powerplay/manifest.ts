import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "powerplay",
  group: "powerplay",
  title: "Powerplay",
  summary: "Your Powerplay standing and merits from the journal",
  order: 0,
  api: ["powerplay"],
  load: () => import("./View.svelte"),
};

export default manifest;
