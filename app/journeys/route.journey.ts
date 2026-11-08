/**
 * The route journeys (success criterion journeys-in-ci; docs/PLOTTING.md, the route follower).
 * The route desk's own cases (legs, detours, parking, copy failures) are route.rs's tests.
 */
import assert from "node:assert/strict";
import { journey } from "./harness.ts";

/**
 * Plot and follow (worlds/journeys/plot): on the Route page the commander types Colonia in the
 * header's plot bar and plots a neutron route from Sol. Spansh answers from its recorded
 * answers, and the page follows the route with PSR J1752-2806 next. A jump into that stop (a
 * journal line) moves the core on at once, and the page once its 2 s route poll runs.
 */
journey("plot a route from the page and follow it", { world: "journeys/plot" }, async ({ page, core, open, runFor }) => {
  await core.step("running");
  await open("route");

  const box = page.getByRole("combobox", { name: "Find or plot" });
  await box.fill("Colonia");
  // The plot bar asks Spansh for system names 250 ms after the last keystroke.
  await runFor(250);
  await page.getByRole("listbox", { name: "Matches" }).getByRole("option", { name: /^Colonia/ }).waitFor();
  await box.press("Enter");
  // The plot panel, with the range the plotter read from the ship's Loadout.
  const panel = page.getByRole("dialog", { name: "Plot to Colonia" });
  await panel.getByText("Asp Explorer: 36.48 ly a jump").waitFor();
  await box.press("Enter");

  await panel.getByText(/^Following Sol to Colonia: 284 jumps · 22,000 ly · 178 neutron boosts/).waitFor();
  const next = page.locator(".next .sys");
  await next.filter({ hasText: /^PSR J1752-2806$/ }).waitFor();

  await core.step("jumped");
  await core.wait({ kind: "clipboard", text: "Nova Aquila No 3" });
  assert.equal(await next.innerText(), "PSR J1752-2806", "the page moved on before its route poll");
  await runFor(2000);
  await next.filter({ hasText: /^Nova Aquila No 3$/ }).waitFor();
});

/**
 * A jump into the next stop (worlds/journeys/next-stop, the steps of worlds/jump): astromech's
 * own route, Sol, Alpha Centauri, Barnard's Star, set from the window. Jumping into Alpha
 * Centauri copies Barnard's Star and reads it out, and the page shows it next.
 */
journey("a jump into the next stop copies and reads out the one after", { world: "journeys/next-stop" }, async ({ page, core, open, runFor }) => {
  await core.step("running");
  const waypoints = [{ system: "Sol" }, { system: "Alpha Centauri" }, { system: "Barnard's Star" }];
  await core.callWindow("route_set", { name: "Trip", waypoints });
  await core.wait({ kind: "clipboard", text: "Alpha Centauri" });
  await open("route");
  const next = page.locator(".next .sys");
  await next.filter({ hasText: /^Alpha Centauri$/ }).waitFor();

  const since = (await core.observed()).next;
  await core.step("jumped");
  await core.wait({ kind: "clipboard", text: "Barnard's Star" }, since);
  await core.wait({ kind: "said", text: "Next waypoint. Barnard's Star." }, since);
  assert.equal(await next.innerText(), "Alpha Centauri", "the page moved on before its route poll");
  await runFor(2000);
  await next.filter({ hasText: /^Barnard's Star$/ }).waitFor();
});
