import type { ModuleManifest } from "../../lib/modules";

const manifest: ModuleManifest = {
  id: "voicetester",
  group: "settings",
  title: "Voice tester",
  summary: "Try a voice command: what it heard, what it matched, the keys it would send",
  help: "voice-tester.md",
  order: 20,
  api: ["core"],
  load: () => import("./View.svelte"),
};

export default manifest;
