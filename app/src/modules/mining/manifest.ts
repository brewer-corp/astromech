import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "mining",
  group: "now",
  title: "Mining",
  summary: "The current mining run: tons refined by ore and tons per hour",
  help: "mining.md",
  order: 30,
  api: ["core"],
  load: () => import("./View.svelte"),
};

export default manifest;
