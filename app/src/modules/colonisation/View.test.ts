import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import projectsSample from "../../lib/fixtures/colonisation.json";
import tradeSample from "../../lib/fixtures/trade.json";
import type { BuySearch, ListPlan, Need, Project } from "../../lib/colonisation";
import View from "./View.svelte";

const load = vi.fn<() => Promise<Project[]>>();
const buy = vi.fn<(needs: Need[], origin: string | null, filters: unknown) => Promise<BuySearch>>();
vi.mock("../../lib/colonisation", async (orig) => ({
  ...(await orig<typeof import("../../lib/colonisation")>()),
  loadProjects: () => load(),
  buyList: (n: Need[], o: string | null, f: unknown) => buy(n, o, f),
}));

const projects = projectsSample as unknown as Project[];
// The recorded search around Sol for the sample site's list (astromech-market dump-trade).
const recorded = (tradeSample as unknown as { list: ListPlan }).list;
const answer = (origin: string | null, plan: ListPlan = recorded): BuySearch => ({
  here: { system: origin ?? "Sol", pos: [0, 0, 0] },
  range: 20,
  plan,
});

beforeEach(() => {
  load.mockReset();
  buy.mockReset();
  localStorage.clear();
});

afterEach(() => {
  vi.useRealTimers();
});

/** A fresh copy each poll, as the shell sends: same content, new objects. */
const fresh = (): Project[] => structuredClone(projects);

