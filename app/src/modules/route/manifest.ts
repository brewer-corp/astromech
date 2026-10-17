import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "route",
  group: "galaxy",
  title: "Route",
  summary: "The route you're following: its next waypoint is on the clipboard for the galaxy map",
  order: 30,
  api: ["route"],
  load: () => import("./View.svelte"),
};

export default manifest;
