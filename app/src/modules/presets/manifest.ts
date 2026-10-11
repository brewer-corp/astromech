import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "presets",
  title: "Presets",
  summary: "Every preset, which is active where, health, copies, backups and restore",
  order: 30,
  api: ["binds"],
  load: () => import("./View.svelte"),
};

export default manifest;
