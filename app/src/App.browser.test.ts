// The design tokens, for the frame's real widths (the route sidebar, the dock width).
import "./design/tokens.css";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { page } from "vitest/browser";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import App from "./App.svelte";
import { clearRoute, followRoute } from "./lib/route";
import { resetScale, setTextSize, STEPS } from "./lib/scale.svelte";
import { DOCK } from "./lib/ui/layout/layout";
import { answer, idle, previewAnswer, startPreview, watch } from "./test/transport";

const stationsSpy = watch("stations_nearby");
const hotspotsSpy = watch("mining_hotspots");
answer("status", () => null);

describe("Find or plot in the window", () => {
  afterEach(() => (location.hash = ""));

  it("opens a material from the Powerplay page, with a link to its Faction states finder", async () => {
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("Next rank");

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
    await screen.findByText("Next rank");
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
    await screen.findByText("Next rank");

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

  it("is under the header on every page below the dock width, and gone in onboarding", async () => {
    startPreview("/?core=ready&route=exact");
    for (const page of ["settings", "materials", "places", "places/services"]) {
      location.hash = `#/${page}`;
      render(App);
      const strip = await screen.findByRole("region", { name: "Route" });
      expect(await within(strip).findByText("Arietis Sector LD-H a11-1"), page).toBeTruthy();
      expect(screen.queryByRole("complementary", { name: "Route" }), "no sidebar until the strip opens it").toBeNull();
      cleanup();
    }
    location.hash = "#/setup";
    render(App);
    await screen.findByRole("heading", { level: 1 });
    expect(screen.queryByRole("region", { name: "Route" })).toBeNull();
  });
});

describe("route sidebar in the frame", () => {
  // DESIGN.md Route sidebar. The test window is 1200 x 800, below the dock width; the docked
  // cases widen it. The preview commander is in Sol.
  const WIDE = 1920;
  beforeEach(() => localStorage.removeItem("astromech.routeCollapsed"));
  /** The preview under `query`, on the Materials page. */
  const preview = (query: string) => {
    startPreview(query);
    location.hash = "#/materials";
  };
  afterEach(async () => {
    cleanup();
    resetScale();
    await page.viewport(1200, 800);
    location.hash = "";
    history.replaceState(null, "", "/");
  });

  const sidebar = () => screen.queryByRole("complementary", { name: "Route" });
  const strip = () => screen.queryByRole("region", { name: "Route" });
  const rail = () => screen.queryByRole("navigation", { name: "No route" });
  const scrim = () => document.querySelector(".scrim");

  it("docks beside every page at the dock width, folds to the strip, and stays folded", async () => {
    await page.viewport(WIDE, 1080);
    preview("/?core=ready&route=exact");
    render(App);
    const side = within(await screen.findByRole("complementary", { name: "Route" }));
    expect(side.getByText("Route", { selector: ".heading" })).toBeTruthy();
    expect(await side.findByText("Next stop · 1 jump away")).toBeTruthy();
    expect(strip(), "the sidebar holds the route: no strip").toBeNull();
    expect(await screen.findByText(/^Raw sites near you/), "the page beside it").toBeTruthy();

    await fireEvent.click(side.getByRole("button", { name: "Collapse the route to a strip" }));
    expect(sidebar()).toBeNull();
    expect(await within(strip()!).findByText("Arietis Sector LD-H a11-1")).toBeTruthy();

    // Folded is kept: the next window opens folded, and the strip's chevron unfolds it.
    cleanup();
    render(App);
    const s = await screen.findByRole("region", { name: "Route" });
    await within(s).findByText("Arietis Sector LD-H a11-1");
    expect(sidebar()).toBeNull();
    await fireEvent.click(within(s).getByRole("button", { name: "Show the route" }));
    expect(await screen.findByRole("complementary", { name: "Route" })).toBeTruthy();
    expect(strip()).toBeNull();
  });

  it("opens over the dimmed page below the dock width, and closes on Escape, the chevron and a click outside", async () => {
    preview("/?core=ready&route=exact");
    render(App);
    const s = await screen.findByRole("region", { name: "Route" });
    await within(s).findByText("Arietis Sector LD-H a11-1");
    const chevron = () => within(s).getByRole("button", { name: /^(Show|Close) the route$/ });
    const open = async () => {
      await fireEvent.click(chevron());
      expect(await screen.findByRole("complementary", { name: "Route" })).toBeTruthy();
      expect(scrim()).toBeTruthy();
      expect(chevron().getAttribute("aria-expanded")).toBe("true");
    };

    await open();
    // Escape in the destination's own box is the box's: it has nothing to undo, so the sidebar closes.
    const dest = screen.getByRole("combobox", { name: "Destination" });
    // A dropdown open inside it takes Escape first.
    await fireEvent.click(screen.getByRole("button", { name: /^Plotter/ }));
    await fireEvent.keyDown(screen.getByRole("listbox", { name: "Plotter" }), { key: "Escape" });
    expect(sidebar(), "Escape closed the dropdown only").toBeTruthy();
    await fireEvent.keyDown(dest, { key: "Escape" });
    expect(sidebar()).toBeNull();
    expect(scrim()).toBeNull();

    await open();
    await fireEvent.click(chevron());
    expect(sidebar()).toBeNull();

    await open();
    await fireEvent.pointerDown(scrim()!);
    expect(sidebar()).toBeNull();
    expect(location.hash, "the page stayed").toBe("#/materials");
  });

  it("shows the no-route rail docked, and the strip's No route below the dock width, and imports a first route from each", async () => {
    const csv = '"System Name","Jumps"\n"Sol","0"\n"Shinrarta Dezhra","3"\n"Colonia","40"\n';
    const importFrom = async (button: HTMLElement) => {
      await fireEvent.click(button);
      const dialog = screen.getByRole("dialog", { name: "Import a route" });
      await fireEvent.input(within(dialog).getByPlaceholderText(/Paste a Spansh CSV/), { target: { value: csv } });
      await fireEvent.click(within(dialog).getByRole("button", { name: "Follow this route" }));
    };

    await page.viewport(WIDE, 1080);
    preview("/?core=ready");
    render(App);
    const r = await screen.findByRole("navigation", { name: "No route" });
    expect(strip(), "the rail is all there is").toBeNull();
    expect(r.getBoundingClientRect().width).toBe(48);
    await fireEvent.click(within(r).getByRole("button", { name: "Find or plot" }));
    expect(document.activeElement).toBe(screen.getByRole("combobox", { name: "Find or plot" }));
    await importFrom(within(r).getByRole("button", { name: "Import a route" }));
    const side = within(await screen.findByRole("complementary", { name: "Route" }));
    expect(await side.findByText((_, el) => !!el?.matches(".name") && el.textContent === "Shinrarta Dezhra")).toBeTruthy();
    expect(rail()).toBeNull();

    cleanup();
    await page.viewport(1200, 800);
    preview("/?core=ready");
    render(App);
    const s = within(await screen.findByRole("region", { name: "Route" }));
    await s.findByText("No route");
    expect(rail()).toBeNull();
    await importFrom(s.getByRole("button", { name: "Import a route" }));
    expect(await s.findByText("Shinrarta Dezhra")).toBeTruthy();
  });

  it("opens a cold #/route over the page when the saved Text size, read after the window draws, puts it below the dock width", async () => {
    // 1920 px docks at 100 percent, the size the window draws at before the core answers, and is
    // below the dock width at a saved 130 percent (1,477 px at 100).
    // The first read fails (the core not ready), and the window tries again on the next status
    // poll, whose reply the test holds.
    let reply: (r: { ok: true; text_size: number }) => void = () => {};
    let reads = 0;
    answer("window_settings", () => (++reads === 1 ? { ok: false, error: "core not ready" } : new Promise((r) => (reply = r))));
    answer("status", async () => (await previewAnswer("status")));
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] }); // the 2 s status poll
    await page.viewport(WIDE, 1080);
    startPreview("/?core=ready&route=exact");
    location.hash = "#/route";
    render(App);
    // The route is drawn (the strip, or without the fix the docked sidebar) before the reply.
    await waitFor(() => expect(document.querySelector('[aria-label="Route"]')).toBeTruthy());
    await vi.advanceTimersByTimeAsync(2000);
    await waitFor(() => expect(reads).toBe(2));
    reply({ ok: true, text_size: 130 });
    await waitFor(() => expect(scrim(), "the sidebar over the page").toBeTruthy());
    expect(sidebar()).toBeTruthy();
    expect(document.documentElement.style.fontSize).toBe("130%");
    expect(location.hash).toBe("#/now");
  });

  it("shows the empty sidebar when it is asked for with no route at the dock width, and folds back to the rail", async () => {
    await page.viewport(WIDE, 1080);
    startPreview("/?core=ready");
    location.hash = "#/route";
    render(App);
    const side = within(await screen.findByRole("complementary", { name: "Route" }));
    expect(await side.findByText(/^No route\./)).toBeTruthy();
    expect(side.getByRole("combobox", { name: "Destination" }), "its plotter").toBeTruthy();
    expect(rail()).toBeNull();
    expect(location.hash).toBe("#/now");

    await fireEvent.click(side.getByRole("button", { name: "Collapse the route to a strip" }));
    expect(await screen.findByRole("navigation", { name: "No route" })).toBeTruthy();
    expect(sidebar()).toBeNull();
    expect(localStorage.getItem("astromech.routeCollapsed"), "folding an empty sidebar keeps no choice").not.toBe("1");

    // The palette's Route asks for it again (on a page that has finished loading, which would
    // otherwise take the focus).
    location.hash = "#/materials";
    await screen.findByText(/^Raw sites near you/);
    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const box = screen.getByRole("combobox", { name: "Find or plot" });
    await fireEvent.input(box, { target: { value: "route" } });
    await screen.findByText("The route sidebar");
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(await screen.findByRole("complementary", { name: "Route" })).toBeTruthy();
    expect(rail()).toBeNull();
  });

  it("opens the sidebar's ⋯ menu inside the window, docked and over the page, for a neutron route", async () => {
    // A neutron route's two plot cells push ? and ⋯ to a second line at the sidebar's left edge.
    for (const [w, h] of [[WIDE, 1080], [1200, 800]] as const) {
      await page.viewport(w, h);
      preview("/?core=ready&route=timeline");
      render(App);
      if (w < WIDE) await fireEvent.click(await within(await screen.findByRole("region", { name: "Route" })).findByRole("button", { name: "Show the route" }));
      const side = within(await screen.findByRole("complementary", { name: "Route" }));
      await side.findByText(/^Next stop/);
      await fireEvent.click(side.getByRole("button", { name: "More route actions" }));
      const menu = screen.getByRole("menu", { name: "More route actions" }).getBoundingClientRect();
      expect(menu.left, `${w} px`).toBeGreaterThanOrEqual(0);
      expect(menu.right, `${w} px`).toBeLessThanOrEqual(innerWidth);
      cleanup();
    }
  });

  it("docks when a route appears and gives way to the rail when it is cleared", async () => {
    await page.viewport(WIDE, 1080);
    preview("/?core=ready");
    render(App);
    await screen.findByRole("navigation", { name: "No route" });
    // A route followed from elsewhere (a finder's Set destination): the next poll docks it.
    await followRoute("Trip", "test", [{ system: "Deciat" }, { system: "Kuk" }]);
    expect(await screen.findByRole("complementary", { name: "Route" })).toBeTruthy();
    expect(rail()).toBeNull();
    await clearRoute();
    expect(await screen.findByRole("navigation", { name: "No route" })).toBeTruthy();
    expect(sidebar()).toBeNull();

    // Asked for over a route (#/route in the window), it still gives way to the rail once cleared.
    await followRoute("Trip", "test", [{ system: "Deciat" }, { system: "Kuk" }]);
    await screen.findByRole("complementary", { name: "Route" });
    location.hash = "#/route";
    await waitFor(() => expect(location.hash, "handled: the page's address is back").toBe("#/materials"));
    await clearRoute();
    expect(await screen.findByRole("navigation", { name: "No route" })).toBeTruthy();
    expect(sidebar()).toBeNull();
  });

  it("opens the sidebar for #/route, cold or in the window, and keeps the page", async () => {
    // Cold, from a bookmark: the default page, the sidebar over it below the dock width.
    startPreview("/?core=ready&route=exact");
    location.hash = "#/route";
    render(App);
    expect(await screen.findByRole("complementary", { name: "Route" })).toBeTruthy();
    expect(scrim()).toBeTruthy();
    expect(location.hash).toBe("#/now");
    cleanup();

    // Docked and folded: an old link followed inside the window unfolds it, and the page stays.
    localStorage.setItem("astromech.routeCollapsed", "1");
    await page.viewport(WIDE, 1080);
    location.hash = "#/trade";
    render(App);
    await within(await screen.findByRole("region", { name: "Route" })).findByText("Arietis Sector LD-H a11-1");
    location.hash = "#/route";
    expect(await screen.findByRole("complementary", { name: "Route" })).toBeTruthy();
    expect(location.hash).toBe("#/trade");

  });

  it("opens the sidebar for the palette's Route entry, and keeps the page", async () => {
    localStorage.setItem("astromech.routeCollapsed", "1");
    await page.viewport(WIDE, 1080);
    preview("/?core=ready&route=exact");
    render(App);
    await within(await screen.findByRole("region", { name: "Route" })).findByText("Arietis Sector LD-H a11-1");
    await screen.findByText(/^Raw sites near you/);
    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const box = screen.getByRole("combobox", { name: "Find or plot" });
    await fireEvent.input(box, { target: { value: "route" } });
    await screen.findByText("The route sidebar");
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(await screen.findByRole("complementary", { name: "Route" })).toBeTruthy();
    expect(location.hash).toBe("#/materials");
  });

  /** Every Places finder whose results a side panel describes, at an address with results in the preview. */
  const FINDERS = [
    "sell/painite", "buy/painite", "sell-cargo", "faction-states/proto", "material-traders/raw",
    "mining-hotspots/painite", "outfitting/Frame%20Shift%20Drive", "raw-sites/brain_trees",
    "services/interstellar_factors", "shipyard/Anaconda",
  ];
  /** The widest of them with a result selected, which sets the dock width. */
  const WIDEST = "mining-hotspots/painite";
  /** `finder` at Text size `step` in a window `width` px wide, its first result selected: its side panel. */
  const selected = async (finder: string, step: number, width: number) => {
    startPreview("/?core=ready&route=exact");
    await setTextSize(step);
    await page.viewport(width, 1000);
    location.hash = `#/places/${finder}`;
    render(App);
    const table = await screen.findByRole("table", { name: "Results" });
    await fireEvent.click(within(table).getAllByRole("button", { pressed: false })[0]!);
    return { table, panel: await screen.findByRole("complementary", { name: "Details" }) };
  };

  it("docks from the dock width, where every finder keeps its full rows and its panel beside them, at every Text size", async () => {
    // The dock width (lib/ui/layout/layout.ts) is the narrowest window at which the widest finder
    // row, with a result selected, keeps its side panel beside the rows next to the sidebar. A
    // finder that grows wider than it fails here.
    for (const step of STEPS) {
      const at = Math.ceil((DOCK * step) / 100);
      for (const finder of FINDERS) {
        const { table, panel } = await selected(finder, step, at);
        expect(screen.getByRole("complementary", { name: "Route" }), `${step} %: docked at ${at} px`).toBeTruthy();
        await waitFor(() => expect(panel.classList.contains("stacked"), `${finder} at ${step} %: beside the rows at ${at} px`).toBe(false));
        expect(table.classList.contains("compact"), `${finder} at ${step} %: full rows at ${at} px`).toBe(false);
        cleanup();
      }
    }
  });

  it("is no wider than the widest finder needs: its panel goes below the rows a few px narrower, at every Text size", async () => {
    for (const step of STEPS) {
      const at = Math.ceil((DOCK * step) / 100);
      const { panel } = await selected(WIDEST, step, at);
      await waitFor(() => expect(panel.classList.contains("stacked")).toBe(false));
      const split = panel.parentElement!;
      const docked = split.getBoundingClientRect().width;
      // Folded, the page column is the window less its gutters, which narrow with it (3vw). A
      // window whose column is the docked one's less 6 px at 100 percent (the text's rounding moves
      // the true edge by up to 5 px across the steps) puts the panel below the rows.
      const target = docked - Math.ceil((6 * step) / 100);
      await fireEvent.click(screen.getByRole("button", { name: "Collapse the route to a strip" }));
      let width = at;
      for (let i = 0; i < 3; i++) {
        await page.viewport(width, 1000);
        await waitFor(() => expect(split.getBoundingClientRect().width).toBeLessThan(width));
        width -= Math.ceil(split.getBoundingClientRect().width - target);
      }
      await page.viewport(width, 1000);
      await waitFor(() => expect(panel.classList.contains("stacked"), `${step} %: below the rows at ${width} px`).toBe(true));
      expect(split.getBoundingClientRect().width, `${step} %: the column no wider than the docked one less the slack`).toBeLessThanOrEqual(target + 1);
      localStorage.removeItem("astromech.routeCollapsed");
      cleanup();
    }
  });

  it("goes back to full rows when the window widens after they went compact, at 160 percent with the sidebar folded", async () => {
    // The rows' width once compact must not decide whether they stay compact (astromech-rhn3).
    localStorage.setItem("astromech.routeCollapsed", "1");
    const { table, panel } = await selected("sell/painite", 160, 1100);
    await waitFor(() => expect(table.classList.contains("compact"), "compact in a 1,100 px window").toBe(true));
    await page.viewport(2130, 1000);
    await waitFor(() => expect(panel.classList.contains("stacked"), "the panel beside the rows at 2,130 px").toBe(false));
    await waitFor(() => expect(table.classList.contains("compact"), "full rows again at 2,130 px").toBe(false));
    localStorage.removeItem("astromech.routeCollapsed");
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

  it("opens PLACES on the Places home: no page tabs, finder tiles by section, no request", async () => {
    location.hash = "#/settings";
    render(App);
    await fireEvent.click(await screen.findByRole("button", { name: "Places" }));
    expect(location.hash).toBe("#/places");
    expect(await screen.findByRole("heading", { name: "Ship" })).toBeTruthy();
    expect(document.querySelector("nav.sub"), "one page, no page tab row").toBeNull();
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
    expect(document.querySelector("nav.sub"), "no page tab row, and no third").toBeNull();

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
    location.hash = "#/settings";
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
    await waitFor(() => expect(hotspotsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ want: { ore: "painite", ring_type: null, reserve: null }, origin: null })));
    expect(await screen.findByText("Within 100 ly of Sol · top 10 of 1,101 bodies with Painite")).toBeTruthy();
    const heads = screen.getAllByRole("columnheader").map((h) => h.textContent);
    expect(heads).toEqual(["Hotspots", "Ring · arrival", "Type · reserve", "Trip", ""]);
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
    await fireEvent.click(screen.getByRole("button", { name: "Trade" }));
    await waitFor(() => expect(location.hash).toBe("#/trade"));
    await idle();
    expect(location.hash).toBe("#/trade");
  });
});

