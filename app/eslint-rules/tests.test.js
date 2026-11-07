import { RuleTester } from "eslint";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import plugin from "./tests.js";
import config from "../eslint.config.js";

RuleTester.describe = describe;
RuleTester.it = it;
const tester = new RuleTester();

tester.run("no-timer-wait", plugin.rules["no-timer-wait"], {
  valid: ["await screen.findByText('Saved');", "await vi.waitFor(() => expect(x).toBe(1));", "vi.advanceTimersByTime(1000);"],
  invalid: [
    { code: "await new Promise((r) => setTimeout(r, 50));", errors: [{ messageId: "wait" }] },
    { code: "window.setTimeout(done, 0);", errors: [{ messageId: "wait" }] },
  ],
});

tester.run("no-wall-clock", plugin.rules["no-wall-clock"], {
  valid: ["new Date('2026-10-16T12:00:00Z');", "vi.setSystemTime(new Date(0));", "new Date(NOW).toISOString();", "Date.parse(s);", "clock.now();"],
  invalid: [
    { code: "const t = Date.now();", errors: [{ messageId: "clock" }] },
    { code: "performance.now() - start;", errors: [{ messageId: "clock" }] },
    { code: "new Date().toISOString();", errors: [{ messageId: "clock" }] },
    { code: "const today = Date();", errors: [{ messageId: "clock" }] },
  ],
});

tester.run("no-playwright-time", plugin.rules["no-playwright-time"], {
  valid: [
    "await page.getByRole('alert').waitFor();",
    "await page.waitForFunction(() => document.title === 'x', null, { polling: 'mutation' });",
    "await page.waitForRequest((r) => r.url().endsWith('/x'));",
    "await page.clock.runFor(2000);",
  ],
  invalid: [
    { code: "await page.waitForTimeout(500);", errors: [{ messageId: "sleep" }] },
    { code: "await page.waitForFunction(() => ready, null, { polling: 100 });", errors: [{ messageId: "polling" }] },
    { code: "await page.waitForFunction(() => ready, null, { polling: 'raf' });", errors: [{ messageId: "polling" }] },
    { code: "await page.waitForFunction(() => ready, null, { polling: every });", errors: [{ messageId: "polling" }] },
  ],
});

// The rules are on for every test file in the app's config, not only defined.
describe("eslint.config.js", () => {
  it("turns both rules on for test files", async () => {
    const { ESLint } = await import("eslint");
    const cwd = fileURLToPath(new URL("..", import.meta.url));
    const eslint = new ESLint({ cwd, overrideConfigFile: true, overrideConfig: config });
    const code = "await new Promise((r) => setTimeout(r, 50));\nexport const t = Date.now();\n";
    for (const filePath of ["src/modules/x/View.browser.test.ts", "src/lib/x.test.ts", "journeys/x.journey.ts", "journeys/harness.ts"]) {
      const [r] = await eslint.lintText(code, { filePath });
      expect(r.messages.map((m) => m.ruleId), filePath).toEqual(expect.arrayContaining(["astromech/no-timer-wait", "astromech/no-wall-clock"]));
    }
  });

  it("turns the Playwright rule on for journeys", async () => {
    const { ESLint } = await import("eslint");
    const cwd = fileURLToPath(new URL("..", import.meta.url));
    const eslint = new ESLint({ cwd, overrideConfigFile: true, overrideConfig: config });
    const code = "export const f = (page) => page.waitForTimeout(500);\n";
    for (const filePath of ["journeys/x.journey.ts", "journeys/harness.ts"]) {
      const [r] = await eslint.lintText(code, { filePath });
      expect(r.messages.map((m) => m.ruleId), filePath).toContain("astromech/no-playwright-time");
    }
  });
});
