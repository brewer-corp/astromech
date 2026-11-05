import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { fakeCall, fakeClipboard, fakePlotInGame, resetFakeCore } from "../../lib/fakecore";
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

  it("shows a failed action as a hard error, never a filled block, and TRY AGAIN repeats it", async () => {
    history.replaceState(null, "", "/?core=ready&fail=route_skip");
    resetFakeCore();
    await followRoute("Trip", "test", [{ system: "Deciat" }, { system: "Kuk" }, { system: "Colonia" }]);
    render(View);
    expect(await screen.findByText("Trip")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Skip" }));
    const alert = await screen.findByRole("alert");
    expect(alert.textContent).toMatch(/^! preview: route_skip failed on purpose \(\?fail=\)\s*Try again$/);
    expect(alert.classList.contains("failure")).toBe(true);

    history.replaceState(null, "", "/?core=ready"); // the core takes it this time
    expect(screen.getByText("3 of 3 waypoints left", { exact: false })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    await screen.findByText("2 of 3 waypoints left", { exact: false });
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("shows a clipboard write that failed as a hard error, and TRY AGAIN copies again", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    fakeClipboard("the clipboard is locked by another app");
    await followRoute("Trip", "test", [{ system: "Deciat" }, { system: "Kuk" }]);
    render(View);
    const alert = await screen.findByRole("alert");
    // DESIGN.md's hard error: a red "!" and plain words, then the recovery; no copy glyph stands in for the "!".
    expect(alert.textContent).toMatch(/^! Not on the clipboard: the clipboard is locked by another app\s*Try again$/);
    expect(alert.classList.contains("failure")).toBe(true);
    expect(alert.querySelector(".glyph")).toBeNull();
    expect(alert.closest(".next")).toBeTruthy();
    expect(screen.queryByText("On your clipboard")).toBeNull();

    fakeClipboard(null);
    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    expect(await screen.findByText("On your clipboard")).toBeTruthy();
    expect(screen.queryByRole("alert")).toBeNull();
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

  it("follows a galaxy-map route quietly, parks the route it replaced and resumes it", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    await followRoute("Sol to Colonia", "neutron plotter", [{ system: "Sol" }, { system: "N1" }, { system: "Colonia" }]);
    // A leg to the next neutron star is part of the route.
    fakePlotInGame(["Sol", "A", "N1"]);
    render(View);
    expect(await screen.findByText("Sol to Colonia")).toBeTruthy();
    expect(screen.queryByText("Paused")).toBeNull();

    fakePlotInGame(["Sol", "Wolf 359", "Ross 128"]);
    // The page sees it on its next read of the route.
    expect(await screen.findByText("Route to Ross 128", {}, { timeout: 3000 })).toBeTruthy();
    expect(screen.getByText("Next jump")).toBeTruthy();
    expect(screen.queryByText("On your clipboard"), "the game has the route plotted").toBeNull();
    expect(screen.getByText("Paused")).toBeTruthy();
    expect(screen.getByText("Sol to Colonia · 2 waypoints left", { exact: false })).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Resume" }));
    expect(await screen.findByText("Sol to Colonia")).toBeTruthy();
    expect(screen.queryByText("Paused")).toBeNull();
    expect(screen.getByText("On your clipboard")).toBeTruthy();

    // A route set from the window parks the unfinished route of ours it replaces, as in the
    // core; the galaxy map's route isn't parked, so the parked route stays.
    fakePlotInGame(["Sol", "Wolf 359"]);
    expect((await fakeCall("route")).view.parked?.name).toBe("Sol to Colonia");
    await followRoute("Trip", "engineer trip", [{ system: "Deciat" }]);
    expect((await fakeCall("route")).view.parked?.name).toBe("Sol to Colonia");
    await followRoute("Deciat", "search", [{ system: "Deciat" }]);
    expect((await fakeCall("route")).view.parked?.name).toBe("Trip");
  });
});
