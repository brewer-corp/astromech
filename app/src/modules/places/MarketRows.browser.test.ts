// The Sell rows' geometry, with the design tokens loaded so the row has its real widths.
import "../../design/tokens.css";
import { afterEach, expect, it } from "vitest";
import { page } from "vitest/browser";
import { cleanup, render, screen, within } from "@testing-library/svelte";
import { FINDERS, forgetAnswers } from "../../lib/places";
import { DEFAULT_FILTERS } from "../../lib/search";
import type { Side } from "../../lib/trade";
import { answer, previewAnswer, startPreview } from "../../test/transport";
import Market from "./Market.svelte";

answer("trade_anywhere", (args: { side: Side }) => previewAnswer("trade_anywhere", args));
answer("search_filters", () => DEFAULT_FILTERS);
answer("set_search_filters", () => {});

afterEach(async () => {
  await page.viewport(1200, 800);
  cleanup();
  forgetAnswers();
  location.hash = "";
});

it("lines up every station's name and system on Sell anywhere, with a type icon or without (DESIGN.md Aligned Columns Rule)", async () => {
  await page.viewport(1100, 700);
  startPreview("/?core=ready#/places/sell/painite?range=anywhere");
  render(Market, { finder: FINDERS.find((f) => f.id === "sell")! });
  const table = await screen.findByRole("table", { name: "Results" });
  await within(table).findByText(/FCX-800/);
  // Ardent names a type only for carriers and settlements, so the answer mixes rows with an icon and without.
  const rows = [...table.querySelectorAll<HTMLElement>(".result")];
  const iconed = rows.filter((r) => within(r).queryByRole("img", { name: /Carrier|Settlement/ }));
  expect(iconed.length).toBeGreaterThan(0);
  expect(iconed.length).toBeLessThan(rows.length);
  const left = (e: Element | null) => Math.round(e!.getBoundingClientRect().left);
  const x = left(within(rows[0]!).getByRole("button", { pressed: false }));
  for (const r of rows) {
    expect(left(within(r).getByRole("button", { pressed: false }))).toBe(x);
    expect(left(r.querySelector(".l2 .sys"))).toBe(x);
  }
});
