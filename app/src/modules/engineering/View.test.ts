// Ships page cards. The fleet and card stats come from the fixture astromech-ship's
// dump-ships-fixture example writes (lib/fixtures/ships.json); the Krait MkII combat build's shield,
// DPS and maximum jump are what Coriolis shows (crates/astromech-ship/tests/coriolis.rs,
// krait_mkii_combat and card_headline_for_the_krait_mkii_combat_build).
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import ships from "../../lib/fixtures/ships.json";
import type { Fleet, Headline } from "../../lib/engineering";
import View from "./View.svelte";

let headlines: Record<string, Headline> = {};
let fleet: Fleet;
let headlinesFail: string | null = null;

vi.mock("../../lib/engineering", async (importActual) => {
  const real = await importActual<typeof import("../../lib/engineering")>();
  return {
    ...real,
    loadFleet: async () => fleet,
    loadHeadlines: async () => {
      if (headlinesFail) throw new Error(headlinesFail);
      return headlines;
    },
  };
});

/** Label → value pairs on one ship's card. */
async function card(name: string): Promise<Record<string, string>> {
  const c = (await screen.findByText(name)).closest(".card")!;
  const keys = [...c.querySelectorAll(".k")];
  return Object.fromEntries(keys.map((k) => [k.textContent ?? "", k.nextElementSibling?.textContent?.trim() ?? ""]));
}

beforeEach(() => {
  location.hash = "#/engineering";
  headlines = structuredClone(ships.headlines) as unknown as Record<string, Headline>;
  fleet = structuredClone(ships.fleet) as unknown as Fleet;
  headlinesFail = null;
});
afterEach(cleanup);

it("shows shield, DPS and jump range from the ship calculations", async () => {
  render(View);
  const c = await card("Synthetic_Krait");
  expect(c["Shield"]).toBe("1428 MJ");
  expect(c["DPS"]).toBe("107.2");
  expect(c["Jump"]).toBe("24.9 ly");
  expect(c["Stats"]).toBeUndefined();
});

it("marks a ship incomplete when the calculations leave something out or don't know the hull", async () => {
  headlines["1"] = { jump: null, shield: null, dps: null, unresolved: ["Hull: krait_mkii"] };
  render(View);
  const c = await card("Synthetic_Krait");
  expect(c["Stats"]).toBe("! Incomplete");
  expect(c["Shield"]).toBe("–");
  expect(c["DPS"]).toBe("–");
  expect(screen.getByText("Incomplete").closest("[title]")!.getAttribute("title")).toBe("Hull: krait_mkii");
});

it("shows a stored ship's system as a chip that copies its name, and opens the ship from its name", async () => {
  const krait = fleet.ships[0]!;
  Object.assign(krait, { current: false, system: "Shinrarta Dezhra" });
  const writeText = vi.fn(async () => {});
  Object.defineProperty(navigator, "clipboard", { value: { writeText }, configurable: true });
  render(View);
  const c = (await screen.findByText("Synthetic_Krait")).closest(".card") as HTMLElement;
  const copy = within(c).getByRole("button", { name: "Copy Shinrarta Dezhra" });
  expect(copy.closest(".chip")!.textContent).toContain("Shinrarta Dezhra");
  await fireEvent.click(copy);
  expect(writeText).toHaveBeenCalledWith("Shinrarta Dezhra");
  expect(location.hash).toBe("#/engineering");
  const open = within(c).getByRole("link", { name: "Synthetic_Krait, Krait MkII" });
  expect(open.getAttribute("href")).toBe("#/engineering/1");
});

it("says where a ship in transit is going, as a chip", async () => {
  Object.assign(fleet.ships[0]!, { current: false, in_transit: true, system: "Sol" });
  render(View);
  const c = (await screen.findByText("Synthetic_Krait")).closest(".card") as HTMLElement;
  const where = c.querySelector(".where")!;
  expect(where.textContent!.replace(/\s+/g, " ").trim()).toBe("In transit to Sol Copy");
  // The location wraps and its name may be cut short on a narrow card: the full name is on hover.
  expect(where.getAttribute("title")).toBe("Sol");
  expect(within(c).getByRole("button", { name: "Copy Sol" })).toBeTruthy();
});

it("names each card's link by ship name or ident and type", async () => {
  Object.assign(fleet.ships[0]!, { name: null, ident: "CU-01", type_name: "Imperial Cutter" });
  render(View);
  const link = await screen.findByRole("link", { name: "CU-01, Imperial Cutter" });
  expect(link.textContent).toBe("Unnamed");
});

it("says when the card stats can't be worked out", async () => {
  headlinesFail = "journal folder not found: C:\\nowhere";
  render(View);
  const note = await screen.findByText(
    (_, el) => el?.tagName === "P" && /^! Shield, DPS and calculated jump range are unavailable: journal folder not found/.test(el.textContent?.trim() ?? ""),
  );
  expect(note).toBeTruthy();
  expect((await card("Synthetic_Krait"))["Shield"]).toBe("–");
});
