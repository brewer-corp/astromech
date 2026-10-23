import { afterEach, describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { resetFakeCore } from "../../lib/fakecore";
import { resetPreviewNeutron } from "../../lib/neutron";
import NeutronFile from "./NeutronFile.svelte";
import Plot from "./Plot.svelte";

afterEach(() => {
  history.replaceState(null, "", "/");
  resetPreviewNeutron();
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

  it("shows only under the neutron plotter in the plot panel", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(Plot, { to: "Colonia" });
    expect(await screen.findByText(/Neutron star file: not downloaded\./)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Exact" }));
    expect(screen.queryByText(/Neutron star file/)).toBeNull();
  });
});
