import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import type { Filters, Origin } from "../../lib/search";
import type { Side } from "../../lib/trade";
import View from "./View.svelte";

const near = vi.fn<(side: Side, symbols: string[], origin: Origin, filters: Filters | null) => void>();
let cut = false;
const anywhere = vi.fn<(side: Side, symbol: string, origin: Origin, filters: Filters | null) => void>();
vi.mock("../../lib/trade", async (orig) => {
  const real = await orig<typeof import("../../lib/trade")>();
  return {
    ...real,
    tradeSearch: async (side: Side, symbols: string[], origin: Origin = null, filters: Filters | null = null) => {
      near(side, symbols, origin, filters);
      const r = await real.tradeSearch(side, symbols, origin, filters);
      return cut ? { ...r, results: r.results.map((p) => ({ ...p, truncated: true })) } : r;
    },
    tradeAnywhere: (side: Side, symbol: string, origin: Origin = null, filters: Filters | null = null) => {
      anywhere(side, symbol, origin, filters);
      return real.tradeAnywhere(side, symbol, origin, filters);
    },
  };
});

beforeEach(() => {
  near.mockReset();
  anywhere.mockReset();
  cut = false;
  localStorage.clear();
});

describe("Trade", () => {
  it("switches Sell to the best prices anywhere, in price order with data age and no radius filter", async () => {
    render(View);
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.getByText("Within")).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Anywhere" }));
    expect(await screen.findByText(/FCX-800/)).toBeTruthy();
    expect(anywhere).toHaveBeenLastCalledWith("sell", "painite", null, expect.anything());
    expect(screen.getByText(/Best prices anywhere/)).toBeTruthy();
    expect(screen.getByText(/anywhere in the galaxy, highest price first/)).toBeTruthy();
    expect(screen.queryByText("Within")).toBeNull();

    const prices = screen.getAllByText(/^[\d,]+ cr · /).map((e) => Number((e.textContent ?? "").split(" cr")[0]?.replace(/,/g, "")));
    expect(prices[0]).toBe(390780);
    expect(prices.every((p, i) => i === 0 || p <= (prices[i - 1] ?? 0))).toBe(true);
    expect(screen.getAllByText(/reported /).length).toBeGreaterThan(0);

    await fireEvent.click(screen.getByRole("button", { name: "Buy" }));
    expect(await screen.findByText(/Cheapest anywhere/)).toBeTruthy();
    expect(anywhere).toHaveBeenLastCalledWith("buy", "painite", null, expect.anything());

    await fireEvent.click(screen.getByRole("button", { name: "Near" }));
    expect(await screen.findByText(/^Cheapest/)).toBeTruthy();
    expect(near).toHaveBeenLastCalledWith("buy", ["painite"], null, expect.anything());
  });

  it("says when Ardent cut the nearby answer at 1000 markets, and only then", async () => {
    render(View);
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();

    cut = true;
    await fireEvent.click(screen.getByRole("button", { name: "Buy" }));
    expect(await screen.findByText(/Too many markets for Painite: narrow the radius to see them all/)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Sell" }));
    expect(await screen.findByText(/narrow the radius to see them all/)).toBeTruthy();
  });
});
