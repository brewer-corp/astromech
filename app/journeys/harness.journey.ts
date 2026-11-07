/**
 * The harness's own test, not a user journey: the suite finds journeys, and the page's timers
 * fire only when a journey runs the page's clock (harness.ts). Two of them: the window's 2 s
 * status poll (app/src/App.svelte), seen as one more `status` call posted to the dev bridge, and
 * a toast's dismissal after 8 s (app/src/lib/ui/Toasts.svelte), on a toast from a docking request
 * denied in worlds/journeys/clock.
 */
import assert from "node:assert/strict";
import { test } from "node:test";
import { isCall, journey, journeysFound, PATTERN } from "./harness.ts";

test("the suite finds journeys", () => {
  assert.notDeepEqual(journeysFound(), [], `no journey matches ${PATTERN} in app/: a run of none would pass`);
});

journey("the page's timers fire only on runFor", { world: "journeys/clock" }, async ({ page, core, open, runFor, calls, settled }) => {
  await core.step("running");
  await open("controller");
  await page.getByText("Elite running").waitFor();
  await settled();
  const first = calls("status").length;

  // The poll is due 2 s after the page started it, and not a millisecond sooner.
  await runFor(1999);
  await settled();
  assert.equal(calls("status").length, first, "a status call before the poll was due");
  const polled = page.waitForRequest((r) => isCall(r, "status"));
  await runFor(1);
  await polled;
  await settled();
  assert.equal(calls("status").length, first + 1);

  // The core has the toast at once; the page shows it from the next poll, for 8 s.
  await core.step("denied");
  await core.wait({ kind: "said", text: "Docking denied. No free pad." });
  const toast = page.getByRole("status").getByText("Docking denied. No free pad.");
  await settled();
  assert.equal(await toast.count(), 0, "the toast before the poll that brings it");
  await runFor(2000);
  await toast.waitFor();
  await runFor(7999);
  assert.equal(await toast.count(), 1, "the toast dismissed before its 8 s were up");
  await runFor(1);
  assert.equal(await toast.count(), 0, "the toast still up after its 8 s");
});
