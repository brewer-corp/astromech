import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "now",
  group: "now",
  title: "Now",
  summary: "What you are doing: the station and its services, or the system; what the hold would fetch, this session, Powerplay and a mining run",
  help: "now.md",
  order: 0,
  api: ["history", "trade", "core"],
  load: () => import("./View.svelte"),
};

export default manifest;
