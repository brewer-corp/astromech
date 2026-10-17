import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { fakeCall, resetFakeCore } from "../../lib/fakecore";
import { followRoute } from "../../lib/route";
import View from "./View.svelte";

describe("Galaxy → Route", () => {
  it("imports a route, skips, goes back and clears it through the core", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(View);
    expect(await screen.findByText(/No route\./)).toBeTruthy();
    // The preview commander is in Sol, the first stop: the route starts at the second.
    await fireEvent.input(screen.getByPlaceholderText(/Paste a Spansh CSV/), {
      target: { value: '"System Name","Jumps"\n"Sol","0"\n"Shinrarta Dezhra","3"\n"Colonia","40"\n' },
    });
    await fireEvent.click(screen.getByRole("button", { name: "Follow this route" }));
    expect(await screen.findByText("On your clipboard")).toBeTruthy();
    const next = () => document.querySelector(".next .sys")?.textContent;
    expect(next()).toBe("Shinrarta Dezhra");
    await fireEvent.click(screen.getByRole("button", { name: "Skip" }));
    await screen.findByText("1 of 3 waypoints left", { exact: false });
    expect(next()).toBe("Colonia");
    await fireEvent.click(screen.getByRole("button", { name: "Back" }));
    await screen.findByText("2 of 3 waypoints left", { exact: false });
    await fireEvent.click(screen.getByRole("button", { name: "Clear route" }));
    expect(await screen.findByText(/No route\./)).toBeTruthy();
    expect((await fakeCall("route")).view.route).toBeNull();
  });

  it("shows a route followed from elsewhere, with its labels", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    await followRoute("Engineer trip", "engineer trip", [
      { system: "Deciat", label: "Felicity Farseer" },
      { system: "deciat", label: "Professor Palin" },
      { system: "Kuk", label: "Ram Tah" },
    ]);
    render(View);
    expect(await screen.findByText("Engineer trip")).toBeTruthy();
    // Two engineers in one system are one stop.
    expect(screen.getAllByText("Felicity Farseer, Professor Palin").length).toBeGreaterThan(0);
    expect(screen.getByText("engineer trip · 2 of 2 waypoints left")).toBeTruthy();
  });

  it("plots a route with Spansh and follows it", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(View);
    expect(await screen.findByText(/Asp Explorer: 39\.87 ly a jump with a full tank/)).toBeTruthy();
    const plot = screen.getByRole("button", { name: "Plot and follow" }) as HTMLButtonElement;
    expect(plot.disabled, "nothing to plot without a destination").toBe(true);
    await fireEvent.click(screen.getByRole("button", { name: "Exact" }));
    await fireEvent.input(screen.getByLabelText("Destination system"), { target: { value: "Maia" } });
    await fireEvent.click(plot);
    expect(await screen.findByText("Following Sol to Maia: 9 jumps · 452 ly · 1 neutron boost · 3 refuel stops.")).toBeTruthy();
    // The commander is in Sol, the start: the first jump is next, with what to do there.
    expect(document.querySelector(".next .sys")?.textContent).toBe("Tascheter Sector MN-T b3-6");
    expect(document.querySelector(".next .lo")?.textContent).toBe("fuel");
    expect(screen.getByText("exact plotter · 9 of 10 waypoints left")).toBeTruthy();
  });
});
