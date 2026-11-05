import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "materials",
  group: "engineering",
  title: "Materials",
  summary: "Engineering materials as the traders show them and where each comes from, with links to the Raw sites and Faction states finders",
  help: "materials.md",
  order: 10,
  api: ["materials"],
  load: () => import("./View.svelte"),
  palette: async () => (await import("./palette")).default(),
};

export default manifest;
