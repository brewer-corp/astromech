/**
 * The journeys about what matters most while playing (strategy, Shape: copilot reliability wins
 * over window features). The core's half of both is in crates/astromech-core/tests/world_voice.rs;
 * these add what the commander sees.
 */
import assert from "node:assert/strict";
import { journey } from "./harness.ts";

/**
 * The kill switch (worlds/journeys/kill-switch): a press and release stops everything, and the
 * header says "Kill switch on" from the next status poll. Holding it for rearm_hold_ms (1000 ms)
 * re-arms it, and the header stops saying so from the poll after.
 */
journey("the kill switch shows on in the page and goes off after a re-arm", { world: "journeys/kill-switch" }, async ({ page, core, open, runFor }) => {
  await core.step("running");
  await open("controller");
  const header = page.locator("header .state");
  await header.filter({ hasText: /^Elite running · Voice ready$/ }).waitFor();

  let since = (await core.observed()).next;
  await core.step("killed");
  await core.wait({ kind: "said", text: "Kill switch on. All actions stopped." }, since);
  assert.doesNotMatch(await header.innerText(), /Kill switch on/, "the header changed before its status poll");
  await runFor(2000);
  await header.filter({ hasText: /^Elite running · Voice ready · Kill switch on$/ }).waitFor();

  // The step returns once the push-to-talk loop has handled the release (world.rs, press and
  // release steps), so the next poll reads the kill switch re-armed.
  since = (await core.observed()).next;
  await core.step("rearmed");
  await runFor(2000);
  await header.filter({ hasText: /^Elite running · Voice ready$/ }).waitFor();
  await core.wait({ kind: "said", text: "Re-armed." }, since);
});

/**
 * The voice tester runs gear down live (worlds/gear-down): Elite is in front, and the game lowers
 * the gear (Status.json's flag) when Key_L goes down. On the Voice tester page the commander
 * tests "gear down" and clicks Run live; the core presses Key_L, reads the flag the game set from
 * Status.json, and the page says the game confirmed it. The verify settles on the reaction alone, with no clock advance.
 */
journey("the voice tester runs gear down live and shows it confirmed", { world: "gear-down" }, async ({ page, core, open }) => {
  await core.step("running");
  await open("voicetester");
  await page.getByRole("textbox", { name: "Phrase" }).fill("gear down");
  await page.getByRole("button", { name: "Test", exact: true }).click();
  // Run live is on once the page's first voice poll says push-to-talk is in test mode.
  const live = page.getByRole("button", { name: "Run live", exact: true });
  await page.getByText("Sends it to Elite like a spoken command.").waitFor();

  const since = (await core.observed()).next;
  await live.click();
  await page.getByText(/^Pressed; the game confirmed it in \d+ ms\.$/).waitFor();
  const keys = (await core.observed(since)).observed.filter((o) => o.kind === "key").map((o) => [o.key, o.up]);
  assert.deepEqual(keys, [["Key_L", false], ["Key_L", true]], "one press of the gear key");
});
