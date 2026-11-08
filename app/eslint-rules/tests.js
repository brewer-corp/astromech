// Lint rules for test files (CONTRIBUTING.md, Tests): a test waits on what the page or the fake
// does, never on the clock, and asserts on events and counts, never on elapsed time.

/** `setTimeout` in a test: a sleep that waits for time to pass instead of for something to happen. */
const noTimerWait = {
  meta: {
    type: "problem",
    docs: { description: "disallow setTimeout in tests" },
    messages: {
      wait: "Don't wait on a timer in a test: wait for the result (findBy…, waitFor, vi.waitFor; in a journey, a locator or the core's wait), or drive the clock (vi.useFakeTimers; in a journey, runFor).",
    },
    schema: [],
  },
  create(context) {
    return {
      CallExpression(node) {
        const c = node.callee;
        const named = c.type === "Identifier" ? c.name : c.type === "MemberExpression" && !c.computed ? c.property.name : null;
        if (named === "setTimeout") context.report({ node, messageId: "wait" });
      },
    };
  },
};

/** `Date.now()`, `performance.now()`, or `new Date()` / `Date()` with no arguments: a wall-clock
 * read, so the test depends on when and how fast it runs. */
const noWallClock = {
  meta: {
    type: "problem",
    docs: { description: "disallow wall-clock reads in tests" },
    messages: {
      clock: "Don't read the wall clock in a test: use a fixed date (in a UI test, vi.useFakeTimers and vi.setSystemTime; in a journey, the world's start and runFor).",
    },
    schema: [],
  },
  create(context) {
    const today = (node) => {
      if (node.callee.type === "Identifier" && node.callee.name === "Date" && node.arguments.length === 0) context.report({ node, messageId: "clock" });
    };
    return {
      NewExpression: today,
      CallExpression(node) {
        today(node);
        const c = node.callee;
        if (c.type !== "MemberExpression" || c.computed || c.property.name !== "now") return;
        if (c.object.type === "Identifier" && (c.object.name === "Date" || c.object.name === "performance")) {
          context.report({ node, messageId: "clock" });
        }
      },
    };
  },
};

/** Playwright's timed waits and real-time clock moves in a journey:
 * - `waitForTimeout` (a sleep);
 * - any `polling` but `"mutation"`: a number polls the page on a timer
 *   (`waitForFunction(fn, arg, { polling: 100 })`), and so does `"raf"`, once a frame;
 * - `"networkidle"` (`waitForLoadState("networkidle")`, `waitUntil: "networkidle"`): it waits
 *   for 500 ms with no request;
 * - the page clock's `resume` (real time again), `setSystemTime`, `setFixedTime` and
 *   `fastForward`: the harness installs and pauses the clock, and a journey moves it with
 *   `runFor` only. The harness itself is given `{ clock: true }`.
 * A journey waits on the page with a locator or `polling: "mutation"`, and on the core with its
 * observations. */
const noPlaywrightTime = {
  meta: {
    type: "problem",
    docs: { description: "disallow Playwright's timed waits in journeys" },
    messages: {
      sleep: "Don't sleep in a journey: wait for what the page shows (a locator) or what the core observed (wait).",
      polling: 'Don\'t poll the page on a timer: use a locator, or polling: "mutation".',
      idle: 'Don\'t wait for "networkidle" (500 ms with no request): wait for what the page shows, or for the call (waitForRequest).',
      clock: "Move the page's clock with runFor only: {{name}} takes it off the paused world clock the harness installed.",
    },
    schema: [{ type: "object", properties: { clock: { type: "boolean" } }, additionalProperties: false }],
  },
  create(context) {
    const clockAllowed = context.options[0]?.clock === true;
    const CLOCK = new Set(["resume", "setSystemTime", "setFixedTime", "fastForward"]);
    const idle = (v) => v && v.type === "Literal" && v.value === "networkidle";
    const keyOf = (node) => (node.computed ? null : node.key.type === "Identifier" ? node.key.name : node.key.type === "Literal" ? node.key.value : null);
    return {
      CallExpression(node) {
        const c = node.callee;
        const named = c.type === "Identifier" ? c.name : c.type === "MemberExpression" && !c.computed ? c.property.name : null;
        if (named === "waitForTimeout") context.report({ node, messageId: "sleep" });
        if (named === "waitForLoadState" && idle(node.arguments[0])) context.report({ node, messageId: "idle" });
        if (!clockAllowed && c.type === "MemberExpression" && CLOCK.has(named)) context.report({ node, messageId: "clock", data: { name: named } });
      },
      Property(node) {
        const key = keyOf(node);
        if (key === "waitUntil" && idle(node.value)) context.report({ node, messageId: "idle" });
        if (key !== "polling") return;
        const v = node.value;
        if (!(v.type === "Literal" && v.value === "mutation")) context.report({ node, messageId: "polling" });
      },
    };
  },
};

export default { rules: { "no-timer-wait": noTimerWait, "no-wall-clock": noWallClock, "no-playwright-time": noPlaywrightTime } };
