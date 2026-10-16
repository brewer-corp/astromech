import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "unlocks",
  group: "engineering",
  title: "Engineers",
  summary: "How to unlock every ship and suit engineer, as a tech tree with your progress and what to do next",
  order: 20,
  api: ["engineering"],
  load: () => import("./View.svelte"),
};

export default manifest;
