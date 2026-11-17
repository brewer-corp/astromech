// Which saved picks belong to which owned item, on the recorded odyssey world's suits and weapons
// (preview/fixtures/odyssey.json: two grade-1 Mavericks, one in Explore and Raid, one in Spare).
import { describe, expect, it } from "vitest";
import sample from "../preview/fixtures/odyssey.json";
import type { ModRef, OwnedGear, Pick } from "./odyssey";
import { addPick, fits, inNoLoadout, planFor, slotRows, tie, untied, type SuitLoadout } from "./suits";

const owned = sample.view.owned as OwnedGear[];
const mods = sample.view.mods as ModRef[];
const loadouts = sample.view.loadouts as SuitLoadout[];
const byId = (id: number) => owned.find((o) => o.id === id)!;
const MAV_A = 1700000000001; // Explore and Raid
const MAV_B = 1700000000005; // Spare
const mav13 = (item?: number): Pick => ({ kind: "upgrade", gear: "maverick", from: 1, to: 3, ...(item ? { item } : {}) });
const nv = (item?: number): Pick => ({ kind: "mod", id: "nightvision", maker: null, ...(item ? { item } : {}) });

describe("a pick shows on the item it is tied to, and only there", () => {
  const cases: { name: string; picks: Pick[]; item: number; plan: Pick[] }[] = [
    { name: "two items of one recipe, a pick for one: the one", picks: [mav13(MAV_A)], item: MAV_A, plan: [mav13(MAV_A)] },
    { name: "two items of one recipe, a pick for one: not the other", picks: [mav13(MAV_A)], item: MAV_B, plan: [] },
    { name: "a pick with no item: on no item", picks: [mav13()], item: MAV_A, plan: [] },
    { name: "a mod pick that fits but is tied elsewhere", picks: [nv(MAV_A)], item: MAV_B, plan: [] },
  ];
  for (const c of cases) it(c.name, () => expect(planFor(c.picks, byId(c.item))).toEqual(c.plan));

  it("an item in two loadouts has its plan in each", () => {
    const rows = loadouts.filter((l) => ["Explore", "Raid"].includes(l.name)).map((l) => slotRows(l)[0]!.item!);
    expect(rows.map((i) => planFor([mav13(MAV_A)], i))).toEqual([[mav13(MAV_A)], [mav13(MAV_A)]]);
  });

  it("lists a pick with no item, or for an item no longer owned, as tied to nothing", () => {
    const sold: Pick = { ...nv(), item: 42 };
    expect(untied([mav13(MAV_A), mav13(), sold], owned)).toEqual([mav13(), sold]);
  });

  it("offers a pick with no item only the owned items it fits", () => {
    expect(fits(mav13(), owned, mods).map((o) => o.id)).toEqual([MAV_A, MAV_B]);
    // The G3 Maverick already has Night Vision.
    expect(fits(nv(), owned, mods).map((o) => o.id)).toEqual([MAV_A, MAV_B, 1700000000002]);
  });
});

describe("adding never doubles a plan", () => {
  const cases: { name: string; picks: Pick[]; add: Pick; then: Pick[] }[] = [
    { name: "a matching pick with no item is tied, nothing added", picks: [mav13()], add: mav13(MAV_A), then: [mav13(MAV_A)] },
    { name: "the same recipe for a second item is a second pick", picks: [mav13(MAV_A)], add: mav13(MAV_B), then: [mav13(MAV_A), mav13(MAV_B)] },
    { name: "a recipe saved with no item adds nothing for no item", picks: [mav13()], add: mav13(), then: [mav13()] },
    { name: "the same pick again adds nothing", picks: [nv(MAV_A)], add: nv(MAV_A), then: [nv(MAV_A)] },
    {
      name: "a higher grade for an item raises its upgrade",
      picks: [mav13(MAV_A), nv(MAV_A)],
      add: { kind: "upgrade", gear: "maverick", from: 1, to: 4, item: MAV_A },
      then: [{ kind: "upgrade", gear: "maverick", from: 1, to: 4, item: MAV_A }, nv(MAV_A)],
    },
  ];
  for (const c of cases) it(c.name, () => expect(addPick(c.picks, c.add)).toEqual(c.then));

  it("ties a pick from the list of picks with no item", () => {
    expect(tie([mav13(), nv(MAV_A)], mav13(), MAV_B)).toEqual([nv(MAV_A), mav13(MAV_B)]);
  });
});

it("keeps owned gear in no loadout apart", () => {
  expect(inNoLoadout(owned, loadouts).map((o) => o.id)).toEqual([1700000000004]);
});
