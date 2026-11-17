// The token check (DESIGN.md, tokens in src/design/tokens.css): a component's styles use the
// design system's colours and sizes, never their own. Three rules, each its own so
// eslint.config.js can exempt a file from one part:
//
// colour: a raw colour (a hex, a colour function such as rgb() or hsl(), or a named colour) in
// any declaration, a custom property's value included. Use a colour token (`var(--accent)`);
// `transparent`, `currentColor` and the global keywords are not colours of their own, and a
// colour-mix() of tokens passes.
// font-size: a `font-size`, or a `font` shorthand, that writes a length, a percentage or a size
// keyword (in a var() fallback too), or works one out with calc() and the like. Name a type role
// (`.type-label`) or a size token (`var(--fs-s)`).
// spacing: a margin, padding or gap that writes a length other than 0, or scales a token by a
// factor that is not a whole number. Use the spacing scale (`var(--sp-3)`) or another token; a
// percentage, a viewport or container unit (shares of the window), `auto` and a whole factor
// (`calc(-1 * var(--sp-2))`) pass. A custom property the component declares itself is not
// followed: `--pad: 10px` passes, and so does the padding that reads it.
//
// Each rule reads the style block, `style:` directives and static `style` attributes of a
// Svelte file, and the declarations of a .css file (through @eslint/css).

