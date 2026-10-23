import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { fakeCall, resetFakeCore } from "../../lib/fakecore";
import { resetPreviewNeutron } from "../../lib/neutron";
import PlotBar from "./PlotBar.svelte";

// The top bar's system search: pick a system, and the panel plots from the current system (the
// preview commander is in Sol) and follows the route.
afterEach(() => {
  cleanup();
  resetPreviewNeutron();
});

async function search(text: string) {
  const box = screen.getByLabelText("Plot to a system");
  await fireEvent.input(box, { target: { value: text } });
  await fireEvent.keyDown(box, { key: "Enter" });
  return box;
}

describe("Top bar → Plot to a system", () => {
  it("suggests systems and picks one with a click", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(PlotBar);
    await fireEvent.input(screen.getByLabelText("Plot to a system"), { target: { value: "Col" } });
    await fireEvent.click(await screen.findByRole("button", { name: "Colonia" }));
    expect(screen.getByRole("dialog", { name: "Plot to Colonia" })).toBeTruthy();
    expect(screen.queryByText(/^From/), "routes always start where you are").toBeNull();
    // Enter takes the highlighted suggestion, not a partial name.
    await fireEvent.keyDown(screen.getByLabelText("Plot to a system"), { key: "Escape" });
    await fireEvent.input(screen.getByLabelText("Plot to a system"), { target: { value: "Shin" } });
    await screen.findByRole("button", { name: "Shinrarta Dezhra" });
    await fireEvent.keyDown(screen.getByLabelText("Plot to a system"), { key: "Enter" });
    expect(screen.getByRole("dialog", { name: "Plot to Shinrarta Dezhra" })).toBeTruthy();
    // Escape closes the panel.
    await fireEvent.keyDown(screen.getByLabelText("Plot to a system"), { key: "Escape" });
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("plots an exact route with Spansh and follows it", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(PlotBar);
    await search("Maia");
    expect(await screen.findByText(/Asp Explorer: 39\.87 ly a jump with a full tank/)).toBeTruthy();
    expect(screen.getByText("Sol to")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Exact" }));
    await fireEvent.click(screen.getByRole("button", { name: "Plot and follow" }));
    expect(await screen.findByText(/^Following Sol to Maia: 9 jumps · 452 ly · 1 neutron boost · 3 refuel stops, at 39\.87 ly a jump\. Plotted by Spansh\./)).toBeTruthy();
    const v = (await fakeCall("route")).view;
    expect(v.route.source).toBe("exact plotter");
    expect(v.next.system).toBe("Tascheter Sector MN-T b3-6");
  });

  it("plots a neutron route with Spansh until the neutron star file is downloaded, then on this PC, with Enter", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    resetPreviewNeutron();
    render(PlotBar);
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
    expect((await fakeCall("route")).view.route.source).toBe("neutron plotter (this PC)");
  });
});
