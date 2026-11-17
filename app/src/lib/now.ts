/**
 * The Now page's own logic (modules/now): when its Sell your cargo answer is out of date.
 *
 * `trade_cargo` reads the hold, the ship's pad and range, and where the commander is on every
 * call, so an answer holds while all of those do. The page asks again when this key changes, and
 * only then: never on a timer alone.
 */
import type { ShipStatus } from "./api/types/ShipStatus";

/**
 * What a Sell your cargo answer was asked for: each sellable commodity and its tonnes, the ship
 * (its id, the pad it needs and its jump range) and the system. Cargo that is not the
 * commander's to sell (mission cargo, limpets) is not in `sellable`, so it doesn't change the key.
 */
export function saleKey(ship: Pick<ShipStatus, "sellable" | "ship_id" | "pad" | "max_jump_range">, system: string | null): string {
  const hold = (ship.sellable ?? []).map((l) => `${l.symbol}:${l.tonnes}`).sort();
  return JSON.stringify([hold, ship.ship_id, ship.pad, ship.max_jump_range, system?.trim().toLowerCase() ?? null]);
}
