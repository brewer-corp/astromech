import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { fakeCall, resetFakeCore } from "../fakecore";
import { DEFAULT_FILTERS, type Filters, type Origin, type Results } from "../search";
import SearchKit from "./SearchKit.svelte";

// Storage fails: the kit must still search (with the defaults) instead of showing nothing.
const loadFilters = vi.fn<(lookup: string) => Promise<Filters>>();
const saveFilters = vi.fn<(lookup: string, f: Filters) => Promise<void>>();
vi.mock("../search", async (orig) => ({
  ...(await orig<typeof import("../search")>()),
  loadFilters: (l: string) => loadFilters(l),
  saveFilters: (l: string, f: Filters) => saveFilters(l, f),
  systemNames: async () => [],
}));

const results: Results = {
  origin: "Sol",
  range: 20,
  source: "Spansh",
  fetched: "2026-10-16T12:00:00Z",
  rows: [{ key: 1, system: "Ross 154", ly: 9.7, jumps: 1, exact: true, updated_at: "2026-10-16T11:00:00Z", detail: "Ross 154 Boys" }],
};

describe("SearchKit", () => {
  it("searches with the default filters when the saved ones can't be read", async () => {
    vi.spyOn(console, "warn").mockImplementation(() => {});
    loadFilters.mockRejectedValue("couldn't open astromech.sqlite");
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(results);
    render(SearchKit, { lookup: "faction.famine", offers: ["age"], run });

    expect(await screen.findByText("Ross 154")).toBeTruthy();
    expect(run).toHaveBeenCalledExactlyOnceWith(null, DEFAULT_FILTERS);
    expect(screen.getByRole("combobox")).toBeTruthy(); // the data-age filter still shows
  });

  it("keeps the later filter change when an earlier save fails after it", async () => {
    vi.spyOn(console, "warn").mockImplementation(() => {});
    const saves: Array<{ f: Filters; fail: (e: Error) => void; ok: () => void }> = [];
    let stored = DEFAULT_FILTERS;
    loadFilters.mockImplementation(async () => stored);
    saveFilters.mockImplementation(
      (_, f) =>
        new Promise<void>((ok, fail) => {
          saves.push({ f, ok: () => ((stored = f), ok()), fail });
        }),
    );
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(results);
    render(SearchKit, { lookup: "faction.famine", offers: ["age"], run });
    const select = (await screen.findByRole("combobox")) as HTMLSelectElement;

    await fireEvent.change(select, { target: { value: "7" } });
    await fireEvent.change(select, { target: { value: "30" } });
    expect(saves.map((s) => s.f.max_age_days)).toEqual([7, 30]);
    // the second save lands, then the first fails
    saves[1]!.ok();
    await waitFor(() => expect(select.value).toBe("30"));
    saves[0]!.fail(new Error("database is locked"));
    await new Promise((r) => setTimeout(r, 20));
    expect(select.value).toBe("30");
    expect(screen.queryByText(/Couldn't save/)).toBeNull();
  });

  it("labels the ship's pad with the current ship's, not the one the filters loaded with", async () => {
    // Filters as the shell resolves them: not chosen, so the pad is the current ship's.
    let pad: Filters["pad"] = "M";
    loadFilters.mockImplementation(async () => ({ ...DEFAULT_FILTERS, pad }));
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(results);
    render(SearchKit, { lookup: "stations.service", offers: ["pad", "carriers", "odyssey"], run });
    expect(await screen.findByRole("button", { name: "Pad M, current ship" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Carriers" }).getAttribute("aria-pressed")).toBe("true");
    expect(screen.getByRole("button", { name: "Odyssey" }).getAttribute("aria-pressed")).toBe("true");
    expect(screen.getByRole("button", { name: "Pad M, current ship" }).getAttribute("aria-pressed")).toBe("true");

    pad = "L"; // the commander swaps to an Anaconda, then searches from another system
    const box = screen.getByLabelText("Search from system");
    await fireEvent.input(box, { target: { value: "Deciat" } });
    await fireEvent.keyDown(box, { key: "Enter" });
    await waitFor(() => expect(run).toHaveBeenLastCalledWith("Deciat", expect.anything()));
    expect(await screen.findByRole("button", { name: "Pad L, current ship" })).toBeTruthy();
    expect(screen.queryByRole("button", { name: "Pad M, current ship" })).toBeNull();
  });

  it("puts arrival on the station name, distance with the system, and allegiance in the row", async () => {
    loadFilters.mockResolvedValue(DEFAULT_FILTERS);
    const station: Results = {
      ...results,
      rows: [
        {
          key: 2,
          system: "Sol",
          station: "Grebby Prospecting",
          ly: 4.4,
          jumps: 1,
          exact: true,
          arrival_ls: 2202,
          station_type: "Settlement",
          allegiance: "Independent",
          faction: "Sol Mining Syndicate",
          state: "Boom",
          updated_at: null,
        },
      ],
    };
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(station);
    render(SearchKit, { lookup: "stations.service", offers: [], run });
    expect(await screen.findByText(/Grebby Prospecting/)).toBeTruthy();
    expect(screen.getByText("(2202 ls)")).toBeTruthy();
    expect(screen.getByText("4.4 ly · 1 jump")).toBeTruthy();
    expect(screen.getByText("Independent")).toBeTruthy();
    expect(screen.getByText("Sol Mining Syndicate")).toBeTruthy();
    expect(screen.getByText("Boom")).toBeTruthy();
    expect(screen.queryByText(/^[SML]$/)).toBeNull();
  });

  it("Go follows a station result, labelled with the station, and says when the core can't", async () => {
    loadFilters.mockResolvedValue(DEFAULT_FILTERS);
    const station: Results = {
      ...results,
      rows: [{ key: 2, system: "Deciat", station: "Garay Terminal", ly: 12, jumps: 1, exact: true, updated_at: null }],
    };
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(station);
    // Outside the app with no preview core, the core isn't there to follow it.
    history.replaceState(null, "", "/");
    const { unmount } = render(SearchKit, { lookup: "stations.service", offers: [], run });
    await fireEvent.click(await screen.findByRole("button", { name: "Go to Deciat" }));
    expect(await screen.findByText("Couldn't follow it: not running inside the app")).toBeTruthy();
    unmount();

    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(SearchKit, { lookup: "stations.service", offers: [], run });
    await fireEvent.click(await screen.findByRole("button", { name: "Go to Deciat" }));
    expect(await screen.findByText("Following. Deciat is on your clipboard.")).toBeTruthy();
    expect((await fakeCall("route")).view.route.waypoints).toEqual([{ system: "Deciat", label: "Garay Terminal" }]);
    history.replaceState(null, "", "/");
  });
});
