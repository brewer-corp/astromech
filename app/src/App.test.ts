import { afterEach, beforeAll, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import App from "./App.svelte";

const stationsSpy = vi.fn();
vi.mock("./lib/stations", async (orig) => {
  const real = await orig<typeof import("./lib/stations")>();
  return { ...real, stations: (...a: Parameters<typeof real.stations>) => (stationsSpy(...a), real.stations(...a)) };
});

vi.mock("./lib/core", async (orig) => ({
  ...(await orig<typeof import("./lib/core")>()),
  coreStatus: () => Promise.resolve(null),
}));

describe("command palette in the window", () => {
  beforeAll(() => (Element.prototype.scrollIntoView = vi.fn())); // jsdom has no layout
  afterEach(() => (location.hash = ""));

  it("opens a material's nearby search from the Powerplay page", async () => {
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("My standing");

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const input = await screen.findByRole("combobox", { name: "Go to" });
    await fireEvent.input(input, { target: { value: "proto heat" } });
    await screen.findByText("Proto Heat Radiators");
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(location.hash).toBe("#/materials/protoheatradiators");
    expect(await screen.findByRole("heading", { name: "Proto Heat Radiators" })).toBeTruthy();
    expect(await screen.findByText(/^Nearest to Sol/)).toBeTruthy();
    expect(await screen.findByText("Ross 154")).toBeTruthy();
  });

  it("switches material on the Materials page itself", async () => {
    location.hash = "#/materials/protoheatradiators";
    render(App);
    await screen.findByRole("heading", { name: "Proto Heat Radiators" });

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const input = await screen.findByRole("combobox", { name: "Go to" });
    await fireEvent.input(input, { target: { value: "datamined wake" } });
    await screen.findByText("Datamined Wake Exceptions", { selector: ".l" });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(await screen.findByRole("heading", { name: "Datamined Wake Exceptions" })).toBeTruthy();
    expect(screen.getByRole("button", { name: /Encoded/i }).className).toContain("on");
  });

  it("finds a ship from the Shipyard finder's main input", async () => {
    location.hash = "#/places/shipyard/Krait%20MkII";
    render(App);
    const ship = await screen.findByRole("combobox", { name: "Ship" });
    await fireEvent.input(ship, { target: { value: "anaconda" } });
    await screen.findByText("Anaconda");
    await fireEvent.keyDown(ship, { key: "Enter" });
    expect(location.hash).toBe("#/places/shipyard/Anaconda");
    await waitFor(() =>
      expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "ship", ship: "Anaconda" }, null, expect.anything()),
    );
  });

  it("opens a module's nearby outfitting search", async () => {
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("My standing");

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const input = await screen.findByRole("combobox", { name: "Go to" });
    await fireEvent.input(input, { target: { value: "frame shift" } });
    await screen.findAllByText("Outfitting · Core internals");
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(location.hash).toBe("#/places/outfitting/Frame%20Shift%20Drive");
    expect(await screen.findByText(/Abraham Lincoln/)).toBeTruthy();
    expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "module", name: "Frame Shift Drive", class: null, rating: null, mount: null }, null, expect.anything());
  });

  it("opens a fitted module's search at its size, and a ship's shipyard search", async () => {
    location.hash = "#/places/outfitting";
    render(App);
    await screen.findByText(/Abraham Lincoln/);

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    let input = await screen.findByRole("combobox", { name: "Go to" });
    await fireEvent.input(input, { target: { value: "2f pulse laser" } });
    await screen.findByText(/^Outfitting · fitted on/);
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(location.hash).toBe("#/places/outfitting/Pulse%20Laser?class=2&rating=F&mount=Gimballed");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "module", name: "Pulse Laser", class: 2, rating: "F", mount: "Gimballed" }, null, expect.anything()));

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    input = await screen.findByRole("combobox", { name: "Go to" });
    await fireEvent.input(input, { target: { value: "anaconda" } });
    await screen.findByText("Shipyard", { selector: ".d" });
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(location.hash).toBe("#/places/shipyard/Anaconda");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "ship", ship: "Anaconda" }, null, expect.anything()));
  });

  it("keeps the Outfitting pick in the address for back and reload", async () => {
    location.hash = "#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(App);
    await screen.findByText(/Abraham Lincoln/);

    await fireEvent.click(screen.getByRole("button", { name: "Class 6" }));
    await waitFor(() => expect(location.hash).toBe("#/places/outfitting/Frame%20Shift%20Drive?class=6&rating=A"));
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ class: 6, rating: "A" }), null, expect.anything()));

    history.back();
    await waitFor(() => expect(location.hash).toBe("#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A"));
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ class: 5, rating: "A" }), null, expect.anything()));

    cleanup(); // reload
    stationsSpy.mockClear();
    render(App);
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ name: "Frame Shift Drive", class: 5 }), null, expect.anything()));
  });

  it("keeps a partial Outfitting pick when one part goes back to Any", async () => {
    location.hash = "#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(App);
    await screen.findByText(/Abraham Lincoln/);

    await fireEvent.click(screen.getByRole("button", { name: "Rating Any" }));
    await waitFor(() => expect(location.hash).toBe("#/places/outfitting/Frame%20Shift%20Drive?class=5"));
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ class: 5, rating: null }), null, expect.anything()));
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
    history.replaceState(null, "", "/?core=ready&route=exact");
    const { resetFakeCore } = await import("./lib/fakecore");
    resetFakeCore();
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
  beforeAll(() => (Element.prototype.scrollIntoView = vi.fn()));
  beforeEach(async () => {
    (await import("./lib/places")).forgetAnswers();
    stationsSpy.mockClear();
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
    expect(screen.getAllByRole("heading", { level: 2 }).map((h) => h.textContent)).toEqual(["Ship", "Engineering"]);
    const ship = within(screen.getByRole("region", { name: "Ship" }));
    expect(ship.getAllByRole("link").map((a) => a.getAttribute("href"))).toEqual(["#/places/services", "#/places/outfitting", "#/places/shipyard"]);
    expect(screen.getByRole("link", { name: /Material traders/ }).getAttribute("href")).toBe("#/places/material-traders");
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
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "service", service: "raw_trader" }, null, expect.anything()));
    expect(document.querySelectorAll("nav.sub")).toHaveLength(1);
    expect(pageTabs()).toEqual(["Places", "Route"]);

    await fireEvent.click(screen.getByRole("button", { name: "Encoded trader" }));
    expect(location.hash).toBe("#/places/material-traders/encoded");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "service", service: "encoded_trader" }, null, expect.anything()));
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
    expect(tile.querySelector(".v")?.textContent).toMatch(/^~?\d+$/);
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
      await waitFor(() => expect(stationsSpy, old).toHaveBeenLastCalledWith(want, null, expect.anything()));
      cleanup();
    }
  });

  it("rewrites an old link followed inside the window", async () => {
    location.hash = "#/places/services/refuel";
    render(App);
    await screen.findByRole("heading", { level: 1, name: "Services" });
    location.hash = "#/shipyard/Anaconda";
    await waitFor(() => expect(location.hash).toBe("#/places/shipyard/Anaconda"));
    expect(await screen.findByRole("heading", { level: 1, name: "Shipyard" })).toBeTruthy();
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "ship", ship: "Anaconda" }, null, expect.anything()));
  });

  it("picks a module in the Outfitting finder's main input", async () => {
    location.hash = "#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(App);
    const box = await screen.findByRole("combobox", { name: "Module" });
    await fireEvent.input(box, { target: { value: "pulse laser" } });
    await screen.findByText("Pulse Laser");
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(location.hash).toBe("#/places/outfitting/Pulse%20Laser");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "module", name: "Pulse Laser", class: null, rating: null, mount: null }, null, expect.anything()));
  });

  it("leaves a finder for another page without rewriting the address", async () => {
    location.hash = "#/places/shipyard/Anaconda";
    render(App);
    await screen.findByRole("heading", { level: 1, name: "Shipyard" });
    await fireEvent.click(screen.getByRole("button", { name: "Route" }));
    await waitFor(() => expect(location.hash).toBe("#/route"));
    await new Promise((r) => setTimeout(r, 50));
    expect(location.hash).toBe("#/route");
  });
});
