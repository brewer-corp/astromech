import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { svelteTesting } from "@testing-library/svelte/vite";

// The Tauri shell loads dist/ (production) or this dev server (tauri dev).
export default defineConfig({
  plugins: [svelte(), svelteTesting()],
  clearScreen: false,
  // fs.allow: the repository root, so the data catalogue and the guide (docs/guide, read by the help-link test) import from outside app/.
  server: { port: 5173, strictPort: true, fs: { allow: [".."] } },
  test: { environment: "jsdom", include: ["src/**/*.test.ts"] },
  build: { target: "es2022", outDir: "dist", emptyOutDir: true },
});
