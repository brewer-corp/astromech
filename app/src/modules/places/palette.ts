// Command palette entries: every finder, every service, every module stations sell (for any
// size) and every ship, each opening its finder with that input.
import { CATEGORIES, MODULES, SHIPS, moduleRoute, shipRoute } from "../../lib/outfitting";
import type { PaletteItem } from "../../lib/palette";
import { FINDERS, placesRoute, serviceRoute } from "../../lib/places";
import { SERVICES } from "../../lib/stations";

export default async function entries(): Promise<PaletteItem[]> {
  return [
    ...FINDERS.map((f) => ({ label: f.title, detail: "Places", route: placesRoute(f.id), keywords: f.what })),
    ...SERVICES.map((s) => ({ label: s.label, detail: "Services", route: serviceRoute(s.id), keywords: s.id.replaceAll("_", " ") })),
    ...MODULES.map((m) => ({
      label: m.name,
      detail: `Outfitting · ${CATEGORIES.find((c) => c.id === m.category)?.title ?? m.category}`,
      route: moduleRoute({ name: m.name, class: null, rating: null, mount: null }),
    })),
    ...SHIPS.map((s) => ({ label: s.name, detail: "Shipyard", route: shipRoute(s.name) })),
  ];
}
