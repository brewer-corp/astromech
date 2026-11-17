import { RuleTester } from "eslint";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, it } from "vitest";
import plugin from "./imports.js";

RuleTester.describe = describe;
RuleTester.it = it;
const tester = new RuleTester();

// The layout that broke the Windows build: a component and a module whose import names it.
const dir = mkdtempSync(join(tmpdir(), "astromech-imports-"));
mkdirSync(join(dir, "layout"));
for (const f of ["layout/ListDetail.svelte", "layout/listDetail.svelte.ts", "layout/Split.svelte"]) writeFileSync(join(dir, f), "");
const filename = join(dir, "Page.ts");

tester.run("no-case-clash", plugin.rules["no-case-clash"], {
  valid: [
    { code: "import x from './layout/ListDetail.svelte';", filename },
    { code: "import x from './layout/Split.svelte';", filename },
    { code: "import x from 'svelte';", filename },
    { code: "import x from './missing/thing';", filename },
  ],
  invalid: [
    { code: "import { provideDetail } from './layout/listDetail.svelte';", filename, errors: [{ messageId: "clash" }] },
    { code: "export * from './layout/split.svelte';", filename, errors: [{ messageId: "clash" }] },
    { code: "const m = import('./layout/listDetail.svelte');", filename, errors: [{ messageId: "clash" }] },
  ],
});
