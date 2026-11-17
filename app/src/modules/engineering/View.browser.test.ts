// Ships > Ships: the selected ship's head and hero values, its Loadout, and the fleet strip.
// Stats come from preview/fixtures/ships.json (the real core's, in the world worlds/preview/krait).
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import ships from "../../preview/fixtures/ships.json";
import type { Compared, Fleet, Headline } from "../../lib/engineering";
import { answer } from "../../test/transport";
import View from "./View.svelte";

let headlines: Record<string, Headline> = {};
let fleet: Fleet;
let headlinesFail: string | null = null;
let compared: Compared | null = null;

answer("engineering_fleet", () => fleet);
answer("engineering_headlines", () => {
  if (headlinesFail) throw new Error(headlinesFail);
  return headlines;
});
answer("engineering_stats", () => compared);

function emptyStats(): Compared {
  return structuredClone(ships.cases[0]!.stats) as unknown as Compared;
}

/** The hero value under a label, as shown with its unit. */
async function hero(label: string): Promise<string> {
  const l = await screen.findByText(label, { selector: ".hero > span" });
  return l.parentElement!.textContent!.replace(label, "").replace(/\s+/g, " ").trim();
}

beforeEach(() => {
  location.hash = "#/engineering";
  localStorage.clear();
  headlines = structuredClone(ships.headlines) as unknown as Record<string, Headline>;
  fleet = structuredClone(ships.fleet) as unknown as Fleet;
  headlinesFail = null;
  compared = emptyStats();
});
afterEach(cleanup);

it("shows jump range, shields, top speed and power used in the head", async () => {
  render(View);
  expect(await screen.findByRole("button", { name: "Loadout", pressed: true })).toBeTruthy();
  await screen.findByText("1428");
  expect(await hero("Jump range")).toBe("24.9ly");
  expect(await hero("Shields")).toBe("1428MJ");
  expect(await hero("Top speed")).toBe("376m/s");
  expect(await hero("Power used")).toBe("28.2of 44.1 MW");
});

it("lists the fitted modules in the game's outfitting groups, leaving out the fixed ones", async () => {
  render(View);
  const groups = await screen.findAllByRole("heading", { level: 3 });
  expect(groups.map((h) => h.textContent)).toEqual(["Hardpoints", "Utility", "Core", "Optional"]);
  const core = screen.getByRole("region", { name: "Core" });
  expect(within(core).getByRole("button", { name: "Frame shift drive: 5A Frame Shift Drive" })).toBeTruthy();
  expect(screen.queryByRole("button", { name: /Planetary/ })).toBeNull();
});

it("shows the plan on Plan, with the hull's handling in the stats", async () => {
  location.hash = "#/engineering/1/plan";
  render(View);
  const ship = await screen.findByRole("rowgroup", { name: "Handling (°/s)" });
  const row = (name: string) => within(ship).getByRole("rowheader", { name }).parentElement!.querySelector("td")!.textContent;
  expect([row("Pitch"), row("Roll"), row("Yaw")]).toEqual(["36", "90", "16"]);
  expect(screen.queryByRole("region", { name: "Core" })).toBeNull();
});

it("switches from Loadout to Plan", async () => {
  render(View);
  expect(await screen.findByRole("region", { name: "Core" })).toBeTruthy();
  await fireEvent.click(screen.getByRole("button", { name: "Plan", pressed: false }));
  expect(await screen.findByRole("region", { name: "Ship stats" })).toBeTruthy();
  expect(screen.getByRole("button", { name: "Plan", pressed: true })).toBeTruthy();
  expect(location.hash).toBe("#/engineering/1/plan");
});

it("opens the Loadout for the selected ship", async () => {
  location.hash = "#/engineering/1/plan";
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Loadout", pressed: false }));
  expect(location.hash).toBe("#/engineering/1");
});

it("says when the hull is not in the ship data", async () => {
  compared = { unknown_hull: "mediumtransport01" };
  render(View);
  const warn = await screen.findByText(/Incomplete: this hull is not in the ship data/);
  expect(document.getElementById(warn.getAttribute("aria-describedby")!)?.textContent).toBe("Hull: mediumtransport01");
});

it("marks a ship incomplete when the calculations leave something out or don't know the hull", async () => {
  const c = emptyStats();
  if ("current" in c) {
    c.current.unresolved = ["Hull: krait_mkii"];
    c.planned.unresolved = ["Hull: krait_mkii"];
  }
  compared = c;
  render(View);
  const warn = await screen.findByText(/Incomplete/);
  expect(document.getElementById(warn.getAttribute("aria-describedby")!)?.textContent).toBe("Hull: krait_mkii");
});

it("shows a stored ship's system as a chip that copies its name", async () => {
  const krait = fleet.ships[0]!;
  Object.assign(krait, { current: false, system: "Shinrarta Dezhra" });
  const writeText = vi.fn(async () => {});
  Object.defineProperty(navigator, "clipboard", { value: { writeText }, configurable: true });
  render(View);
  const head = (await screen.findByRole("heading", { name: "Synthetic_Krait" })).closest(".head") as HTMLElement;
  const copy = within(head).getByRole("button", { name: "Copy Shinrarta Dezhra" });
  expect(copy.closest(".chip")!.textContent).toContain("Shinrarta Dezhra");
  expect(copy.closest(".where")!.textContent).toMatch(/stored at\s+Shinrarta Dezhra/);
  await fireEvent.click(copy);
  expect(writeText).toHaveBeenCalledWith("Shinrarta Dezhra");
  const open = screen.getByRole("link", { name: "Synthetic_Krait, Krait MkII" });
  expect(open.getAttribute("href")).toBe("#/engineering/1");
});

it("says where a ship in transit is going, as a chip", async () => {
  Object.assign(fleet.ships[0]!, { current: false, in_transit: true, system: "Sol" });
  render(View);
  const head = (await screen.findByRole("heading", { name: "Synthetic_Krait" })).closest(".head") as HTMLElement;
  expect(head.querySelector(".where")!.textContent!.replace(/\s+/g, " ")).toMatch(/^in transit to Sol /);
  expect(within(head).getByRole("button", { name: "Copy Sol" })).toBeTruthy();
});

it("names each fleet strip link by ship name or ident and type", async () => {
  Object.assign(fleet.ships[0]!, { name: null, ident: "CU-01", type_name: "Imperial Cutter" });
  render(View);
  const link = await screen.findByRole("link", { name: "CU-01, Imperial Cutter" });
  expect(link.textContent).toContain("Unnamed");
});

it("says when the row stats can't be worked out", async () => {
  headlinesFail = "journal folder not found: C:\\nowhere";
  render(View);
  const note = await screen.findByText(
    (_, el) => el?.tagName === "P" && /^! Could not work out shield, DPS and jump range: .*journal folder not found/.test(el.textContent?.trim() ?? ""),
  );
  expect(note).toBeTruthy();
});

it("says a ship never flown while a journal was kept has unknown modules, and shows no loadout", async () => {
  Object.assign(fleet.ships[0]!, { loadout_ts: null });
  render(View);
  expect(await screen.findByText(/its modules are unknown/)).toBeTruthy();
  expect(screen.queryByRole("region", { name: "Core" })).toBeNull();
});
