import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "keybinds",
  title: "Keybinds",
  summary: "Profiles, search, key tester and the binds health check",
  order: 10,
  api: ["binds"],
  load: () => import("./KeybindsView.svelte"),
};

export default manifest;
