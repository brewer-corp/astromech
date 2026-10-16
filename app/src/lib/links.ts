/**
 * Links out to Inara and Spansh for systems and stations. Pure string builders, shared so any
 * page (and the system-name chip) can offer them. Opening a link goes through `openUrl` in core.ts.
 *
 * Spansh takes the 64-bit system address (journal `SystemAddress`) and the market id
 * (journal `MarketID`) directly, and a name search otherwise. Inara has no id routes: its system
 * and station pages are search URLs, and an exact system name redirects to the system page.
 */

const enc = encodeURIComponent;

/** Spansh system page. Pass the `SystemAddress` when known; a name falls back to Spansh search. */
export function spanshSystem(system: { id64?: number | string | null; name: string }): string {
  return system.id64 != null && system.id64 !== ""
    ? `https://spansh.co.uk/system/${enc(String(system.id64))}`
    : `https://spansh.co.uk/search/${enc(system.name)}`;
}

/** Spansh station page from the journal `MarketID`; a name falls back to Spansh search. */
export function spanshStation(station: { marketId?: number | string | null; name: string }): string {
  return station.marketId != null && station.marketId !== ""
    ? `https://spansh.co.uk/station/${enc(String(station.marketId))}`
    : `https://spansh.co.uk/search/${enc(station.name)}`;
}

export function inaraSystem(name: string): string {
  return `https://inara.cz/elite/starsystem/?search=${enc(name)}`;
}

export function inaraStation(name: string): string {
  return `https://inara.cz/elite/stations/?search=${enc(name)}`;
}
