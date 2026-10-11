import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "keybinds",
  title: "All keybinds",
  summary: "Every binding: search, press a control to find it, inline health",
  order: 10,
  api: ["binds"],
  load: () => import("./KeybindsView.svelte"),
};

export default manifest;
