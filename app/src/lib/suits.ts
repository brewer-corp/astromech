/**
 * Ships > Suits as a loadout (astromech-nnn0): which saved picks belong to which owned suit or
 * weapon, and how adding a pick ties or extends one instead of doubling a plan. A pick belongs to
 * an item only through `Pick.item`, the item's journal id (astromech-w1lj); a pick with none, or
 * for an item no longer owned, is tied to nothing and shows on no row.
 */
import type { ModRef, OwnedGear, Pick } from "./odyssey";
import { pickKey } from "./odyssey";
import type { SuitLoadout } from "./api/types/SuitLoadout";

export type { SuitLoadout } from "./api/types/SuitLoadout";

/** One row of a loadout: its slot and the item in it, or none. */
export type SlotRow = { slot: "Suit" | "Primary" | "Secondary"; key: string; item: OwnedGear | null };

/** The journal's weapon slots, in the order a loadout lists them. */
const WEAPON_SLOTS: [string, SlotRow["slot"]][] = [
  ["PrimaryWeapon1", "Primary"],
  ["PrimaryWeapon2", "Primary"],
  ["SecondaryWeapon", "Secondary"],
];

/** A loadout's four rows, Suit, Primary, Primary, Secondary, an empty slot with no item. */
export function slotRows(l: SuitLoadout): SlotRow[] {
  return [
    { slot: "Suit", key: "Suit", item: l.suit },
    ...WEAPON_SLOTS.map(([name, slot]) => ({ slot, key: name, item: l.slots.find((s) => s.slot === name)?.item ?? null })),
  ];
}

/** A pick's recipe, without the item it is for. */
export const recipeKey = (p: Pick) => pickKey({ ...p, item: undefined });

/** The picks tied to `item`, and only those. */
export function planFor(picks: Pick[], item: OwnedGear): Pick[] {
  return picks.filter((p) => p.item === item.id);
}

/** The picks tied to no owned item: saved before picks could be, or for an item since sold. */
export function untied(picks: Pick[], owned: OwnedGear[]): Pick[] {
  return picks.filter((p) => p.item == null || !owned.some((o) => o.id === p.item));
}

/** The grade `item` is planned to reach: its upgrade pick's, else its own. */
export function plannedGrade(picks: Pick[], item: OwnedGear): number {
  return Math.max(item.grade, ...planFor(picks, item).map((p) => (p.kind === "upgrade" ? p.to : 0)));
}

/** The owned items `p` could be tied to: an upgrade's gear at its from grade, a mod's kind (and maker) not yet fitted. */
export function fits(p: Pick, owned: OwnedGear[], mods: ModRef[]): OwnedGear[] {
  if (p.kind === "upgrade") return owned.filter((o) => o.gear === p.gear && o.grade === p.from);
  const m = mods.find((x) => x.id === p.id);
  if (!m) return [];
  return owned.filter((o) => o.kind === m.kind && !o.mods.includes(m.id) && (!p.maker || o.maker === p.maker));
}

/** The modifications `item` can still take: its kind, not fitted, not already planned for it. */
export function modsFor(item: OwnedGear, mods: ModRef[], picks: Pick[]): ModRef[] {
  const planned = planFor(picks, item).flatMap((p) => (p.kind === "mod" ? [p.id] : []));
  return mods.filter((m) => m.kind === item.kind && !item.mods.includes(m.id) && !planned.includes(m.id) && (!m.makers.length || (item.maker !== null && m.makers.includes(item.maker))));
}

/** The pick of mod `m` for `item`: a maker's own cost where the mod has one. */
export function modPick(item: OwnedGear, m: ModRef): Pick {
  return { kind: "mod", id: m.id, maker: m.makers.length ? item.maker : null, item: item.id };
}

/**
 * `picks` with `p` added, never doubling a plan:
 * - a pick for an item uses up a matching pick that has no item (the same recipe, saved before
 *   picks could be tied) instead of adding one;
 * - an item has one upgrade pick: an upgrade for an item that has one combines with it, keeping
 *   the higher target, so neither adding nor tying lowers a plan or counts grades twice;
 * - a pick already there, for an item or for none, adds nothing.
 */
export function addPick(picks: Pick[], p: Pick): Pick[] {
  if (picks.some((x) => pickKey(x) === pickKey(p))) return picks;
  if (p.item == null) return [...picks, p];
  const loose = picks.findIndex((x) => x.item == null && recipeKey(x) === recipeKey(p));
  const up = p.kind === "upgrade" ? picks.findIndex((x) => x.kind === "upgrade" && x.item === p.item) : -1;
  if (up >= 0 && p.kind === "upgrade") {
    return picks.flatMap((x, i) => (i === loose ? [] : i === up && x.kind === "upgrade" ? [{ ...x, to: Math.max(x.to, p.to) }] : [x]));
  }
  if (loose >= 0) return picks.map((x, i) => (i === loose ? { ...x, item: p.item } : x));
  return [...picks, p];
}

/** `picks` with `p` tied to `item`; if that item already has the same pick, the two are one. */
export function tie(picks: Pick[], p: Pick, item: number): Pick[] {
  const rest = picks.filter((x) => pickKey(x) !== pickKey(p));
  return addPick(rest, { ...p, item });
}

/** The owned items in no loadout. */
export function inNoLoadout(owned: OwnedGear[], loadouts: SuitLoadout[]): OwnedGear[] {
  const worn = new Set(loadouts.flatMap((l) => [l.suit.id, ...l.slots.map((s) => s.item.id)]));
  return owned.filter((o) => !worn.has(o.id));
}
