import { mount } from "svelte";
import App from "./App.svelte";
import "./design/tokens.css";

const target = document.getElementById("app");
if (!target) throw new Error("#app missing");

// Design mockups (docs/V1-DESIGN.md §3) live at #/mockup/<id>; everything else is the app.
const app = location.hash.startsWith("#/mockup/")
  ? import("./mockups/Mockups.svelte").then((m) => mount(m.default, { target }))
  : Promise.resolve(mount(App, { target }));

export default app;
