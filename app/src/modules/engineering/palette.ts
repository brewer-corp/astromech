// Command palette entries from the journal fleet: each ship and the blueprints and experimentals
// for its modules open the ship's page; each fitted module opens where to buy it nearby.
import { loadFleet, type Ship } from "../../lib/engineering";
import { moduleRoute, wantFor } from "../../lib/outfitting";
import type { PaletteItem } from "../../lib/palette";

const shipName = (s: Ship) => s.name ?? s.type_name;

export default async function entries(): Promise<PaletteItem[]> {
  const fleet = await loadFleet();
  // The current ship first, so a blueprint opens on it when it has a module of that kind.
  const ships = [...fleet.ships].sort((a, b) => Number(b.current) - Number(a.current));
  const items: PaletteItem[] = ships.map((s) => ({ label: shipName(s), detail: `Ship · ${s.type_name}`, route: `engineering/${s.id}` }));
  // A fitted module opens the nearby outfitting search for that size; one it can't search for
  // (armour, sold per ship) opens its ship.
  const seen = new Set<string>();
  for (const s of ships) {
    for (const m of s.modules) {
      const want = wantFor(m.item);
      const route = want ? moduleRoute(want) : `engineering/${s.id}`;
      if (seen.has(route + m.name)) continue;
      seen.add(route + m.name);
      items.push({ label: m.name, detail: want ? `Outfitting · fitted on ${shipName(s)}` : `Module · ${m.slot_name} on ${shipName(s)}`, route });
    }
  }
  for (const [key, g] of Object.entries(fleet.groups)) {
    const ship = ships.find((s) => s.modules.some((m) => m.group === key));
    const route = ship ? `engineering/${ship.id}/plan` : "engineering";
    for (const b of g.blueprints) items.push({ label: b.name, detail: `Blueprint · ${g.name}`, route, keywords: key });
    for (const x of g.experimentals) items.push({ label: x.name, detail: `Experimental · ${g.name}`, route, keywords: key });
  }
  return items;
}
