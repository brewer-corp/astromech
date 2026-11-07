import { mount } from "svelte";
import App from "./App.svelte";
import "./design/tokens.css";
import { inTauri } from "./lib/transport";

const target = document.getElementById("app");
if (!target) throw new Error("#app missing");

// A preview build (scripts/preview-publish) opens on its page: an artifact link can't carry one.
const start = import.meta.env.VITE_PREVIEW_PAGE;
if (start && !location.hash) history.replaceState(null, "", `#/${start}`);

// Design mockups (docs/V1-DESIGN.md §3) live at #/mockup/<id>; everything else is the app.
// Outside Tauri in `pnpm dev` and a preview build, the browser preview's fake answers the window's
// calls (lib/transport.ts): loaded before anything calls. A plain `pnpm build` drops it.
const preview = !inTauri() && (import.meta.env.DEV || import.meta.env.VITE_PREVIEW) ? import("./preview/transport") : Promise.resolve();

const app = preview.then(() =>
  location.hash.startsWith("#/mockup/") ? import("./mockups/Mockups.svelte").then((m) => mount(m.default, { target })) : mount(App, { target }),
);

export default app;
