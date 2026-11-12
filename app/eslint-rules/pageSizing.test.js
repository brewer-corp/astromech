import { RuleTester } from "eslint";
import svelte from "eslint-plugin-svelte";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import plugin from "./pageSizing.js";
import config from "../eslint.config.js";

RuleTester.describe = describe;
RuleTester.it = it;
const parser = svelte.configs.recommended.find((c) => c.files?.includes("**/*.svelte")).languageOptions.parser;
const tester = new RuleTester({ languageOptions: { parser } });
const page = (css, markup = "<p>x</p>") => ({ code: `${markup}\n<style>\n${css}\n</style>\n`, filename: "View.svelte" });
const bad = (css, markup) => ({ ...page(css, markup), errors: [{ messageId: "sized" }] });

tester.run("page-sizing/width", plugin.rules.width, {
  valid: [
    page("p { max-width: var(--measure); }"),
    page("img { max-width: 100%; } .a { max-width: none; }"),
    page("table { width: 100%; } .b { width: max-content; } .c { width: auto; }"),
    page("@media (max-width: 900px) { .a { display: block; } }"),
    page("@container (min-width: 60rem) { .a { gap: 8px; } }"),
    page(".a { min-width: 0; height: 12px; grid-template-columns: 200px 1fr; }"),
    page(".a { font-size: 13px; }"),
    page("", '<div style:--cols={n}></div><div style="height: 4px"></div>'),
  ],
  invalid: [
    bad("section { max-width: 60rem; }"),
    bad(".finder { max-width: var(--list-w); }"),
    bad(".about { max-width: 68ch; }"),
    bad(".dlg { width: 600px; }"),
    bad(".w { width: calc(100% - 2rem); }"),
    bad(".w { width: var(--panel); }"),
    bad(".w { inline-size: 20vw; }"),
    bad("@media (max-width: 900px) { .a { max-width: 30rem; } }"),
    bad("", "<div style:width={w}></div>"),
    bad("", '<div style="color: red; max-width: 40rem"></div>'),
  ],
});

tester.run("page-sizing/font-size", plugin.rules["font-size"], {
  valid: [page(".a { font-weight: 600; letter-spacing: var(--caps); }"), page(".a { max-width: 60rem; }")],
  invalid: [bad(".a { font-size: var(--fs-s); }"), bad(".a { font: 600 15px var(--face); }"), bad("", "<b style:font-size={s}>x</b>")],
});

// The parts are on only for the files eslint.config.js lists, and never for the exempt ones.
describe("eslint.config.js page-sizing", () => {
  const lint = async (filePath) => {
    const { ESLint } = await import("eslint");
    const cwd = fileURLToPath(new URL("..", import.meta.url));
    const eslint = new ESLint({ cwd, overrideConfigFile: true, overrideConfig: config });
    const [r] = await eslint.lintText("<p>x</p>\n<style>\n  section { max-width: 60rem; font-size: 13px; }\n</style>\n", { filePath });
    return r.messages.map((m) => m.ruleId).filter((id) => id?.startsWith("page-sizing/"));
  };

  it("checks an enrolled page's width", async () => {
    expect(await lint("src/modules/materials/View.svelte")).toEqual(["page-sizing/width"]);
    expect(await lint("src/modules/places/Market.svelte")).toEqual(["page-sizing/width"]);
  });

  it("leaves a page not yet enrolled, Controls and the frame's parts alone", async () => {
    for (const f of ["src/modules/places/Home.svelte", "src/modules/controller/View.svelte", "src/modules/route/Strip.svelte", "src/modules/route/PlotBar.svelte"]) {
      expect(await lint(f), f).toEqual([]);
    }
  });
});
