import js from "@eslint/js";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import ts from "typescript-eslint";
import tests from "./eslint-rules/tests.js";
import pageSizing from "./eslint-rules/pageSizing.js";
import tokens from "./eslint-rules/tokens.js";
import css from "@eslint/css";

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
  // astromech-8wr4: Ships > Ships as a loadout.
  ["src/modules/engineering/{View,Build,Stats,Loadout,ModulePanel}.svelte", "width", "font-size"],
  // astromech-nnn0: Ships > Suits as a loadout, on List with detail.
  ["src/modules/suits/**", "width", "font-size"],
  // astromech-irrv: the Import a route dialog, a page part opened from the frame.
  ["src/modules/route/ImportDialog.svelte", "width", "font-size"],
  // astromech-rj6i: Powerplay on the Board.
  ["src/modules/powerplay/{View,Suggestions,Targets}.svelte", "width", "font-size"],
  // astromech-m07p: Colonisation as progress per commodity, on List with detail.
  ["src/modules/colonisation/View.svelte", "width", "font-size"],
  // astromech-coq6: the Now page, and the station head and services board it draws.
  ["src/modules/now/**", "width", "font-size"],
  ["src/modules/station/{Head,Board}.svelte", "width", "font-size"],
  // astromech-7vxi: Station on the Board.
  ["src/modules/station/{View,Market}.svelte", "width", "font-size"],
];
/**
 * Never checked: the Controls pages, which keep their own sizes (DESIGN.md), and the frame's parts
 * that live in the route module, which are not pages. Since astromech-uc7i the route is frame (the
 * route sidebar, sized by App.svelte's --sidebar): its title, plot setup, timeline and neutron
 * file status lay out for the sidebar, and still use the type roles.
 */
export const PAGE_SIZING_EXEMPT = [
  "src/modules/{controller,keybinds,presets,keytester}/**",
  "src/modules/route/Strip.svelte",
  "src/modules/route/PlotBar.svelte",
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
  { files: ["src/lib/ui/Button.svelte"], off: ["spacing"], why: "DESIGN.md Buttons gives the primary button 20 px padding, a step the scale lacks." },
  {
    files: ["src/lib/ui/Sheet.svelte"],
    off: ["font-size", "spacing"],
    why: "The labels on a controller sheet follow the image's scale (--k), not Text size, so they are px.",
  },
  // Pending: the page is rebuilt on a layout by the item named, which takes it off this list.
  { files: ["src/modules/unlocks/**"], off: ["spacing"], until: "astromech-m6gw", why: "Rebuilds Engineers." },
  { files: ["src/modules/exobiology/**"], off: ["spacing"], until: "astromech-evj3", why: "Rebuilds Biology." },
  { files: ["src/modules/exploration/**"], off: ["spacing"], until: "astromech-dpaz", why: "Rebuilds System." },
  { files: ["src/modules/{session,logbook}/**"], off: ["spacing"], until: "astromech-fzhp", why: "Rebuilds Session and Logbook." },
  { files: ["src/modules/trade/**"], off: ["spacing"], until: "astromech-el3i", why: "Rebuilds Trade route." },
  {
    files: ["src/modules/{stored,mining,datasharing,diagnostics,voicetester}/**", "src/modules/places/Home.svelte"],
    off: ["spacing"],
    until: "astromech-gvy0", why: "Moves the remaining pages onto the layouts.",
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
