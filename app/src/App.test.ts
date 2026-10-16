import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import App from "./App.svelte";

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

  it("opens a ship's page for a fitted module", async () => {
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("My standing");

    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    const input = await screen.findByRole("combobox", { name: "Go to" });
    await fireEvent.input(input, { target: { value: "6a frame shift" } });
    await screen.findByText("Module · Frame shift drive on Heavy Lifter");
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(location.hash).toBe("#/engineering/3");
    expect(await screen.findByRole("button", { name: "← Ships" })).toBeTruthy(); // the ship detail, not the fleet
    expect(screen.getByText(/Heavy Lifter/, { selector: "*:not(button):not(span.l)" })).toBeTruthy();
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
