import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "keytester",
  group: "controls",
  title: "Key tester",
  summary: "Press anything: the name Elite uses and what it's bound to",
  order: 40,
  api: ["binds"],
  load: () => import("./View.svelte"),
};

export default manifest;
