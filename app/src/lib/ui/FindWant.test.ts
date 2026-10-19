import { afterEach, describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import FindWant from "./FindWant.svelte";

describe("FindWant", () => {
  afterEach(() => {
    location.hash = "";
    document.body.replaceChildren();
  });

  it("opens a shipyard search when a ship is picked", async () => {
    render(FindWant, { current: "Interstellar Factors" });
    const box = screen.getByRole("combobox", { name: "Find" });
    await fireEvent.input(box, { target: { value: "anaconda" } });
    expect(await screen.findByText("Anaconda")).toBeTruthy();
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(location.hash).toBe("#/shipyard/Anaconda");
  });

  it("opens a module search when a module is picked", async () => {
    render(FindWant, { current: "Interstellar Factors" });
    const box = screen.getByRole("combobox", { name: "Find" });
    await fireEvent.input(box, { target: { value: "pulse laser" } });
    expect(await screen.findByText("Pulse Laser")).toBeTruthy();
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(location.hash).toBe("#/outfitting/Pulse%20Laser");
  });
});
