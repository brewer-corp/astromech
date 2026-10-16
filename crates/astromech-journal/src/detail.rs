//! Typed forms of the journal events added for the toolbox work (lookups, EDDN, reactions).
//!
//! Field names follow the Journal manual v35 (July 2022). Anything newer (Odyssey, Powerplay 2.0,
//! Colonisation) comes from the EDCD community journal docs and `elite-journal` schemas; the
//! fields marked `// community` below are not in the manual and may shift between game builds.
//! Every field is optional or defaulted: the game omits fields freely and a missing one must
//! not drop the event. The original line is kept on [`crate::JournalLine::raw_json`].

use super::{opt_f, opt_i, opt_s, s, stacks, MaterialStack};
use serde_json::Value;

pub(crate) fn opt_b(v: &Value, k: &str) -> Option<bool> {
    v.get(k).and_then(Value::as_bool)
}

pub(crate) fn flag(v: &Value, k: &str) -> bool {
    opt_b(v, k).unwrap_or(false)
}

pub(crate) fn arr<'a>(v: &'a Value, k: &str) -> impl Iterator<Item = &'a Value> {
    v.get(k).and_then(Value::as_array).into_iter().flatten()
}

/// Localised text is optional and only worth keeping when it differs from the symbol.
pub(crate) fn loc(v: &Value, k: &str) -> Option<String> {
    opt_s(v, &format!("{k}_Localised"))
}

/// Station a `Market` / `Outfitting` / `Shipyard` event points at; the data is in the companion file.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StationRef {
    pub market_id: Option<i64>,
    pub station_name: String,
    pub station_type: Option<String>,
    pub star_system: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CargoItem {
    pub name: String,
    pub name_localised: Option<String>,
    pub count: i64,
    pub stolen: bool,
    pub mission_id: Option<i64>,
}

/// `MarketBuy` / `MarketSell`.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketTrade {
    pub sell: bool,
    pub market_id: Option<i64>,
    /// Commodity symbol (`Type`).
    pub commodity: String,
    pub count: i64,
    /// `BuyPrice` or `SellPrice`, per tonne.
    pub price: i64,
    /// `TotalCost` or `TotalSale`.
    pub total: i64,
    /// `AvgPricePaid`, sells only.
    pub avg_price_paid: Option<i64>,
    pub black_market: bool,
    pub illegal_goods: bool,
    pub stolen_goods: bool,
}

/// `FSSBodySignals` and `SAASignalsFound`. Genuses only come with the latter.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BodySignals {
    pub body_name: String,
    pub body_id: Option<i64>,
    pub system_address: Option<i64>,
    pub signals: Vec<SignalCount>,
    pub genuses: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignalCount {
    /// Symbol, e.g. `$SAA_SignalType_Biological;`.
    pub kind: String,
    pub kind_localised: Option<String>,
    pub count: i64,
}

