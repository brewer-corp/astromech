import { RuleTester } from "eslint";
import css from "@eslint/css";
import svelte from "eslint-plugin-svelte";
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import plugin from "./tokens.js";
import config, { TOKENS_EXEMPT } from "../eslint.config.js";

RuleTester.describe = describe;
RuleTester.it = it;
const parser = svelte.configs.recommended.find((c) => c.files?.includes("**/*.svelte")).languageOptions.parser;
const tester = new RuleTester({ languageOptions: { parser } });
const cssTester = new RuleTester({ plugins: { css }, language: "css/css" });
const part = (style, markup = "<p>x</p>") => ({ code: `${markup}\n<style>\n${style}\n</style>\n`, filename: "Part.svelte" });
const bad = (style, markup) => ({ ...part(style, markup), errors: [{ messageId: "raw" }] });
const sheet = (code) => ({ code, filename: "part.css" });

tester.run("tokens/colour", plugin.rules.colour, {
  valid: [
    part(".a { color: var(--accent); background: transparent; border-color: currentColor; }"),
    part(".a { color: var(--red-low); outline: 1px solid var(--line); }"),
    part(".a { background: color-mix(in srgb, var(--accent) 40%, transparent); }"),
    part('.a { grid-template-areas: "red blue"; content: "white"; animation-name: orange; }'),
    part("", '<p style:color={tone}>x</p><p style="color: var(--prose)">x</p>'),
    part(".a { filter: drop-shadow(0 0 2px var(--accent)); animation: red 1s; }"),
  ],
  invalid: [
    bad(".a { color: #fff; }"),
    bad(".a { background: rgb(0 0 0 / 0.7); }"),
    bad(".a { border: 1px solid hsl(20 100% 50%); }"),
    bad(".a { color: white; }"),
    bad(".a { box-shadow: inset 0 -2px orange; }"),
    bad(".a { --tone: #ff7100; }"),
    bad(".a { color: var(--tone, #ff7100); }"),
    bad("", '<p style="color: red">x</p>'),
    bad("", '<p style:background="black">x</p>'),
    bad(".a { filter: drop-shadow(0 0 2px red); }"),
    bad(".a { -webkit-text-fill-color: red; }"),
    bad(".a { -webkit-text-stroke: 1px red; }"),
  ],
});

tester.run("tokens/font-size", plugin.rules["font-size"], {
  valid: [
    part(".a { font-size: var(--fs-s); font-weight: 600; } .b { font-size: inherit; }"),
    part(".a { font: 400 var(--fs)/1.3 var(--face); } .b { font: inherit; }"),
    part(".a { font: 600 var(--fs-s)/1.25rem var(--face); }"),
  ],
  invalid: [
    bad(".a { font-size: 13px; }"),
    bad(".a { font-size: 0.8rem; }"),
    bad(".a { font-size: 120%; }"),
    bad(".a { font-size: small; }"),
    bad(".a { font: 600 15px var(--face); }"),
    bad(".a { font-size: var(--missing, 17px); }"),
    bad(".a { font: 400 var(--missing, 17px)/1.3 sans-serif; }"),
    bad(".a { font-size: calc(var(--fs) * 1.25); }"),
    bad(".a { font-size: max(var(--fs), var(--fs-s)); }"),
    bad(".a { font-size: 1e1px; }"),
    bad("", '<b style="font-size: 2em">x</b>'),
  ],
});

tester.run("tokens/spacing", plugin.rules.spacing, {
  valid: [
    part(".a { margin: 0; padding: 0 var(--sp-3); gap: var(--gap); }"),
    part(".a { margin: var(--sp-2) auto; padding-left: 0px; row-gap: 5%; }"),
    part(".a { margin-top: calc(-1 * var(--sp-2)); padding: calc(2 * var(--sp-4)); }"),
    part(".a { gap: clamp(var(--sp-4), 2.5vw, var(--gutter)); }"),
    part(".a { width: 12rem; height: 3px; inset: 4px; }"),
    part("", "<p style:padding={pad}>x</p>"),
  ],
  invalid: [
    bad(".a { padding: 0.625rem var(--sp-3); }"),
    bad(".a { margin: 0 0 6px; }"),
    bad(".a { gap: 1.75rem; }"),
    bad(".a { padding-inline-start: 1em; }"),
    bad(".a { margin: calc(var(--sp-2) + 1px); }"),
    bad(".a { padding: calc(var(--sp-2) * 1.25); }"),
    bad(".a { padding: var(--pad, 10px); }"),
    bad(".a { padding: 1e1px; }"),
    bad(".a { padding: 2rcap; }"),
    bad("", '<p style="margin-top: 3px">x</p>'),
  ],
});

