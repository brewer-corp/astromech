import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "places",
  group: "galaxy",
  title: "Places",
  summary: "Finders for a service, module, ship, material trader or mining hotspot near you",
  help: "places.md",
  order: 0,
  api: ["stations"],
  load: () => import("./View.svelte"),
  palette: async () => (await import("./palette")).default(),
};

export default manifest;
