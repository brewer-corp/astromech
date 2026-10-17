import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "services",
  group: "galaxy",
  title: "Services",
  summary: "The nearest Interstellar Factors, material trader, technology broker or other station service",
  help: "galaxy.md#services",
  order: 0,
  api: ["stations"],
  load: () => import("./View.svelte"),
};

export default manifest;
