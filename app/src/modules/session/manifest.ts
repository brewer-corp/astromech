import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "session",
  group: "now",
  title: "Session",
  summary: "This session: credits by activity and per hour, jumps, distance, bounties, data sold",
  help: "logbook.md#session",
  order: 20,
  api: ["history"],
  load: () => import("./View.svelte"),
};

export default manifest;
