// Both test projects' setup (vite.config.ts): outside Tauri the window's calls go to the browser
// preview's fake, as in `pnpm dev`, where main.ts loads it (lib/transport.ts never imports it).
import "../preview/transport";
