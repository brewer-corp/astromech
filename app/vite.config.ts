import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { svelteTesting } from "@testing-library/svelte/vite";

// The Tauri shell loads dist/ (production) or this dev server (tauri dev).
export default defineConfig({
  plugins: [svelte(), svelteTesting()],
  clearScreen: false,
  server: { port: 5173, strictPort: true },
  test: { environment: "jsdom", include: ["src/**/*.test.ts"] },
  build: { target: "es2022", outDir: "dist", emptyOutDir: true },
});
