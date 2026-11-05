import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { FINDERS, forgetAnswers, lastAnswer, type FinderId } from "../../lib/places";
import { DEFAULT_FILTERS, type Filters, type Origin } from "../../lib/search";
import type { Side } from "../../lib/trade";
import Cargo from "./Cargo.svelte";
import Market from "./Market.svelte";

const near = vi.fn<(side: Side, symbols: string[], origin: Origin, filters: Filters | null) => void>();
let cut: Side | "all" | null = null;
let extraMission = false;
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
    tradeCargo: async (origin: Origin = null, filters: Filters | null = null) => {
      const r = await real.tradeCargo(origin, filters);
      if (!extraMission) return r;
      return { ...r, plan: { ...r.plan, kept: [...r.plan.kept, { name: "Gold", reason: "mission cargo", tonnes: 4 }] } };
    },
  };
});

const loaded = vi.fn<(lookup: string) => void>();
vi.mock("../../lib/search", async (orig) => ({
  ...(await orig<typeof import("../../lib/search")>()),
  loadFilters: (lookup: string) => (loaded(lookup), Promise.resolve(DEFAULT_FILTERS)),
  saveFilters: () => Promise.resolve(),
}));

const finder = (id: FinderId) => FINDERS.find((f) => f.id === id)!;

beforeEach(() => {
  near.mockReset();
  anywhere.mockReset();
  loaded.mockReset();
  cut = null;
  extraMission = false;
  forgetAnswers();
});

afterEach(() => {
  cleanup();
  location.hash = "";
});

describe("Sell and Buy finders", () => {
  it("opens Sell on painite near you, and switches to the best prices anywhere, in price order with data age and no radius filter", async () => {
    location.hash = "#/places/sell";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(location.hash).toBe("#/places/sell/painite");
    expect((screen.getByRole("combobox", { name: "Commodity" }) as HTMLInputElement).value).toBe("Painite");
    expect(near).toHaveBeenLastCalledWith("sell", ["painite"], null, expect.anything());
    expect(loaded).toHaveBeenLastCalledWith("trade.sell");
    expect(screen.getByText("Within")).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Anywhere" }));
    expect(location.hash).toBe("#/places/sell/painite?range=anywhere");
    expect(await screen.findByText(/FCX-800/)).toBeTruthy();
    expect(anywhere).toHaveBeenLastCalledWith("sell", "painite", null, expect.anything());
    // Each range keeps the filters Trade remembered for it, which voice questions use too.
    expect(loaded).toHaveBeenLastCalledWith("trade.sell.anywhere");
    expect(screen.getByText(/Best prices anywhere/)).toBeTruthy();
    expect(screen.queryByText("Within")).toBeNull();

    // The price leads each row, highest first, and every row says how old its report is.
    expect(screen.getAllByRole("columnheader")[0]?.textContent).toBe("Price");
    const prices = [...document.querySelectorAll(".main .v")].map((e) => Number((e.textContent ?? "").replace(/,/g, "")));
    expect(prices[0]).toBe(390780);
    expect(prices.every((p, i) => i === 0 || p <= (prices[i - 1] ?? 0))).toBe(true);
    expect([...document.querySelectorAll(".fact.age")].every((e) => /^[\d,]+(d|h|min)$/.test(e.textContent ?? ""))).toBe(true);

    // Its tile keeps the answer and reopens the same search.
    expect(lastAnswer("sell")).toMatchObject({ figure: "390,780", unit: "cr", context: "Painite", route: "places/sell/painite?range=anywhere" });
  });

  it("opens Buy from its address, cheapest first, near or anywhere", async () => {
    location.hash = "#/places/buy/lowtemperaturediamond?range=anywhere";
    render(Market, { finder: finder("buy") });
    expect(await screen.findByText(/Cheapest anywhere/)).toBeTruthy();
    expect(anywhere).toHaveBeenLastCalledWith("buy", "lowtemperaturediamond", null, expect.anything());
    expect(loaded).toHaveBeenLastCalledWith("trade.buy.anywhere");
    expect((screen.getByRole("combobox", { name: "Commodity" }) as HTMLInputElement).value).toBe("Low Temperature Diamonds");

    await fireEvent.click(screen.getByRole("button", { name: "Near" }));
    expect(location.hash).toBe("#/places/buy/lowtemperaturediamond");
    expect(await screen.findByText(/^Within 100 ly of Sol/)).toBeTruthy();
    expect(near).toHaveBeenLastCalledWith("buy", ["lowtemperaturediamond"], null, expect.anything());
    expect(loaded).toHaveBeenLastCalledWith("trade.buy");
  });

  it("picks a commodity by a short name in the main input", async () => {
    location.hash = "#/places/sell/painite";
    render(Market, { finder: finder("sell") });
    await screen.findByText("Good Landing", { exact: false });
    const box = screen.getByRole("combobox", { name: "Commodity" });
    await fireEvent.input(box, { target: { value: "LTDs" } });
    await screen.findByText("Low Temperature Diamonds");
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(location.hash).toBe("#/places/sell/lowtemperaturediamond");
    await waitFor(() => expect(near).toHaveBeenLastCalledWith("sell", ["lowtemperaturediamond"], null, expect.anything()));
  });

  it("reads an unknown commodity in the address as painite", async () => {
    location.hash = "#/places/sell/unobtainium";
    render(Market, { finder: finder("sell") });
    await waitFor(() => expect(near).toHaveBeenLastCalledWith("sell", ["painite"], null, expect.anything()));
    expect(location.hash).toBe("#/places/sell/painite");
  });

  const hint = /Too many markets for Painite: narrow the radius to see them all/;

  it("a full Sell answer has no narrow-the-radius hint", async () => {
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
  });

  it("a cut Sell answer says to narrow the radius", async () => {
    cut = "sell";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.getByText(hint)).toBeTruthy();
  });

  it("a cut Buy answer says to narrow the radius, and a Sell-only cut does not reach Buy", async () => {
    cut = "buy";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
    cleanup();
    render(Market, { finder: finder("buy") });
    expect(await screen.findByText(hint)).toBeTruthy();
  });

  it("an Anywhere answer, always cut at 100, shows no hint", async () => {
    cut = "all";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.getByText(hint)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Anywhere" }));
    expect(await screen.findByText(/FCX-800/)).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
  });
});

describe("Sell my cargo finder", () => {
  it("lists the hold, and two mission lots of the same commodity do not crash the page", async () => {
    extraMission = true;
    location.hash = "#/places/sell-cargo";
    render(Cargo, { finder: finder("sell-cargo") });
    expect(await screen.findByText("Low Temperature Diamonds")).toBeTruthy();
    expect(loaded).toHaveBeenLastCalledWith("trade.cargo");
    expect(screen.getByRole("heading", { level: 1, name: "Sell my cargo" })).toBeTruthy();
    expect(document.querySelector(".finder .head .main"), "no main input").toBeNull();
    expect(screen.getByText("300 t")).toBeTruthy();
    expect(screen.getByText("stolen")).toBeTruthy();
    expect(screen.getAllByText("mission cargo")).toHaveLength(2);
    expect(screen.getByText("4 t")).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Best for each on its own" })).toBeTruthy();
    expect(lastAnswer("sell-cargo")).toMatchObject({ unit: "cr", route: "places/sell-cargo" });
  });
});
