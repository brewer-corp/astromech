import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
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
});
