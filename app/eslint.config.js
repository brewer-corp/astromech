import js from "@eslint/js";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import ts from "typescript-eslint";
import tests from "./eslint-rules/tests.js";

const PREVIEW = {
  group: ["**/preview", "**/preview/*", "**/fakecore", "**/previewquery", "**/fixtures/*"],
  message: "Reach the browser preview through lib/transport.ts (call, subscribe, previewOption).",
};
const TAURI = { group: ["@tauri-apps/api", "@tauri-apps/api/*"], message: "Reach the shell through lib/transport.ts (call, subscribe)." };
const BACKDOORS = [
  // The same paths as no-restricted-imports, which doesn't see import().
  {
    selector: "ImportExpression[source.value=/preview|fakecore|fixtures|^@tauri-apps/]",
    message: "Reach the browser preview and the shell through lib/transport.ts (call, subscribe, previewOption).",
  },
  { selector: "Literal[value='__TAURI_INTERNALS__']", message: "Only lib/transport.ts tells the app from the browser preview." },
  { selector: "MemberExpression[property.name='__TAURI_INTERNALS__']", message: "Only lib/transport.ts tells the app from the browser preview." },
];

export default ts.config(
  {
    ignores: [
      "dist/",
      "dist-preview/",
      "dist-shots/",
      "src-tauri/",
      "node_modules/",
      // ts-rs's output (astromech-se44): generated from the Rust command layer, not hand-written.
      // src/lib/api/commands.ts, generated beside it, is linted.
      "src/lib/api/types/",
    ],
  },
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
    ignores: ["src/preview/**", "src/lib/transport.ts", "src/test/**", "src/**/*.test.ts"],
    rules: {
      "no-restricted-imports": ["error", { patterns: [PREVIEW, TAURI] }],
      "no-restricted-syntax": ["error", ...BACKDOORS],
    },
  },
  {
    // Tests reach the fake through src/test/transport.ts too (answer, watch, previewAnswer,
    // startPreview). They may read its fixtures (recorded inputs, as data) and drive the fake
    // world's own events: a jump in game, a galaxy-map plot, a refused clipboard, the voice
    // previews played. The preview's own tests (src/preview/) and the three that fake Tauri's
    // invoke to test the app's side of the transport (the shell's proxy) are exempt.
    files: ["src/**/*.test.ts"],
    ignores: [
      "src/preview/**",
      "src/lib/proxy.browser.test.ts",
      "src/lib/ui/staleTauri.browser.test.ts",
      "src/lib/ui/tauriFailure.browser.test.ts",
    ],
    rules: {
      "no-restricted-imports": [
        "error",
        {
          patterns: [
            { group: ["**/preview/fakecore"], allowImportNames: ["fakeJump", "fakePlotInGame", "fakeClipboard", "fakePreviews", "Scenario"], message: "Reach the fake through src/test/transport.ts (answer, watch, previewAnswer, startPreview)." },
            { group: ["**/preview/transport", "**/preview/shell", "**/preview/binds", "**/preview/query", "**/previewquery"], message: "Reach the fake through src/test/transport.ts (answer, watch, previewAnswer, startPreview)." },
            TAURI,
          ],
        },
      ],
      "no-restricted-syntax": ["error", ...BACKDOORS],
    },
  },
  {
    // Tests wait on events, never on time (CONTRIBUTING.md, Tests).
    files: ["src/**/*.test.ts"],
    plugins: { astromech: tests },
    rules: { "astromech/no-timer-wait": "error", "astromech/no-wall-clock": "error" },
  },
);
