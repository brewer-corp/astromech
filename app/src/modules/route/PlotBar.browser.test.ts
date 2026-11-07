import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import type { PaletteItem } from "../../lib/palette";
import PlotBar from "./PlotBar.svelte";
import { previewAnswer, startPreview } from "../../test/transport";

// The header's Find or plot box: systems (the preview's Spansh suggestions) with Plot, Add stop and
// Set destination, ranked with the window's entries; the plot panel plots from the current system
// (the preview commander is in Sol) and follows the route.
afterEach(() => {
  cleanup();
  document.body.replaceChildren();
});

const entries: PaletteItem[] = [
  { label: "Colonisation", detail: "Page · Trade", route: "colonisation" },
  { label: "Raw trader", detail: "Material traders", route: "places/material-traders/raw", finder: "material-traders" },
  { label: "Raw material trader", detail: "Services", route: "places/services/raw_trader", finder: "services" },
  { label: "Painite", detail: "Sell", route: "places/sell/painite", finder: "sell" },
];

function setup(url = "/?core=ready") {
  startPreview(url);
  const onpick = vi.fn<(route: string) => void>();
  const load = vi.fn(() => Promise.resolve(entries));
  render(PlotBar, { load, onpick });
  return { onpick, load, box: screen.getByRole("combobox", { name: "Find or plot" }) as HTMLInputElement };
}

const rows = () => screen.queryAllByRole("option").map((o) => within(o).getAllByText(/./, { selector: ".l, .sub" }).map((e) => e.textContent).join(" · "));
const chosen = () => screen.getAllByRole("option").findIndex((o) => o.getAttribute("aria-selected") === "true");

async function type(box: HTMLElement, text: string) {
  await fireEvent.focus(box);
  await fireEvent.input(box, { target: { value: text } });
}

async function search(text: string) {
  const box = screen.getByRole("combobox", { name: "Find or plot" });
  await fireEvent.input(box, { target: { value: text } });
  await fireEvent.keyDown(box, { key: "Enter" });
  return box;
}

