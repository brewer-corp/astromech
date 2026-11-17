// Ships > Suits as a loadout (astromech-nnn0; the On foot page's blueprints until astromech-5a01).
// The view and the sample shopping list come from the fixture the real core dumps
// (preview/fixtures/odyssey.json, from the world worlds/preview/odyssey: its synthetic ShipLocker,
// Backpack and journal, with four loadouts and two grade-1 Mavericks); the numbers below are what
// the crate's tests check.
import { afterEach, beforeEach, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import sample from "../../preview/fixtures/odyssey.json";
import type { Pick, Shopping } from "../../lib/odyssey";
import { answer, previewAnswer } from "../../test/transport";
import View from "./View.svelte";

const MAV_A = 1700000000001; // worn, in Explore and Raid
const MAV_B = 1700000000005; // in Spare

// null: the real preview behaviour (the sample list for the sample picks).
let shopping: ((picks: Pick[]) => Promise<Shopping | null>) | null = null;
/** The picks each odyssey_shopping call sent to the backend. */
let sent: Pick[][] = [];

answer("odyssey_shopping", (args: { picks: Pick[] }) => {
  sent.push(args.picks);
  return shopping ? shopping(args.picks) : previewAnswer("odyssey_shopping", args);
});

beforeEach(() => {
  localStorage.clear();
  location.hash = "";
  shopping = null;
  sent = [];
});
afterEach(cleanup);

const saved = () => JSON.parse(localStorage.getItem("astromech.odyssey.picks") ?? "[]") as Pick[];
const panel = () => screen.getByRole("complementary", { name: "Details" });
const row = (name: string) => screen.getByRole("button", { name });
const cells = (t: HTMLElement) => [...t.querySelectorAll("[role=row]:not(.heads)")].map((r) => [...r.querySelectorAll("[role=cell]")].map((c) => c.textContent));

it("shows what each item's picks need in the panel and one shopping list", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Suit: Maverick Suit" }));
  const p = panel();
  expect(within(p).getByText("2,850,000")).toBeTruthy();
  const up = within(p).getByRole("table", { name: "What Maverick Suit grade 1 → 3 needs" });
  expect(cells(up).slice(0, 2)).toEqual([
    ["Carbon Fibre Plating", "7", "!0"],
    ["Health Monitor", "3", "!1"],
  ]);
  const fitters = within(p).getByRole("list", { name: "Engineers who fit Night Vision" });
  expect(fitters.textContent).toContain("Oden Geiger");
  expect(fitters.textContent).toContain("Yi Shen");
  // The list adds Graphene from both upgrades: 7 for the suit, none for the P-15.
  const all = [...screen.getAllByRole("table", { name: /^Shopping list/ })].flatMap(cells);
  expect(all.find((c) => c[0] === "Graphene")).toEqual(["Graphene", "7", "9"]);
  expect(screen.getByText(/short assets are worth 77 barter/)).toBeTruthy();
});

it("removes and adds picks, and keeps them", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Suit: Maverick Suit" }));
  await fireEvent.click(within(panel()).getByRole("button", { name: "Remove Maverick Suit grade 1 → 3" }));
  expect(saved()).toHaveLength(2);
  await fireEvent.click(screen.getByRole("button", { name: "Plan other gear" }));
  await fireEvent.change(screen.getByLabelText("Blueprint type"), { target: { value: "mod" } });
  const add = within(panel()).getByRole("button", { name: "Add to plan" }) as HTMLButtonElement;
  expect(add.disabled).toBe(true);
  await fireEvent.change(screen.getByLabelText("Modification"), { target: { value: "headshotdamage" } });
  // A weapon mod's cost depends on the maker, so it needs one.
  expect(add.disabled).toBe(true);
  await fireEvent.change(screen.getByLabelText("Weapon maker"), { target: { value: "manticore" } });
  await fireEvent.click(add);
  expect(saved().at(-1)).toEqual({ kind: "mod", id: "headshotdamage", maker: "manticore" });
  expect(saved()).toHaveLength(3);
  // Adding the same pick twice does nothing.
  await fireEvent.click(add);
  expect(saved()).toHaveLength(3);
});

it("keeps every saved pick removable when the list can't be worked out", async () => {
  const stale: Pick = { kind: "mod", id: "renamedmod", maker: null };
  localStorage.setItem("astromech.odyssey.picks", JSON.stringify([{ kind: "upgrade", gear: "maverick", from: 1, to: 3 }, stale]));
  shopping = async () => {
    throw new Error("journal folder not found");
  };
  render(View);
  expect(await screen.findByText(/Could not work out the shopping list: journal folder not found\./)).toBeTruthy();
  expect(screen.getByRole("button", { name: "Remove Maverick Suit grade 1 → 3" })).toBeTruthy();
  await fireEvent.click(screen.getByRole("button", { name: "Remove renamedmod" }));
  expect(saved()).toEqual([{ kind: "upgrade", gear: "maverick", from: 1, to: 3 }]);
});

