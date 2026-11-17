// A relative import that opens one file on Linux and another on Windows. Windows' file system
// ignores case, so `./listDetail.svelte` is ListDetail.svelte there, and listDetail.svelte.ts
// (what Linux opens, adding .ts) is never reached: the Windows build failed this way from
// f82b686b to astromech-9uj3, with no Linux check to catch it.
import { readdirSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";

const listing = new Map();
function entries(dir) {
  if (!listing.has(dir)) {
    let names = null;
    try {
      names = readdirSync(dir);
    } catch {
      // Not a folder: resolution fails the same way everywhere, which the build reports.
    }
    listing.set(dir, names);
  }
  return listing.get(dir);
}

/** `import … from "./x"` where no entry is named x but one differs from it only in case. */
const noCaseClash = {
  meta: {
    type: "problem",
    docs: { description: "disallow a relative import that names a file only by a different case" },
    messages: {
      clash:
        "On Windows '{{spec}}' opens {{other}}, which differs from it only in case. Rename one file so no two names in the folder differ only in case and extension.",
    },
    schema: [],
  },
  create(context) {
    const check = (source) => {
      const spec = source?.value;
      if (typeof spec !== "string" || !spec.startsWith(".")) return;
      const target = resolve(dirname(context.filename), spec);
      const names = entries(dirname(target));
      const name = basename(target);
      if (!names || names.includes(name)) return;
      const other = names.find((n) => n.toLowerCase() === name.toLowerCase());
      if (other) context.report({ node: source, messageId: "clash", data: { spec, other } });
    };
    return {
      ImportDeclaration: (node) => check(node.source),
      ImportExpression: (node) => check(node.source),
      ExportNamedDeclaration: (node) => check(node.source),
      ExportAllDeclaration: (node) => check(node.source),
    };
  },
};

export default { rules: { "no-case-clash": noCaseClash } };
