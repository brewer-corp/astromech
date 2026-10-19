/**
 * The Galaxy find box: one typed list of services, modules and ships. Picking one opens that
 * lookup (the same routes the command palette uses).
 */
import { MODULES, SHIPS, CATEGORIES, moduleRoute, shipRoute } from "./outfitting";
import { rank, type PaletteItem } from "./palette";
import { SERVICES, type ServiceId } from "./stations";

/** Every service, module and ship the find box can recognise. */
export function findItems(): PaletteItem[] {
  return [
    ...SERVICES.map((s) => ({
      label: s.label,
      detail: "Service",
      route: serviceRoute(s.id),
      keywords: s.id.replaceAll("_", " "),
    })),
    ...MODULES.map((m) => ({
      label: m.name,
      detail: `Module · ${CATEGORIES.find((c) => c.id === m.category)?.title ?? m.category}`,
      route: moduleRoute({ name: m.name, class: null, rating: null, mount: null }),
    })),
    ...SHIPS.map((s) => ({
      label: s.name,
      detail: "Ship",
      route: shipRoute(s.name),
    })),
  ];
}

/** The best matches for what was typed; a blank query matches nothing (type to search). */
export function suggestFind(query: string, limit = 8): PaletteItem[] {
  return query.trim() ? rank(findItems(), query, limit) : [];
}

export const serviceRoute = (id: ServiceId | string) => `services/${id}`;

/** The service a `#/services/<id>` hash asks for, or null for none or an unknown id. */
export function parseServiceRoute(hash: string): ServiceId | null {
  const m = /^#\/services\/([^/?]+)$/.exec(hash);
  if (!m?.[1]) return null;
  let id: string;
  try {
    id = decodeURIComponent(m[1]);
  } catch {
    return null;
  }
  return SERVICES.some((s) => s.id === id) ? (id as ServiceId) : null;
}
