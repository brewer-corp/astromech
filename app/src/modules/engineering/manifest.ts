import type { ModuleManifest } from "../../lib/modules";

// The id stays "engineering" so the old #/engineering route opens this page.
const manifest: ModuleManifest = {
  id: "engineering",
  group: "engineering",
  title: "Ships",
  summary: "Your fleet at a glance, the modules in storage, and a build plan for any ship: materials, trades, farming and engineer trips",
  help: "ships.md",
  order: 0,
  api: ["engineering"],
  load: () => import("./View.svelte"),
  palette: async () => (await import("./palette")).default(),
};

export default manifest;
