import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import sample from "../../lib/fixtures/trade.json";
import { DEFAULT_FILTERS, type Filters, type Origin } from "../../lib/search";
import type { RouteAsk, RouteDefaults, RouteKind, RouteSearch, TradeRoute } from "../../lib/trade";
import Route from "./Route.svelte";

// The preview's recorded route (fixtures/trade.json: Spansh's one-hop answer from Abraham
// Lincoln in Sol, and the way back from Ardent's markets), behind spies.
const recorded = sample as unknown as { route: { defaults: RouteDefaults } & Record<RouteKind, TradeRoute> };
const here: RouteSearch["here"] = { system: "Sol", pos: [0, 0, 0] };
const route = vi.fn<(ask: RouteAsk, origin: Origin, filters: Filters | null) => Promise<RouteSearch>>();
const defaults = vi.fn<() => Promise<RouteDefaults>>();
vi.mock("../../lib/trade", async (orig) => ({
  ...(await orig<typeof import("../../lib/trade")>()),
  tradeRoute: (a: RouteAsk, o: Origin, f: Filters | null) => route(a, o, f),
  tradeRouteDefaults: () => defaults(),
}));

// The shell resolves the pad for the ship flown now when it loads the filters.
let shipPad: Filters["pad"] = "M";
vi.mock("../../lib/search", async (orig) => ({
  ...(await orig<typeof import("../../lib/search")>()),
  loadFilters: async () => ({ ...DEFAULT_FILTERS, pad: shipPad }),
  saveFilters: async () => {},
}));

beforeEach(() => {
  shipPad = "M";
  localStorage.clear();
  route.mockReset();
  defaults.mockReset();
  defaults.mockResolvedValue(recorded.route.defaults);
  route.mockImplementation(async (a) => ({ here, route: recorded.route[a.kind] }));
});

const field = (name: string) => screen.getByRole(name === "Start station" ? "textbox" : "spinbutton", { name }) as HTMLInputElement;

