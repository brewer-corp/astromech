import js from "@eslint/js";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import ts from "typescript-eslint";
import imports from "./eslint-rules/imports.js";
import tests from "./eslint-rules/tests.js";
import pageSizing from "./eslint-rules/pageSizing.js";
import tokens from "./eslint-rules/tokens.js";
import css from "@eslint/css";

/**
 * The page-sizing check (DESIGN.md Page layouts): the page files it is on, and which of its parts
 * (`width`, `font-size`). Pages were enrolled a part at a time as mission astromech-xajc moved them
 * onto the layouts; since astromech-gvy0 both parts are on for every page file outside
 * PAGE_SIZING_EXEMPT. An entry is a file or a glob under src/modules/.
 */
export const PAGE_SIZING = [["src/modules/**/*.svelte", "width", "font-size"]];
/**
 * Never checked: the Controls pages, which keep their own sizes (DESIGN.md), and the frame's parts
 * that live in the route module, which are not pages. Since astromech-uc7i the route is frame (the
 * route sidebar, sized by App.svelte's --sidebar): its title, plot setup, timeline and neutron
 * file status lay out for the sidebar, and still use the type roles. Plot is drawn only inside
 * PlotBar, the header's Find or plot panel, so it lays out for that panel.
 */
export const PAGE_SIZING_EXEMPT = [
  "src/modules/{controller,keybinds,presets,keytester}/**",
  "src/modules/route/Strip.svelte",
  "src/modules/route/{PlotBar,Plot}.svelte",
  "src/modules/route/{Sidebar,Rail,Title,PlotSetup,Timeline,NeutronFile}.svelte",
];

/**
 * The token check (DESIGN.md Token map, eslint-rules/tokens.js): every .svelte and .css file under
 * src/ uses the design system's colours, font sizes and spacing. Off for the tokens themselves and
 * the mockups, which draw the system rather than use it.
 */
export const TOKENS = ["src/**/*.svelte", "src/**/*.css"];
export const TOKENS_IGNORED = ["src/design/tokens.css", "src/mockups/**"];
/**
 * Files a part of the token check is off for, and why. A pending entry names, in `until`, the item
 * that rebuilds the page and takes the entry off; a lasting one says why the file keeps its own
 * value. tokens.test.js fails an entry whose files no longer need it.
 */
export const TOKENS_EXEMPT = [
  // Lasting.
  {
    files: ["src/modules/{controller,keybinds,presets,keytester}/**", "src/lib/ui/{About,PresetPicker}.svelte"],
    off: ["spacing"],
    why: "Controls keep their own spacing: mission astromech-xajc gives them only the frame, Text size, colours and Tip. About and PresetPicker are only on Controls pages.",
  },
  {
    files: ["src/lib/ui/Sheet.svelte"],
    off: ["font-size", "spacing"],
    why: "The labels on a controller sheet follow the image's scale (--k), not Text size, so they are px.",
  },
];

const SCRIPT = ["**/*.{js,mjs,cjs,ts,mts,cts,svelte}"];

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
  // The script configs are for script files; a .css file has its own language (below).
  ...[js.configs.recommended, ...ts.configs.recommended, ...svelte.configs.recommended, { languageOptions: { globals: { ...globals.browser } } }].map(
    (c) => ({ ...c, files: c.files ?? SCRIPT }),
  ),
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
    // Windows ignores case in file names, so an import that names a file only by another
    // case opens a different file there (eslint-rules/imports.js).
    files: SCRIPT,
    plugins: { imports },
    rules: { "imports/no-case-clash": "error" },
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
  { files: ["**/*.css"], language: "css/css", plugins: { css } },
  { files: TOKENS, ignores: TOKENS_IGNORED, plugins: { tokens }, rules: { "tokens/colour": "error", "tokens/font-size": "error", "tokens/spacing": "error" } },
  ...TOKENS_EXEMPT.map(({ files, off }) => ({ files, rules: Object.fromEntries(off.map((p) => [`tokens/${p}`, "off"])) })),
);
