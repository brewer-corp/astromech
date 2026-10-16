// Ships page cards. The fleet and card stats come from the fixture astromech-ship's
// dump-ships-fixture example writes (lib/fixtures/ships.json); the Krait MkII combat build's shield,
// DPS and maximum jump are what Coriolis shows (crates/astromech-ship/tests/coriolis.rs,
// krait_mkii_combat and card_headline_for_the_krait_mkii_combat_build).
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, render, screen } from "@testing-library/svelte";
import ships from "../../lib/fixtures/ships.json";
import type { Fleet, Headline } from "../../lib/engineering";
import View from "./View.svelte";

let headlines: Record<string, Headline> = {};

vi.mock("../../lib/engineering", async (importActual) => {
  const real = await importActual<typeof import("../../lib/engineering")>();
  return {
    ...real,
    loadFleet: async () => ships.fleet as unknown as Fleet,
    loadHeadlines: async () => headlines,
  };
});

/** Label → value pairs on one ship's card. */
async function card(name: string): Promise<Record<string, string>> {
  const c = (await screen.findByText(name)).closest("button")!;
  const keys = [...c.querySelectorAll(".k")];
  return Object.fromEntries(keys.map((k) => [k.textContent ?? "", k.nextElementSibling?.textContent?.trim() ?? ""]));
}

beforeEach(() => {
  location.hash = "#/engineering";
  headlines = structuredClone(ships.headlines) as unknown as Record<string, Headline>;
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
