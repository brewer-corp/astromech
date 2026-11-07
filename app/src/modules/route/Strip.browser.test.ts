import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import { fakeClipboard, fakeJump, fakePlotInGame } from "../../preview/fakecore";
import { addStop, followRoute, type Waypoint } from "../../lib/route";
import PlotBar from "./PlotBar.svelte";
import Strip from "./Strip.svelte";
import { previewAnswer, startPreview } from "../../test/transport";

// The route strip under the header (DESIGN.md Route strip): the next stop, how far is left, and
// one warning when the commander should act. The preview commander starts in Sol.
afterEach(() => cleanup());

function start(url = "/?core=ready") {
  startPreview(url);
}

const strip = () => within(screen.getByRole("region", { name: "Route" }));
/** The strip's text as it reads: each piece of text, in order, a space apart. */
function line(): string {
  const w = document.createTreeWalker(screen.getByRole("region", { name: "Route" }), NodeFilter.SHOW_TEXT);
  const parts: string[] = [];
  while (w.nextNode()) parts.push(w.currentNode.textContent ?? "");
  return parts.join(" ").replace(/\s+/g, " ").trim();
}

/** An exact route: every stop one jump on from the one before. */
const exact = (systems: string[], labels: Record<string, string> = {}): Waypoint[] =>
  systems.map((system, i) => ({ system, jumps: i === 0 ? 0 : 1, ...(labels[system] ? { label: labels[system] } : {}) }));

