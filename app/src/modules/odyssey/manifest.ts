import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "odyssey",
  group: "engineering",
  title: "On foot",
  summary: "Odyssey micro-resources: ShipLocker inventory, suit and weapon blueprints with what's missing, on-foot engineers and bartender values",
  order: 20,
  api: ["odyssey"],
  load: () => import("./View.svelte"),
};

export default manifest;
