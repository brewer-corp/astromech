import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "engineering",
  group: "engineering",
  title: "Builds",
  summary: "Plan a build for any ship in your fleet and see the materials it needs, what to trade and what to farm",
  order: 0,
  api: ["engineering"],
  load: () => import("./View.svelte"),
};

export default manifest;
