/**
 * The station you are docked at, and where you saw a module before: the core's `docked_here`
 * and `outfitting_search` commands (crates/astromech-station; the core keeps every
 * `Outfitting.json` in `outfitting.sqlite`).
 */
import type { DockedHere as Docked } from "./api/types/DockedHere";
import type { Outfitting } from "./api/types/Outfitting";
import { call, type Sampled } from "./transport";

export type { StationEconomy as Economy } from "./api/types/StationEconomy";
export type { Upgrade } from "./api/types/Upgrade";
export type { Seen } from "./api/types/Seen";
/** Where the commander is docked; `outfitting` null when none was seen at this station yet. */
export type DockedHere = Sampled<Docked>;
/** The outfitting history searched: stations and `Outfitting.json` files kept so far, and the matches. */
export type OutfittingSearch = Sampled<Outfitting>;

export async function loadHere(): Promise<DockedHere> {
  return await call("docked_here");
}

export async function searchOutfitting(query: string): Promise<OutfittingSearch> {
  return await call("outfitting_search", { query });
}

/** How the game's service names read. */
export const SERVICE_NAMES: Record<string, string> = {
  dock: "Docking",
  autodock: "Autodock",
  commodities: "Commodities",
  contacts: "Contacts",
  exploration: "Universal Cartographics",
  missions: "Missions",
  outfitting: "Outfitting",
  crewlounge: "Crew lounge",
  rearm: "Restock",
  refuel: "Refuel",
  repair: "Repair",
  shipyard: "Shipyard",
  tuning: "Tuning",
  engineer: "Engineer",
  flightcontroller: "Flight controller",
  stationoperations: "Station operations",
  powerplay: "Powerplay",
  searchrescue: "Search and rescue",
  socialspace: "Social space",
  materialtrader: "Material trader",
  techBroker: "Technology broker",
  blackmarket: "Black market",
  voucherredemption: "Vouchers",
  facilitator: "Interstellar Factors",
  shop: "Shop",
  carrierfuel: "Carrier fuel",
  carriermanagement: "Carrier management",
  carriervendor: "Carrier vendor",
  modulepacks: "Module packs",
  livery: "Livery",
  pioneersupplies: "Pioneer supplies",
  apexinterstellar: "Apex Interstellar",
  vistagenomics: "Vista Genomics",
  bartender: "Bartender",
  captainslog: "Captain's log",
  colonisationcontribution: "Colonisation contribution",
  frontlinesolutions: "Frontline Solutions",
};

/** "Refuel", or the service name capitalised when it is not listed. */
export const serviceName = (s: string) => SERVICE_NAMES[s] ?? s.replace(/^./, (c) => c.toUpperCase());