describe("Route strip", () => {
  it("shows the next stop, where it is in the route, its flags and the jumps left", async () => {
    start();
    await followRoute("Sol to Maia", "exact plotter", exact(["Sol", "A", "B", "Maia"], { A: "neutron boost and fuel" }));
    render(Strip);
    await strip().findByText("A");
    expect(line()).toBe("Next A 2 of 4 neutron boost fuel To Maia 3 jumps left");
    expect(strip().getByText("neutron boost").classList.contains("badge")).toBe(true);
    expect(strip().getByText("fuel").classList.contains("badge")).toBe(true);
    expect(strip().getByRole("link").getAttribute("href"), "the destination opens the Route page").toBe("#/route");
    // Copy writes the next stop to the clipboard again.
    await fireEvent.click(strip().getByRole("button", { name: "Copy A" }));

    // Jumping on moves it, and one jump left is singular.
    fakeJump("A");
    fakeJump("B");
    await waitFor(() => expect(line()).toBe("Next Maia 4 of 4 To Maia 1 jump left"), { timeout: 3000 });
  });

  it("counts stops where the route doesn't know its jumps", async () => {
    start();
    await followRoute("Engineer trip", "engineer trip", [
      { system: "Deciat", label: "Felicity Farseer" },
      { system: "deciat", label: "Professor Palin" },
      { system: "Kuk", label: "Ram Tah" },
    ]);
    render(Strip);
    await strip().findByText("Deciat");
    expect(line()).toBe("Next Deciat 1 of 2 Felicity Farseer Professor Palin To Kuk 2 stops left");
    // Between the stops of a trip the commander is elsewhere: that is not off route.
    fakeJump("Deciat");
    fakeJump("Wolf 359");
    await waitFor(() => expect(line()).toBe("Next Kuk 2 of 2 Ram Tah To Kuk 1 stop left"), { timeout: 3000 });
    // A lookup's stop added to a plotted route makes the jumps unknown.
    start();
    await followRoute("Sol to Maia", "exact plotter", exact(["Sol", "A", "Maia"]));
    await addStop({ system: "Deciat", label: "Jameson Memorial" }, "services");
    await waitFor(() => expect(line()).toBe("Next Deciat 2 of 4 Jameson Memorial To Maia 3 stops left"), { timeout: 3000 });
  });

  it("says No route, and Find or plot puts the cursor in the header's box", async () => {
    start();
    render(PlotBar);
    render(Strip);
    await strip().findByText("No route");
    expect(line()).toBe("No route Find or plot");
    await fireEvent.click(strip().getByRole("button", { name: "Find or plot" }));
    expect(document.activeElement).toBe(screen.getByRole("combobox", { name: "Find or plot" }));
  });

  it("says Off route when the commander isn't in the last stop or the next, a jump apart", async () => {
    start();
    await followRoute("Sol to Maia", "exact plotter", exact(["Sol", "A", "B", "Maia"]));
    render(Strip);
    await strip().findByText("A");
    expect(line()).not.toContain("Off route");
    fakeJump("Wolf 359");
    await strip().findByText("Off route", {}, { timeout: 3000 });
    expect(line()).toBe("Next A 2 of 4 ! Off route To Maia 3 jumps left");
    // Back on it: the warning goes.
    fakeJump("A");
    await waitFor(() => expect(line()).toBe("Next B 3 of 4 To Maia 2 jumps left"), { timeout: 3000 });
  });

  it("says to refuel at a fuel stop", async () => {
    start();
    await followRoute("Sol to Maia", "exact plotter", exact(["Sol", "A", "B", "Maia"], { A: "fuel" }));
    fakeJump("A");
    render(Strip);
    await strip().findByText("B");
    expect(line()).toBe("Next B 3 of 4 ! Refuel before the next jump To Maia 2 jumps left");
  });

  it("says when the next stop could not be copied, and Copy tries again", async () => {
    start();
    fakeClipboard("another program has the clipboard open");
    await followRoute("Sol to Maia", "exact plotter", exact(["Sol", "A", "Maia"]));
    render(Strip);
    await strip().findByText("A");
    expect(line()).toBe("Next A 2 of 3 ! Not on the clipboard: another program has the clipboard open To Maia 2 jumps left");
    fakeClipboard(null);
    await fireEvent.click(strip().getByRole("button", { name: "Copy A" }));
    await waitFor(() => expect(line()).toBe("Next A 2 of 3 To Maia 2 jumps left"));
  });

  it("follows a galaxy-map route by its jumps, and offers Resume for the route it paused once it ends", async () => {
    start();
    await followRoute("Sol to Colonia", "neutron plotter", [{ system: "Sol" }, { system: "N1" }, { system: "Colonia" }]);
    fakePlotInGame(["Sol", "Wolf 359", "Ross 128"]);
    render(Strip);
    await strip().findByText("Wolf 359");
    expect(line(), "the paused route waits on the Route page").toBe("Next Wolf 359 1 of 2 To Ross 128 2 jumps left");
    fakeJump("Wolf 359");
    fakeJump("Ross 128");
    await strip().findByText("Route finished", {}, { timeout: 3000 });
    expect(line()).toBe("Route finished Find or plot Paused Sol to Colonia Resume");
    await fireEvent.click(strip().getByRole("button", { name: "Resume" }));
    await strip().findByText("N1");
    expect((await previewAnswer("route")).view.route?.name).toBe("Sol to Colonia");
  });

  it("starts on the exact route in the preview with ?route=exact", async () => {
    start("/?core=ready&route=exact");
    render(Strip);
    await strip().findByText("Arietis Sector LD-H a11-1");
    expect(line()).toBe("Next Arietis Sector LD-H a11-1 4 of 10 To Maia 7 jumps left");
  });

  it("says plainly when the route can't be read, as a hard error with TRY AGAIN", async () => {
    start("/?core=ready&fail=route");
    render(Strip);
    await strip().findByText(/Could not read the route/);
    expect(strip().getByRole("alert").textContent).toMatch(/^! Could not read the route: .*Try again$/);
    expect(line()).toMatch(/^! Could not read the route: preview: route failed on purpose \(\?fail=\)\. Try again$/);

    start("/?core=ready&route=exact"); // the core answers again
    await fireEvent.click(strip().getByRole("button", { name: "Try again" }));
    await strip().findByText("Arietis Sector LD-H a11-1");
    expect(strip().queryByRole("alert")).toBeNull();
  });
});