it("shows a pick the game data no longer has on its own line, and the rest of the list", async () => {
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
  const bad = await screen.findByRole("listitem", { name: "oldsuit grade 1 → 2" });
  expect(bad.textContent).toContain("unknown suit or weapon `oldsuit`. Remove it and pick again.");
  await fireEvent.click(row("Suit: Maverick Suit"));
  expect(cells(within(panel()).getByRole("table", { name: "What Maverick Suit grade 1 → 3 needs" }))).toHaveLength(5);
  expect(screen.getAllByRole("table", { name: /^Shopping list/ }).length).toBeGreaterThan(0);
  await fireEvent.click(within(bad).getByRole("button", { name: "Remove oldsuit grade 1 → 2" }));
  expect(saved()).toEqual([good]);
});

it("starts an owned suit's upgrade at a grade it has and leaves out mods already fitted", async () => {
  render(View);
  // From the journal: a Maverick at G3 is in no loadout, and is listed under the strip.
  expect((await screen.findByText(/Also owned:/)).textContent).toContain("Maverick Suit G3");
  await fireEvent.click(screen.getByRole("button", { name: "Plan other gear" }));
  await fireEvent.change(screen.getByLabelText("For"), { target: { value: "" } });
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
  const mods = opts("Modification");
  expect(mods).not.toContain("Stability");
  expect(mods).toContain("Night Vision");
  await fireEvent.change(screen.getByLabelText("Modification"), { target: { value: "improvedhipfireaccuracy" } });
  const makers = opts("Weapon maker");
  expect(makers).not.toContain("Takada");
  expect(makers).toContain("Kinematic Armaments");
});

it("suggests what surplus to trade at a bartender and what is still short after", async () => {
  render(View);
  const swaps = await screen.findByRole("list", { name: "Bartender trade" });
  const swap = swaps.querySelector("li")!;
  expect(swap.textContent).toContain("Trade 2 Graphene, 1 Weapon Component, 1 Tungsten Carbide, 1 Circuit Switch for 7 Carbon Fibre Plating");
  expect(swap.textContent).toContain("79 barter given for 77, 2 over");
  // The goods and data the bartender can't cover are still short; the plating is not.
  const left = screen.getByText(/Still short after the trade:/).textContent!;
  expect(left).toContain("2 Health Monitor");
  expect(left).not.toContain("Carbon Fibre Plating");
});

it("opens on the worn loadout and fills the rows from the loadout picked", async () => {
  render(View);
  expect((await screen.findByRole("button", { name: "Suit: Maverick Suit" })).textContent).toContain("Grade 3 · Night Vision");
  expect(row("Primary: Karma AR-50")).toBeTruthy();
  await fireEvent.click(screen.getByRole("button", { name: "Loadout Spare" }));
  expect(location.hash).toBe("#/suits?loadout=4293000004");
  // Spare's Maverick is the other grade-1 one, with no plan; its only weapon is the P-15.
  const suit = await screen.findByRole("button", { name: "Suit: Maverick Suit" });
  expect(suit.textContent).not.toContain("Grade 3");
  expect(row("Secondary: Karma P-15").textContent).toContain("Grade 2");
  expect(screen.queryByRole("button", { name: /^Primary:/ })).toBeNull();
});

it("adds a plan to the item selected, tied to it, and leaves the other of its kind alone", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Loadout Spare" }));
  await fireEvent.click(await screen.findByRole("button", { name: "Suit: Maverick Suit" }));
  const p = panel();
  expect((within(p).getByLabelText("Add to plan for Maverick Suit") as HTMLSelectElement).value).toBe("g2");
  await fireEvent.click(within(p).getByRole("button", { name: "Add to plan" }));
  expect(sent.at(-1)).toContainEqual({ kind: "upgrade", gear: "maverick", from: 1, to: 2, item: MAV_B });
  expect(row("Suit: Maverick Suit").textContent).toContain("Grade 2");
  await fireEvent.click(screen.getByRole("button", { name: "Loadout Explore" }));
  const first = await screen.findByRole("button", { name: "Suit: Maverick Suit" });
  expect(first.textContent).toContain("Grade 3 · Night Vision");
  expect(first.textContent).not.toContain("Grade 2");
});

it("ties a pick saved before picks had an item instead of adding a second", async () => {
  const old: Pick = { kind: "upgrade", gear: "maverick", from: 1, to: 3 };
  localStorage.setItem("astromech.odyssey.picks", JSON.stringify([old]));
  const list = sample.shopping as unknown as Shopping;
  shopping = async (picks) => ({ ...list, picks: picks.map((pick) => ({ ...list.picks[0]!, pick })) });
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Suit: Maverick Suit" }));
  await fireEvent.change(within(panel()).getByLabelText("Add to plan for Maverick Suit"), { target: { value: "g3" } });
  await fireEvent.click(within(panel()).getByRole("button", { name: "Add to plan" }));
  expect(saved()).toEqual([{ ...old, item: MAV_A }]);
  expect(sent.at(-1)).toEqual([{ ...old, item: MAV_A }]);
  expect(await within(panel()).findByText("2,850,000")).toBeTruthy();
  expect(row("Suit: Maverick Suit").textContent).toContain("Grade 3");
  expect(screen.queryByText("Not tied to an item")).toBeNull();
  await fireEvent.click(screen.getByRole("button", { name: "Loadout Spare" }));
  expect((await screen.findByRole("button", { name: "Suit: Maverick Suit" })).textContent).not.toContain("Grade 3");
});
