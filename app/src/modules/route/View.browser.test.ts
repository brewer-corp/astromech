import { afterEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import { fakeClipboard, fakePlotInGame } from "../../preview/fakecore";
import { followRoute } from "../../lib/route";
import View from "./View.svelte";
import { previewAnswer, startPreview } from "../../test/transport";

/** The timeline row of a stop or jump, by its system. */
const row = (system: string) => screen.getByText(system, { selector: ".name, .jn" }).closest("li")!;

describe("Galaxy → Route", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("imports a route from the ⋯ menu's dialog, makes a stop ahead or behind the next, and clears it through the core", async () => {
    startPreview("/?core=ready");
    render(View);
    expect(await screen.findByText(/No route\./)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "More route actions" }));
    await fireEvent.click(screen.getByRole("menuitem", { name: /Import a route/ }));
    const dialog = screen.getByRole("dialog", { name: "Import a route" });
    // The preview commander is in Sol, the first stop: the route starts at the second.
    await fireEvent.input(within(dialog).getByPlaceholderText(/Paste a Spansh CSV/), {
      target: { value: '"System Name","Jumps"\n"Sol","0"\n"Shinrarta Dezhra","3"\n"Colonia","40"\n' },
    });
    await fireEvent.click(within(dialog).getByRole("button", { name: "Follow this route" }));
    // The preview's import keeps the systems alone, so the jumps away aren't known.
    expect(await screen.findByText("Next stop")).toBeTruthy();
    expect(screen.queryByRole("dialog"), "following closes the dialog").toBeNull();
    expect(within(row("Sol")).getByText("You are here")).toBeTruthy();
    expect(row("Shinrarta Dezhra").classList.contains("next")).toBe(true);
    expect(within(row("Shinrarta Dezhra")).queryByRole("button", { name: "Make next" }), "it is next already").toBeNull();
    await fireEvent.click(within(row("Colonia")).getByRole("button", { name: "Make next" }));
    await screen.findByText("1 of 3 waypoints left", { exact: false });
    expect(row("Colonia").classList.contains("next")).toBe(true);
    await fireEvent.click(within(row("Sol")).getByRole("button", { name: "Make next" }));
    await screen.findByText("3 of 3 waypoints left", { exact: false });
    await fireEvent.click(screen.getByRole("button", { name: "More route actions" }));
    await fireEvent.click(screen.getByRole("menuitem", { name: "Clear route" }));
    expect(await screen.findByText(/No route\./)).toBeTruthy();
    expect((await previewAnswer("route")).view.route).toBeNull();
  });

  it("keeps a failed import in its dialog with the text, and Cancel closes it", async () => {
    startPreview("/?core=ready&fail=route_set");
    render(View);
    await screen.findByText(/No route\./);
    await fireEvent.click(screen.getByRole("button", { name: "More route actions" }));
    await fireEvent.click(screen.getByRole("menuitem", { name: /Import a route/ }));
    const dialog = screen.getByRole("dialog", { name: "Import a route" });
    const box = within(dialog).getByPlaceholderText(/Paste a Spansh CSV/) as HTMLTextAreaElement;
    await fireEvent.input(box, { target: { value: "Deciat\nKuk" } });
    await fireEvent.click(within(dialog).getByRole("button", { name: "Follow this route" }));
    expect((await within(dialog).findByRole("alert")).textContent).toMatch(/^! preview: route_set failed on purpose/);
    expect(box.value).toBe("Deciat\nKuk");
    await fireEvent.click(within(dialog).getByRole("button", { name: "Cancel" }));
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  it("shows a failed action as a hard error, never a filled block, and TRY AGAIN repeats it", async () => {
    startPreview("/?core=ready&fail=route_go");
    await followRoute("Trip", "test", [{ system: "Deciat" }, { system: "Kuk" }, { system: "Colonia" }]);
    render(View);
    expect(await screen.findByText("Following Trip")).toBeTruthy();
    await fireEvent.click(within(row("Kuk")).getByRole("button", { name: "Make next" }));
    const alert = await screen.findByRole("alert");
    expect(alert.textContent).toMatch(/^! preview: route_go failed on purpose \(\?fail=\)\s*Try again$/);
    expect(alert.classList.contains("failure")).toBe(true);

    history.replaceState(null, "", "/?core=ready"); // the core takes it this time
    expect(screen.getByText("3 of 3 waypoints left", { exact: false })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    await screen.findByText("2 of 3 waypoints left", { exact: false });
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("shows a clipboard write that failed as a hard error at the next stop, and TRY AGAIN copies it with a check", async () => {
    startPreview("/?core=ready");
    fakeClipboard("the clipboard is locked by another app");
    await followRoute("Trip", "test", [{ system: "Deciat" }, { system: "Kuk" }]);
    render(View);
    const alert = await screen.findByRole("alert");
    // DESIGN.md's hard error: a red "!" and plain words, then the recovery; no copy glyph stands in for the "!".
    expect(alert.textContent).toMatch(/^! Not on the clipboard: the clipboard is locked by another app\s*Try again$/);
    expect(alert.classList.contains("failure")).toBe(true);
    expect(alert.querySelector(".glyph")).toBeNull();
    expect(alert.closest("li")).toBe(row("Deciat"));

    fakeClipboard(null);
    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    expect(await screen.findByRole("button", { name: "Copied Deciat" })).toBeTruthy();
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("copies another stop with the window's clipboard: a check when it worked, the red error when it didn't", async () => {
    startPreview("/?core=ready");
    await followRoute("Trip", "test", [{ system: "Deciat" }, { system: "Kuk" }]);
    const writeText = vi.fn().mockResolvedValueOnce(undefined).mockRejectedValueOnce(new Error("denied"));
    vi.stubGlobal("navigator", { clipboard: { writeText } });
    render(View);
    await fireEvent.click(await screen.findByRole("button", { name: "Copy Kuk" }));
    expect(writeText).toHaveBeenCalledExactlyOnceWith("Kuk");
    expect(await screen.findByRole("button", { name: "Copied Kuk" })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Copied Kuk" }));
    const alert = await screen.findByRole("alert");
    expect(alert.textContent).toMatch(/^! Not on the clipboard: denied\s*Try again$/);
    expect(alert.closest("li")).toBe(row("Kuk"));
    expect(screen.getByRole("button", { name: "Copy Kuk" }), "the check goes with the failure").toBeTruthy();
  });

  it("opens the flown leg one row a jump from NavRoute, with the commander's row marked", async () => {
    startPreview("/?core=ready&route=timeline");
    render(View);
    // fixtures/route.json: Sol to Colonia by neutron plotter, its leg to PSR J1752-2806 plotted in
    // the galaxy map, the commander two jumps along it in Alrai Sector LH-V b2-4.
    expect(await screen.findByText("Next stop · 9 jumps away")).toBeTruthy();
    const here = row("Alrai Sector LH-V b2-4");
    expect(here.classList.contains("here")).toBe(true);
    expect(within(here).getByText("You are here")).toBeTruthy();
    expect(row("Core Sys Sector RT-R b4-5").classList.contains("past")).toBe(true);
    expect(row("Erh Longo").classList.contains("ahead")).toBe(true);
    expect(within(row("Praea Euq RD-T b3-5")).getByRole("img", { name: "T Tauri star" })).toBeTruthy();
    // The plotter knows nothing of Colonia's star, and its one-jump leg has no row of its own.
    expect(within(row("Colonia")).getByRole("img", { name: "Unknown star" })).toBeTruthy();
    expect(screen.getAllByRole("listitem").filter((li) => li.classList.contains("leg"))).toHaveLength(0);
    expect(within(row("Colonia")).getByText("Expansion", { selector: ".state" })).toBeTruthy();
  });

  it("marks the commander where the leg was plotted again partway along, not at the stop before", async () => {
    startPreview("/?core=ready&route=replotted");
    render(View);
    // fixtures/route.json's /replotted: two jumps along, the leg plotted again from Alrai Sector
    // LH-V b2-4, so the core's position 0 is Alrai, and Sol is only the stop before.
    expect(await screen.findByText("Next stop · 9 jumps away")).toBeTruthy();
    const here = row("Alrai Sector LH-V b2-4");
    expect(here.classList.contains("here")).toBe(true);
    expect(within(here).getByText("You are here")).toBeTruthy();
    expect(row("Sol").classList.contains("here")).toBe(false);
    expect(row("Erh Longo").classList.contains("ahead")).toBe(true);
  });

  it("opens the flown leg before the route's first waypoint", async () => {
    startPreview("/?core=ready&route=first");
    render(View);
    // fixtures/route.json's /first: a route set to PSR J1752-2806 and Colonia while the galaxy map
    // has the leg to PSR J1752-2806 plotted from Alrai Sector LH-V b2-4.
    expect(await screen.findByText("Next stop · 9 jumps away")).toBeTruthy();
    const items = screen.getAllByRole("listitem");
    expect(items[0]).toBe(row("Alrai Sector LH-V b2-4"));
    expect(within(items[0]!).getByText("You are here")).toBeTruthy();
    expect(items.filter((li) => li.classList.contains("jump")), "Alrai, then the eight jumps between").toHaveLength(9);
    expect(row("PSR J1752-2806").classList.contains("next")).toBe(true);
  });

  it("gives a leg whose jumps aren't listed its count, and says when the flown leg's stars will appear", async () => {
    startPreview("/?core=ready");
    await followRoute("Neutron run", "neutron plotter", [
      { system: "Sol", jumps: 0 },
      { system: "N1", jumps: 3, neutron: true },
      { system: "N2", jumps: 4, neutron: true },
    ]);
    render(View);
    const legs = async () => (await screen.findAllByRole("listitem")).filter((li) => li.classList.contains("leg"));
    const [flown, ahead] = await legs();
    expect(flown!.textContent!.trim()).toMatch(/^3 jumps\s*The stars appear once the leg is plotted in the galaxy map\.$/);
    expect(ahead!.textContent!.trim()).toBe("4 jumps");
    expect(within(row("N1")).getByRole("img", { name: "Neutron star" })).toBeTruthy();
  });

  it("shows a leg ahead's listed stars, and warns of nowhere to scoop only where every jump is listed and known dry", async () => {
    startPreview("/?core=ready");
    const dry = (system: string) => ({ system, scoopable: false });
    await followRoute("Exact-ish", "exact plotter", [
      { system: "Sol", jumps: 0 },
      // The leg being flown: its jumps are rows, and it keeps its warning.
      { system: "A", jumps: 2, hops: [dry("A1"), dry("A")] },
      { system: "B", jumps: 2, hops: [dry("B1"), dry("B")] },
      // One star the plotter didn't flag: it may be scoopable.
      { system: "C", jumps: 2, hops: [{ system: "C1" }, dry("C")] },
      // A jump missing from the list.
      { system: "D", jumps: 3, hops: [dry("D1"), dry("D")] },
      { system: "E", jumps: 2, hops: [{ system: "E1", scoopable: true }, dry("E")] },
    ]);
    render(View);
    await screen.findByText("Following Exact-ish");
    const [flown, ...legs] = screen.getAllByRole("listitem").filter((li) => li.classList.contains("leg"));
    expect(flown!.textContent!.trim()).toBe("! No scoopable star for 2 jumps");
    expect(row("A1").classList.contains("jump")).toBe(true);
    expect(legs.map((l) => l.querySelectorAll(".pass").length)).toEqual([1, 1, 1, 1]);
    expect(within(legs[1]!).getByRole("img", { name: "Unknown star" })).toBeTruthy();
    expect(within(legs[3]!).getByRole("img", { name: "Scoopable star" })).toBeTruthy();
    expect(legs.map((l) => !!l.querySelector(".warn"))).toEqual([true, false, false, false]);
    expect(legs[0]!.querySelector(".warn")!.textContent).toBe("! No scoopable star for 2 jumps");
  });

  it("shows a route followed from elsewhere, with its labels", async () => {
    startPreview("/?core=ready");
    await followRoute("Engineer trip", "engineer trip", [
      { system: "Deciat", label: "Felicity Farseer" },
      { system: "deciat", label: "Professor Palin" },
      { system: "Kuk", label: "Ram Tah" },
    ]);
    render(View);
    expect(await screen.findByText("Following Engineer trip")).toBeTruthy();
    // Two engineers in one system are one stop.
    expect(screen.getAllByText("Felicity Farseer, Professor Palin").length).toBeGreaterThan(0);
    expect(screen.getByText("engineer trip · 2 of 2 waypoints left")).toBeTruthy();
  });

  it("follows a galaxy-map route quietly, parks the route it replaced and resumes it", async () => {
    startPreview("/?core=ready");
    await followRoute("Sol to Colonia", "neutron plotter", [{ system: "Sol" }, { system: "N1" }, { system: "Colonia" }]);
    // A leg to the next neutron star is part of the route.
    fakePlotInGame(["Sol", "A", "N1"]);
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] }); // the route poll; the test moves its clock
    render(View);
    expect(await screen.findByText("Following Sol to Colonia")).toBeTruthy();
    expect(screen.queryByText("Paused")).toBeNull();

    fakePlotInGame(["Sol", "Wolf 359", "Ross 128"]);
    // The page sees it on its next read of the route.
    await vi.advanceTimersByTimeAsync(2000);
    expect(await screen.findByText("Following Route to Ross 128")).toBeTruthy();
    expect(row("Wolf 359").classList.contains("next")).toBe(true);
    expect(screen.queryByRole("button", { name: "Make next" }), "the game routes its own plot").toBeNull();
    expect(screen.getByText("Paused")).toBeTruthy();
    expect(screen.getByText("Sol to Colonia · 2 waypoints left", { exact: false })).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Resume" }));
    expect(await screen.findByText("Following Sol to Colonia")).toBeTruthy();
    expect(screen.queryByText("Paused")).toBeNull();
    expect(row("N1").classList.contains("next")).toBe(true);

    // A route set from the window parks the unfinished route of ours it replaces, as in the
    // core; the galaxy map's route isn't parked, so the parked route stays.
    fakePlotInGame(["Sol", "Wolf 359"]);
    expect((await previewAnswer("route")).view.parked?.name).toBe("Sol to Colonia");
    await followRoute("Trip", "engineer trip", [{ system: "Deciat" }]);
    expect((await previewAnswer("route")).view.parked?.name).toBe("Sol to Colonia");
    await followRoute("Deciat", "search", [{ system: "Deciat" }]);
    expect((await previewAnswer("route")).view.parked?.name).toBe("Trip");
  });
});
