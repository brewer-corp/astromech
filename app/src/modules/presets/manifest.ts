import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "presets",
  group: "controls",
  title: "Presets",
  summary: "Every preset, which is active where, health, copies, backups and restore",
  help: "controls.md#presets",
  order: 30,
  api: ["binds"],
  load: () => import("./View.svelte"),
};

export default manifest;