describe("Colonisation", () => {
  it("lists what is left to buy and where, stations stocking most of the list first, from the site", async () => {
    load.mockResolvedValue(projects);
    buy.mockImplementation(async (_n, o) => answer(o));
    render(View);

    expect(await screen.findByRole("heading", { name: "Planetary Construction Site: Test Hub" })).toBeTruthy();
    expect(screen.getByText("42% built", { exact: false })).toBeTruthy();
    // The shopping list, in loads for the current ship; cargo aboard is already taken off.
    expect(screen.getByText("4,912 t")).toBeTruthy();
    expect(screen.getByText("90 t aboard")).toBeTruthy();
    expect(screen.getByText("9,554 t in all · 13 loads at 784 t")).toBeTruthy();

    // Asked for what is left to buy, from the construction site's system.
    await waitFor(() => expect(buy).toHaveBeenCalled());
    expect(buy).toHaveBeenLastCalledWith(
      [
        { symbol: "steel", name: "Steel", tonnes: 4912 },
        { symbol: "cmmcomposite", name: "CMM Composite", tonnes: 3820 },
        { symbol: "ceramiccomposites", name: "Ceramic Composites", tonnes: 512 },
        { symbol: "insulatingmembrane", name: "Insulating Membrane", tonnes: 310 },
      ],
      "Wolf 359",
      expect.anything(),
    );

    // Efremov Plant sells three of the four and comes first; the carrier at Barnard's Star is
    // nearer but sells only steel, so it comes last. Each row shows the stock and pad.
    const efremov = await screen.findByText("Efremov Plant", { exact: false });
    const carrier = screen.getByText("FCX-801", { exact: false });
    expect(efremov.compareDocumentPosition(carrier) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    const rows = screen.getAllByRole("listitem").map((li) => li.textContent ?? "").filter((t) => / of 4 · /.test(t));
    expect(rows[0]).toContain("3 of 4");
    expect(rows[0]).toContain("Steel 25,553 t, CMM Composite 10,061 t, Ceramic Composites 2,120 t");
    expect(rows[0]).toContain("L pad");
    expect(rows.at(-1)).toContain("1 of 4");
    expect(rows.at(-1)).toContain("fleet carrier");
  });

  it("searches near the commander once the site's system is cleared", async () => {
    load.mockResolvedValue(projects);
    buy.mockImplementation(async (_n, o) => answer(o));
    render(View);
    await waitFor(() => expect(buy).toHaveBeenLastCalledWith(expect.anything(), "Wolf 359", expect.anything()));
    await fireEvent.click(await screen.findByTitle("Search from the current system"));
    await waitFor(() => expect(buy).toHaveBeenLastCalledWith(expect.anything(), null, expect.anything()));
  });

  it("refreshes after a purchase, keeps the picked project, and re-searches only when the list changes", async () => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    load.mockImplementation(async () => fresh());
    buy.mockImplementation(async (_n, o) => answer(o));
    render(View);
    expect(await screen.findByText("Efremov Plant", { exact: false })).toBeTruthy();
    const searches = buy.mock.calls.length;
    const polls = load.mock.calls.length;

    // A poll with nothing changed: no new search.
    await vi.advanceTimersByTimeAsync(2000);
    await waitFor(() => expect(load.mock.calls.length).toBeGreaterThan(polls));
    expect(buy.mock.calls.length).toBe(searches);

    // 912 t of steel bought: the list, the total and the search follow.
    load.mockImplementation(async () => {
      const p = fresh();
      const site = p[0];
      const steel = site?.shopping.lines[0];
      if (!site || !steel) throw new Error("sample changed");
      steel.in_cargo = 912;
      steel.to_buy = 4000;
      site.shopping.to_buy_total = 8642;
      site.shopping.loads = 12;
      return p;
    });
    await vi.advanceTimersByTimeAsync(2000);
    expect(await screen.findByText("912 t aboard")).toBeTruthy();
    expect(screen.getByText("8,642 t in all · 12 loads at 784 t")).toBeTruthy();
    await waitFor(() => expect(buy.mock.calls.length).toBe(searches + 1));
    expect(buy.mock.lastCall?.[0][0]).toEqual({ symbol: "steel", name: "Steel", tonnes: 4000 });

    // The picked project stays picked across polls.
    await fireEvent.click(screen.getByRole("button", { name: /Test Relay/ }));
    expect(await screen.findByText("Nothing left to deliver.")).toBeTruthy();
    const after = load.mock.calls.length;
    await vi.advanceTimersByTimeAsync(4000);
    await waitFor(() => expect(load.mock.calls.length).toBeGreaterThan(after + 1));
    expect(screen.getByRole("heading", { name: "Orbital Construction Site: Test Relay" })).toBeTruthy();
  });

  it("shows a poll slower than the interval, without asking again meanwhile", async () => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    buy.mockImplementation(async (_n, o) => answer(o));
    // A cold replay: the first answer takes three poll intervals and is the only one.
    let slow: (p: Project[]) => void = () => {};
    load.mockImplementation(() => new Promise<Project[]>((r) => (slow = r)));
    render(View);
    await vi.advanceTimersByTimeAsync(6000);
    expect(load).toHaveBeenCalledTimes(1);
    slow(fresh());
    expect(await screen.findByRole("heading", { name: "Planetary Construction Site: Test Hub" })).toBeTruthy();

    // Polling resumes once it has answered.
    load.mockClear();
    load.mockImplementation(async () => fresh());
    await vi.advanceTimersByTimeAsync(2000);
    expect(load).toHaveBeenCalledTimes(1);
  });

  it("shows a slow failure when it is the only answer", async () => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    load.mockImplementation(() => new Promise<Project[]>((_r, reject) => setTimeout(() => reject("journal folder not found: X:\\gone"), 5000)));
    render(View);
    await vi.advanceTimersByTimeAsync(5000);
    expect(await screen.findByText("Could not read the construction sites: journal folder not found: X:\\gone.")).toBeTruthy();
    expect(load).toHaveBeenCalledTimes(1);
  });

  it("names commodities no market in range sells", async () => {
    load.mockResolvedValue(projects);
    buy.mockImplementation(async (_n, o) => answer(o, { ...recorded, unavailable: ["Insulating Membrane"] }));
    render(View);
    expect(await screen.findByText("No market in range sells Insulating Membrane.")).toBeTruthy();
  });

  it("doesn't search for a finished project", async () => {
    load.mockResolvedValue(projects);
    buy.mockImplementation(async (_n, o) => answer(o));
    render(View);
    await fireEvent.click(await screen.findByRole("button", { name: /Test Relay/ }));
    expect(await screen.findByText("Nothing left to deliver.")).toBeTruthy();
    buy.mockClear();
    await new Promise((r) => setTimeout(r, 20));
    expect(buy).not.toHaveBeenCalled();
    expect(screen.queryByText("Where to buy", { exact: false })).toBeNull();
  });

  it("says when there are no projects, or the journals can't be read", async () => {
    load.mockResolvedValue([]);
    const { unmount } = render(View);
    expect(await screen.findByText(/No construction sites in your journals yet/)).toBeTruthy();
    // Systems open to claim show without a project.
    expect(screen.getByRole("heading", { name: /Colonisation candidates/ })).toBeTruthy();
    expect(await screen.findByText("Greae Phoea VS-M b39-29")).toBeTruthy();
    unmount();

    load.mockRejectedValue("journal folder not found: C:\\nowhere");
    render(View);
    expect(await screen.findByText("Could not read the construction sites: journal folder not found: C:\\nowhere.")).toBeTruthy();
    expect(buy).not.toHaveBeenCalled();
  });
});
