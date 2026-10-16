import { afterEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import type { PaletteItem } from "../palette";
import Palette from "./Palette.svelte";

const items: PaletteItem[] = [
  { label: "Materials", detail: "Page · Engineering", route: "materials" },
  { label: "Powerplay", detail: "Page · Powerplay", route: "powerplay" },
  { label: "Proto Heat Radiators", detail: "Material · Manufactured G5", route: "materials/protoheatradiators" },
  { label: "Proto Light Alloys", detail: "Material · Manufactured G4", route: "materials/protolightalloys" },
];

function setup() {
  const load = vi.fn(() => Promise.resolve(items));
  const onpick = vi.fn<(route: string) => void>();
  render(Palette, { load, onpick });
  return { load, onpick };
}

const ctrlK = () => fireEvent.keyDown(window, { key: "k", ctrlKey: true });
const box = () => screen.queryByRole("dialog", { name: "Command palette" });
const options = () => screen.getAllByRole("option").map((o) => o.textContent);

async function open() {
  await ctrlK();
  const input = await screen.findByRole("combobox", { name: "Go to" });
  await screen.findByText("Materials");
  return input;
}

describe("command palette", () => {
  afterEach(() => document.body.replaceChildren());

  it("opens on Ctrl+K with the entries loaded and the input focused", async () => {
    const { load } = setup();
    expect(box()).toBeNull();
    const input = await open();
    expect(load).toHaveBeenCalledOnce();
    expect(document.activeElement).toBe(input);
  });

  it("filters as you type and opens the first match on Enter", async () => {
    const { onpick } = setup();
    const input = await open();
    await fireEvent.input(input, { target: { value: "proto" } });
    expect(options()).toEqual(["Proto Heat RadiatorsMaterial · Manufactured G5", "Proto Light AlloysMaterial · Manufactured G4"]);
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(onpick).toHaveBeenCalledExactlyOnceWith("materials/protoheatradiators");
    expect(box()).toBeNull();
  });

  it("moves the choice with the arrows, staying within the list", async () => {
    const { onpick } = setup();
    const input = await open();
    await fireEvent.input(input, { target: { value: "proto" } });
    await fireEvent.keyDown(input, { key: "ArrowDown" });
    await fireEvent.keyDown(input, { key: "ArrowDown" });
    expect(screen.getAllByRole("option").map((o) => o.getAttribute("aria-selected"))).toEqual(["false", "true"]);
    await fireEvent.keyDown(input, { key: "ArrowUp" });
    await fireEvent.keyDown(input, { key: "ArrowUp" });
    await fireEvent.keyDown(input, { key: "ArrowDown" });
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(onpick).toHaveBeenCalledExactlyOnceWith("materials/protolightalloys");
  });

  it("opens a match on click", async () => {
    const { onpick } = setup();
    await open();
    await fireEvent.click(screen.getByText("Powerplay"));
    expect(onpick).toHaveBeenCalledExactlyOnceWith("powerplay");
  });

  it("closes on Esc or a second Ctrl+K without picking, and says when nothing matches", async () => {
    const { onpick } = setup();
    let input = await open();
    await fireEvent.input(input, { target: { value: "zzz" } });
    expect(screen.getByText("Nothing matches.")).toBeTruthy();
    await fireEvent.keyDown(input, { key: "Enter" });
    await fireEvent.keyDown(input, { key: "Escape" });
    expect(box()).toBeNull();
    input = await open();
    expect((input as HTMLInputElement).value).toBe("");
    await ctrlK();
    expect(box()).toBeNull();
    expect(onpick).not.toHaveBeenCalled();
  });

  it("works while typing in another text field, but not while a bind capture holds the keyboard", async () => {
    setup();
    const field = document.createElement("input");
    document.body.append(field);
    field.focus();
    await fireEvent.keyDown(field, { key: "k", ctrlKey: true });
    await waitFor(() => expect(box()).not.toBeNull());
    await fireEvent.keyDown(screen.getByRole("combobox", { name: "Go to" }), { key: "Escape" });
    expect(document.activeElement).toBe(field);

    const modal = document.createElement("div");
    modal.setAttribute("aria-modal", "true");
    document.body.append(modal);
    await ctrlK();
    expect(box()).toBeNull();
  });

  it("ignores K without Ctrl and Ctrl+Shift+K", async () => {
    setup();
    await fireEvent.keyDown(window, { key: "k" });
    await fireEvent.keyDown(window, { key: "K", ctrlKey: true, shiftKey: true });
    expect(box()).toBeNull();
  });
});