/// `Scan`, the detailed scan of a star, planet or ring. Only what lookups and callouts read.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScanBody {
    pub scan_type: String,
    pub body_name: String,
    pub body_id: Option<i64>,
    pub system_address: Option<i64>,
    pub star_system: Option<String>,
    pub distance_from_arrival_ls: Option<f64>,
    /// Stars only.
    pub star_type: Option<String>,
    pub subclass: Option<i64>,
    pub planet_class: Option<String>,
    pub terraform_state: Option<String>,
    pub atmosphere: Option<String>,
    pub volcanism: Option<String>,
    pub landable: bool,
    pub was_discovered: Option<bool>,
    pub was_mapped: Option<bool>,
    pub was_footfalled: Option<bool>, // community (Odyssey)
    pub mass_em: Option<f64>,
    pub stellar_mass: Option<f64>,
    pub radius: Option<f64>,
    pub surface_gravity: Option<f64>,
    pub surface_temperature: Option<f64>,
    pub surface_pressure: Option<f64>,
    pub tidal_lock: Option<bool>,
    pub reserve_level: Option<String>,
    /// Surface materials: (name, percent).
    pub materials: Vec<(String, f64)>,
    pub ring_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BioSale {
    pub genus: String,
    pub species: String,
    pub variant: Option<String>,
    pub value: i64,
    pub bonus: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CodexEntry {
    pub entry_id: Option<i64>,
    pub name: String,
    pub name_localised: Option<String>,
    pub sub_category: Option<String>,
    pub category: Option<String>,
    pub region: Option<String>,
    pub system: Option<String>,
    pub system_address: Option<i64>,
    pub is_new_entry: bool,
    pub new_traits_discovered: bool,
    pub voucher_amount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MissionRef {
    pub mission_id: i64,
    pub name: String,
    pub passenger_mission: bool,
    pub expires_secs: Option<i64>,
}

/// `MissionAccepted`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MissionOffer {
    pub mission_id: i64,
    pub name: String,
    pub faction: String,
    pub destination_system: Option<String>,
    pub destination_station: Option<String>,
    pub commodity: Option<String>,
    pub count: Option<i64>,
    pub target: Option<String>,
    pub target_type: Option<String>,
    pub target_faction: Option<String>,
    pub kill_count: Option<i64>,
    /// ISO timestamp the mission expires at.
    pub expiry: Option<String>,
    pub reward: Option<i64>,
    pub influence: Option<String>,
    pub reputation: Option<String>,
    pub passenger_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BountyReward {
    pub faction: String,
    pub reward: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsteroidMaterial {
    pub name: String,
    pub proportion: f64,
}

/// Every `Powerplay*` event. `kind` is the event name; the rest is whatever that event carried.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PowerplayEvent {
    pub kind: String,
    pub power: Option<String>,
    /// `PowerplayDefect`.
    pub from_power: Option<String>,
    pub to_power: Option<String>,
    pub rank: Option<i64>,
    /// `Merits` on `Powerplay`; `TotalMerits` on `PowerplayMerits`.
    pub total_merits: Option<i64>,
    pub merits_gained: Option<i64>,
    pub votes: Option<i64>,
    pub time_pledged_secs: Option<i64>,
    /// `PowerplayDeliver` / `PowerplayCollect`: what and how many.
    pub item: Option<String>,
    pub count: Option<i64>,
    /// `PowerplaySalary` amount or `PowerplayFastTrack` cost.
    pub amount: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DepotResource {
    pub name: String,
    pub name_localised: Option<String>,
    pub required: i64,
    pub provided: i64,
    pub payment: i64,
}

/// `ColonisationConstructionDepot`. community (Trailblazers, 2025).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConstructionDepot {
    pub market_id: Option<i64>,
    /// 0.0 to 1.0.
    pub progress: f64,
    pub complete: bool,
    pub failed: bool,
    pub resources: Vec<DepotResource>,
}

/// `CarrierStats`. Only the headline numbers; the rest is in the raw line.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CarrierInfo {
    pub carrier_id: Option<i64>,
    pub callsign: String,
    pub name: String,
    pub docking_access: Option<String>,
    pub allow_notorious: Option<bool>,
    pub fuel_level: Option<i64>,
    pub jump_range_curr: Option<f64>,
    pub jump_range_max: Option<f64>,
    pub pending_decommission: bool,
    pub balance: Option<i64>,
    pub reserve_balance: Option<i64>,
    pub free_space: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MicroResource {
    /// `Items`, `Components`, `Consumables` or `Data`.
    pub category: String,
    pub name: String,
    pub name_localised: Option<String>,
    pub count: i64,
    pub owner_id: Option<i64>,
    pub mission_id: Option<i64>,
}

/// Rank or progress per career. `Rank` carries ranks; `Progress` carries percent to the next.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RankSet {
    pub combat: Option<i64>,
    pub trade: Option<i64>,
    pub explore: Option<i64>,
    pub soldier: Option<i64>,
    pub exobiologist: Option<i64>,
    pub empire: Option<i64>,
    pub federation: Option<i64>,
    pub cqc: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Reputation {
    pub empire: Option<f64>,
    pub federation: Option<f64>,
    pub independent: Option<f64>,
    pub alliance: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interdicted {
    pub submitted: bool,
    pub interdictor: String,
    pub is_player: bool,
    pub faction: Option<String>,
    pub power: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShipTargeted {
    pub target_locked: bool,
    pub ship: Option<String>,
    /// 0 none, 1 ship, 2 loadout, 3 cargo.
    pub scan_stage: Option<i64>,
    pub pilot_name: Option<String>,
    pub pilot_rank: Option<String>,
    pub faction: Option<String>,
    pub legal_status: Option<String>,
    pub bounty: Option<i64>,
    pub power: Option<String>,
    pub shield_health: Option<f64>,
    pub hull_health: Option<f64>,
    pub subsystem: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Touchdown {
    pub player_controlled: bool,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub on_station: Option<bool>,
    pub on_planet: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Disembark {
    pub srv: bool,
    pub taxi: bool,
    pub multicrew: bool,
    pub star_system: Option<String>,
    pub body: Option<String>,
    pub body_id: Option<i64>,
    pub on_station: bool,
    pub on_planet: bool,
    pub station_name: Option<String>,
    pub market_id: Option<i64>,
}

/// Typed events added after the first 15. See [`parse_extra`].
#[derive(Debug, Clone, PartialEq)]
pub enum ExtraEvent {
    Market(StationRef),
    Outfitting(StationRef),
    Shipyard(StationRef),
    Cargo {
        vessel: Option<String>,
        count: Option<i64>,
        inventory: Vec<CargoItem>,
    },
    MarketTrade(MarketTrade),
    FsdTarget {
        name: String,
        system_address: Option<i64>,
        star_class: Option<String>,
        remaining_jumps: Option<i64>,
    },
    StartJump {
        jump_type: String,
        star_system: Option<String>,
        system_address: Option<i64>,
        star_class: Option<String>,
    },
    /// The route is in `NavRoute.json`.
    NavRoute,
    NavRouteClear,
    Docking {
        granted: bool,
        station_name: String,
        market_id: Option<i64>,
        station_type: Option<String>,
        landing_pad: Option<i64>,
        /// `DockingDenied` only.
        reason: Option<String>,
    },
    SupercruiseExit {
        star_system: String,
        body: Option<String>,
        body_id: Option<i64>,
        body_type: Option<String>,
        system_address: Option<i64>,
    },
    SupercruiseEntry {
        star_system: String,
        system_address: Option<i64>,
    },
    FssDiscoveryScan {
        /// 0.0 to 1.0.
        progress: f64,
        body_count: i64,
        non_body_count: i64,
        system_name: Option<String>,
        system_address: Option<i64>,
    },
    FssAllBodiesFound {
        system_name: String,
        system_address: Option<i64>,
        count: i64,
    },
    FssBodySignals(BodySignals),
    SaaSignalsFound(BodySignals),
    SaaScanComplete {
        body_name: String,
        body_id: Option<i64>,
        system_address: Option<i64>,
        probes_used: i64,
        efficiency_target: i64,
    },
    Scan(Box<ScanBody>),
    ScanBaryCentre {
        star_system: Option<String>,
        body_id: Option<i64>,
        system_address: Option<i64>,
    },
    ScanOrganic {
        scan_type: String,
        genus: String,
        genus_localised: Option<String>,
        species: String,
        species_localised: Option<String>,
        variant: Option<String>,
        body: Option<i64>,
        system_address: Option<i64>,
    },
    SellOrganicData {
        market_id: Option<i64>,
        bio_data: Vec<BioSale>,
    },
    /// `SellExplorationData` (`multi: false`, systems by name) and `MultiSellExplorationData`.
    ExplorationSale {
        multi: bool,
        systems: Vec<String>,
        base_value: i64,
        bonus: i64,
        total_earnings: i64,
    },
    CodexEntry(Box<CodexEntry>),
    ApproachSettlement {
        name: String,
        market_id: Option<i64>,
        system_address: Option<i64>,
        body_name: Option<String>,
        body_id: Option<i64>,
        latitude: Option<f64>,
        longitude: Option<f64>,
    },
    NavBeaconScan {
        system_address: Option<i64>,
        num_bodies: i64,
    },
    Missions {
        active: Vec<MissionRef>,
        failed: Vec<MissionRef>,
        complete: Vec<MissionRef>,
    },
    MissionAccepted(Box<MissionOffer>),
    MissionCompleted {
        mission_id: Option<i64>,
        name: String,
        faction: Option<String>,
        reward: Option<i64>,
        donation: Option<i64>,
        materials_reward: Vec<MaterialStack>,
    },
    /// `MissionAbandoned` and `MissionFailed`.
    MissionEnded {
        failed: bool,
        mission_id: Option<i64>,
        name: String,
        fine: Option<i64>,
    },
    MissionRedirected {
        mission_id: Option<i64>,
        name: String,
        new_destination_system: Option<String>,
        new_destination_station: Option<String>,
    },
    Bounty {
        rewards: Vec<BountyReward>,
        total_reward: Option<i64>,
        victim_faction: Option<String>,
        target: Option<String>,
    },
    RedeemVoucher {
        kind: String,
        amount: i64,
        factions: Vec<String>,
    },
    ProspectedAsteroid {
        materials: Vec<AsteroidMaterial>,
        /// `Low` / `Medium` / `High` / `Core`-style content grade, symbol form.
        content: Option<String>,
        motherlode_material: Option<String>,
        remaining: Option<f64>,
    },
    MiningRefined {
        commodity: String,
    },
    Powerplay(PowerplayEvent),
    ColonisationConstructionDepot(Box<ConstructionDepot>),
    ColonisationContribution {
        market_id: Option<i64>,
        contributions: Vec<(String, i64)>,
    },
    CarrierStats(Box<CarrierInfo>),
    /// `CarrierJumpRequest` (`cancelled: false`) and `CarrierJumpCancelled`.
    CarrierJumpPlan {
        cancelled: bool,
        carrier_id: Option<i64>,
        system_name: Option<String>,
        system_address: Option<i64>,
        body: Option<String>,
        departure_time: Option<String>,
    },
    ShipLocker {
        items: Vec<MicroResource>,
    },
    /// Counters by section: `Statistics` -> `Bank_Account` -> `Current_Wealth` and so on.
    Statistics {
        sections: Vec<(String, Vec<(String, f64)>)>,
    },
    Rank(RankSet),
    Progress(RankSet),
    Reputation(Reputation),
    Interdicted(Interdicted),
    ShipTargeted(Box<ShipTargeted>),
    Scanned {
        scan_type: String,
    },
    FuelScoop {
        scooped: f64,
        total: f64,
    },
    ReceiveText {
        from: String,
        message: String,
        channel: String,
    },
    Touchdown(Touchdown),
    Disembark(Box<Disembark>),
}

fn station_ref(v: &Value) -> StationRef {
    StationRef {
        market_id: opt_i(v, "MarketID"),
        station_name: s(v, "StationName"),
        station_type: opt_s(v, "StationType"),
        star_system: s(v, "StarSystem"),
    }
}

fn body_signals(v: &Value) -> BodySignals {
    BodySignals {
        body_name: s(v, "BodyName"),
        body_id: opt_i(v, "BodyID"),
        system_address: opt_i(v, "SystemAddress"),
        signals: arr(v, "Signals")
            .map(|x| SignalCount {
                kind: s(x, "Type"),
                kind_localised: loc(x, "Type"),
                count: opt_i(x, "Count").unwrap_or(0),
            })
            .collect(),
        genuses: arr(v, "Genuses").map(|g| s(g, "Genus")).filter(|g| !g.is_empty()).collect(),
    }
}

fn scan_body(v: &Value) -> ScanBody {
    ScanBody {
        scan_type: s(v, "ScanType"),
        body_name: s(v, "BodyName"),
        body_id: opt_i(v, "BodyID"),
        system_address: opt_i(v, "SystemAddress"),
        star_system: opt_s(v, "StarSystem"),
        distance_from_arrival_ls: opt_f(v, "DistanceFromArrivalLS"),
        star_type: opt_s(v, "StarType"),
        subclass: opt_i(v, "Subclass"),
        planet_class: opt_s(v, "PlanetClass"),
        terraform_state: opt_s(v, "TerraformState"),
        atmosphere: opt_s(v, "Atmosphere"),
        volcanism: opt_s(v, "Volcanism"),
        landable: flag(v, "Landable"),
        was_discovered: opt_b(v, "WasDiscovered"),
        was_mapped: opt_b(v, "WasMapped"),
        was_footfalled: opt_b(v, "WasFootfalled"),
        mass_em: opt_f(v, "MassEM"),
        stellar_mass: opt_f(v, "StellarMass"),
        radius: opt_f(v, "Radius"),
        surface_gravity: opt_f(v, "SurfaceGravity"),
        surface_temperature: opt_f(v, "SurfaceTemperature"),
        surface_pressure: opt_f(v, "SurfacePressure"),
        tidal_lock: opt_b(v, "TidalLock"),
        reserve_level: opt_s(v, "ReserveLevel"),
        materials: arr(v, "Materials").map(|m| (s(m, "Name"), opt_f(m, "Percent").unwrap_or(0.0))).collect(),
        ring_count: arr(v, "Rings").count(),
    }
}

fn mission_refs(v: &Value, k: &str) -> Vec<MissionRef> {
    arr(v, k)
        .filter_map(|m| {
            Some(MissionRef {
                mission_id: opt_i(m, "MissionID")?,
                name: s(m, "Name"),
                passenger_mission: flag(m, "PassengerMission"),
                expires_secs: opt_i(m, "Expires"),
            })
        })
        .collect()
}

pub(crate) fn micro_resources(v: &Value) -> Vec<MicroResource> {
    ["Items", "Components", "Consumables", "Data"]
        .into_iter()
        .flat_map(|cat| {
            arr(v, cat).map(move |x| MicroResource {
                category: cat.to_string(),
                name: s(x, "Name"),
                name_localised: loc(x, "Name"),
                count: opt_i(x, "Count").unwrap_or(0),
                owner_id: opt_i(x, "OwnerID"),
                mission_id: opt_i(x, "MissionID"),
            })
        })
        .filter(|m| !m.name.is_empty())
        .collect()
}

fn rank_set(v: &Value) -> RankSet {
    RankSet {
        combat: opt_i(v, "Combat"),
        trade: opt_i(v, "Trade"),
        explore: opt_i(v, "Explore"),
        soldier: opt_i(v, "Soldier"),
        exobiologist: opt_i(v, "Exobiologist"),
        empire: opt_i(v, "Empire"),
        federation: opt_i(v, "Federation"),
        cqc: opt_i(v, "CQC"),
    }
}

fn powerplay(event: &str, v: &Value) -> PowerplayEvent {
    PowerplayEvent {
        kind: event.to_string(),
        power: opt_s(v, "Power"),
        from_power: opt_s(v, "FromPower"),
        to_power: opt_s(v, "ToPower"),
        rank: opt_i(v, "Rank"),
        total_merits: opt_i(v, "TotalMerits").or_else(|| opt_i(v, "Merits")),
        merits_gained: opt_i(v, "MeritsGained"),
        votes: opt_i(v, "Votes"),
        time_pledged_secs: opt_i(v, "TimePledged"),
        item: opt_s(v, "Type"),
        count: opt_i(v, "Count"),
        amount: opt_i(v, "Amount").or_else(|| opt_i(v, "Cost")),
    }
}

/// Type the events that have a dedicated form, `None` for everything else (the caller keeps it
/// as `Other`). `event` is the journal event name.
pub fn parse_extra(event: &str, v: &Value) -> Option<ExtraEvent> {
    use ExtraEvent as E;
    Some(match event {
        "Market" => E::Market(station_ref(v)),
        "Outfitting" => E::Outfitting(station_ref(v)),
        "Shipyard" => E::Shipyard(station_ref(v)),
        "Cargo" => E::Cargo {
            vessel: opt_s(v, "Vessel"),
            count: opt_i(v, "Count"),
            inventory: arr(v, "Inventory")
                .map(|i| CargoItem {
                    name: s(i, "Name"),
                    name_localised: loc(i, "Name"),
                    count: opt_i(i, "Count").unwrap_or(0),
                    stolen: opt_i(i, "Stolen").unwrap_or(0) > 0,
                    mission_id: opt_i(i, "MissionID"),
                })
                .collect(),
        },
        "MarketBuy" | "MarketSell" => {
            let sell = event == "MarketSell";
            E::MarketTrade(MarketTrade {
                sell,
                market_id: opt_i(v, "MarketID"),
                commodity: s(v, "Type"),
                count: opt_i(v, "Count").unwrap_or(0),
                price: opt_i(v, if sell { "SellPrice" } else { "BuyPrice" }).unwrap_or(0),
                total: opt_i(v, if sell { "TotalSale" } else { "TotalCost" }).unwrap_or(0),
                avg_price_paid: opt_i(v, "AvgPricePaid"),
                black_market: flag(v, "BlackMarket"),
                illegal_goods: flag(v, "IllegalGoods"),
                stolen_goods: flag(v, "StolenGoods"),
            })
        }
        "FSDTarget" => E::FsdTarget {
            name: s(v, "Name"),
            system_address: opt_i(v, "SystemAddress"),
            star_class: opt_s(v, "StarClass"),
            remaining_jumps: opt_i(v, "RemainingJumpsInRoute"),
        },
        "StartJump" => E::StartJump {
            jump_type: s(v, "JumpType"),
            star_system: opt_s(v, "StarSystem"),
            system_address: opt_i(v, "SystemAddress"),
            star_class: opt_s(v, "StarClass"),
        },
        "NavRoute" => E::NavRoute,
        "NavRouteClear" => E::NavRouteClear,
        "DockingGranted" | "DockingDenied" => E::Docking {
            granted: event == "DockingGranted",
            station_name: s(v, "StationName"),
            market_id: opt_i(v, "MarketID"),
            station_type: opt_s(v, "StationType"),
            landing_pad: opt_i(v, "LandingPad"),
            reason: opt_s(v, "Reason"),
        },
        "SupercruiseExit" => E::SupercruiseExit {
            star_system: s(v, "StarSystem"),
            body: opt_s(v, "Body"),
            body_id: opt_i(v, "BodyID"),
            body_type: opt_s(v, "BodyType"),
            system_address: opt_i(v, "SystemAddress"),
        },
        "SupercruiseEntry" => {
            E::SupercruiseEntry { star_system: s(v, "StarSystem"), system_address: opt_i(v, "SystemAddress") }
        }
        "FSSDiscoveryScan" => E::FssDiscoveryScan {
            progress: opt_f(v, "Progress").unwrap_or(0.0),
            body_count: opt_i(v, "BodyCount").unwrap_or(0),
            non_body_count: opt_i(v, "NonBodyCount").unwrap_or(0),
            system_name: opt_s(v, "SystemName"),
            system_address: opt_i(v, "SystemAddress"),
        },
        "FSSAllBodiesFound" => E::FssAllBodiesFound {
            system_name: s(v, "SystemName"),
            system_address: opt_i(v, "SystemAddress"),
            count: opt_i(v, "Count").unwrap_or(0),
        },
        "FSSBodySignals" => E::FssBodySignals(body_signals(v)),
        "SAASignalsFound" => E::SaaSignalsFound(body_signals(v)),
        "SAAScanComplete" => E::SaaScanComplete {
            body_name: s(v, "BodyName"),
            body_id: opt_i(v, "BodyID"),
            system_address: opt_i(v, "SystemAddress"),
            probes_used: opt_i(v, "ProbesUsed").unwrap_or(0),
            efficiency_target: opt_i(v, "EfficiencyTarget").unwrap_or(0),
        },
        "Scan" => E::Scan(Box::new(scan_body(v))),
        "ScanBaryCentre" => E::ScanBaryCentre {
            star_system: opt_s(v, "StarSystem"),
            body_id: opt_i(v, "BodyID"),
            system_address: opt_i(v, "SystemAddress"),
        },
        "ScanOrganic" => E::ScanOrganic {
            scan_type: s(v, "ScanType"),
            genus: s(v, "Genus"),
            genus_localised: loc(v, "Genus"),
            species: s(v, "Species"),
            species_localised: loc(v, "Species"),
            variant: opt_s(v, "Variant"),
            body: opt_i(v, "Body"),
            system_address: opt_i(v, "SystemAddress"),
        },
        "SellOrganicData" => E::SellOrganicData {
            market_id: opt_i(v, "MarketID"),
            bio_data: arr(v, "BioData")
                .map(|b| BioSale {
                    genus: s(b, "Genus"),
                    species: s(b, "Species"),
                    variant: opt_s(b, "Variant"),
                    value: opt_i(b, "Value").unwrap_or(0),
                    bonus: opt_i(b, "Bonus").unwrap_or(0),
                })
                .collect(),
        },
        "SellExplorationData" => E::ExplorationSale {
            multi: false,
            systems: arr(v, "Systems").filter_map(|x| x.as_str().map(str::to_string)).collect(),
            base_value: opt_i(v, "BaseValue").unwrap_or(0),
            bonus: opt_i(v, "Bonus").unwrap_or(0),
            total_earnings: opt_i(v, "TotalEarnings").unwrap_or(0),
        },
        "MultiSellExplorationData" => E::ExplorationSale {
            multi: true,
            systems: arr(v, "Discovered").map(|d| s(d, "SystemName")).filter(|n| !n.is_empty()).collect(),
            base_value: opt_i(v, "BaseValue").unwrap_or(0),
            bonus: opt_i(v, "Bonus").unwrap_or(0),
            total_earnings: opt_i(v, "TotalEarnings").unwrap_or(0),
        },
        "CodexEntry" => E::CodexEntry(Box::new(CodexEntry {
            entry_id: opt_i(v, "EntryID"),
            name: s(v, "Name"),
            name_localised: loc(v, "Name"),
            sub_category: opt_s(v, "SubCategory"),
            category: opt_s(v, "Category"),
            region: opt_s(v, "Region"),
            system: opt_s(v, "System"),
            system_address: opt_i(v, "SystemAddress"),
            is_new_entry: flag(v, "IsNewEntry"),
            new_traits_discovered: flag(v, "NewTraitsDiscovered"),
            voucher_amount: opt_i(v, "VoucherAmount"),
        })),
        "ApproachSettlement" => E::ApproachSettlement {
            name: s(v, "Name"),
            market_id: opt_i(v, "MarketID"),
            system_address: opt_i(v, "SystemAddress"),
            body_name: opt_s(v, "BodyName"),
            body_id: opt_i(v, "BodyID"),
            latitude: opt_f(v, "Latitude"),
            longitude: opt_f(v, "Longitude"),
        },
        "NavBeaconScan" => E::NavBeaconScan {
            system_address: opt_i(v, "SystemAddress"),
            num_bodies: opt_i(v, "NumBodies").unwrap_or(0),
        },
        "Missions" => E::Missions {
            active: mission_refs(v, "Active"),
            failed: mission_refs(v, "Failed"),
            complete: mission_refs(v, "Complete"),
        },
        "MissionAccepted" => E::MissionAccepted(Box::new(MissionOffer {
            mission_id: opt_i(v, "MissionID").unwrap_or(0),
            name: s(v, "Name"),
            faction: s(v, "Faction"),
            destination_system: opt_s(v, "DestinationSystem"),
            destination_station: opt_s(v, "DestinationStation"),
            commodity: opt_s(v, "Commodity"),
            count: opt_i(v, "Count"),
            target: opt_s(v, "Target"),
            target_type: opt_s(v, "TargetType"),
            target_faction: opt_s(v, "TargetFaction"),
            kill_count: opt_i(v, "KillCount"),
            expiry: opt_s(v, "Expiry"),
            reward: opt_i(v, "Reward"),
            influence: opt_s(v, "Influence"),
            reputation: opt_s(v, "Reputation"),
            passenger_count: opt_i(v, "PassengerCount"),
        })),
        "MissionCompleted" => E::MissionCompleted {
            mission_id: opt_i(v, "MissionID"),
            name: s(v, "Name"),
            faction: opt_s(v, "Faction"),
            reward: opt_i(v, "Reward"),
            donation: opt_i(v, "Donation"),
            materials_reward: stacks(v, "MaterialsReward", None),
        },
        "MissionAbandoned" | "MissionFailed" => E::MissionEnded {
            failed: event == "MissionFailed",
            mission_id: opt_i(v, "MissionID"),
            name: s(v, "Name"),
            fine: opt_i(v, "Fine"),
        },
        "MissionRedirected" => E::MissionRedirected {
            mission_id: opt_i(v, "MissionID"),
            name: s(v, "MissionName"),
            new_destination_system: opt_s(v, "NewDestinationSystem"),
            new_destination_station: opt_s(v, "NewDestinationStation"),
        },
        "Bounty" => E::Bounty {
            // Older builds wrote a single `Faction` + `Reward`; newer ones a `Rewards` list.
            rewards: if v.get("Rewards").is_some() {
                arr(v, "Rewards")
                    .map(|r| BountyReward { faction: s(r, "Faction"), reward: opt_i(r, "Reward").unwrap_or(0) })
                    .collect()
            } else {
                opt_i(v, "Reward").map(|reward| BountyReward { faction: s(v, "Faction"), reward }).into_iter().collect()
            },
            total_reward: opt_i(v, "TotalReward").or_else(|| opt_i(v, "Reward")),
            victim_faction: opt_s(v, "VictimFaction"),
            target: opt_s(v, "Target"),
        },
        "RedeemVoucher" => E::RedeemVoucher {
            kind: s(v, "Type"),
            amount: opt_i(v, "Amount").unwrap_or(0),
            factions: if v.get("Factions").is_some() {
                arr(v, "Factions").map(|f| s(f, "Faction")).filter(|f| !f.is_empty()).collect()
            } else {
                opt_s(v, "Faction").into_iter().collect()
            },
        },
        "ProspectedAsteroid" => E::ProspectedAsteroid {
            materials: arr(v, "Materials")
                .map(|m| AsteroidMaterial { name: s(m, "Name"), proportion: opt_f(m, "Proportion").unwrap_or(0.0) })
                .collect(),
            content: opt_s(v, "Content"),
            motherlode_material: opt_s(v, "MotherlodeMaterial"),
            remaining: opt_f(v, "Remaining"),
        },
        "MiningRefined" => E::MiningRefined { commodity: s(v, "Type") },
        e if e == "Powerplay" || e.starts_with("Powerplay") => E::Powerplay(powerplay(e, v)),
        "ColonisationConstructionDepot" => E::ColonisationConstructionDepot(Box::new(ConstructionDepot {
            market_id: opt_i(v, "MarketID"),
            progress: opt_f(v, "ConstructionProgress").unwrap_or(0.0),
            complete: flag(v, "ConstructionComplete"),
            failed: flag(v, "ConstructionFailed"),
            resources: arr(v, "ResourcesRequired")
                .map(|r| DepotResource {
                    name: s(r, "Name"),
                    name_localised: loc(r, "Name"),
                    required: opt_i(r, "RequiredAmount").unwrap_or(0),
                    provided: opt_i(r, "ProvidedAmount").unwrap_or(0),
                    payment: opt_i(r, "Payment").unwrap_or(0),
                })
                .collect(),
        })),
        "ColonisationContribution" => E::ColonisationContribution {
            market_id: opt_i(v, "MarketID"),
            contributions: arr(v, "Contributions").map(|c| (s(c, "Name"), opt_i(c, "Amount").unwrap_or(0))).collect(),
        },
        "CarrierStats" => {
            let fin = v.get("Finance").cloned().unwrap_or(Value::Null);
            let space = v.get("SpaceUsage").cloned().unwrap_or(Value::Null);
            E::CarrierStats(Box::new(CarrierInfo {
                carrier_id: opt_i(v, "CarrierID"),
                callsign: s(v, "Callsign"),
                name: s(v, "Name"),
                docking_access: opt_s(v, "DockingAccess"),
                allow_notorious: opt_b(v, "AllowNotorious"),
                fuel_level: opt_i(v, "FuelLevel"),
                jump_range_curr: opt_f(v, "JumpRangeCurr"),
                jump_range_max: opt_f(v, "JumpRangeMax"),
                pending_decommission: flag(v, "PendingDecommission"),
                balance: opt_i(&fin, "CarrierBalance"),
                reserve_balance: opt_i(&fin, "ReserveBalance"),
                free_space: opt_i(&space, "FreeSpace"),
            }))
        }
        "CarrierJumpRequest" | "CarrierJumpCancelled" => E::CarrierJumpPlan {
            cancelled: event == "CarrierJumpCancelled",
            carrier_id: opt_i(v, "CarrierID"),
            system_name: opt_s(v, "SystemName"),
            system_address: opt_i(v, "SystemAddress"),
            body: opt_s(v, "Body"),
            departure_time: opt_s(v, "DepartureTime"),
        },
        "ShipLocker" => E::ShipLocker { items: micro_resources(v) },
        "Statistics" => E::Statistics {
            sections: v
                .as_object()
                .into_iter()
                .flatten()
                .filter(|(k, _)| *k != "timestamp" && *k != "event")
                .filter_map(|(k, sec)| {
                    let counters =
                        sec.as_object()?.iter().filter_map(|(n, x)| Some((n.clone(), x.as_f64()?))).collect::<Vec<_>>();
                    Some((k.clone(), counters))
                })
                .collect(),
        },
        "Rank" => E::Rank(rank_set(v)),
        "Progress" => E::Progress(rank_set(v)),
        "Reputation" => E::Reputation(Reputation {
            empire: opt_f(v, "Empire"),
            federation: opt_f(v, "Federation"),
            independent: opt_f(v, "Independent"),
            alliance: opt_f(v, "Alliance"),
        }),
        "Interdicted" => E::Interdicted(Interdicted {
            submitted: flag(v, "Submitted"),
            interdictor: s(v, "Interdictor"),
            is_player: flag(v, "IsPlayer"),
            faction: opt_s(v, "Faction"),
            power: opt_s(v, "Power"),
        }),
        "ShipTargeted" => E::ShipTargeted(Box::new(ShipTargeted {
            target_locked: flag(v, "TargetLocked"),
            ship: opt_s(v, "Ship"),
            scan_stage: opt_i(v, "ScanStage"),
            pilot_name: opt_s(v, "PilotName"),
            pilot_rank: opt_s(v, "PilotRank"),
            faction: opt_s(v, "Faction"),
            legal_status: opt_s(v, "LegalStatus"),
            bounty: opt_i(v, "Bounty"),
            power: opt_s(v, "Power"),
            shield_health: opt_f(v, "ShieldHealth"),
            hull_health: opt_f(v, "HullHealth"),
            subsystem: opt_s(v, "SubSystem"),
        })),
        "Scanned" => E::Scanned { scan_type: s(v, "ScanType") },
        "FuelScoop" => {
            E::FuelScoop { scooped: opt_f(v, "Scooped").unwrap_or(0.0), total: opt_f(v, "Total").unwrap_or(0.0) }
        }
        "ReceiveText" => E::ReceiveText { from: s(v, "From"), message: s(v, "Message"), channel: s(v, "Channel") },
        "Touchdown" => E::Touchdown(Touchdown {
            // Absent on older builds, which only wrote player landings.
            player_controlled: opt_b(v, "PlayerControlled").unwrap_or(true),
            latitude: opt_f(v, "Latitude"),
            longitude: opt_f(v, "Longitude"),
            on_station: opt_b(v, "OnStation"),
            on_planet: opt_b(v, "OnPlanet"),
        }),
        "Disembark" => E::Disembark(Box::new(Disembark {
            srv: flag(v, "SRV"),
            taxi: flag(v, "Taxi"),
            multicrew: flag(v, "Multicrew"),
            star_system: opt_s(v, "StarSystem"),
            body: opt_s(v, "Body"),
            body_id: opt_i(v, "BodyID"),
            on_station: flag(v, "OnStation"),
            on_planet: flag(v, "OnPlanet"),
            station_name: opt_s(v, "StationName"),
            market_id: opt_i(v, "MarketID"),
        })),
        _ => return None,
    })
}
