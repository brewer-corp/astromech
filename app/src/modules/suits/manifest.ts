import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "suits",
  group: "ships",
  title: "Suits",
  summary: "Suit and weapon blueprints with what each is missing, one shopping list for them all, and the bartender trade that covers it",
  help: "ships.md#suits",
  order: 10,
  api: ["odyssey"],
  load: () => import("./View.svelte"),
};

export default manifest;