// A .css file is read through @eslint/css, by the same three rules.
cssTester.run("tokens in a .css file", plugin.rules.colour, {
  valid: [sheet(".a { color: var(--accent); }"), sheet(".a { color: var(--accent) /* was red */; }")],
  invalid: [{ ...sheet(".a { color: #c4c4c4; }"), errors: [{ messageId: "raw" }] }],
});
cssTester.run("tokens in a .css file", plugin.rules.spacing, {
  valid: [sheet(".a { padding: 0 var(--sp-3); }"), sheet(".a { padding: var(--sp-2) /* used to be 10px */; }")],
  invalid: [{ ...sheet(".a { gap: 1.75rem; }"), errors: [{ messageId: "raw" }] }],
});
cssTester.run("tokens in a .css file", plugin.rules["font-size"], {
  valid: [sheet(".a { font-size: var(--fs-s) /* was 13px */; }")],
  invalid: [
    { ...sheet(".a { font-size: var(--missing, 17px); }"), errors: [{ messageId: "raw" }] },
    { ...sheet(".a { font-size: calc(var(--fs) * 1.25); }"), errors: [{ messageId: "raw" }] },
  ],
});

describe("eslint.config.js tokens", () => {
  const cwd = fileURLToPath(new URL("..", import.meta.url));
  const lintWith = async (cfg, filePath, code) => {
    const { ESLint } = await import("eslint");
    const eslint = new ESLint({ cwd, overrideConfigFile: true, overrideConfig: cfg });
    const results = code === undefined ? await eslint.lintFiles([filePath]) : await eslint.lintText(code, { filePath });
    return results.flatMap((r) => r.messages.map((m) => m.ruleId)).filter((id) => id?.startsWith("tokens/"));
  };
  const SVELTE = "<p>x</p>\n<style>\n  p { color: #fff; font-size: 13px; padding: 10px; }\n</style>\n";
  const CSS = "p { color: #fff; font-size: 13px; padding: 10px; }\n";
  const ALL = ["tokens/colour", "tokens/font-size", "tokens/spacing"];

  it("checks every component and stylesheet under src/", async () => {
    expect(await lintWith(config, "src/lib/ui/New.svelte", SVELTE)).toEqual(ALL);
    expect(await lintWith(config, "src/modules/places/New.svelte", SVELTE)).toEqual(ALL);
    expect(await lintWith(config, "src/lib/new.css", CSS)).toEqual(ALL);
  });

  it("leaves the tokens and the mockups alone", async () => {
    expect(await lintWith(config, "src/design/tokens.css", CSS)).toEqual([]);
    expect(await lintWith(config, "src/mockups/New.svelte", SVELTE)).toEqual([]);
    expect(await lintWith(config, "src/mockups/ds/new.css", CSS)).toEqual([]);
  });

  it("turns off only the parts an exemption names", async () => {
    expect(await lintWith(config, "src/modules/keybinds/New.svelte", SVELTE)).toEqual(["tokens/colour", "tokens/font-size"]);
    expect(await lintWith(config, "src/lib/ui/Sheet.svelte", SVELTE)).toEqual(["tokens/colour"]);
  });

  it("gives every exemption a reason, and a pending one an item that exists", () => {
    for (const { files, why, until } of TOKENS_EXEMPT) {
      expect(why, files.join()).toMatch(/.{10,}/);
      if (until) expect(existsSync(new URL(`../../.atelier/items/${until}/item.md`, import.meta.url)), until).toBe(true);
    }
  });

  // An exemption whose files no longer break the part it turns off is stale: take it off the list.
  it("holds no exemption its files no longer need", async () => {
    for (const entry of TOKENS_EXEMPT) {
      const without = config.filter((c) => !(c.files === entry.files && c.rules && Object.values(c.rules).every((v) => v === "off")));
      expect(without.length, entry.files.join()).toBe(config.length - 1);
      const hits = new Set();
      for (const f of entry.files) for (const id of await lintWith(without, f)) hits.add(id.slice("tokens/".length));
      expect([...hits].sort(), entry.files.join()).toEqual(expect.arrayContaining(entry.off));
    }
  }, 60_000);
});
