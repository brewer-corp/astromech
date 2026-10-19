import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
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
    expect(await screen.findByRole("heading", { name: /^Nearest/ })).toBeTruthy();
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

  it("finds a ship from the Galaxy find box", async () => {
    location.hash = "#/services";
    render(App);
    const find = await screen.findByRole("combobox", { name: "Find" });
    await fireEvent.input(find, { target: { value: "anaconda" } });
    await screen.findByText("Anaconda");
    await fireEvent.keyDown(find, { key: "Enter" });
    expect(location.hash).toBe("#/shipyard/Anaconda");
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

    expect(location.hash).toBe("#/outfitting/Frame%20Shift%20Drive");
    expect(await screen.findByText(/Abraham Lincoln/)).toBeTruthy();
    expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "module", name: "Frame Shift Drive", class: null, rating: null, mount: null }, null, expect.anything());
  });

  it("opens a fitted module's search at its size, and a ship's shipyard search", async () => {
    location.hash = "#/outfitting";
    render(App);
    await screen.findByText(/Abraham Lincoln/);

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    let input = await screen.findByRole("combobox", { name: "Go to" });
    await fireEvent.input(input, { target: { value: "2f pulse laser" } });
    await screen.findByText(/^Outfitting · fitted on/);
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(location.hash).toBe("#/outfitting/Pulse%20Laser?class=2&rating=F&mount=Gimballed");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "module", name: "Pulse Laser", class: 2, rating: "F", mount: "Gimballed" }, null, expect.anything()));

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    input = await screen.findByRole("combobox", { name: "Go to" });
    await fireEvent.input(input, { target: { value: "anaconda" } });
    await screen.findByText("Shipyard", { selector: ".d" });
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(location.hash).toBe("#/shipyard/Anaconda");
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith({ kind: "ship", ship: "Anaconda" }, null, expect.anything()));
  });

  it("keeps the Outfitting pick in the address for back and reload", async () => {
    location.hash = "#/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(App);
    await screen.findByText(/Abraham Lincoln/);

    await fireEvent.click(screen.getByRole("button", { name: "Class 6" }));
    await waitFor(() => expect(location.hash).toBe("#/outfitting/Frame%20Shift%20Drive?class=6&rating=A"));
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ class: 6, rating: "A" }), null, expect.anything()));

    history.back();
    await waitFor(() => expect(location.hash).toBe("#/outfitting/Frame%20Shift%20Drive?class=5&rating=A"));
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ class: 5, rating: "A" }), null, expect.anything()));

    cleanup(); // reload
    stationsSpy.mockClear();
    render(App);
    await waitFor(() => expect(stationsSpy).toHaveBeenLastCalledWith(expect.objectContaining({ name: "Frame Shift Drive", class: 5 }), null, expect.anything()));
  });

  it("keeps a partial Outfitting pick when one part goes back to Any", async () => {
    location.hash = "#/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(App);
    await screen.findByText(/Abraham Lincoln/);

    await fireEvent.click(screen.getByRole("button", { name: "Rating Any" }));
    await waitFor(() => expect(location.hash).toBe("#/outfitting/Frame%20Shift%20Drive?class=5"));
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