describe("Trade route", () => {
  it("starts from the ship's numbers and shows the single hop with profit per ton and an estimated per hour", async () => {
    render(Route);
    await vi.waitFor(() => expect(field("Cargo capacity").value).toBe("200"));
    expect(field("Credits").value).toBe("10000000");
    expect(field("Max hop distance").value).toBe("31.5");
    expect(field("Start station").placeholder).toBe("Abraham Lincoln");
    expect(route).not.toHaveBeenCalled(); // a Spansh job only on request

    await fireEvent.click(screen.getByRole("button", { name: "Find route" }));
    expect(await screen.findByText("200 t Biowaste: buy 110 cr, sell 754 cr")).toBeTruthy();
    expect(route).toHaveBeenCalledExactlyOnceWith(
      { kind: "single", station: "Abraham Lincoln", capacity: 200, credits: 10_000_000, max_hop_ly: 31.5 },
      null,
      expect.objectContaining({ pad: "M" }),
    );
    expect(screen.getByText(/644 cr\/t · 128,800 cr · ~1 jump · 23\.8 ly/)).toBeTruthy();
    // Per hour counts the flight back empty, so it compares with a loop.
    expect(screen.getByText(/128,800 cr a trip · about 1,065,123 cr\/h/)).toBeTruthy();
    expect(screen.getByText(/Spansh gives no profit per hour, so it's estimated: 7 min a round trip \(flying back empty\)/)).toBeTruthy();
    expect(screen.getByText("L pad · 6 ls · Spansh")).toBeTruthy();
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("finds a loop with the cargo back to the start, from overridden numbers", async () => {
    render(Route);
    await vi.waitFor(() => expect(field("Cargo capacity").value).toBe("200"));
    await fireEvent.click(screen.getByRole("button", { name: "Loop" }));
    await fireEvent.input(field("Cargo capacity"), { target: { value: "96" } });
    await fireEvent.input(field("Credits"), { target: { value: "2500000" } });
    await fireEvent.input(field("Max hop distance"), { target: { value: "18" } });
    await fireEvent.input(field("Start station"), { target: { value: "Daedalus" } });
    await fireEvent.click(screen.getByRole("button", { name: "Find route" }));

    expect(await screen.findByText("200 t Reactive Armour: buy 1,581 cr, sell 2,674 cr")).toBeTruthy();
    expect(route).toHaveBeenLastCalledWith({ kind: "loop", station: "Daedalus", capacity: 96, credits: 2_500_000, max_hop_ly: 18 }, null, expect.anything());
    expect(screen.getByText(/347,400 cr a round · about 2,872,855 cr\/h/)).toBeTruthy();
    expect(screen.getAllByText(/Ardent Insight/)).toHaveLength(1); // the way back's source
  });

  it("says when nothing pays, when the way back is empty, and when Spansh fails", async () => {
    const single = recorded.route.single;
    const back = { ...recorded.route.loop.legs[1]!, cargo: [], profit: 0, profit_per_t: 0 };
    route.mockResolvedValueOnce({ here, route: { ...single, legs: [], profit: 0, per_hour: 0 } });
    render(Route);
    await vi.waitFor(() => expect(field("Cargo capacity").value).toBe("200"));
    const find = screen.getByRole("button", { name: "Find route" });

    await fireEvent.click(find);
    expect(await screen.findByText(/No profitable route within these limits/)).toBeTruthy();

    route.mockResolvedValueOnce({ here, route: { ...recorded.route.loop, legs: [single.legs[0]!, back] } });
    await fireEvent.click(find);
    expect(await screen.findByText("Nothing worth carrying back: fly back empty.")).toBeTruthy();

    route.mockRejectedValueOnce("Spansh took too long to answer. Try again in a minute.");
    await fireEvent.click(find);
    expect(await screen.findByText("Spansh took too long to answer. Try again in a minute.")).toBeTruthy();
  });

  it("warns when a stop's largest pad is too small for the ship", async () => {
    const single = recorded.route.single;
    const leg = single.legs[0]!;
    const warning = "Starbase Pratchett (LP 71-165) has no M pad: its largest is S. Spansh can only require a large pad.";
    route.mockResolvedValueOnce({ here, route: { ...single, legs: [{ ...leg, to: { ...leg.to, pad: "S" } }], warning } });
    render(Route);
    await vi.waitFor(() => expect(field("Cargo capacity").value).toBe("200"));
    await fireEvent.click(screen.getByRole("button", { name: "Find route" }));
    expect((await screen.findByRole("alert")).textContent).toBe(warning);
    expect(screen.getByText("S pad · 6 ls · Spansh")).toBeTruthy();
  });

  it("can't search without a hold or credits", async () => {
    defaults.mockResolvedValue({ capacity: null, credits: null, range: null, station: null, system: null });
    render(Route);
    await vi.waitFor(() => expect(field("Max hop distance").value).toBe("20"));
    expect(field("Start station").placeholder).toBe("Nearest the star");
    expect((screen.getByRole("button", { name: "Find route" }) as HTMLButtonElement).disabled).toBe(true);
    await fireEvent.input(field("Cargo capacity"), { target: { value: "64" } });
    await fireEvent.input(field("Credits"), { target: { value: "1000000" } });
    expect((screen.getByRole("button", { name: "Find route" }) as HTMLButtonElement).disabled).toBe(false);
  });

  it("labels the ship's pad with the ship flown at the last search", async () => {
    render(Route);
    await vi.waitFor(() => expect(field("Cargo capacity").value).toBe("200"));
    expect(await screen.findByText("Ship's (M)")).toBeTruthy();
    shipPad = "L"; // swapped to an Anaconda
    await fireEvent.click(screen.getByRole("button", { name: "Find route" }));
    expect(await screen.findByText("Ship's (L)")).toBeTruthy();
  });
});
