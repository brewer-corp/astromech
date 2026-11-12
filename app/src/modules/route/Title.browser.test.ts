import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import Plot from "./Plot.svelte";
import View from "./View.svelte";
import { previewAnswer, startPreview, watch } from "../../test/transport";

// The Route page's title is the plotter: from where the commander is (the preview's commander is
// in Sol, in an Asp Explorer at 36.48 ly) to a destination edited in place, with the plot setup
// the header's plot panel shares. The preview plots from fixtures/plot.json.
const dest = () => screen.getByRole("combobox", { name: "Destination" }) as HTMLInputElement;
const title = () => screen.getByRole("heading", { level: 1 });

async function openPage(url = "/?core=ready") {
  startPreview(url);
  render(View);
  await screen.findByText(/No route\.|Following|waypoints left/);
  await waitFor(() => expect(title().textContent).toMatch(/^\s*Sol\s+to/));
}

async function choose(picker: RegExp, option: RegExp) {
  await fireEvent.click(screen.getByRole("button", { name: picker }));
  await fireEvent.click(screen.getByRole("option", { name: option }));
}

describe("Route page title: plotting", () => {
  it("plots to a destination typed in place, then offers Replot from here", async () => {
    const plots = watch("route_plot");
    await openPage();
    expect(dest().value).toBe("");
    expect((screen.getByRole("button", { name: "Plot route" }) as HTMLButtonElement).disabled, "nowhere to plot to yet").toBe(true);
    await fireEvent.input(dest(), { target: { value: "Colonia" } });
    await fireEvent.keyDown(dest(), { key: "Enter" });
    expect(await screen.findByText("Plotted by Spansh.")).toBeTruthy();
    expect(plots).toHaveBeenCalledExactlyOnceWith({ ask: { plotter: "neutron", to: "Colonia", efficiency: 60, refuel_every_scoopable: true, engine: "local" } });
    expect((await previewAnswer("route")).view.route.name).toBe("Sol to Colonia");
    expect(dest().value).toBe("Colonia");

    await fireEvent.click(screen.getByRole("button", { name: "Replot from here" }));
    await waitFor(() => expect(plots).toHaveBeenCalledTimes(2));
    expect(plots.mock.calls[1]![0].ask.to).toBe("Colonia");

    // A new destination in the box plots there instead; Escape goes back to the route's.
    await fireEvent.input(dest(), { target: { value: "Maia" } });
    expect(screen.getByRole("button", { name: "Plot route" })).toBeTruthy();
    await fireEvent.keyDown(dest(), { key: "Escape" });
    expect(dest().value).toBe("Colonia");
    expect(screen.getByRole("button", { name: "Replot from here" })).toBeTruthy();
  });

  it("says it is plotting while Spansh works, and keeps the header's panel from plotting too", async () => {
    await openPage("/?core=ready&slow=never:route_plot");
    render(Plot, { to: "Maia" });
    await fireEvent.input(dest(), { target: { value: "Colonia" } });
    await fireEvent.click(screen.getByRole("button", { name: "Plot route" }));
    const busy = screen.getByRole("button", { name: "Plotting…" }) as HTMLButtonElement;
    expect(busy.disabled).toBe(true);
    expect(screen.getByText("Asking Spansh. A route can take a minute.")).toBeTruthy();
    expect(screen.getByText("A route is being plotted on the Route page.")).toBeTruthy();
    expect((screen.getByRole("button", { name: "Plot and follow" }) as HTMLButtonElement).disabled).toBe(true);
  });

  it("shows a failed plot as a hard error, and TRY AGAIN plots again", async () => {
    await openPage("/?core=ready&fail=route_plot");
    await fireEvent.input(dest(), { target: { value: "Colonia" } });
    await fireEvent.click(screen.getByRole("button", { name: "Plot route" }));
    const alert = await screen.findByRole("alert");
    expect(alert.textContent).toMatch(/^! preview: route_plot failed on purpose \(\?fail=\)\s*Try again$/);

    history.replaceState(null, "", "/?core=ready");
    await fireEvent.click(within(alert).getByRole("button", { name: "Try again" }));
    expect(await screen.findByText("Plotted by Spansh.")).toBeTruthy();
    expect(screen.queryByRole("alert")).toBeNull();
  });
});

