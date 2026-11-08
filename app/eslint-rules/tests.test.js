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
    "await page.waitForLoadState('load');",
    "await page.goto(url, { waitUntil: 'domcontentloaded' });",
  ],
  invalid: [
    { code: "await page.waitForTimeout(500);", errors: [{ messageId: "sleep" }] },
    { code: "await page.waitForFunction(() => ready, null, { polling: 100 });", errors: [{ messageId: "polling" }] },
    { code: "await page.waitForFunction(() => ready, null, { polling: 'raf' });", errors: [{ messageId: "polling" }] },
    { code: "await page.waitForFunction(() => ready, null, { polling: every });", errors: [{ messageId: "polling" }] },
    { code: "await page.waitForLoadState('networkidle');", errors: [{ messageId: "idle" }] },
    { code: "await page.goto(url, { waitUntil: 'networkidle' });", errors: [{ messageId: "idle" }] },
    { code: "await page.clock.resume();", errors: [{ messageId: "clock" }] },
    { code: "await page.clock.setSystemTime(new Date(0));", errors: [{ messageId: "clock" }] },
    { code: "await page.clock.setFixedTime(0);", errors: [{ messageId: "clock" }] },
    { code: "const clock = page.clock; await clock.fastForward(1000);", errors: [{ messageId: "clock" }] },
  ],
});

// The harness installs and pauses the clock: its rule is given { clock: true }.
tester.run("no-playwright-time, for the harness", plugin.rules["no-playwright-time"], {
  valid: [{ code: "await page.clock.resume(); await page.clock.setSystemTime(0);", options: [{ clock: true }] }],
  invalid: [{ code: "await page.waitForTimeout(5);", options: [{ clock: true }], errors: [{ messageId: "sleep" }] }],
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

  it("turns the Playwright rule on for journeys, the clock part outside the harness", async () => {
    const { ESLint } = await import("eslint");
    const cwd = fileURLToPath(new URL("..", import.meta.url));
    const eslint = new ESLint({ cwd, overrideConfigFile: true, overrideConfig: config });
    const code = "export const f = (page) => page.waitForTimeout(500);\nexport const g = (page) => page.clock.resume();\n";
    const ids = async (filePath) => (await eslint.lintText(code, { filePath }))[0].messages.map((m) => m.messageId);
    expect(await ids("journeys/x.journey.ts")).toEqual(expect.arrayContaining(["sleep", "clock"]));
    expect(await ids("journeys/harness.ts")).toEqual(["sleep"]);
  });
});