/** CSS's named colours (CSS Color 4), and the system colours. */
const NAMED = new Set(
  (
    "aliceblue antiquewhite aqua aquamarine azure beige bisque black blanchedalmond blue blueviolet brown burlywood " +
    "cadetblue chartreuse chocolate coral cornflowerblue cornsilk crimson cyan darkblue darkcyan darkgoldenrod darkgray " +
    "darkgreen darkgrey darkkhaki darkmagenta darkolivegreen darkorange darkorchid darkred darksalmon darkseagreen " +
    "darkslateblue darkslategray darkslategrey darkturquoise darkviolet deeppink deepskyblue dimgray dimgrey dodgerblue " +
    "firebrick floralwhite forestgreen fuchsia gainsboro ghostwhite gold goldenrod gray green greenyellow grey honeydew " +
    "hotpink indianred indigo ivory khaki lavender lavenderblush lawngreen lemonchiffon lightblue lightcoral lightcyan " +
    "lightgoldenrodyellow lightgray lightgreen lightgrey lightpink lightsalmon lightseagreen lightskyblue lightslategray " +
    "lightslategrey lightsteelblue lightyellow lime limegreen linen magenta maroon mediumaquamarine mediumblue " +
    "mediumorchid mediumpurple mediumseagreen mediumslateblue mediumspringgreen mediumturquoise mediumvioletred " +
    "midnightblue mintcream mistyrose moccasin navajowhite navy oldlace olive olivedrab orange orangered orchid " +
    "palegoldenrod palegreen paleturquoise palevioletred papayawhip peachpuff peru pink plum powderblue purple " +
    "rebeccapurple red rosybrown royalblue saddlebrown salmon sandybrown seagreen seashell sienna silver skyblue " +
    "slateblue slategray slategrey snow springgreen steelblue tan teal thistle tomato turquoise violet wheat white " +
    "whitesmoke yellow yellowgreen " +
    "accentcolor accentcolortext activetext buttonborder buttonface buttontext canvas canvastext field fieldtext " +
    "graytext highlight highlighttext linktext mark marktext selecteditem selecteditemtext visitedtext"
  ).split(" "),
);
/** Properties whose value can name a colour by a word. A raw hex or function counts anywhere. */
const TAKES_COLOUR = /^(--|color$|background|border|outline|fill$|stroke$|box-shadow$|text-shadow$|caret-color$|accent-color$|text-decoration|text-emphasis|column-rule|scrollbar-color$|flood-color$|lighting-color$|stop-color$|filter$|backdrop-filter$|-webkit-text-(fill-color|stroke)|-webkit-tap-highlight-color$)/;
const COLOUR_FN = /(^|[^\w-])(rgba?|hsla?|hwb|(ok)?lab|(ok)?lch|color)\(/i;
const HEX = /(^|[^\w-])#[0-9a-f]{3,8}\b/i;

/** The value without its comments, strings, urls and var() names, so `--red-low`, `"red"` or
 * `/* was red *\/` names nothing. A var()'s fallback stays: it is a value the component writes. */
function bare(value) {
  return value
    .replace(/\/\*[\s\S]*?\*\//g, " ")
    .replace(/"[^"]*"|'[^']*'/g, " ")
    .replace(/url\([^)]*\)/gi, " ")
    .replace(/--[\w-]+/g, " ")
    .replace(/\s*!important\s*$/i, "");
}

/** Whether `prop: value` writes a colour that is not a token. */
export function rawColour(prop, value) {
  const v = bare(value);
  if (HEX.test(v) || COLOUR_FN.test(v)) return true;
  if (!TAKES_COLOUR.test(prop.toLowerCase())) return false;
  return (v.toLowerCase().match(/[a-z][\w-]*/g) ?? []).some((w) => NAMED.has(w));
}

/** Absolute and font-relative lengths; a viewport or container unit is a share of the window, like %. */
const FIXED = "px|r?(?:em|ex|ch|cap|ic|lh)|cm|mm|q|in|pt|pc";
const SHARE = "[ds]?v(?:w|h|i|b|min|max)|cq(?:w|h|i|b|min|max)";
const lengths = (units) => new RegExp(`(^|[^\\w.-])[-+]?(\\d*\\.?\\d+(?:e[-+]?\\d+)?)(${units})\\b`, "gi");
const FIXED_LENGTH = lengths(FIXED);
const ANY_LENGTH = lengths(`${FIXED}|${SHARE}`);
/** Whether `v` writes a length other than 0 in one of the units `re` matches. */
function hasLength(v, re = ANY_LENGTH) {
  return [...v.matchAll(re)].some((m) => Number(m[2]) !== 0);
}

const FONT_KEYWORD = /(^|[^\w-])(xx?x?-small|x?-?small|medium|x?x?-?large|xxx-large|smaller|larger|math)(?![\w-])/i;

/** Arithmetic: any calc() or comparison makes a size the type scale lacks, whatever its factor. */
const MATHS = /(^|[^\w-])(calc|min|max|clamp|round|mod|rem|abs|sign)\(/i;

/** Whether `prop: value` sets a font size that is not a token. */
export function rawFontSize(prop, value) {
  const p = prop.toLowerCase();
  if (p !== "font-size" && p !== "font") return false;
  let v = bare(value);
  // In the shorthand, the size is what comes before an optional "/line-height"; a line height
  // in a length is not a size.
  if (p === "font") v = v.split("/")[0];
  return hasLength(v) || /\d%/.test(v) || FONT_KEYWORD.test(v) || MATHS.test(v);
}

const SPACING = /^(margin|padding|gap$|row-gap$|column-gap$|grid-gap$|grid-row-gap$|grid-column-gap$)/;

/** Whether `prop: value` is a margin, padding or gap that writes its own length. */
export function rawSpacing(prop, value) {
  if (!SPACING.test(prop.toLowerCase())) return false;
  // A var()'s fallback is a length the component writes, so it counts: drop only the names.
  const v = bare(value);
  // A factor that is not a whole number makes a step the scale lacks (`var(--sp-2) * 1.25`);
  // a whole one, negation included, lands on a multiple of the step.
  const fraction = /\*\s*-?\d*\.\d+(?![\w.%])|(^|[^\w.-])-?\d*\.\d+\s*\*/.test(v);
  return fraction || hasLength(v, FIXED_LENGTH);
}

/** `prop: value` pairs of a static style attribute's text. */
function inline(text) {
  return text
    .split(";")
    .map((d) => d.split(":"))
    .filter((d) => d.length >= 2)
    .map(([p, ...v]) => [p.trim(), v.join(":")]);
}

/** One rule: `bad(prop, value)` says whether a declaration breaks it. */
function rule(bad, message, description) {
  return {
    meta: { type: "problem", docs: { description }, schema: [], messages: { raw: message } },
    create(context) {
      const sc = context.sourceCode;
      const svc = sc.parserServices;
      if (svc?.isSvelte) {
        return {
          SvelteStyleElement() {
            const style = svc.getStyleContext();
            if (style.status !== "success") return;
            style.sourceAst.walkDecls((d) => {
              if (bad(d.prop, d.value)) context.report({ loc: svc.styleNodeLoc(d), messageId: "raw", data: { decl: `${d.prop}: ${d.value}` } });
            });
          },
          SvelteStyleDirective(node) {
            // Only the literal parts: an expression's value is a token or a computed share.
            const prop = node.key.name.name;
            const text = node.value.map((v) => (v.type === "SvelteLiteral" ? v.value : " ")).join("");
            if (bad(prop, text)) context.report({ node, messageId: "raw", data: { decl: `style:${prop}="${text.trim()}"` } });
          },
          SvelteAttribute(node) {
            if (node.key.name !== "style") return;
            const text = node.value.map((v) => (v.type === "SvelteLiteral" ? v.value : " ")).join("");
            for (const [p, v] of inline(text)) {
              if (bad(p, v)) context.report({ node, messageId: "raw", data: { decl: `${p}: ${v.trim()}` } });
            }
          },
        };
      }
      if (sc.ast?.type === "StyleSheet") {
        return {
          Declaration(node) {
            const value = sc.getText(node.value);
            if (bad(node.property, value)) context.report({ loc: node.loc, messageId: "raw", data: { decl: `${node.property}: ${value}` } });
          },
        };
      }
      return {};
    },
  };
}

export default {
  meta: { name: "tokens" },
  rules: {
    colour: rule(rawColour, "A raw colour ({{decl}}): use a colour token from src/design/tokens.css. DESIGN.md Token map.", "Styles use colour tokens"),
    "font-size": rule(
      rawFontSize,
      "An off-scale font size ({{decl}}): name a type role (.type-label) or a size token (var(--fs-s)). DESIGN.md Typography.",
      "Font sizes come from the type scale",
    ),
    spacing: rule(
      rawSpacing,
      "An off-scale margin, padding or gap ({{decl}}): use the spacing scale (var(--sp-3)) or another token. DESIGN.md Layout.",
      "Margins, padding and gaps come from the spacing scale",
    ),
  },
};