describe("Route page title: the plot setup", () => {
  it("downloads the neutron star file under Worked out, plots on this PC, and changes the engine back to Spansh", async () => {
    const plots = watch("route_plot");
    await openPage();
    await fireEvent.click(screen.getByRole("button", { name: /^Worked out/ }));
    expect(screen.getByLabelText("Efficiency"), "Spansh plots until the file is there").toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Download" }));
    expect(await screen.findByText(/Neutron star file: 5,045,154 neutron stars/)).toBeTruthy();
    expect(screen.getByRole("button", { name: /^Worked out\s*On this PC/ })).toBeTruthy();
    expect(screen.queryByLabelText("Efficiency"), "the local plotter has no setting").toBeNull();
    await fireEvent.keyDown(screen.getByRole("listbox", { name: "Worked out" }), { key: "Escape" });
    expect(screen.queryByRole("listbox"), "Escape closes it").toBeNull();

    await fireEvent.input(dest(), { target: { value: "HR 3230" } });
    await fireEvent.keyDown(dest(), { key: "Enter" });
    expect(await screen.findByText("Plotted on this PC.")).toBeTruthy();
    // The local route was plotted at 39.87 ly over the tests' sample; the ship makes 36.48 now.
    const range = () => document.querySelector(".range")?.textContent?.replace(/\s+/g, " ");
    expect(range()).toBe("Your range is 36.48 ly now; this route was plotted at 39.87.");

    await choose(/^Worked out/, /^By Spansh/);
    await fireEvent.click(screen.getByRole("button", { name: "Replot from here" }));
    expect(await screen.findByText("Plotted by Spansh.")).toBeTruthy();
    expect(plots.mock.calls.map((c) => c[0].ask.engine)).toEqual(["local", "spansh"]);
    expect(range(), "replotted at the ship's range").toBeUndefined();
  });

  it("is the one setup the header's plot panel shows, taken from the route followed", async () => {
    await openPage("/?core=ready&route=exact");
    // The route followed was plotted exact, so the page and the panel say so.
    expect(screen.getByRole("button", { name: /^Plotter\s*Exact/ })).toBeTruthy();
    expect(dest().value).toBe("Maia");
    render(Plot, { to: "Colonia" });
    const panel = () => screen.getAllByRole("button", { name: /^Plotter/ });
    await waitFor(() => expect(panel().map((b) => b.textContent?.replace(/\s+/g, " ").trim())).toEqual(["Plotter Exact", "Plotter Exact"]));

    // Changed in the panel, the page follows.
    await fireEvent.click(panel()[1]!);
    await fireEvent.click(screen.getByRole("option", { name: /^Neutron/ }));
    expect(panel().map((b) => b.textContent?.replace(/\s+/g, " ").trim())).toEqual(["Plotter Neutron", "Plotter Neutron"]);
    expect(screen.getAllByRole("button", { name: /^Worked out/ })).toHaveLength(2);
  });
});

describe("Route page title: the ⋯ menu", () => {
  it("opens the destination on Spansh, and has nothing to clear or open without a route", async () => {
    const opened = watch("open_external");
    await openPage();
    await fireEvent.click(screen.getByRole("button", { name: "More route actions" }));
    expect((screen.getByRole("menuitem", { name: /Open on Spansh/ }) as HTMLButtonElement).disabled).toBe(true);
    expect((screen.getByRole("menuitem", { name: "Clear route" }) as HTMLButtonElement).disabled).toBe(true);
    await fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    expect(screen.queryByRole("menu")).toBeNull();

    await fireEvent.input(dest(), { target: { value: "Colonia" } });
    await fireEvent.keyDown(dest(), { key: "Enter" });
    await screen.findByText("Plotted by Spansh.");
    await fireEvent.click(screen.getByRole("button", { name: "More route actions" }));
    await fireEvent.click(screen.getByRole("menuitem", { name: /Open on Spansh/ }));
    expect(opened).toHaveBeenCalledExactlyOnceWith({ target: "https://spansh.co.uk/search/Colonia" });
  });

});
