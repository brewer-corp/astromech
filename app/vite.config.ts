import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// The Tauri shell loads dist/ (production) or this dev server (tauri dev).
export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: { port: 5173, strictPort: true },
  build: { target: "es2022", outDir: "dist", emptyOutDir: true },
});
