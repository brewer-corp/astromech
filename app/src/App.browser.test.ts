import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import App from "./App.svelte";
import { answer, idle, previewAnswer, startPreview, watch } from "./test/transport";

const stationsSpy = watch("stations_nearby");
const hotspotsSpy = watch("mining_hotspots");
answer("status", () => null);

describe("Find or plot in the window", () => {
  afterEach(() => (location.hash = ""));

  it("opens a material from the Powerplay page, with a link to its Faction states finder", async () => {
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("My standing");

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const input = await screen.findByRole("combobox", { name: "Find or plot" });
    await fireEvent.input(input, { target: { value: "proto heat" } });
    await screen.findByText("Proto Heat Radiators");
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(location.hash).toBe("#/materials/protoheatradiators");
    expect(await screen.findByRole("heading", { name: "Proto Heat Radiators" })).toBeTruthy();
    expect(screen.getByRole("link", { name: "Find the nearest systems" }).getAttribute("href")).toBe("#/places/faction-states/proto");
  });

  it("switches material on the Materials page itself", async () => {
    location.hash = "#/materials/protoheatradiators";
    render(App);
    await screen.findByRole("heading", { name: "Proto Heat Radiators" });

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const input = await screen.findByRole("combobox", { name: "Find or plot" });
    await fireEvent.input(input, { target: { value: "datamined wake" } });
    await screen.findByText("Datamined Wake Exceptions", { selector: ".l" });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(await screen.findByRole("heading", { name: "Datamined Wake Exceptions" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Encoded", pressed: true })).toBeTruthy();
  });

  it("opens finders filled in from phrases typed in the header's one box", async () => {
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("My standing");
    expect(screen.getAllByRole("combobox", { name: "Find or plot" }), "one box, no separate palette").toHaveLength(1);
    for (const [phrase, to] of [
      ["raw trader", "#/places/material-traders/raw"],
      ["interstellar factors", "#/places/services/interstellar_factors"],
      ["outfitting fsd", "#/places/outfitting/Frame%20Shift%20Drive"],
      ["shipyard python", "#/places/shipyard/Python"],
      ["sell painite", "#/places/sell/painite"],
      ["raw sites brain trees", "#/places/raw-sites/brain_trees"],
    ]) {
      await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
      const input = screen.getByRole("combobox", { name: "Find or plot" });
      expect(document.activeElement, phrase).toBe(input);
      await fireEvent.input(input, { target: { value: phrase } });
      await within(await screen.findByRole("listbox", { name: "Matches" })).findAllByRole("option");
      await fireEvent.keyDown(input, { key: "Enter" });
      expect(location.hash, phrase).toBe(to);
    }
  });

  it("finds a ship from the Shipyard finder's main input", async () => {
    location.hash = "#/places/shipyard/Krait%20MkII";
    render(App);
    const ship = await screen.findByRole("combobox", { name: "Ship" });
    await fireEvent.input(ship, { target: { value: "anaconda" } });
    await screen.findByRole("option", { name: "Anaconda" });
    await fireEvent.keyDown(ship, { key: "Enter" });
    expect(location.hash).toBe("#/places/shipyard/Anaconda");
    await waitFor(() =>
      expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { kind: "ship", ship: "Anaconda" }, origin: null })),
    );
  });

  it("opens a module's nearby outfitting search", async () => {
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("My standing");

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const input = await screen.findByRole("combobox", { name: "Find or plot" });
    await fireEvent.input(input, { target: { value: "frame shift" } });
    await screen.findAllByText("Frame Shift Drive", { selector: ".l" });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(location.hash).toBe("#/places/outfitting/Frame%20Shift%20Drive");
    expect(await screen.findByText(/Titan City/)).toBeTruthy();
    expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { kind: "module", name: "Frame Shift Drive", class: null, rating: null, mount: null }, origin: null }));
  });

  it("opens a fitted module's search at its size, and a ship's shipyard search", async () => {
    location.hash = "#/places/outfitting";
    render(App);
    await screen.findByText(/Titan City/);

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    let input = await screen.findByRole("combobox", { name: "Find or plot" });
    await fireEvent.input(input, { target: { value: "2f pulse laser" } });
    await screen.findByText(/^Outfitting · fitted on/);
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(location.hash).toBe("#/places/outfitting/Pulse%20Laser?class=2&rating=F&mount=Gimballed");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { kind: "module", name: "Pulse Laser", class: 2, rating: "F", mount: "Gimballed" }, origin: null })));

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    input = await screen.findByRole("combobox", { name: "Find or plot" });
    await fireEvent.input(input, { target: { value: "anaconda" } });
    await screen.findByText("Shipyard", { selector: ".sub" });
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(location.hash).toBe("#/places/shipyard/Anaconda");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { kind: "ship", ship: "Anaconda" }, origin: null })));
  });

  it("keeps the Outfitting pick in the address for back and reload", async () => {
    location.hash = "#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(App);
    await screen.findByText(/Titan City/);

    await fireEvent.click(screen.getByRole("button", { name: "Class 6" }));
    await waitFor(() => expect(location.hash).toBe("#/places/outfitting/Frame%20Shift%20Drive?class=6&rating=A"));
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: expect.objectContaining({ class: 6, rating: "A" }), origin: null })));

    history.back();
    await waitFor(() => expect(location.hash).toBe("#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A"));
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: expect.objectContaining({ class: 5, rating: "A" }), origin: null })));

    cleanup(); // reload
    stationsSpy.mockClear();
    render(App);
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: expect.objectContaining({ name: "Frame Shift Drive", class: 5 }), origin: null })));
  });

  it("keeps a partial Outfitting pick when one part goes back to Any", async () => {
    location.hash = "#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(App);
    await screen.findByText(/Titan City/);

    await fireEvent.click(screen.getByRole("button", { name: "Rating Any" }));
    await waitFor(() => expect(location.hash).toBe("#/places/outfitting/Frame%20Shift%20Drive?class=5"));
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: expect.objectContaining({ class: 5, rating: null }), origin: null })));
    expect(screen.getByRole("button", { name: "Class 5" }).getAttribute("aria-pressed")).toBe("true");
  });

  it("keeps the selected material through back and reload", async () => {
    location.hash = "#/materials/protoheatradiators";
    render(App);
    await screen.findByRole("heading", { name: "Proto Heat Radiators" });

    await fireEvent.click(screen.getByRole("button", { name: /Proto Light Alloys/ }));
    expect(await screen.findByRole("heading", { name: "Proto Light Alloys" })).toBeTruthy();
    expect(location.hash).toBe("#/materials/protolightalloys");

    history.back();
    await waitFor(() => expect(location.hash).toBe("#/materials/protoheatradiators"));
    expect(await screen.findByRole("heading", { name: "Proto Heat Radiators" })).toBeTruthy();

    cleanup(); // reload
    render(App);
    expect(await screen.findByRole("heading", { name: "Proto Heat Radiators" })).toBeTruthy();
  });

  it("opens the page with nothing selected on a mangled material link", async () => {
    location.hash = "#/materials/%E0%A4%A";
    render(App);
    expect(await screen.findByRole("heading", { name: "Raw" })).toBeTruthy();
  });
});

