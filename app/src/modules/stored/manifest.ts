import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "stored",
  group: "engineering",
  title: "Stored modules",
  summary: "Modules in storage: engineering, where they are, and their value",
  help: "ships.md#stored-modules",
  order: 5,
  api: ["engineering"],
  load: () => import("./View.svelte"),
};

export default manifest;
