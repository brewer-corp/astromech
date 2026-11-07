import js from "@eslint/js";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import ts from "typescript-eslint";

export default ts.config(
  { ignores: ["dist/", "dist-preview/", "src-tauri/", "node_modules/"] },
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs.recommended,
  { languageOptions: { globals: { ...globals.browser } } },
  {
    files: ["**/*.svelte", "**/*.svelte.ts"],
    languageOptions: { parserOptions: { parser: ts.parser, extraFileExtensions: [".svelte"] } },
  },
  {
    // The window reaches its backend only through lib/transport.ts: the browser preview's fake
    // (src/preview/) and Tauri's API stay behind it, so a release build carries none of the fake.
    files: ["src/**/*.ts", "src/**/*.svelte"],
    ignores: ["src/preview/**", "src/lib/transport.ts", "src/**/*.test.ts"],
    rules: {
      "no-restricted-imports": [
        "error",
        {
          patterns: [
            { group: ["**/preview/*", "**/fakecore", "**/previewquery", "**/fixtures/*"], message: "Reach the browser preview through lib/transport.ts (call, subscribe, previewOption)." },
            { group: ["@tauri-apps/api/*"], message: "Reach the shell through lib/transport.ts (call, subscribe)." },
          ],
        },
      ],
      "no-restricted-syntax": [
        "error",
        { selector: "Literal[value='__TAURI_INTERNALS__']", message: "Only lib/transport.ts tells the app from the browser preview." },
      ],
    },
  },
);
