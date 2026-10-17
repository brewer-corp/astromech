// Command palette entries: every ship, opening its nearby shipyard search.
import { SHIPS, shipRoute } from "../../lib/outfitting";
import type { PaletteItem } from "../../lib/palette";

export default async function entries(): Promise<PaletteItem[]> {
  return SHIPS.map((s) => ({ label: s.name, detail: "Shipyard", route: shipRoute(s.name) }));
}
