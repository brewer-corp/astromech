import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "activities",
  group: "controls",
  title: "Activities",
  summary: "Your sticks with one activity's actions lit, what's not bound yet, and Bind this",
  order: 0,
  api: ["binds"],
  load: () => import("./View.svelte"),
};

export default manifest;
