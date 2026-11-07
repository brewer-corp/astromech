/**
 * Colonisation (astromech-colonisation, astromech-market::list): construction projects from the
 * journals with what is left to buy for each, and where to buy that list, stations stocking
 * most of it first (Ardent Insight, asked by the core). The browser preview answers from a
 * synthetic project and a recorded search (app/src/preview/fixtures/colonisation.json, trade.json).
 */
import type { MarketNeed as Need } from "./api/types/MarketNeed";
import type { ProjectView as Project } from "./api/types/ProjectView";
import type { ShoppingPlan as BuySearch } from "./api/types/ShoppingPlan";
import type { Filters, Origin, Results } from "./search";
import { credits } from "./trade";
import { call } from "./transport";

export type { ColonisationLine as ShoppingLine } from "./api/types/ColonisationLine";
/** A construction project and its shopping list (astromech_colonisation::ProjectView). */
export type { ProjectView as Project } from "./api/types/ProjectView";
/** One commodity still to buy (astromech_market::list::Need). */
export type { MarketNeed as Need } from "./api/types/MarketNeed";
export type { StockLine } from "./api/types/StockLine";
export type { Stockist } from "./api/types/Stockist";
export type { ListPlan } from "./api/types/ListPlan";
export type { ShoppingPlan as BuySearch } from "./api/types/ShoppingPlan";

/** The id the search kit remembers this lookup's filters under (astromech_market::list::LOOKUP). */
export const LOOKUP = "colonisation.buy";

/** Construction projects from the journals, those being built first. */
export async function loadProjects(): Promise<Project[]> {
  return await call("colonisation_projects");
}

/** A project's display name: the station, else its market id. */
export function projectName(p: Project): string {
  return p.station_name ?? `Construction site ${p.market_id}`;
}

/** What is left to buy for `p`, as the lookup asks for it. */
export function needsOf(p: Project): Need[] {
  return p.shopping.lines.filter((l) => l.to_buy > 0).map((l) => ({ symbol: l.symbol, name: l.name_localised ?? l.symbol, tonnes: l.to_buy }));
}

/** Where to buy `needs` near `origin` (or the commander), stations stocking most of them first. */
export async function buyList(needs: Need[], origin: Origin = null, filters: Filters | null = null): Promise<BuySearch> {
  return await call("colonisation_buy", { needs, origin, filters });
}

/** The stations as the search kit's rows: how much of the list each sells, and each stock. */
export function buyResults(s: BuySearch, listed: number): Results {
  const notes = [
    s.plan.unavailable.length ? `No market in range sells ${s.plan.unavailable.join(", ")}.` : "",
    s.plan.truncated.length ? `Too many markets for ${s.plan.truncated.join(", ")}: narrow the radius to see them all.` : "",
  ].filter(Boolean);
  return {
    origin: s.here.system,
    range: s.range,
    source: "Ardent Insight",
    fetched: new Date(s.plan.fetched_at * 1000).toISOString(),
    warning: notes.length ? notes.join(" ") : undefined,
    place: "station",
    main: { head: "Sells" },
    columns: [{ key: "stock", head: "Stock", text: true }],
    rows: s.plan.stations.map((x) => ({
      key: x.market_id,
      system: x.system,
      station: x.station,
      ly: x.ly,
      jumps: x.jumps,
      exact: x.exact,
      pad: x.pad,
      arrival_ls: x.arrival_ls,
      updated_at: x.updated_at,
      main: `${x.stocked} of ${listed}`,
      facts: {
        stock: [x.lines.map((l) => `${l.name} ${credits(l.stock)} t`).join(", "), x.fleet_carrier ? "fleet carrier" : "", x.odyssey ? "Odyssey settlement" : ""]
          .filter(Boolean)
          .join(" · "),
      },
    })),
  };
}
