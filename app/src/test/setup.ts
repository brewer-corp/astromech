// The browser project's setup (vite.config.ts). A find… or waitFor gives up after this long: a
// ceiling for a loaded machine running many files at once, not a wait; it ends when the page does.
import { configure } from "@testing-library/svelte";

configure({ asyncUtilTimeout: 5000 });