describe("route strip in the window", () => {
  afterEach(() => {
    cleanup();
    location.hash = "";
    history.replaceState(null, "", "/");
  });

  it("is under the header on every page, Galaxy's included, and gone in onboarding", async () => {
    startPreview("/?core=ready&route=exact");
    for (const page of ["settings", "materials", "places", "places/services", "route"]) {
      location.hash = `#/${page}`;
      render(App);
      const strip = await screen.findByRole("region", { name: "Route" });
      expect(await within(strip).findByText("Arietis Sector LD-H a11-1"), page).toBeTruthy();
      expect(screen.queryByRole("complementary", { name: "Route" }), "no Galaxy rail").toBeNull();
      cleanup();
    }
    location.hash = "#/setup";
    render(App);
    await screen.findByRole("heading", { level: 1 });
    expect(screen.queryByRole("region", { name: "Route" })).toBeNull();
  });
});

describe("Places in the window", () => {
  beforeEach(async () => {
    (await import("./lib/places")).forgetAnswers();
    stationsSpy.mockClear();
    hotspotsSpy.mockClear();
  });
  afterEach(() => {
    cleanup();
    location.hash = "";
  });

  const pageTabs = () => within(document.querySelector("nav.sub") as HTMLElement).getAllByRole("button").map((b) => b.textContent);

  it("opens Galaxy on the Places home: tabs Places and Route, finder tiles by section, no request", async () => {
    location.hash = "#/route";
    render(App);
    await fireEvent.click(await screen.findByRole("button", { name: "Galaxy" }));
    expect(location.hash).toBe("#/places");
    expect(await screen.findByRole("heading", { name: "Ship" })).toBeTruthy();
    expect(pageTabs()).toEqual(["Places", "Route"]);
    expect(screen.getAllByRole("heading", { level: 2 }).map((h) => h.textContent)).toEqual(["Trade", "Ship", "Engineering", "Mining"]);
    const hrefs = (section: string) => within(screen.getByRole("region", { name: section })).getAllByRole("link").map((a) => a.getAttribute("href"));
    expect(hrefs("Trade")).toEqual(["#/places/sell", "#/places/buy", "#/places/sell-cargo"]);
    expect(hrefs("Ship")).toEqual(["#/places/services", "#/places/outfitting", "#/places/shipyard"]);
    expect(hrefs("Engineering")).toEqual(["#/places/material-traders", "#/places/raw-sites", "#/places/faction-states"]);
    expect(screen.getByRole("link", { name: /Mining hotspots/ }).getAttribute("href")).toBe("#/places/mining-hotspots");
    expect(document.querySelector(".home .v"), "no Lit figure without an answer").toBeNull();
    expect(stationsSpy).not.toHaveBeenCalled();
  });

  it("opens a finder from its tile with a Places crumb, its title and main input, and no third tab row", async () => {
    location.hash = "#/places";
    render(App);
    await fireEvent.click(await screen.findByRole("link", { name: /Material traders/ }));
    expect(await screen.findByRole("heading", { level: 1, name: "Material traders" })).toBeTruthy();
    await waitFor(() => expect(location.hash).toBe("#/places/material-traders/raw"));
    expect(screen.getByRole("link", { name: "Places" }).getAttribute("href")).toBe("#/places");
    expect(screen.getByRole("button", { name: "Raw trader" }).getAttribute("aria-pressed")).toBe("true");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { kind: "service", service: "raw_trader" }, origin: null })));
    expect(document.querySelectorAll("nav.sub")).toHaveLength(1);
    expect(pageTabs()).toEqual(["Places", "Route"]);

    await fireEvent.click(screen.getByRole("button", { name: "Encoded trader" }));
    expect(location.hash).toBe("#/places/material-traders/encoded");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { kind: "service", service: "encoded_trader" }, origin: null })));
  });

  it("shows a finder's last answer this session on its tile, and opens that finder again from it", async () => {
    location.hash = "#/places/services/black_market";
    render(App);
    expect(await screen.findByRole("heading", { level: 1, name: "Services" })).toBeTruthy();
    expect((screen.getByRole("combobox", { name: "Service" }) as HTMLSelectElement).value).toBe("black_market");
    await screen.findByText(/^Nearest to Sol/);

    await fireEvent.click(screen.getByRole("link", { name: "Places" }));
    const tile = await screen.findByRole("link", { name: /^Services/ });
    expect(tile.getAttribute("href")).toBe("#/places/services/black_market");
    expect(within(tile).getByText("Black market")).toBeTruthy();
    expect(within(tile).getByText(/jumps?$/)).toBeTruthy();
    expect(tile.querySelector(".type-value-l")?.textContent).toMatch(/^~?\d+$/);
    expect(within(screen.getByRole("link", { name: /^Shipyard/ })).queryByText(/jumps?$/)).toBeNull();
  });

  it("keeps old links working: Services, Outfitting and Shipyard open their finders", async () => {
    const cases: [string, string, Parameters<typeof stationsSpy>[0]][] = [
      ["#/services", "#/places/services/interstellar_factors", { kind: "service", service: "interstellar_factors" }],
      ["#/services/black_market", "#/places/services/black_market", { kind: "service", service: "black_market" }],
      ["#/outfitting/Frame%20Shift%20Drive/5A", "#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A", { kind: "module", name: "Frame Shift Drive", class: 5, rating: "A", mount: null }],
      ["#/outfitting/Pulse%20Laser?mount=Fixed", "#/places/outfitting/Pulse%20Laser?mount=Fixed", { kind: "module", name: "Pulse Laser", class: null, rating: null, mount: "Fixed" }],
      ["#/shipyard/Anaconda", "#/places/shipyard/Anaconda", { kind: "ship", ship: "Anaconda" }],
    ];
    for (const [old, now, want] of cases) {
      location.hash = old;
      render(App);
      await waitFor(() => expect(location.hash, old).toBe(now));
      await waitFor(() => expect(stationsSpy, old).toHaveBeenLastCalledWith(expect.objectContaining({ want, origin: null })));
      cleanup();
    }
  });

  it("links each page a lookup moved from into its finder with the input filled in", async () => {
    // [page, link name, finder address, finder title, main input name, its value]
    const cases: [string, string, string, string, string, string][] = [
      ["#/trade", "Sell", "#/places/sell/painite", "Sell", "Commodity", "Painite"],
      ["#/trade", "Buy", "#/places/buy/painite", "Buy", "Commodity", "Painite"],
      ["#/trade", "Sell my cargo", "#/places/sell-cargo", "Sell my cargo", "", ""],
      ["#/materials", "Crystalline shards", "#/places/raw-sites/crystalline_shards", "Raw sites", "Site", "crystalline_shards"],
      ["#/materials", "Known raw farms", "#/places/raw-sites/raw_farms", "Raw sites", "Site", "raw_farms"],
      ["#/materials/protoheatradiators", "Find the nearest systems", "#/places/faction-states/proto", "Faction states", "State", "proto"],
      ["#/materials/imperialshielding", "Find the nearest systems", "#/places/faction-states/empire_shielding", "Faction states", "State", "empire_shielding"],
      ["#/exploration", "Nearest Universal Cartographics", "#/places/services/universal_cartographics", "Services", "Service", "universal_cartographics"],
      ["#/exploration", "Nearest Vista Genomics", "#/places/services/vista_genomics", "Services", "Service", "vista_genomics"],
    ];
    for (const [page, name, to, title, input, value] of cases) {
      const at = `${page} → ${name}`;
      location.hash = page;
      render(App);
      const link = await screen.findByRole("link", { name });
      expect(link.getAttribute("href"), at).toBe(to);
      await fireEvent.click(link);
      expect(await screen.findByRole("heading", { level: 1, name: title }), at).toBeTruthy();
      await idle();
      expect(location.hash, at).toBe(to);
      if (input) expect((screen.getByRole("combobox", { name: input }) as HTMLInputElement).value, at).toBe(value);
      cleanup();
    }
  });

  it("keeps the source pages' old addresses: Trade, Materials and Exploration still open", async () => {
    for (const [page, sees] of [
      ["#/trade", /^Where to sell or buy is in Places/],
      ["#/materials", /^Raw sites near you/],
      ["#/materials/protoheatradiators", /^Find the nearest systems$/],
      ["#/exploration", /^Where to sell$/],
    ] as const) {
      location.hash = page;
      render(App);
      expect(await screen.findByText(sees), page).toBeTruthy();
      expect(location.hash, page).toBe(page);
      cleanup();
    }
  });

  it("rewrites an old link opened from outside in place, so Back doesn't return to it", async () => {
    location.hash = "#/route";
    location.hash = "#/shipyard/Anaconda";
    const entries = history.length;
    render(App);
    await screen.findByRole("heading", { level: 1, name: "Shipyard" });
    expect(location.hash).toBe("#/places/shipyard/Anaconda");
    expect(history.length).toBe(entries);
  });

  it("rewrites an old link followed inside the window", async () => {
    location.hash = "#/places/services/refuel";
    render(App);
    await screen.findByRole("heading", { level: 1, name: "Services" });
    location.hash = "#/shipyard/Anaconda";
    await waitFor(() => expect(location.hash).toBe("#/places/shipyard/Anaconda"));
    expect(await screen.findByRole("heading", { level: 1, name: "Shipyard" })).toBeTruthy();
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { kind: "ship", ship: "Anaconda" }, origin: null })));
  });

  it("picks a module in the Outfitting finder's main input", async () => {
    location.hash = "#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(App);
    const box = await screen.findByRole("combobox", { name: "Module" });
    await fireEvent.input(box, { target: { value: "pulse laser" } });
    await screen.findByText("Pulse Laser");
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(location.hash).toBe("#/places/outfitting/Pulse%20Laser");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { kind: "module", name: "Pulse Laser", class: null, rating: null, mount: null }, origin: null })));
  });

  it("finds Mining hotspots from its tile: rings most hotspots first, with Add stop and Set destination", async () => {
    location.hash = "#/places";
    render(App);
    await fireEvent.click(await screen.findByRole("link", { name: /Mining hotspots/ }));
    expect(await screen.findByRole("heading", { level: 1, name: "Mining hotspots" })).toBeTruthy();
    await waitFor(() => expect(location.hash).toBe("#/places/mining-hotspots/painite"));
    expect((screen.getByRole("combobox", { name: "Mineral" }) as HTMLInputElement).value).toBe("Painite");
    expect(screen.getByText("Around")).toBeTruthy();
    await waitFor(() => expect(hotspotsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { ore: "painite", ring_type: null, reserve: null }, origin: null })));
    expect(await screen.findByText("Within 100 ly of Sol · top 10 of 1,101 bodies with Painite")).toBeTruthy();
    const heads = screen.getAllByRole("columnheader").map((h) => h.textContent);
    expect(heads).toEqual(["Hotspots", "Ring", "Also", "Type", "Reserve", "Distance", "Jumps", "Arrival", ""]);
    const rows = screen.getAllByRole("row").slice(1);
    expect(rows).toHaveLength(10);
    expect(within(rows[0]!).getByText("3 A Ring")).toBeTruthy();
    expect(within(rows[0]!).getByText("LHS 3602")).toBeTruthy();
    expect(rows[0]!.querySelector(".main .v")?.textContent).toBe("10");
    expect(within(rows[0]!).getByRole("button", { name: "Add LHS 3602 as a stop" })).toBeTruthy();
    expect(within(rows[0]!).getByRole("button", { name: "Set LHS 3602 as the destination" })).toBeTruthy();

    // the tile keeps the answer
    await fireEvent.click(screen.getByRole("link", { name: "Places" }));
    const tile = await screen.findByRole("link", { name: /^Mining hotspots/ });
    expect(tile.getAttribute("href")).toBe("#/places/mining-hotspots/painite");
    expect(within(tile).getByText("Painite")).toBeTruthy();
    expect(within(tile).getByText("hotspots")).toBeTruthy();
    expect(tile.querySelector(".type-value-l")?.textContent).toBe("10");
  });

  it("keeps the Mining hotspots mineral, ring type and reserve in the address and searches again", async () => {
    location.hash = "#/places/mining-hotspots/painite";
    render(App);
    await screen.findByText(/^Within 100 ly of Sol/);
    // Ring type and reserve are filters: they rest in the summary until CHANGE opens them.
    expect(screen.queryByRole("button", { name: "Metallic" })).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
    await fireEvent.click(screen.getByRole("button", { name: "Metallic" }));
    expect(location.hash).toBe("#/places/mining-hotspots/painite?ring=metallic");
    await waitFor(() => expect(hotspotsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { ore: "painite", ring_type: "Metallic", reserve: null }, origin: null })));
    await fireEvent.click(screen.getByRole("button", { name: "Pristine" }));
    expect(location.hash).toBe("#/places/mining-hotspots/painite?ring=metallic&reserve=pristine");
    await waitFor(() => expect(hotspotsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { ore: "painite", ring_type: "Metallic", reserve: "pristine" }, origin: null })));
    expect(await screen.findByText("Within 100 ly of Sol · metallic rings · pristine reserves")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Pristine" }).getAttribute("aria-pressed")).toBe("true"); // still open

    const box = screen.getByRole("combobox", { name: "Mineral" });
    await fireEvent.input(box, { target: { value: "low temp" } });
    await screen.findByText("Low Temperature Diamonds");
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(location.hash).toBe("#/places/mining-hotspots/lowtemperaturediamond?ring=metallic&reserve=pristine");
    await waitFor(() =>
      expect(hotspotsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { ore: "lowtemperaturediamond", ring_type: "Metallic", reserve: "pristine" }, origin: null })),
    );
    await fireEvent.click(screen.getByRole("button", { name: "Any ring type" }));
    await fireEvent.click(screen.getByRole("button", { name: "Any reserve" }));
    expect(location.hash).toBe("#/places/mining-hotspots/lowtemperaturediamond");
  });

  it("names a new radius while its search runs, not the reach of the answer before", async () => {
    // A fault the recording lacks: Spansh's requests ran out at 32 ly; the next search is held.
    let calls = 0;
    answer("mining_hotspots", async (args: unknown) => {
      if (++calls > 1) return new Promise(() => {});
      return { ...(await previewAnswer("mining_hotspots", args)), reach_ly: 32 };
    });
    location.hash = "#/places/mining-hotspots/painite";
    render(App);
    expect(await screen.findByText(/^Within 32 ly of Sol · /)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
    await fireEvent.change(screen.getByRole("combobox", { name: "Within" }), { target: { value: "200" } });
    await fireEvent.click(screen.getByRole("button", { name: "Done" }));
    expect(await screen.findByText("Asking Spansh…")).toBeTruthy();
    expect(screen.getByText("Within 200 ly of Sol")).toBeTruthy();
  });

  it("leaves every finder opened from its tile by the Places crumb without an error", async () => {
    const { FINDERS } = await import("./lib/places");
    const errors: string[] = [];
    let at = "";
    const onError = (e: ErrorEvent) => {
      // Chromium reports a layout that resized again within one frame; it is not the page's error.
      if (e.message.startsWith("ResizeObserver loop")) return;
      errors.push(`${at}: ${e.message}`);
      e.preventDefault();
    };
    window.addEventListener("error", onError);
    try {
      for (const f of FINDERS) {
        at = f.title;
        location.hash = "#/places";
        render(App);
        await fireEvent.click(await screen.findByRole("link", { name: new RegExp(`^${f.title} ${f.what}`) }));
        expect(await screen.findByRole("heading", { level: 1, name: f.title })).toBeTruthy();
        await fireEvent.click(screen.getByRole("link", { name: "Places" }));
        expect(await screen.findByRole("heading", { level: 2, name: "Ship" }), f.title).toBeTruthy();
        await idle();
        cleanup();
      }
    } finally {
      window.removeEventListener("error", onError);
    }
    expect(errors).toEqual([]);
  });

  it("leaves a finder for another page without rewriting the address", async () => {
    location.hash = "#/places/shipyard/Anaconda";
    render(App);
    await screen.findByRole("heading", { level: 1, name: "Shipyard" });
    await fireEvent.click(screen.getByRole("button", { name: "Route" }));
    await waitFor(() => expect(location.hash).toBe("#/route"));
    await idle();
    expect(location.hash).toBe("#/route");
  });
});
