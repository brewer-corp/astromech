import { describe, expect, it } from "vitest";
import { parse } from "svelte/compiler";

// DESIGN.md's Tip replaces the native title on anything the commander points at or tabs to: the
// title is slow, small and has no keyboard. Every component in app/src is read, mockups
// included, and a title= on an interactive element (or on Button or IconButton, which spread it
// onto one) fails here. A title that only repeats a non-interactive element's own visible text
// (a name cut short by an ellipsis) is left alone.
const sources = import.meta.glob<string>("../../**/*.svelte", { query: "?raw", import: "default", eager: true });

const INTERACTIVE = new Set(["a", "button", "input", "select", "textarea", "summary"]);
const ROLES = new Set(["button", "link", "tab", "checkbox", "radio", "switch", "menuitem", "option", "slider"]);
const COMPONENTS = new Set(["Button", "IconButton"]);

type Node = { type?: string; name?: string; attributes?: { type: string; name?: string; value?: unknown }[]; start?: number };

const attr = (n: Node, name: string) => n.attributes?.find((a) => a.type === "Attribute" && a.name === name);
const staticText = (a: { value?: unknown } | undefined) =>
  Array.isArray(a?.value) && a.value.length === 1 && a.value[0].type === "Text" ? (a.value[0].data as string) : null;

function interactive(n: Node): boolean {
  if (n.type === "Component") return COMPONENTS.has(n.name ?? "");
  if (n.type !== "RegularElement") return false;
  if (INTERACTIVE.has(n.name ?? "")) return true;
  if (attr(n, "tabindex") || attr(n, "onclick")) return true;
  return ROLES.has(staticText(attr(n, "role")) ?? "");
}

/** Each title= on an interactive element in a component's markup, as "<tag> at line n". */
function titled(file: string, text: string): string[] {
  const found: string[] = [];
  const line = (at = 0) => text.slice(0, at).split("\n").length;
  const walk = (v: unknown) => {
    if (Array.isArray(v)) return v.forEach(walk);
    if (!v || typeof v !== "object") return;
    const n = v as Node;
    if (interactive(n) && attr(n, "title")) found.push(`${file}: <${n.name}> at line ${line(n.start)}`);
    for (const [k, c] of Object.entries(n)) if (k !== "attributes") walk(c);
  };
  walk(parse(text, { modern: true }).fragment);
  return found;
}

describe("no native title on interactive elements", () => {
  it("reads the components", () => {
    // It would pass on nothing otherwise.
    expect(Object.keys(sources).length).toBeGreaterThan(50);
  });

  it("finds one where it is", () => {
    expect(titled("t", `<button title="Copy">x</button><Button title={t}>y</Button><span role="button" title="z"></span>`)).toHaveLength(3);
    expect(titled("t", `<span class="name" title={n}>{n}</span><About title="Night vision" />`)).toEqual([]);
  });

  it("app/src has none: use Tip", () => {
    expect(Object.entries(sources).flatMap(([f, t]) => titled(f, t))).toEqual([]);
  });
});
