import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "session",
  group: "logbook",
  title: "Session",
  summary: "This session: credits by activity and per hour, jumps, distance, bounties, data sold",
  order: 10,
  api: ["history"],
  load: () => import("./View.svelte"),
};

export default manifest;
