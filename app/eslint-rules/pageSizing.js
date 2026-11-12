// The page-sizing check (DESIGN.md Page layouts): a page module sets no width or font size of its
// own. The layout it is built on (lib/ui/layout/) owns its widths, and the type roles own sizes.
// Each part is a rule of its own, so eslint.config.js can switch it on a page file at a time.
//
// width: a `width` or `max-width` (or `inline-size`, `max-inline-size`) that fixes a length, in
// the style block, a `style:` directive or a static `style` attribute. A query's condition is not
// a declaration, so `@media (max-width: 900px)` passes; so do `max-width: var(--measure)` on
// prose, `max-width: 100%` or `none`, and a width that is a share or a content size (`100%`,
// `auto`, `max-content`).
// font-size: any `font-size`, or the `font` shorthand, which sets one.

const WIDTH = new Set(["width", "max-width", "inline-size", "max-inline-size"]);
const MAX = new Set(["max-width", "max-inline-size"]);
/** A max-width that sizes nothing: the prose measure, or a guard against overflow. */
const FREE_MAX = new Set(["var(--measure)", "100%", "none"]);
/** A length, or a variable (a token is a length too). */
const LENGTH = /(\d|\.)(px|r?em|ch|ex|cap|ic|r?lh|[ds]?v(w|h|i|b|min|max)|cq(w|h|i|b|min|max)|cm|mm|q|in|pt|pc)\b|var\(/i;
const FONT = new Set(["font-size", "font"]);

/** Whether `prop: value` is a width the page sets itself. */
export function fixesWidth(prop, value) {
  const p = prop.toLowerCase();
  if (!WIDTH.has(p)) return false;
  const v = value.trim().toLowerCase().replace(/\s*!important$/, "");
  if (MAX.has(p)) return !FREE_MAX.has(v);
  return LENGTH.test(v);
}

/** Whether `prop` sets a font size. */
export function setsFontSize(prop) {
  return FONT.has(prop.toLowerCase());
}

/** `prop: value` pairs of a static style attribute's text. */
function inline(text) {
  return text
    .split(";")
    .map((d) => d.split(":"))
    .filter((d) => d.length >= 2)
    .map(([p, ...v]) => [p.trim(), v.join(":")]);
}

/** One part of the check: `bad(prop, value)` says whether a declaration breaks it. */
function part(bad, message, description) {
  return {
    meta: { type: "problem", docs: { description }, schema: [], messages: { sized: message } },
    create(context) {
      const sc = context.sourceCode;
      const svc = sc.parserServices;
      if (!svc?.isSvelte) return {};
      return {
        SvelteStyleElement() {
          const style = svc.getStyleContext();
          if (style.status !== "success") return;
          style.sourceAst.walkDecls((d) => {
            if (bad(d.prop, d.value)) context.report({ loc: svc.styleNodeLoc(d), messageId: "sized", data: { decl: `${d.prop}: ${d.value}` } });
          });
        },
        SvelteStyleDirective(node) {
          const prop = node.key.name.name;
          const text = node.value.map((v) => (v.type === "SvelteLiteral" ? v.value : "var(")).join("");
          if (bad(prop, node.value.length ? text : "var(")) context.report({ node, messageId: "sized", data: { decl: `style:${prop}` } });
        },
        SvelteAttribute(node) {
          if (node.key.name !== "style" || !node.value.every((v) => v.type === "SvelteLiteral")) return;
          for (const [p, v] of inline(node.value.map((x) => x.value).join(""))) {
            if (bad(p, v)) context.report({ node, messageId: "sized", data: { decl: `${p}: ${v.trim()}` } });
          }
        },
      };
    },
  };
}

export default {
  meta: { name: "page-sizing" },
  rules: {
    width: part(
      fixesWidth,
      "A page sets no width of its own ({{decl}}): its layout (Board, ListDetail, Workbench or Form in lib/ui/layout/) owns widths. DESIGN.md Page layouts.",
      "A page module sets no width of its own",
    ),
    "font-size": part(
      (p) => setsFontSize(p),
      "A page sets no font size ({{decl}}): it names a type role. DESIGN.md Typography, Hierarchy.",
      "A page module sets no font size",
    ),
  },
};
