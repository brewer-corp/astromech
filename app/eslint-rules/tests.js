// Lint rules for test files (CONTRIBUTING.md, Tests): a test waits on what the page or the fake
// does, never on the clock, and asserts on events and counts, never on elapsed time.

/** `setTimeout` in a test: a sleep that waits for time to pass instead of for something to happen. */
const noTimerWait = {
  meta: {
    type: "problem",
    docs: { description: "disallow setTimeout in tests" },
    messages: {
      wait: "Don't wait on a timer in a test: wait for the result (findBy…, waitFor, vi.waitFor), or drive the clock with vi.useFakeTimers.",
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
      clock: "Don't read the wall clock in a test: use a fixed date, or vi.useFakeTimers and vi.setSystemTime.",
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

export default { rules: { "no-timer-wait": noTimerWait, "no-wall-clock": noWallClock } };
