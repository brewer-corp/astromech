import js from "@eslint/js";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import ts from "typescript-eslint";
import tests from "./eslint-rules/tests.js";
import pageSizing from "./eslint-rules/pageSizing.js";

/**
 * The page-sizing check (DESIGN.md Page layouts): the page files it is on, and which of its parts
 * (`width`, `font-size`). A page is enrolled when it is converted to a layout, a part at a time;
 * by the end of mission astromech-xajc every page file outside PAGE_SIZING_EXEMPT is listed.
 * An entry is a file or a glob under src/modules/.
 */
export const PAGE_SIZING = [
  // astromech-mwh5: one page per layout.
  ["src/modules/materials/View.svelte", "width", "font-size"],
  ["src/modules/places/FinderPage.svelte", "width"],
  ["src/modules/places/{Market,Cargo,Services,Outfitting,Shipyard,Traders,RawSites,FactionStates,Hotspots}.svelte", "width"],
  ["src/modules/trade/{View,Route}.svelte", "width"],
  ["src/modules/settings/View.svelte", "width", "font-size"],
  // astromech-z5oh: the files that name type roles and set no font size of their own.
  ["src/modules/places/Home.svelte", "font-size"],
  ["src/modules/settings/{Callouts,ProspectorThresholds}.svelte", "font-size"],
  ["src/modules/{session,mining}/View.svelte", "font-size"],
  // astromech-irrv: the Route page's title, the plotter.
  ["src/modules/route/{Title,PlotSetup,ImportDialog}.svelte", "width", "font-size"],
  // astromech-qwvs: the Route page's timeline.
  ["src/modules/route/{View,Timeline}.svelte", "width", "font-size"],
  // astromech-ok29: the neutron star file's status, in the title's Worked out dropdown.
  ["src/modules/route/NeutronFile.svelte", "width", "font-size"],
];
/**
 * Never checked: the Controls pages, which keep their own sizes (DESIGN.md), and the frame's parts
 * that live in the route module, which are not pages.
 */
export const PAGE_SIZING_EXEMPT = [
  "src/modules/{controller,keybinds,presets,keytester}/**",
  "src/modules/route/Strip.svelte",
  "src/modules/route/PlotBar.svelte",
];

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
    // main.ts loads the preview outside Tauri in `pnpm dev` and a preview build.
    ignores: ["src/preview/**", "src/lib/transport.ts", "src/main.ts", "src/test/**", "src/**/*.test.ts"],
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
    // invoke to test the app's side of the transport (the shell's proxy) are exempt, and so is
    // lib/transport.test.ts, which loads the preview and lib/transport together.
    files: ["src/**/*.test.ts"],
    ignores: [
      "src/preview/**",
      "src/lib/proxy.browser.test.ts",
      "src/lib/transport.test.ts",
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
  {
    // The journeys and their harness too, and they wait on the page or the core, never on
    // Playwright's timers (CONTRIBUTING.md, Tests: journeys).
    files: ["journeys/**/*.ts"],
    plugins: { astromech: tests },
    languageOptions: { globals: { ...globals.node } },
    rules: { "astromech/no-timer-wait": "error", "astromech/no-wall-clock": "error", "astromech/no-playwright-time": "error" },
  },
  {
    // The harness installs and pauses the page's clock; journeys only run it (runFor).
    files: ["journeys/harness.ts"],
    rules: { "astromech/no-playwright-time": ["error", { clock: true }] },
  },
  ...PAGE_SIZING.map(([files, ...parts]) => ({
    files: [files],
    ignores: PAGE_SIZING_EXEMPT,
    plugins: { "page-sizing": pageSizing },
    rules: Object.fromEntries(parts.map((p) => [`page-sizing/${p}`, "error"])),
  })),
);
