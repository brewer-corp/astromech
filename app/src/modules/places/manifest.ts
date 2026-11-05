import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "places",
  group: "galaxy",
  title: "Places",
  summary: "Finders for where to sell or buy a commodity or sell the hold, and the nearest service, module, ship, material trader, raw site, faction state or mining hotspot",
  help: "places.md",
  order: 0,
  api: ["stations", "trade", "galaxy"],
  load: () => import("./View.svelte"),
  palette: async () => (await import("./palette")).default(),
};

export default manifest;
