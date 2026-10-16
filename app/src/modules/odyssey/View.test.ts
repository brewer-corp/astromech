// On foot page. The view and the sample shopping list come from the fixture astromech-odyssey's
// dump-odyssey-fixture example writes (lib/fixtures/odyssey.json) from its synthetic ShipLocker,
// Backpack and journal; the numbers below are what the crate's tests check.
import { afterEach, beforeEach, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import View from "./View.svelte";

beforeEach(() => localStorage.clear());
afterEach(cleanup);

function row(name: string): string[] {
  const tr = screen.getByText(name).closest("tr")!;
  return [...tr.querySelectorAll("td")].map((td) => td.textContent?.trim() ?? "");
}

it("lists the ShipLocker and Backpack by category with bartender values", async () => {
  render(View);
  await screen.findByText("Suit Schematic");
  expect(row("Suit Schematic")).toEqual(["Suit Schematic", "3", "", "", "", "50,000 CR"]);
  await fireEvent.click(screen.getByRole("button", { name: "Assets" }));
  expect(row("Graphene")).toEqual(["Graphene", "7", "2", "", "23", "1,300 CR"]);
  // Held only by default; unticking shows the rest of the catalogue.
  expect(screen.queryByText("Aerogel")).toBeNull();
  await fireEvent.click(screen.getByLabelText("Held only"));
  expect(screen.getByText("Aerogel")).toBeTruthy();
  // Mission items have their own column.
  await fireEvent.click(screen.getByRole("button", { name: "Goods" }));
  expect(row("Health Monitor").slice(0, 4)).toEqual(["Health Monitor", "1", "", "2"]);
});

it("shows what each picked blueprint is missing and one shopping list", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Blueprints" }));
  const card = (await screen.findByText("Maverick Suit grade 1 → 3")).closest("article") as HTMLElement;
  const missing = [...card.querySelectorAll(".missing li")].map((li) =>
    [...li.querySelectorAll("span")].map((s) => s.textContent).join(" "),
  );
  expect(missing).toEqual(["Carbon Fibre Plating 0 / 7 −7", "Health Monitor 1 / 3 −2"]);
  expect(within(card).getByText("2,850,000 CR")).toBeTruthy();
  const nv = screen.getByText("Night Vision (suit)").closest("article") as HTMLElement;
  expect(nv.textContent).toContain("Fitted by: Oden Geiger, Yi Shen");
  expect(within(nv).getByText("Oden Geiger").classList.contains("fit-on")).toBe(true);
  // The list adds Graphene from both upgrades: 7 for the suit, none for the P-15.
  const list = screen.getByText("Shopping list").nextElementSibling!.nextElementSibling as HTMLElement;
  const g = [...list.querySelectorAll("tbody tr")].find((tr) => tr.querySelector("td")!.textContent!.startsWith("Graphene"))!;
  expect([...g.querySelectorAll("td")].slice(1, 4).map((td) => td.textContent)).toEqual(["7", "9", ""]);
  expect(screen.getByText(/short assets are worth 77 barter/)).toBeTruthy();
});

it("removes and adds picks, and keeps them", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Blueprints" }));
  await screen.findByText("Maverick Suit grade 1 → 3");
  await fireEvent.click(screen.getByRole("button", { name: "Remove Maverick Suit grade 1 → 3" }));
  expect(JSON.parse(localStorage.getItem("astromech.odyssey.picks")!)).toHaveLength(2);
  await fireEvent.change(screen.getByLabelText("Blueprint type"), { target: { value: "mod" } });
  const add = screen.getByRole("button", { name: "Add" });
  expect((add as HTMLButtonElement).disabled).toBe(true);
  await fireEvent.change(screen.getByLabelText("Modification"), { target: { value: "headshotdamage" } });
  // A weapon mod's cost depends on the maker, so it needs one.
  expect((add as HTMLButtonElement).disabled).toBe(true);
  await fireEvent.change(screen.getByLabelText("Weapon maker"), { target: { value: "takada" } });
  await fireEvent.click(add);
  const saved = JSON.parse(localStorage.getItem("astromech.odyssey.picks")!) as unknown[];
  expect(saved.at(-1)).toEqual({ kind: "mod", id: "headshotdamage", maker: "takada" });
  expect(saved).toHaveLength(3);
  // Adding the same pick twice does nothing.
  await fireEvent.click(add);
  expect(JSON.parse(localStorage.getItem("astromech.odyssey.picks")!)).toHaveLength(3);
});

it("shows engineer unlocks, the next step and the referral chain", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Engineers" }));
  const dg = (await screen.findByText("Domino Green")).closest("article") as HTMLElement;
  expect(within(dg).getByText("Unlocked")).toBeTruthy();
  expect(within(dg).getByText("Referral to Kit Fowler: Provide 5 Push")).toBeTruthy();
  expect(within(dg).getByRole("button", { name: "Copy Orishis" })).toBeTruthy();
  const kit = screen.getByText("Kit Fowler").closest("article") as HTMLElement;
  expect(within(kit).getByText("Not yet known")).toBeTruthy();
  expect(within(kit).getByText("Needs a referral from Domino Green")).toBeTruthy();
  const hero = screen.getByText("Hero Ferrari").closest("article") as HTMLElement;
  expect(within(hero).getByText("Visit Siris to unlock")).toBeTruthy();
  expect(hero.textContent).toContain("Increased Sprint Duration (?)");
});