describe("Find or plot", () => {
  it("is one box with Ctrl+K at its end, focused by Ctrl+K from anywhere but a bind capture", async () => {
    const { box, load } = setup();
    expect(box.placeholder).toBe("Find or plot");
    expect(screen.getByText("Ctrl+K")).toBeTruthy();
    const field = document.createElement("input");
    document.body.append(field);
    field.focus();
    await fireEvent.keyDown(field, { key: "k", ctrlKey: true });
    expect(document.activeElement).toBe(box);
    expect(load).toHaveBeenCalled();
    // Escape with nothing open goes back to where the keyboard was.
    await fireEvent.keyDown(box, { key: "Escape" });
    expect(document.activeElement).toBe(field);
    // Not while a dialog has the keyboard, nor Ctrl+Shift+K or a plain K.
    const modal = document.createElement("div");
    modal.setAttribute("aria-modal", "true");
    document.body.append(modal);
    await fireEvent.keyDown(window, { key: "k", ctrlKey: true });
    expect(document.activeElement).toBe(field);
    modal.remove();
    await fireEvent.keyDown(window, { key: "K", ctrlKey: true, shiftKey: true });
    await fireEvent.keyDown(window, { key: "k" });
    expect(document.activeElement).toBe(field);
  });

  it("ranks finders, pages and systems together, and opens an entry with the keyboard", async () => {
    const { box, onpick } = setup();
    await type(box, "raw trader");
    await waitFor(() => expect(rows()).toEqual(["Raw trader · Material traders", "Raw material trader · Services"]));
    expect(chosen()).toBe(0);
    await fireEvent.keyDown(box, { key: "ArrowDown" });
    expect(chosen()).toBe(1);
    await fireEvent.keyDown(box, { key: "ArrowDown" });
    expect(chosen(), "wraps").toBe(0);
    await fireEvent.keyDown(box, { key: "ArrowUp" });
    await fireEvent.keyDown(box, { key: "Enter", isComposing: true });
    expect(onpick, "not on the Enter that ends a composition").not.toHaveBeenCalled();
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(onpick).toHaveBeenCalledExactlyOnceWith("places/services/raw_trader");
    expect(box.value, "cleared after opening").toBe("");
    expect(screen.queryByRole("listbox")).toBeNull();

    await type(box, "col");
    await screen.findByText("Colonia");
    expect(rows()).toEqual(["Colonisation · Page · Trade", "Colonia · System"]);
    await fireEvent.keyDown(box, { key: "Escape" });
    expect(screen.queryByRole("listbox"), "Escape closes the popover").toBeNull();
    await fireEvent.keyDown(box, { key: "ArrowDown" });
    expect(rows(), "ArrowDown opens it again").toHaveLength(2);

    await type(box, "zzz");
    await waitFor(() => expect(screen.getByText(/^Nothing matches\./)).toBeTruthy());
  });

  it("opens a finder phrase with a click", async () => {
    const { box, onpick } = setup();
    await type(box, "sell painite");
    await fireEvent.click(await screen.findByText("Painite"));
    expect(onpick).toHaveBeenCalledExactlyOnceWith("places/sell/painite");
  });

  it("adds a system as the next stop", async () => {
    const { box } = setup("/?core=ready&route=exact");
    await type(box, "Deci");
    await fireEvent.click(await screen.findByRole("button", { name: "Add Deciat as a stop" }));
    expect(await screen.findByText("Deciat is the next stop. Deciat is on your clipboard.")).toBeTruthy();
    const v = (await previewAnswer("route")).view;
    expect(v.next.system).toBe("Deciat");
    expect(v.route.name, "added to the route followed").not.toBe("Deciat");
  });

  it("sets a system as the destination, parking the route it replaces", async () => {
    const { box } = setup("/?core=ready&route=exact");
    const before = (await previewAnswer("route")).view.route.name;
    await type(box, "Achen");
    await fireEvent.click(await screen.findByRole("button", { name: "Set Achenar as the destination" }));
    expect(await screen.findByText(`Replaced ${before}.`)).toBeTruthy();
    const v = (await previewAnswer("route")).view;
    expect([v.route.name, v.next.system, v.parked.name]).toEqual(["Achenar", "Achenar", before]);
    await fireEvent.click(screen.getByRole("button", { name: "Resume" }));
    await waitFor(async () => expect((await previewAnswer("route")).view.route.name).toBe(before));
  });

  it("plots to a system from its Plot action, a click or Enter, and Escape closes the panel", async () => {
    const { box } = setup();
    await type(box, "Col");
    await fireEvent.click(await screen.findByRole("button", { name: "Plot a route to Colonia" }));
    expect(screen.getByRole("dialog", { name: "Plot to Colonia" })).toBeTruthy();
    expect(box.value).toBe("Colonia");
    expect(screen.queryByText(/^From/), "routes always start where you are").toBeNull();
    await fireEvent.keyDown(box, { key: "Escape" });
    expect(screen.queryByRole("dialog")).toBeNull();

    await type(box, "Shin");
    await fireEvent.click(await screen.findByText("Shinrarta Dezhra"));
    expect(screen.getByRole("dialog", { name: "Plot to Shinrarta Dezhra" })).toBeTruthy();

    await type(box, "Ach");
    await screen.findByText("Achenar");
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(screen.getByRole("dialog", { name: "Plot to Achenar" })).toBeTruthy();
  });

  it("plots a system entered before Spansh answers, unless an entry starts with it (astromech-gasg)", async () => {
    const { box, onpick } = setup();
    await type(box, "trader");
    await waitFor(() => expect(rows()[0]).toMatch(/^Raw trader/));
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(screen.getByRole("dialog", { name: "Plot to trader" }), "a looser entry match doesn't take a system typed and entered at once").toBeTruthy();
    expect(onpick).not.toHaveBeenCalled();

    await fireEvent.keyDown(box, { key: "Escape" });
    await type(box, "Col");
    await waitFor(() => expect(rows()[0]).toMatch(/^Colonisation/));
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(onpick).toHaveBeenCalledWith("colonisation");
  });

  it("plots an exact route with Spansh and follows it", async () => {
    setup();
    await search("Maia");
    expect(await screen.findByText(/Asp Explorer: 39\.87 ly a jump with a full tank/)).toBeTruthy();
    expect(screen.getByText("Sol to")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Exact" }));
    await fireEvent.click(screen.getByRole("button", { name: "Plot and follow" }));
    expect(await screen.findByText(/^Following Sol to Maia: 9 jumps · 452 ly · 1 neutron boost · 3 refuel stops, at 39\.87 ly a jump\. Plotted by Spansh\./)).toBeTruthy();
    const v = (await previewAnswer("route")).view;
    expect(v.route.source).toBe("exact plotter");
    expect(v.next.system).toBe("Tascheter Sector MN-T b3-6");
  });

  it("plots a neutron route with Spansh until the neutron star file is downloaded, then on this PC, with Enter", async () => {
    setup();
    const box = await search("Colonia");
    expect(await screen.findByText(/Neutron star file: not downloaded\./)).toBeTruthy();
    expect(screen.getByLabelText("Efficiency"), "Spansh's setting").toBeTruthy();
    await screen.findByText(/39\.87 ly a jump/);
    // The system is picked: Enter again plots.
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(await screen.findByText(/^Following Sol to Colonia: 128 jumps · 22,000 ly · 1 neutron boost, at 39\.87 ly a jump\. Plotted by Spansh\./)).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Download" }));
    expect(await screen.findByText(/Neutron star file: 5,045,154 neutron stars/)).toBeTruthy();
    expect(screen.queryByLabelText("Efficiency"), "local routes have no setting").toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: "Plot and follow" }));
    expect(await screen.findByText(/Plotted on this PC, near the fewest jumps\./)).toBeTruthy();
    expect((await previewAnswer("route")).view.route.source).toBe("neutron plotter (this PC)");
  });
});
