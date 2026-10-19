import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import type { Filters, Origin } from "../../lib/search";
import type { Side } from "../../lib/trade";
import View from "./View.svelte";

const near = vi.fn<(side: Side, symbols: string[], origin: Origin, filters: Filters | null) => void>();
let cut: Side | "all" | null = null;
const anywhere = vi.fn<(side: Side, symbol: string, origin: Origin, filters: Filters | null) => void>();
vi.mock("../../lib/trade", async (orig) => {
  const real = await orig<typeof import("../../lib/trade")>();
  return {
    ...real,
    tradeSearch: async (side: Side, symbols: string[], origin: Origin = null, filters: Filters | null = null) => {
      near(side, symbols, origin, filters);
      const r = await real.tradeSearch(side, symbols, origin, filters);
      return cut === "all" || cut === side ? { ...r, results: r.results.map((p) => ({ ...p, truncated: true })) } : r;
    },
    tradeAnywhere: async (side: Side, symbol: string, origin: Origin = null, filters: Filters | null = null) => {
      anywhere(side, symbol, origin, filters);
      const r = await real.tradeAnywhere(side, symbol, origin, filters);
      return cut === "all" || cut === side ? { ...r, results: r.results.map((p) => ({ ...p, truncated: true })) } : r;
    },
  };
});

beforeEach(() => {
  near.mockReset();
  anywhere.mockReset();
  cut = null;
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
    expect(screen.queryByText(/Ask by voice/)).toBeNull();
    expect(screen.queryByText("Within")).toBeNull();

    const prices = screen.getAllByText(/^[\d,]+ cr$/).map((e) => Number((e.textContent ?? "").split(" cr")[0]?.replace(/,/g, "")));
    expect(prices[0]).toBe(390780);
    expect(prices.every((p, i) => i === 0 || p <= (prices[i - 1] ?? 0))).toBe(true);
    expect(screen.getAllByText(/d ago/).length).toBeGreaterThan(0);

    await fireEvent.click(screen.getByRole("tab", { name: "Buy" }));
    expect(await screen.findByText(/Cheapest anywhere/)).toBeTruthy();
    expect(anywhere).toHaveBeenLastCalledWith("buy", "painite", null, expect.anything());

    await fireEvent.click(screen.getByRole("button", { name: "Near" }));
    expect(await screen.findByText(/^Cheapest/)).toBeTruthy();
    expect(near).toHaveBeenLastCalledWith("buy", ["painite"], null, expect.anything());
  });

  const hint = /Too many markets for Painite: narrow the radius to see them all/;

  it("a full Sell answer has no narrow-the-radius hint", async () => {
    render(View);
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
  });

  it("a cut Sell answer says to narrow the radius", async () => {
    cut = "sell";
    render(View);
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.getByText(hint)).toBeTruthy();
    expect(near).toHaveBeenLastCalledWith("sell", ["painite"], null, expect.anything());
  });

  it("a cut Buy answer says to narrow the radius, and a Sell-only cut does not reach Buy", async () => {
    cut = "buy";
    render(View);
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
    await fireEvent.click(screen.getByRole("tab", { name: "Buy" }));
    expect(await screen.findByText(hint)).toBeTruthy();
  });

  it("an Anywhere answer, always cut at 100, shows no hint", async () => {
    cut = "all";
    render(View);
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.getByText(hint)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Anywhere" }));
    expect(await screen.findByText(/FCX-800/)).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
  });
});
