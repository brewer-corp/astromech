// Command palette entries from the journal fleet: each ship, the modules fitted on it, and the
// blueprints and experimentals for those modules, each opening the ship's page.
import { loadFleet, type Ship } from "../../lib/engineering";
import type { PaletteItem } from "../../lib/palette";

const shipName = (s: Ship) => s.name ?? s.type_name;

export default async function entries(): Promise<PaletteItem[]> {
  const fleet = await loadFleet();
  // The current ship first, so a blueprint opens on it when it has a module of that kind.
  const ships = [...fleet.ships].sort((a, b) => Number(b.current) - Number(a.current));
  const items: PaletteItem[] = ships.map((s) => ({ label: shipName(s), detail: `Ship · ${s.type_name}`, route: `engineering/${s.id}` }));
  for (const s of ships) {
    const seen = new Set<string>();
    for (const m of s.modules) {
      if (seen.has(m.name)) continue;
      seen.add(m.name);
      items.push({ label: m.name, detail: `Module · ${m.slot_name} on ${shipName(s)}`, route: `engineering/${s.id}` });
    }
  }
  for (const [key, g] of Object.entries(fleet.groups)) {
    const ship = ships.find((s) => s.modules.some((m) => m.group === key));
    const route = ship ? `engineering/${ship.id}` : "engineering";
    for (const b of g.blueprints) items.push({ label: b.name, detail: `Blueprint · ${g.name}`, route, keywords: key });
    for (const x of g.experimentals) items.push({ label: x.name, detail: `Experimental · ${g.name}`, route, keywords: key });
  }
  return items;
}
