/**
 * The station you are docked at, and where you saw a module before: the shell's `docked_here`
 * and `outfitting_search` commands (crates/astromech-station; the core keeps every
 * `Outfitting.json` in `outfitting.sqlite`).
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewData } from "./fakecore";
import sample from "./fixtures/station.json";

export interface Economy {
  name: string;
  name_localised: string | null;
  /** Share of the station's economy, 0 to 1. */
  proportion: number;
}

/** A module on sale that beats one fitted. */
export interface Upgrade {
  slot: string;
  slot_name: string;
  fitted: string;
  item: string;
  name: string;
  price: number;
}

export interface DockedHere {
  docked: boolean;
  station: string | null;
  system: string | null;
  market_id: number | null;
  station_type: string | null;
  services: string[];
  economies: Economy[];
  faction: { name: string; state: string | null } | null;
  government: string | null;
  allegiance: string | null;
  landing_pads: { small: number; medium: number; large: number } | null;
  dist_from_star_ls: number | null;
  max_pad: "Large" | "Medium" | "Small" | null;
  /** The ship being flown. */
  ship: string | null;
  /** `null`: no outfitting seen at this station yet. */
  outfitting: { seen: string; items: number; upgrades: Upgrade[] } | null;
  /** Where it came from, when not the live journal. */
  source?: string;
}

/** One station that sold a module that was searched for. */
export interface Seen {
  market_id: number;
  station_name: string;
  star_system: string;
  seen: string;
  item: string;
  name: string;
  price: number;
  distance_ly: number | null;
}

export interface OutfittingSearch {
  /** Stations and `Outfitting.json` files kept so far. */
  stations: number;
  snapshots: number;
  found: Seen[];
  source?: string;
}

export async function loadHere(): Promise<DockedHere> {
  if (!inTauri) {
    const notDocked: DockedHere = {
      docked: false, station: null, system: null, market_id: null, station_type: null, services: [], economies: [], faction: null,
      government: null, allegiance: null, landing_pads: null, dist_from_star_ls: null, max_pad: null, ship: null, outfitting: null,
    };
    return previewData("docked_here", () => ({ ...(sample.here as DockedHere), source: "Sample docking" }), () => notDocked);
  }
  return await invoke<DockedHere>("docked_here");
}

/** The sample book searched the way the core searches it (outside the app): every word must be in
 *  the module's symbol or name, and `fsd` stands for `hyperdrive`. */
export function searchSample(query: string): OutfittingSearch {
  const words = query.toLowerCase().split(/\s+/).filter(Boolean).map((w) => (w === "fsd" ? "hyperdrive" : w));
  const found = words.length
    ? (sample.book.found as Seen[]).filter((s) => words.every((w) => `${s.item} ${s.name}`.toLowerCase().includes(w)))
    : [];
  return { stations: sample.book.stations, snapshots: sample.book.snapshots, found, source: "Sample outfitting history" };
}

export async function searchOutfitting(query: string): Promise<OutfittingSearch> {
  if (!inTauri) return previewData("outfitting_search", () => searchSample(query), () => ({ stations: 0, snapshots: 0, found: [] }));
  return await invoke<OutfittingSearch>("outfitting_search", { query });
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
