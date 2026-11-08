import { defineConfig } from "vitest/config";
import { playwright } from "@vitest/browser-playwright";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { svelteTesting } from "@testing-library/svelte/vite";
import { astromechBridge } from "./dev/bridge.ts";
import pkg from "./package.json" with { type: "json" };

// The app's window (astromech-core.exe, app/src-tauri) loads dist/ (production) or this dev
// server (tauri dev). `ASTROMECH_WORLD=<dir> pnpm dev` adds the dev bridge to the headless core
// (dev/bridge.ts).
export default defineConfig({
  plugins: [svelte(), svelteTesting({ resolveBrowser: false }), astromechBridge()],
  // The window's version (lib/core.ts WINDOW_VERSION), shown by Diagnostics: the bundle ships
  // inside the app's one exe, so it is the app's version.
  define: { __APP_VERSION__: JSON.stringify(pkg.version) },
  clearScreen: false,
  server: { port: 5173, strictPort: true },
  test: {
    projects: [
      // Pure logic: parsers, maths, the preview's fake. No DOM.
      {
        extends: true,
        test: { name: "node", environment: "node", include: ["src/**/*.test.ts", "dev/**/*.test.ts", "eslint-rules/*.test.js"], exclude: ["src/**/*.browser.test.ts"], setupFiles: ["src/test/preview.ts"] },
      },
      // UI behaviour: pages rendered in headless Chromium behind the fake transport (CONTRIBUTING.md, Tests).
      {
        extends: true,
        test: {
          name: "browser",
          include: ["src/**/*.browser.test.ts"],
          setupFiles: ["src/test/setup.ts", "src/test/preview.ts"],
          browser: {
            enabled: true,
            headless: true,
            provider: playwright(),
            instances: [{ browser: "chromium" }],
            // The app window's opening size (src-tauri/src/host.rs), not Vitest's phone-sized default.
            viewport: { width: 1200, height: 800 },
            screenshotFailures: false,
          },
        },
      },
    ],
  },
  build: { target: "es2022", outDir: "dist", emptyOutDir: true },
});
