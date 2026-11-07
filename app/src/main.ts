import { mount } from "svelte";
import App from "./App.svelte";
import "./design/tokens.css";

const target = document.getElementById("app");
if (!target) throw new Error("#app missing");

// A preview build (scripts/preview-publish) opens on its page: an artifact link can't carry one.
const start = import.meta.env.VITE_PREVIEW_PAGE;
if (start && !location.hash) history.replaceState(null, "", `#/${start}`);

// Design mockups (docs/V1-DESIGN.md §3) live at #/mockup/<id>; everything else is the app.
const app = location.hash.startsWith("#/mockup/")
  ? import("./mockups/Mockups.svelte").then((m) => mount(m.default, { target }))
  : Promise.resolve(mount(App, { target }));

export default app;
