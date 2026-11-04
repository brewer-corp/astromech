import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";

// A screen reader reads a live region again only when its text changes. These check that a
// second save, or the same error twice, empties the region before filling it again.
const callout = { id: "prospector", title: "Prospector", enabled: true, cooldown_s: 0, default_enabled: true, default_cooldown_s: 0 };
vi.mock("../../lib/callouts", async (orig) => ({
  ...(await orig<typeof import("../../lib/callouts")>()),
  loadCallouts: vi.fn(async () => [callout]),
  changeCallout: vi.fn(async () => {}),
}));
vi.mock("../../lib/prospector", async (orig) => ({
  ...(await orig<typeof import("../../lib/prospector")>()),
  loadThresholds: vi.fn(async () => [{ ore: "painite", name: "Painite", threshold: 20, default: 20 }]),
}));

import Callouts from "./Callouts.svelte";
import ProspectorThresholds from "./ProspectorThresholds.svelte";

/** Every text the live region shows from now on, in order, repeats in a row merged. */
function texts(region: Element): string[] {
  const seen: string[] = [];
  new MutationObserver(() => {
    const t = region.textContent ?? "";
    if (seen.at(-1) !== t) seen.push(t);
  }).observe(region, {
    childList: true,
    characterData: true,
    subtree: true,
  });
  return seen;
}

describe("Settings live regions", () => {
  it("announce a second Saved on the Callouts settings", async () => {
    const { container } = render(Callouts);
    const button = await screen.findByLabelText("Prospector on or off");
    const region = container.querySelector("[aria-live]")!;
    const seen = texts(region);
    const saved = "Saved. It applies the next time Elite starts.";
    await fireEvent.click(button);
    await waitFor(() => expect(region.textContent).toBe(saved));
    await fireEvent.click(button);
    await waitFor(() => expect(seen).toEqual([saved, "", saved]));
  });

  it("announce the same Callouts error twice", async () => {
    const { container } = render(Callouts);
    const box = await screen.findByLabelText("Prospector cooldown, seconds");
    const region = container.querySelector("[aria-live]")!;
    const seen = texts(region);
    const err = "Prospector: enter a number of seconds from 0 to 86400.";
    await fireEvent.change(box, { target: { value: "lots" } });
    await fireEvent.change(box, { target: { value: "lots" } });
    await waitFor(() => expect(seen).toEqual([err, "", err]));
    expect(region.classList.contains("warn-text")).toBe(true);
  });

  it("announce the same prospector threshold error twice", async () => {
    const { container } = render(ProspectorThresholds);
    const box = await screen.findByLabelText("Painite threshold, percent");
    const region = container.querySelector("[aria-live]")!;
    const seen = texts(region);
    const err = "Painite: enter a percentage from 0 to 100, or leave it empty to turn it off.";
    await fireEvent.change(box, { target: { value: "lots" } });
    await fireEvent.change(box, { target: { value: "lots" } });
    await waitFor(() => expect(seen).toEqual([err, "", err]));
  });
});
