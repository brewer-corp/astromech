import { expect, it, vi } from "vitest";
import type { Transport } from "./transport";

// A module that imports lib/transport while it, or the browser preview, is still loading must get
// it whole. When lib/transport imported the preview, which imports the app's lib modules back,
// the module runner handed a module imported meanwhile the half-loaded lib/transport, whose first
// call threw "Cannot access … before initialization": Find or plot then lost a page's entries
// (palette.test.ts on PR #216's CI). Each round starts the load and, a few more microtasks in each
// time, imports a page's loader and calls it, until the load is done. The load starts at
// lib/transport alone (as a page's loader starts it), or at lib/transport and the preview, as the
// app (main.ts) and the tests' setup load them; the second is the long one, so it must take rounds.
const starts: [string, () => Promise<unknown>, number][] = [
  ["lib/transport", () => import("./transport"), 0],
  [
    "lib/transport and the preview",
    () => {
      const transport = import("./transport");
      return Promise.resolve()
        .then(() => import("../preview/transport"))
        .then(() => transport);
    },
    10,
  ],
];

it.each(starts)("hands a module imported while %s loads a whole lib/transport", async (_name, start, minRounds) => {
  const stub: Transport = { call: () => Promise.resolve({ kinds: [], other: [] }), subscribe: () => () => {}, option: () => null };
  const failures: string[] = [];
  let rounds = 0;
  for (let step = 0; ; step += 20) {
    vi.resetModules();
    let loaded = false;
    const load = start().then(() => (loaded = true));
    for (let i = 0; i < step; i++) await Promise.resolve();
    if (loaded) break;
    rounds++;
    try {
      const { loadMaterials } = await import("./materials");
      (await import("./transport")).setTransport(stub);
      await loadMaterials();
    } catch (e) {
      failures.push(`microtask ${step}: ${String(e)}`);
    }
    await load;
  }
  expect(rounds, "the rounds cover the load").toBeGreaterThanOrEqual(minRounds);
  expect(failures).toEqual([]);
});