describe("header in a narrow window", () => {
  // DESIGN.md Frame: the header fits the window at every Text size (astromech-z4vm). The window's
  // own minimum (app/src-tauri/src/host.rs, 800 x 500) is smaller than the design's 1100 x 700,
  // so both are checked.
  afterEach(async () => {
    cleanup();
    resetScale();
    await page.viewport(1200, 800);
    location.hash = "";
    history.replaceState(null, "", "/");
  });

  /** The header at each Text size step in a `width` x `height` window, with Finish setup if `setup`. */
  async function eachStep(width: number, height: number, setup: boolean, check: (header: HTMLElement, step: number) => void) {
    if (setup) answer("status", async () => ({ ...(await previewAnswer("status")), setup_needed: ["push-to-talk not set"] }));
    startPreview("/?core=ready");
    location.hash = "#/materials";
    await page.viewport(width, height);
    render(App);
    await screen.findByRole("button", { name: "Raw", pressed: true });
    if (setup) await screen.findByRole("button", { name: "Finish setup" });
    const header = document.querySelector("header")!;
    for (const step of STEPS) {
      await setTextSize(step);
      await idle();
      await waitFor(() => expect(header.classList.contains("stacked"), `${step} %: stacked`).toBe(true));
      const doc = document.documentElement;
      expect(doc.scrollWidth, `${step} %: the document scrolls sideways`).toBeLessThanOrEqual(doc.clientWidth);
      const box = header.getBoundingClientRect();
      for (const tab of header.querySelectorAll("nav button")) {
        const r = tab.getBoundingClientRect();
        const name = `${tab.textContent} at ${step} %`;
        expect(r.width, `${name}: drawn`).toBeGreaterThan(0);
        expect(r.left, `${name}: inside the header's left edge`).toBeGreaterThanOrEqual(box.left);
        expect(r.right, `${name}: inside the header's right edge`).toBeLessThanOrEqual(box.right);
        expect(r.bottom, `${name}: inside the header`).toBeLessThanOrEqual(box.bottom);
        expect(r.bottom, `${name}: inside the window`).toBeLessThanOrEqual(innerHeight);
        // Not clipped by its own box: its label fits.
        expect(tab.scrollWidth, `${name}: its label fits`).toBeLessThanOrEqual(tab.clientWidth);
      }
      for (const part of header.children) {
        const name = `${part.className.split(" ")[0] || part.tagName} at ${step} %`;
        expect(part.getBoundingClientRect().right, `${name}: inside the header's right edge`).toBeLessThanOrEqual(box.right);
      }
      check(header, step);
    }
  }

  it.each([
    [800, 500],
    [1100, 700],
  ])("keeps every top tab in a %i x %i window, with no sideways scroll and one name row, at every Text size", async (width, height) => {
    await eachStep(width, height, false, (header, step) => {
      // The name, Find or plot and Help stay on one row above the tabs.
      const first = header.querySelector(".app")!.getBoundingClientRect();
      for (const part of header.children) {
        if (part.tagName === "NAV") continue;
        const name = `${part.className.split(" ")[0] || part.tagName} at ${step} %`;
        expect(part.getBoundingClientRect().bottom, `${name}: on the name's row`).toBeLessThanOrEqual(first.bottom);
      }
    });
  });

  it("keeps every top tab in an 800 x 500 window with Finish setup, the widest header, at every Text size", async () => {
    await eachStep(800, 500, true, () => {});
  });
});
