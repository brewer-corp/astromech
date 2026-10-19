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

  it("does not change the page when Find loses focus with the current name", async () => {
    location.hash = "#/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(FindWant, { current: "Frame Shift Drive" });
    const box = screen.getByRole("combobox", { name: "Find" });
    box.focus();
    await fireEvent.blur(box);
    await new Promise((r) => setTimeout(r, 200));
    expect(location.hash).toBe("#/outfitting/Frame%20Shift%20Drive?class=5&rating=A");
  });

  it("does not navigate after the box is gone", async () => {
    location.hash = "#/engineering";
    const { unmount } = render(FindWant, { current: "Interstellar Factors" });
    const box = screen.getByRole("combobox", { name: "Find" });
    box.focus();
    await fireEvent.blur(box);
    unmount();
    await new Promise((r) => setTimeout(r, 200));
    expect(location.hash).toBe("#/engineering");
  });

  it("keeps class and rating when Enter is pressed on the current module name", async () => {
    location.hash = "#/outfitting/Frame%20Shift%20Drive?class=5&rating=A";
    render(FindWant, { current: "Frame Shift Drive" });
    const box = screen.getByRole("combobox", { name: "Find" });
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(location.hash).toBe("#/outfitting/Frame%20Shift%20Drive?class=5&rating=A");
  });
});
