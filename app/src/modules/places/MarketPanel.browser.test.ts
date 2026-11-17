// The Sell side panel in the window's frame, with the design tokens loaded so the page column,
// the route strip and the status bar have their real sizes.
import "../../design/tokens.css";
import { afterEach, expect, it } from "vitest";
import { page } from "vitest/browser";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import App from "../../App.svelte";
import { forgetAnswers } from "../../lib/places";
import { resetScale, setTextSize } from "../../lib/scale.svelte";
import { DEFAULT_FILTERS } from "../../lib/search";
import { answer, startPreview } from "../../test/transport";

answer("search_filters", () => DEFAULT_FILTERS);
answer("set_search_filters", () => {});

afterEach(async () => {
  await page.viewport(1200, 800);
  resetScale();
  cleanup();
  forgetAnswers();
  location.hash = "";
  history.replaceState(null, "", "/");
});

it("fills the side panel from the selected result: every fact under its head, and its two actions in reach", async () => {
  // A short window at a large Text size, with a route: the strip over the page, the status bar
  // under it, and the list filling what is left. The panel sits below the list, taller than the
  // page column, so the commander scrolls to its actions.
  await page.viewport(1100, 500);
  startPreview("/?core=ready&route=exact#/places/sell/painite");
  expect(await setTextSize(145)).toBeNull();
  render(App);
  const strip = await screen.findByRole("region", { name: "Route" });
  expect(screen.queryByRole("complementary", { name: "Details" })).toBeNull();
  const [pick] = within(await screen.findByRole("table", { name: "Results" })).getAllByRole("button", { pressed: false });
  const name = pick!.textContent!;
  await fireEvent.click(pick!);
  expect(pick!.getAttribute("aria-pressed")).toBe("true");
  const aside = await screen.findByRole("complementary", { name: "Details" });
  const panel = within(aside);
  expect(panel.getByRole("heading", { name }).textContent).toBe(name);
  // Demand under the price, then every other fact under its name, the report's age among them,
  // which the row leaves out.
  expect(panel.getByRole("img", { name: /^Demand [\d,]+ t/ })).toBeTruthy();
  expect(panel.getAllByRole("term").map((d) => d.textContent)).toEqual(["Pad", "Arrival", "Trip", "Updated"]);

  // Each action scrolls into the page column, clear of the strip above it and the status bar
  // below: the point at its middle is the button itself. At 145 percent the rem sizes fall on
  // fractions of a pixel, so the edges may meet a fraction apart.
  const column = document.querySelector("main")!;
  const bar = document.querySelector("footer")!;
  expect(aside.getBoundingClientRect().height, "the panel is taller than the page column").toBeGreaterThan(column.clientHeight);
  for (const label of ["Set destination", "Add stop"]) {
    const button = panel.getByRole("button", { name: label });
    button.scrollIntoView({ block: "nearest" });
    const r = button.getBoundingClientRect();
    const col = column.getBoundingClientRect();
    expect(r.top, label).toBeGreaterThan(Math.max(col.top, strip.getBoundingClientRect().bottom) - 1);
    expect(r.bottom, label).toBeLessThan(Math.min(col.bottom, bar.getBoundingClientRect().top) + 1);
    expect(document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2), label).toBe(button);
  }
  // What the panel's action did shows beside its buttons, not under a list that may be off screen.
  await fireEvent.click(panel.getByRole("button", { name: "Set destination" }));
  expect(await panel.findByText(/Following\. .+ is on your clipboard\./)).toBeTruthy();
});
