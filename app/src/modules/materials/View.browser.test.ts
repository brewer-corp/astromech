// Materials' On foot kind: the ShipLocker and Backpack, moved whole from the On foot page
// (astromech-5a01). The view comes from the fixture the real core dumps (preview/fixtures/
// odyssey.json, from the world worlds/preview/odyssey); the numbers are what the crate's tests check.
import { afterEach, beforeEach, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import { answer, previewAnswer } from "../../test/transport";
import View from "./View.svelte";

/** null: the preview's own answer. */
let onFoot: (() => Promise<unknown>) | null = null;
answer("odyssey_view", () => (onFoot ? onFoot() : previewAnswer("odyssey_view")));

beforeEach(() => {
  onFoot = null;
  location.hash = "#/materials?kind=onfoot";
});
afterEach(() => {
  cleanup();
  location.hash = "";
});

const tile = (name: string) => screen.getByRole("button", { name: new RegExp(`^${name}, `) });

it("shows what is held on foot by the game's categories, and one item's lines in the panel", async () => {
  render(View);
  await screen.findByRole("button", { name: /^Graphene, / });
  // Held is the locker and the backpack: 7 + 2.
  expect(tile("Graphene").getAttribute("aria-label")).toBe("Graphene, 9 held");
  expect([...document.querySelectorAll("h3.section")].map((h) => h.textContent)).toEqual(["Goods", "Assets", "Data", "Consumables"]);
  // Mission items are marked on their tile: they go back when the mission ends.
  expect(tile("Health Monitor").textContent).toContain("2 mission");

  await fireEvent.click(tile("Graphene"));
  expect(location.hash).toBe("#/materials/onfoot/graphene");
  const panel = (await screen.findByRole("heading", { name: "Graphene" })).parentElement as HTMLElement;
  const lines = [...panel.querySelectorAll("dt")].map((dt) => `${dt.textContent} ${dt.nextElementSibling?.textContent}`);
  expect(lines).toEqual(["Ship locker 7", "Backpack 2", "Mission 0", "Barter 23", "A bartender pays 1,300 CR"]);
  expect(within(panel).getByText("On foot · Assets")).toBeTruthy();
});

it("shows held items only until All items is chosen", async () => {
  render(View);
  await screen.findByRole("button", { name: /^Graphene, / });
  expect(screen.getByRole("button", { name: "Held only" }).getAttribute("aria-pressed")).toBe("true");
  expect(screen.queryByRole("button", { name: /^Aerogel, / })).toBeNull();
  await fireEvent.click(screen.getByRole("button", { name: "All items" }));
  expect(tile("Aerogel").getAttribute("aria-label")).toBe("Aerogel, 0 held");
});

it("shows no ship materials under On foot while the ShipLocker is unread or its read failed", async () => {
  // Pending: the ship materials have answered, the ShipLocker has not.
  onFoot = () => new Promise(() => {});
  render(View);
  await screen.findByText(/Reading the ShipLocker/);
  await waitFor(() => expect(screen.getByRole("button", { name: "Raw" })).toBeTruthy());
  expect(screen.queryAllByRole("button", { name: /, \d+ of \d+$/ })).toEqual([]);
  cleanup();

  onFoot = async () => {
    throw new Error("ShipLocker.json is locked");
  };
  render(View);
  await screen.findByText(/Could not read the inventory: ShipLocker.json is locked/);
  expect(screen.getByRole("button", { name: "On foot", pressed: true })).toBeTruthy();
  expect(screen.queryAllByRole("button", { name: /, \d+ of \d+$/ })).toEqual([]);
});
