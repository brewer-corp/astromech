// On foot page. The view and the sample shopping list come from the fixture astromech-odyssey's
// dump-odyssey-fixture example writes (lib/fixtures/odyssey.json) from its synthetic ShipLocker,
// Backpack and journal; the numbers below are what the crate's tests check.
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import sample from "../../lib/fixtures/odyssey.json";
import type { Pick, Shopping } from "../../lib/odyssey";
import View from "./View.svelte";

// null: the real preview behaviour (the sample list for the sample picks).
let shopping: ((picks: Pick[]) => Promise<Shopping | null>) | null = null;

vi.mock("../../lib/odyssey", async (importActual) => {
  const real = await importActual<typeof import("../../lib/odyssey")>();
  return { ...real, loadShopping: (picks: Pick[]) => (shopping ? shopping(picks) : real.loadShopping(picks)) };
});

beforeEach(() => {
  localStorage.clear();
  shopping = null;
});
afterEach(cleanup);

const saved = () => JSON.parse(localStorage.getItem("astromech.odyssey.picks") ?? "[]") as Pick[];

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
  await fireEvent.change(screen.getByLabelText("Weapon maker"), { target: { value: "manticore" } });
  await fireEvent.click(add);
  const saved = JSON.parse(localStorage.getItem("astromech.odyssey.picks")!) as unknown[];
  expect(saved.at(-1)).toEqual({ kind: "mod", id: "headshotdamage", maker: "manticore" });
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

it("keeps every saved pick removable when the list can't be worked out", async () => {
  const stale: Pick = { kind: "mod", id: "renamedmod", maker: null };
  localStorage.setItem("astromech.odyssey.picks", JSON.stringify([{ kind: "upgrade", gear: "maverick", from: 1, to: 3 }, stale]));
  shopping = async () => {
    throw new Error("journal folder not found");
  };
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Blueprints" }));
  expect(await screen.findByText(/Could not work out the shopping list: journal folder not found\./)).toBeTruthy();
  expect(screen.getByText("Maverick Suit grade 1 → 3")).toBeTruthy();
  await fireEvent.click(screen.getByRole("button", { name: "Remove renamedmod" }));
  expect(saved()).toEqual([{ kind: "upgrade", gear: "maverick", from: 1, to: 3 }]);
});

it("shows a pick the game data no longer has on its own card, and the rest of the list", async () => {
  const good = sample.picks[0] as Pick;
  const stale: Pick = { kind: "upgrade", gear: "oldsuit", from: 1, to: 2 };
  localStorage.setItem("astromech.odyssey.picks", JSON.stringify([good, stale]));
  const list = sample.shopping as unknown as Shopping;
  shopping = async (picks) => ({
    ...list,
    picks: [
      { ...list.picks[0]!, pick: picks[0]!, error: null },
      { pick: picks[1]!, title: "oldsuit grade 1 → 2", lines: [], credits: 0, credits_partial: false, fitted_by: [], ready: false, error: "unknown suit or weapon `oldsuit`" },
    ],
  });
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Blueprints" }));
  const bad = (await screen.findByText("oldsuit grade 1 → 2")).closest("article") as HTMLElement;
  expect(bad.textContent).toContain("unknown suit or weapon `oldsuit`. Remove it and pick again.");
  const ok = screen.getByText("Maverick Suit grade 1 → 3").closest("article") as HTMLElement;
  expect(ok.querySelectorAll(".missing li")).toHaveLength(2);
  expect(screen.getByText("Shopping list")).toBeTruthy();
  await fireEvent.click(within(bad).getByRole("button", { name: "Remove oldsuit grade 1 → 2" }));
  expect(saved()).toEqual([good]);
});

it("starts an owned suit's upgrade at a grade it has and leaves out mods already fitted", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Blueprints" }));
  // From the journal: a Maverick at G1 and another at G3, and what is fitted to them.
  expect((await screen.findByText(/Owned \(from the journal\)/)).textContent).toContain("Maverick Suit G3");
  const gear = screen.getByLabelText("Suit or weapon") as HTMLSelectElement;
  await fireEvent.change(gear, { target: { value: "maverick" } });
  const opts = (label: string) => [...(screen.getByLabelText(label) as HTMLSelectElement).options].map((o) => o.textContent);
  expect(opts("From grade")).toEqual(["G1", "G3"]);
  expect(opts("To grade")).toEqual(["G2", "G3", "G4", "G5"]);
  await fireEvent.change(screen.getByLabelText("From grade"), { target: { value: "3" } });
  expect(opts("To grade")).toEqual(["G4", "G5"]);
  // A suit the commander doesn't own is still a free choice.
  await fireEvent.change(gear, { target: { value: "artemis" } });
  expect(opts("From grade")).toEqual(["G1", "G2", "G3", "G4"]);
  // Stability is on every owned weapon; Takada's Improved Hip Fire Accuracy is on the Aphelion.
  await fireEvent.change(screen.getByLabelText("Blueprint type"), { target: { value: "mod" } });
  const mods = [...(screen.getByLabelText("Modification") as HTMLSelectElement).options].map((o) => o.textContent);
  expect(mods).not.toContain("Stability");
  expect(mods).toContain("Night Vision");
  await fireEvent.change(screen.getByLabelText("Modification"), { target: { value: "improvedhipfireaccuracy" } });
  const makers = [...(screen.getByLabelText("Weapon maker") as HTMLSelectElement).options].map((o) => o.textContent);
  expect(makers).not.toContain("Takada");
  expect(makers).toContain("Kinematic Armaments");
});

it("suggests what surplus to trade at a bartender and what is still short after", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Blueprints" }));
  const h = await screen.findByText("Bartender trade");
  const swap = h.nextElementSibling!.querySelector("li")!;
  expect(swap.textContent).toContain("Trade 2 Graphene, 1 Weapon Component, 1 Tungsten Carbide, 1 Circuit Switch for 7 Carbon Fibre Plating");
  expect(swap.textContent).toContain("79 barter given for 77, 2 over");
  // The goods and data the bartender can't cover are still short; the plating is not.
  const left = screen.getByText(/Still short after the trade:/).textContent!;
  expect(left).toContain("2 Health Monitor");
  expect(left).not.toContain("Carbon Fibre Plating");
});
