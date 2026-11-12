import { afterEach, describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import NeutronFile from "./NeutronFile.svelte";
import Plot from "./Plot.svelte";
import { startPreview } from "../../test/transport";

afterEach(() => {
  history.replaceState(null, "", "/");
});

describe("Route → neutron star file", () => {
  it("is missing, downloads with progress when asked, then is ready", async () => {
    history.replaceState(null, "", "/?core=ready");
    render(NeutronFile);
    // Missing: nothing downloads until Download is pressed.
    expect(await screen.findByText(/Neutron star file: not downloaded\./)).toBeTruthy();
    expect(screen.getByText(/about 71 MB to download, 184 MB in astromech's data folder/)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Download" }));
    // Downloading: the progress so far.
    expect(await screen.findByText(/Downloading the neutron star file…/)).toBeTruthy();
    expect(await screen.findByText(/\d+ of 71 MB/)).toBeTruthy();
    expect(document.querySelector("progress")).toBeTruthy();
    expect(screen.queryByRole("button", { name: "Download" })).toBeNull();
    // Ready: its build date, and a check for a newer one.
    expect(await screen.findByText("Neutron star file: 5,045,154 neutron stars, built 22 Oct 2026.")).toBeTruthy();
    expect(document.querySelector("progress")).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: "Check for a newer file" }));
    expect(await screen.findByText("This is the latest file.")).toBeTruthy();
  });

  it("stays downloading while the download runs", async () => {
    history.replaceState(null, "", "/?core=ready&slow=never:neutron_download");
    render(NeutronFile);
    await fireEvent.click(await screen.findByRole("button", { name: "Download" }));
    expect(await screen.findByText(/Downloading the neutron star file…/)).toBeTruthy();
    expect(screen.queryByRole("button", { name: "Download" })).toBeNull();
  });

  it("says why a download failed and offers it again", async () => {
    history.replaceState(null, "", "/?core=ready&fail=neutron_download");
    render(NeutronFile);
    await fireEvent.click(await screen.findByRole("button", { name: "Download" }));
    expect(await screen.findByText(/neutron_download failed on purpose/)).toBeTruthy();
    expect(screen.getByRole("button", { name: "Download" })).toBeTruthy();
    expect(screen.getByText(/Neutron star file: not downloaded\./)).toBeTruthy();
  });

  it("shows under Worked out for a neutron route only, in the plot panel", async () => {
    startPreview("/?core=ready");
    render(Plot, { to: "Colonia" });
    await fireEvent.click(await screen.findByRole("button", { name: /^Worked out/ }));
    expect(await screen.findByText(/Neutron star file: not downloaded\./)).toBeTruthy();
    expect(screen.getByRole("option", { name: /On this PC/ }).getAttribute("aria-disabled"), "needs the file").toBe("true");
    await fireEvent.click(screen.getByRole("button", { name: /^Plotter/ }));
    await fireEvent.click(screen.getByRole("option", { name: /^Exact/ }));
    expect(screen.queryByRole("button", { name: /^Worked out/ }), "an exact route is always Spansh's").toBeNull();
    expect(screen.getByText("By Spansh")).toBeTruthy();
    expect(screen.queryByText(/Neutron star file/)).toBeNull();
  });
});
