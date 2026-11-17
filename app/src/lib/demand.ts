/**
 * Demand and supply against the hold (DESIGN.md Colours that carry a meaning): on Sell, a
 * market's demand against the tonnes of the commodity aboard that are the commander's to sell;
 * on Buy, its stock against the hold's free space. Both figures come from `ship_status`, by the
 * rules of the Sell my cargo lookup (astromech_market::cargo::sellable).
 */
import type { ShipStatus } from "./api/types/ShipStatus";
import type { Pad } from "./api/types/Pad";
import { figure } from "./search";

/** Short: the market takes or has less than the figure. Tight: less than TIGHT_TIMES it. Plenty: the rest. */
export type Depth = "short" | "tight" | "plenty";

/** A market is tight below this many times the figure measured against. */
export const TIGHT_TIMES = 5;

/** How many of the market's three bars are lit. */
export const BARS: Record<Depth, number> = { short: 1, tight: 2, plenty: 3 };

/**
 * `volume` against `against` tonnes; null when there is nothing to measure against (none aboard,
 * a full hold, or not known), so the bars carry no warning colour.
 */
export function depth(volume: number, against: number | null): Depth | null {
  if (against == null || against <= 0) return null;
  return volume < against ? "short" : volume < TIGHT_TIMES * against ? "tight" : "plenty";
}

/** What a finder measures its markets against, and the words the page states it in. */
export interface Against {
  /** Tonnes; null when not known. 0 is known: none aboard, or a full hold. */
  tonnes: number | null;
  /** "200 t of Painite aboard", "No Painite aboard", "136 t free", "Hold full", or why it isn't known. */
  text: string;
}

/** Sell: the tonnes of `symbol` aboard that are the commander's to sell. */
export function sellAgainst(ship: ShipStatus | null, symbol: string, name: string): Against {
  if (!ship?.sellable) return { tonnes: null, text: "Cargo not known: Elite isn't running" };
  const t = ship.sellable.find((l) => l.symbol === symbol)?.tonnes ?? 0;
  return t > 0 ? { tonnes: t, text: `${figure(t)} t of ${name} aboard` } : { tonnes: 0, text: `No ${name} aboard` };
}

/** Buy: the hold's free space, an empty hold included. */
export function buyAgainst(ship: ShipStatus | null): Against {
  if (ship?.cargo_free == null) return { tonnes: null, text: "Free space not known: Elite isn't running" };
  return ship.cargo_free > 0 ? { tonnes: ship.cargo_free, text: `${figure(ship.cargo_free)} t free` } : { tonnes: 0, text: "Hold full" };
}

/** The Tip on a short row: what happens if the commander goes there anyway. */
export function shortTip(side: "sell" | "buy", volume: number, against: Against): string {
  const what = against.text.replace(/ aboard$/, "");
  return side === "sell"
    ? `${figure(volume)} t demand, less than the ${what} aboard. The rest won't sell here, and the price falls as you sell.`
    : `${figure(volume)} t in stock, less than the ${what}. It won't fill your hold.`;
}

const PAD_SIZE: Record<Pad, number> = { S: 0, M: 1, L: 2 };

/** A station's pad is too small for a ship that needs `need`. Unknown either way: it fits. */
export function padTooSmall(pad: Pad | null | undefined, need: Pad | null | undefined): boolean {
  return pad != null && need != null && PAD_SIZE[pad] < PAD_SIZE[need];
}
