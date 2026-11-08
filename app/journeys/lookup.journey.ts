/**
 * A lookup fails and recovers (success criterion journeys-in-ci; DESIGN.md States, hard error).
 * On the Services finder's Black Market page, Spansh first answers the core's station search with
 * a 503 (worlds/journeys/lookup-retry/net.json): the page shows the red "!", the reason and TRY
 * AGAIN. TRY AGAIN searches again, Spansh answers with the recorded page, and the page lists it.
 */
import assert from "node:assert/strict";
import { journey } from "./harness.ts";

const SEARCH = "https://spansh.co.uk/api/stations/search";

journey("a lookup fails with its error and TRY AGAIN, and recovers", { world: "journeys/lookup-retry" }, async ({ page, core, open }) => {
  await open("places/services/black_market");

  const failure = page.getByRole("alert").filter({ hasText: "Spansh is having trouble (HTTP 503)" });
  await failure.waitFor();
  assert.equal(await failure.locator("b").innerText(), "!");
  const again = failure.getByRole("button", { name: "Try again" });
  assert.equal(await again.innerText(), "TRY AGAIN");

  await again.click();
  // The recorded page's first station (worlds/preview/sol-nov/spansh-black-market.json).
  const results = page.getByRole("table", { name: "Results" });
  await results.getByRole("row").filter({ hasText: "Rescue Ship - Li Qing Jao" }).waitFor();
  assert.equal(await page.getByRole("alert").count(), 0);
  assert.equal(await results.getByRole("row").count(), 1 + 10);

  const asked = (await core.observed()).observed.filter((o) => o.kind === "http");
  assert.equal(asked.filter((o) => o.url === SEARCH).length, 2, `the core asked Spansh twice: ${JSON.stringify(asked)}`);
  assert.deepEqual(asked.filter((o) => o.unrecorded), [], "every request the core made is in the world's recording");
});
