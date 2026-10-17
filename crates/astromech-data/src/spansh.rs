//! Spansh: systems, stations and bodies search, and the neutron and trade route plotters.
//!
//! These APIs are undocumented. Request shapes were checked against spansh.co.uk on
//! 16 Oct 2026. Spansh silently ignores a filter it doesn't recognise (and a recognised name
//! with the wrong shape), returning unfiltered results, so callers should re-check hits with
//! the helpers on the result types ([`Station::has_service`], ...). Verified filter shapes:
//!
//! - `services`, `ships`: `[{"name": [..]}]` (`{"value": [..]}` is ignored);
//! - `modules`: `[{"name": [..], "class": [..], "rating": [..]}]` (any of the names);
//! - `market`: `[{"name": "Gold", "supply"|"demand": {"value": [min, max], "comparison": "<=>"}}]`;
//! - `distance`: `{"min": "0", "max": "40"}`;
//! - `material_trader`, `technology_broker`, `type` (stations): `{"value": [..]}` (`type` only
//!   includes; `"comparison": "not"` is ignored);
//! - `has_large_pad` (stations): `{"value": true}`;
//! - `distance_to_arrival`, `updated_at`, `outfitting_updated_at`, `shipyard_updated_at`
//!   (stations): `{"value": [min, max], "comparison": "<=>"}` (`{"min", "max"}` is ignored);
//! - `minor_faction_presences` (systems): `[{"allegiance": {"value": [..]}, "state": {"value": [..]}}]`;
//! - `subtype` (bodies): `{"value": [..]}`;
//! - `landmark_subtype` (bodies, codex points of interest such as "Crystalline Shards" or
//!   "Roseum Brain Tree"): `{"value": [..]}`; `landmark_type` with the same shape matched
//!   nothing (16 Oct 2026), so search by subtype;
//! - `signals` (bodies): `[{"name": ["Geological"]}]`;
//! - `is_landable` (bodies): `{"value": true}`;
//! - `ring_signals` (bodies, ring hotspots): `[{"name": ["Platinum"]}]` with the ore's name as
//!   the rings list it (`Void Opal`, not `Void Opals`; an unknown name matches nothing). It
//!   matches a body with the ore in any of its rings, not a particular ring, so check
//!   [`Ring::signals`] on the result. Each ring of a hit carries `signals: [{"name", "count"}]`
//!   (absent for a ring nobody has scanned) and `signals_updated_at` (checked 16 Oct 2026; a
//!   `rings` filter in any shape tried was ignored);
//! - `controlling_power`, `power` (any power active there, controller included) and
//!   `power_state` (systems; `Unoccupied`, `Exploited`, `Fortified`, `Stronghold`):
//!   `{"value": [..]}`, matching any listed value (checked 16 Oct 2026: an unknown filter name
//!   left the count at 204, `power_state: Unoccupied` cut it to 63).
//!
//! Systems answer Powerplay as `controlling_power`, `power` (list), `power_state`,
//! `power_state_control_progress` (a fraction of the state's band),
//! `power_state_reinforcement`, `power_state_undermining`, and for unoccupied systems
//! `power_conflict_progress: [{"power", "progress"}]`.
//!
//! System-name autocomplete (`/api/systems/field_values/system_names?q=`) answers
//! `{"values": [names], "min_max": [{id64, name, x, y, z}]}`, up to a handful of names that
//! resemble `q`; no match is two empty lists. The match is fuzzy, not a prefix match:
//! `q=Col 285 Sector AB-C b1` answered `Col 285 Sector AB-E b12-4`, so check names exactly
//! before treating a suggestion as the system typed.
//!
//! Station types, material trader and technology broker kinds are listed by
//! `/api/stations/field_values/{type,material_trader,technology_broker}` (see [`STATION_TYPES`]).
//! Module names, classes and ratings filter; the mount does not (`ed_symbol` is ignored), so
//! check it on [`Module::ed_symbol`].
//!
//! An unknown reference system is HTTP 400 `{"error": "Invalid request"}`. `count` is capped
//! at 10000.
//!
//! Route plotters are jobs: the submit (a form post) answers HTTP 202 `{"job", "status":
//! "queued"}`; `/api/results/{job}` answers HTTP 202 with `"status": "queued"` (`"state":
//! "started"`) while it works and HTTP 200 with `"status": "ok"` and the `result` when done
//! (checked 16 Oct 2026 for neutron and trade routes). An unknown job is HTTP 404
//! `{"error": "Unable to find route"}`. A trade job that finds nothing is done with an empty
//! `result` list.

use crate::{encode_segment, parse_json, ttl, Client, Error, Fetched, Request, Result, Service};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::time::{Duration, Instant};

/// Every station type Spansh knows (its `type` field values, 16 Oct 2026).
pub const STATION_TYPES: &[&str] = &[
    "Asteroid base",
    "Coriolis Starport",
    "Dockable Planet Station",
    "Dodec Starport",
    "Drake-Class Carrier",
    "Mega ship",
    "Ocellus Starport",
    "Orbis Starport",
    "Outpost",
    "Planetary Construction Depot",
    "Planetary Outpost",
    "Planetary Port",
    "Settlement",
    "Space Construction Depot",
    "Surface Settlement",
];
/// Fleet carriers' station type.
pub const CARRIER_TYPES: &[&str] = &["Drake-Class Carrier"];
/// Odyssey settlements' station types.
pub const SETTLEMENT_TYPES: &[&str] = &["Settlement", "Surface Settlement"];

/// Where a search measures distance from.
#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    System(String),
    Coords([f64; 3]),
}

/// A systems, stations or bodies search, nearest first.
#[derive(Debug, Clone, PartialEq)]
pub struct Search {
    pub reference: Reference,
    pub filters: Map<String, Value>,
    pub size: usize,
    pub page: usize,
}

impl Search {
    pub fn near(reference: Reference) -> Self {
        Search { reference, filters: Map::new(), size: 20, page: 0 }
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    /// Any filter, as Spansh's site sends it (see the module docs for verified shapes).
    pub fn filter(mut self, key: &str, value: Value) -> Self {
        self.filters.insert(key.to_string(), value);
        self
    }

    fn push(mut self, key: &str, item: Value) -> Self {
        match self.filters.entry(key).or_insert_with(|| json!([])) {
            Value::Array(a) => a.push(item),
            other => *other = json!([item]),
        }
        self
    }

    pub fn within(self, ly: f64) -> Self {
        self.filter("distance", json!({ "min": "0", "max": ly.to_string() }))
    }

    /// Stations with a material trader of `kind` ("Raw", "Manufactured", "Encoded").
    pub fn material_trader(self, kind: &str) -> Self {
        self.filter("material_trader", json!({ "value": [kind] }))
    }

    /// Stations with a technology broker of `kind` ("Human", "Guardian").
    pub fn technology_broker(self, kind: &str) -> Self {
        self.filter("technology_broker", json!({ "value": [kind] }))
    }

    /// Only stations of these types (see [`STATION_TYPES`]).
    pub fn types(self, types: &[&str]) -> Self {
        self.filter("type", json!({ "value": types }))
    }

    /// Only stations with a large pad.
    pub fn large_pad(self) -> Self {
        self.filter("has_large_pad", json!({ "value": true }))
    }

    /// Only stations at most `ls` light seconds from the arrival star. Stations Spansh has no
    /// distance for are kept.
    pub fn max_arrival(self, ls: f64) -> Self {
        self.filter("distance_to_arrival", json!({ "value": ["0", ls.to_string()], "comparison": "<=>" }))
    }

    /// Only stations whose `field` (`updated_at`, `outfitting_updated_at`,
    /// `shipyard_updated_at`) is at or after `since` (RFC 3339).
    pub fn updated_since(self, field: &str, since: &str) -> Self {
        self.filter(field, json!({ "value": [since, "2999-12-31T23:59:59Z"], "comparison": "<=>" }))
    }

    /// Stations offering a service, by Spansh's name (e.g. "Interstellar Factors Contact").
    pub fn service(self, name: &str) -> Self {
        self.push("services", json!({ "name": [name] }))
    }

    /// Stations selling a module under any of `names` (Spansh lists some modules under more
    /// than one). `class` 1–8; `rating` "A"–"I".
    pub fn module(self, names: &[&str], class: Option<u8>, rating: Option<&str>) -> Self {
        let mut m = json!({ "name": names });
        if let Some(c) = class {
            m["class"] = json!([c.to_string()]);
        }
        if let Some(r) = rating {
            m["rating"] = json!([r]);
        }
        self.push("modules", m)
    }

    /// Stations selling a ship, by name (e.g. "Mandalay").
    pub fn ship(self, name: &str) -> Self {
        self.push("ships", json!({ "name": [name] }))
    }

    /// Stations with at least `min` units of `commodity` for sale.
    pub fn supply(self, commodity: &str, min: u64) -> Self {
        self.market(commodity, "supply", min)
    }

    /// Stations buying at least `min` units of `commodity`.
    pub fn demand(self, commodity: &str, min: u64) -> Self {
        self.market(commodity, "demand", min)
    }

    fn market(self, commodity: &str, side: &str, min: u64) -> Self {
        let range = json!({ "value": [min.to_string(), "999999999"], "comparison": "<=>" });
        self.push("market", json!({ "name": commodity, side: range }))
    }

    /// Systems controlled by any of `powers`.
    /// Bodies with a ring holding a hotspot of `ore` (the name the rings list, e.g. "Platinum").
    pub fn ring_signal(self, ore: &str) -> Self {
        self.push("ring_signals", json!({ "name": [ore] }))
    }

    pub fn controlling_power(self, powers: &[&str]) -> Self {
        self.filter("controlling_power", json!({ "value": powers }))
    }

    /// Systems where any of `powers` is active (controlling, acquiring or undermining).
    pub fn power(self, powers: &[&str]) -> Self {
        self.filter("power", json!({ "value": powers }))
    }

    /// Systems in any of these Powerplay states.
    pub fn power_state(self, states: &[&str]) -> Self {
        self.filter("power_state", json!({ "value": states }))
    }

    /// The JSON body Spansh's site posts.
    pub fn body(&self) -> Value {
        let mut b = json!({
            "filters": self.filters,
            "sort": [{ "distance": { "direction": "asc" } }],
            "size": self.size,
            "page": self.page,
        });
        match &self.reference {
            Reference::System(s) => b["reference_system"] = json!(s),
            Reference::Coords(p) => b["reference_coords"] = json!({ "x": p[0], "y": p[1], "z": p[2] }),
        }
        b
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    /// Total matches, capped at 10000.
    pub count: u64,
    pub results: Vec<T>,
    #[serde(default)]
    pub reference: Option<ReferenceSystem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceSystem {
    pub name: String,
    pub id64: i64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Faction {
    pub name: String,
    #[serde(default)]
    pub allegiance: String,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub active_states: Vec<String>,
    #[serde(default)]
    pub influence: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub id64: i64,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[serde(default)]
    pub distance: Option<f64>,
    #[serde(default)]
    pub population: i64,
    #[serde(default)]
    pub allegiance: Option<String>,
    #[serde(default)]
    pub controlling_power: Option<String>,
    #[serde(default)]
    pub power_state: Option<String>,
    /// Every power active in the system, the controller included.
    #[serde(default)]
    pub power: Vec<String>,
    /// Progress through the state's band, a fraction.
    #[serde(default)]
    pub power_state_control_progress: Option<f64>,
    #[serde(default)]
    pub power_state_reinforcement: Option<i64>,
    #[serde(default)]
    pub power_state_undermining: Option<i64>,
    /// Unoccupied systems: each power's acquisition progress.
    #[serde(default)]
    pub power_conflict_progress: Vec<PowerProgress>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub minor_faction_presences: Vec<Faction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerProgress {
    pub power: String,
    pub progress: f64,
}

impl System {
    pub fn pos(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Named {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub class: u8,
    pub rating: String,
    #[serde(default)]
    pub category: Option<String>,
    /// Ship-specific modules (armour) name their ship.
    #[serde(default)]
    pub ship: Option<String>,
    #[serde(default)]
    pub price: Option<i64>,
    /// The journal symbol, e.g. `Hpt_MultiCannon_Gimbal_Medium`: the only place the mount shows.
    #[serde(default)]
    pub ed_symbol: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    pub name: String,
    #[serde(default)]
    pub price: Option<i64>,
    /// The journal symbol (`Krait_MkII`).
    #[serde(default)]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commodity {
    pub commodity: String,
    #[serde(default)]
    pub category: Option<String>,
    pub buy_price: i64,
    pub sell_price: i64,
    pub supply: i64,
    pub demand: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    pub name: String,
    #[serde(default)]
    pub market_id: Option<i64>,
    #[serde(default, rename = "type")]
    pub station_type: Option<String>,
    pub system_name: String,
    pub system_id64: i64,
    pub system_x: f64,
    pub system_y: f64,
    pub system_z: f64,
    /// Light years from the reference.
    pub distance: f64,
    /// Light seconds from the arrival star.
    #[serde(default)]
    pub distance_to_arrival: Option<f64>,
    #[serde(default)]
    pub has_large_pad: Option<bool>,
    #[serde(default)]
    pub large_pads: Option<u32>,
    #[serde(default)]
    pub medium_pads: Option<u32>,
    #[serde(default)]
    pub small_pads: Option<u32>,
    #[serde(default)]
    pub is_planetary: Option<bool>,
    /// "Raw", "Manufactured" or "Encoded".
    #[serde(default)]
    pub material_trader: Option<String>,
    /// "Human" or "Guardian".
    #[serde(default)]
    pub technology_broker: Option<String>,
    #[serde(default)]
    pub services: Vec<Named>,
    #[serde(default)]
    pub modules: Vec<Module>,
    #[serde(default)]
    pub ships: Vec<Ship>,
    #[serde(default)]
    pub market: Vec<Commodity>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub market_updated_at: Option<String>,
    #[serde(default)]
    pub outfitting_updated_at: Option<String>,
    #[serde(default)]
    pub shipyard_updated_at: Option<String>,
}

impl Station {
    pub fn pos(&self) -> [f64; 3] {
        [self.system_x, self.system_y, self.system_z]
    }

    pub fn has_service(&self, name: &str) -> bool {
        self.services.iter().any(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn sells_module(&self, name: &str, class: Option<u8>, rating: Option<&str>) -> bool {
        self.modules.iter().any(|m| {
            m.name.eq_ignore_ascii_case(name)
                && class.is_none_or(|c| m.class == c)
                && rating.is_none_or(|r| m.rating.eq_ignore_ascii_case(r))
        })
    }

    pub fn sells_ship(&self, name: &str) -> bool {
        self.ships.iter().any(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn commodity(&self, name: &str) -> Option<&Commodity> {
        self.market.iter().find(|c| c.commodity.eq_ignore_ascii_case(name))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub count: u32,
}

/// A surface point of interest Spansh knows from the codex: a crystalline shard site, a brain
/// tree, a geyser, a crashed ship, ...
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Landmark {
    /// e.g. "Shards", "Brain Tree", "Geyser", "Crashed Ship".
    #[serde(rename = "type")]
    pub landmark_type: String,
    /// e.g. "Crystalline Shards", "Roseum Brain Tree", "Water Ice Geyser".
    pub subtype: String,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
}

/// One element's share of a landable body's surface, in percent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Share {
    pub name: String,
    pub share: f64,
}

/// A ring of a body, with the hotspots scanned in it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ring {
    /// e.g. "Barnard's Star 5 A Ring".
    pub name: String,
    /// "Icy", "Rocky", "Metallic", "Metal Rich".
    #[serde(rename = "type")]
    pub ring_type: String,
    #[serde(default)]
    pub signals: Vec<Signal>,
    #[serde(default)]
    pub signals_updated_at: Option<String>,
}

impl Ring {
    /// Hotspots of `ore` in this ring (0 if none).
    pub fn hotspots(&self, ore: &str) -> u32 {
        self.signals.iter().filter(|s| s.name.eq_ignore_ascii_case(ore)).map(|s| s.count).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub id64: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub body_type: String,
    #[serde(default)]
    pub subtype: Option<String>,
    pub system_name: String,
    pub system_id64: i64,
    pub system_x: f64,
    pub system_y: f64,
    pub system_z: f64,
    pub distance: f64,
    #[serde(default)]
    pub distance_to_arrival: Option<f64>,
    #[serde(default)]
    pub is_landable: Option<bool>,
    #[serde(default)]
    pub gravity: Option<f64>,
    #[serde(default)]
    pub terraforming_state: Option<String>,
    #[serde(default)]
    pub signals: Vec<Signal>,
    #[serde(default)]
    pub rings: Vec<Ring>,
    #[serde(default)]
    pub estimated_mapping_value: Option<i64>,
    #[serde(default)]
    pub estimated_scan_value: Option<i64>,
    #[serde(default)]
    pub landmarks: Vec<Landmark>,
    #[serde(default)]
    pub materials: Vec<Share>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

impl Body {
    pub fn pos(&self) -> [f64; 3] {
        [self.system_x, self.system_y, self.system_z]
    }

    pub fn has_signal(&self, name: &str) -> bool {
        self.signals.iter().any(|s| s.name.eq_ignore_ascii_case(name) && s.count > 0)
    }
}

/// An autocomplete suggestion: a system Spansh knows, with its position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemName {
    pub id64: i64,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl SystemName {
    pub fn pos(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Deserialize)]
struct SystemNames {
    min_max: Vec<SystemName>,
}

/// A neutron-boosted route request.
#[derive(Debug, Clone, PartialEq)]
pub struct NeutronRoute {
    pub from: String,
    pub to: String,
    /// Laden jump range in ly.
    pub range: f64,
    /// 1–100: how far Spansh may deviate to use neutron stars.
    pub efficiency: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub job: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jump {
    pub system: String,
    pub id64: i64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Jumps to reach this waypoint from the previous one.
    pub jumps: u32,
    pub distance_jumped: f64,
    pub distance_left: f64,
    pub neutron_star: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeutronPlan {
    pub source_system: String,
    pub destination_system: String,
    pub distance: f64,
    pub total_jumps: u32,
    pub system_jumps: Vec<Jump>,
}

/// Where a Spansh job (a route plotted on its servers) stands.
#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus<T> {
    /// Still being worked out (`"status": "queued"`, HTTP 202): ask again in a few seconds.
    Pending,
    Done(T),
}

pub type RouteStatus = JobStatus<NeutronPlan>;

#[derive(Deserialize)]
struct ResultEnvelope {
    status: String,
    #[serde(default)]
    result: Option<Value>,
}

/// A `/api/results/{job}` answer. A failed job is an error object (`{"error": ...}`, as the
/// site handles it), which [`parse_json`] turns into [`Error::Rejected`].
fn parse_job_status<T: DeserializeOwned>(body: &str) -> Result<JobStatus<T>> {
    let s = Service::Spansh;
    let env: ResultEnvelope = parse_json(s, body)?;
    match (env.status.as_str(), env.result) {
        ("queued", _) => Ok(JobStatus::Pending),
        ("ok", Some(r)) => serde_json::from_value(r)
            .map(JobStatus::Done)
            .map_err(|e| Error::Schema { service: s, detail: e.to_string() }),
        (other, _) => Err(Error::Schema { service: s, detail: format!("unexpected job status {other:?}") }),
    }
}

/// A trade route request (`/api/trade/route`), as Spansh's trade router sends it (checked
/// 16 Oct 2026). Spansh starts from a station and needs both names: a missing one is HTTP 400
/// `{"error": "system and station are required"}`, an unknown station
/// `{"error": "Could not find station"}`. It has no loop option (a `loop` field is ignored),
/// only a large-pad switch for pads, and no figure for profit per hour.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeRoute {
    pub system: String,
    pub station: String,
    /// Legs to plan, each a buy at one station and a sale at the next.
    pub max_hops: u32,
    /// Light years between the systems of consecutive stations.
    pub max_hop_distance: f64,
    pub starting_capital: i64,
    /// Tonnes.
    pub max_cargo: u32,
    /// Light seconds from the arrival star.
    pub max_system_distance: u32,
    /// Seconds; market data older than this is ignored. `None`: any age.
    pub max_price_age: Option<u64>,
    pub requires_large_pad: bool,
    /// Planetary ports and settlements.
    pub allow_planetary: bool,
    /// Fleet carriers.
    pub allow_player_owned: bool,
}

impl TradeRoute {
    fn form(&self) -> Vec<(String, String)> {
        let flag = |b: bool| if b { "1" } else { "0" }.to_string();
        let mut f = vec![
            ("system".into(), self.system.clone()),
            ("station".into(), self.station.clone()),
            ("max_hops".into(), self.max_hops.to_string()),
            ("max_hop_distance".into(), self.max_hop_distance.to_string()),
            ("starting_capital".into(), self.starting_capital.to_string()),
            ("max_cargo".into(), self.max_cargo.to_string()),
            ("max_system_distance".into(), self.max_system_distance.to_string()),
            ("requires_large_pad".into(), flag(self.requires_large_pad)),
            ("allow_prohibited".into(), flag(false)),
            ("allow_planetary".into(), flag(self.allow_planetary)),
            ("allow_player_owned".into(), flag(self.allow_player_owned)),
            ("allow_restricted_access".into(), flag(false)),
            ("unique".into(), flag(false)),
            ("permit".into(), flag(false)),
        ];
        if let Some(age) = self.max_price_age {
            f.push(("max_price_age".into(), age.to_string()));
        }
        f
    }
}

/// A station on a trade route.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeStop {
    pub station: String,
    pub system: String,
    pub system_id64: i64,
    pub market_id: i64,
    /// Light seconds from the arrival star.
    #[serde(default)]
    pub distance_to_arrival: Option<f64>,
    /// Unix seconds.
    #[serde(default)]
    pub market_updated_at: Option<i64>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// One market's side of a commodity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradePrice {
    pub buy_price: i64,
    pub sell_price: i64,
    pub supply: i64,
    pub demand: i64,
}

/// A commodity carried on a leg.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeCargo {
    /// Display name, e.g. "Biowaste".
    pub name: String,
    /// Tonnes.
    pub amount: i64,
    /// Credits per tonne.
    pub profit: i64,
    pub total_profit: i64,
    pub source_commodity: TradePrice,
    pub destination_commodity: TradePrice,
}

/// One leg: buy at `source`, sell at `destination`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeHop {
    pub source: TradeStop,
    pub destination: TradeStop,
    /// Light years between the two systems.
    pub distance: f64,
    pub commodities: Vec<TradeCargo>,
    pub total_profit: i64,
    pub cumulative_profit: i64,
}

/// How long [`Client::spansh_trade_route_wait`] waits between polls and in all.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Polling {
    pub every: Duration,
    pub timeout: Duration,
}

impl Default for Polling {
    /// Trade jobs took 25–70 s on 16 Oct 2026.
    fn default() -> Self {
        Polling { every: Duration::from_secs(3), timeout: Duration::from_secs(180) }
    }
}

impl Client {
    pub fn spansh_systems(&self, q: &Search) -> Result<Fetched<Page<System>>> {
        self.fetch(Service::Spansh, &Request::post_json("/api/systems/search", q.body()), Some(ttl::SEARCH))
    }

    pub fn spansh_stations(&self, q: &Search) -> Result<Fetched<Page<Station>>> {
        self.fetch(Service::Spansh, &Request::post_json("/api/stations/search", q.body()), Some(ttl::SEARCH))
    }

    pub fn spansh_bodies(&self, q: &Search) -> Result<Fetched<Page<Body>>> {
        self.fetch(Service::Spansh, &Request::post_json("/api/bodies/search", q.body()), Some(ttl::SEARCH))
    }

    /// Systems whose names resemble `prefix` (Spansh's fuzzy autocomplete, not only names
    /// starting with it), with positions. A blank prefix answers nothing without a request.
    pub fn spansh_system_names(&self, prefix: &str) -> Result<Fetched<Vec<SystemName>>> {
        let prefix = prefix.trim();
        if prefix.is_empty() {
            return Ok(Fetched { value: vec![], fetched_at: self.clock.now_s(), cached: true });
        }
        let req = Request::get("/api/systems/field_values/system_names").query("q", prefix);
        self.fetch_with(Service::Spansh, &req, Some(ttl::SEARCH), |b| {
            Ok(parse_json::<SystemNames>(Service::Spansh, b)?.min_max)
        })
    }

    /// Queue a neutron route; poll [`Client::spansh_route_result`] with the job.
    pub fn spansh_route(&self, r: &NeutronRoute) -> Result<Job> {
        let form = vec![
            ("from".into(), r.from.clone()),
            ("to".into(), r.to.clone()),
            ("range".into(), r.range.to_string()),
            ("efficiency".into(), r.efficiency.clamp(1, 100).to_string()),
        ];
        Ok(self.fetch(Service::Spansh, &Request::post_form("/api/route", form), None)?.value)
    }

    /// Not cached: a pending answer must be asked again, and a finished plan is the caller's.
    pub fn spansh_route_result(&self, job: &str) -> Result<RouteStatus> {
        self.spansh_job(job)
    }

    /// Queue a trade route; poll [`Client::spansh_trade_result`] with the job.
    pub fn spansh_trade_route(&self, r: &TradeRoute) -> Result<Job> {
        Ok(self.fetch(Service::Spansh, &Request::post_form("/api/trade/route", r.form()), None)?.value)
    }

    /// A trade job's legs once done. An empty list: no route fits the request.
    pub fn spansh_trade_result(&self, job: &str) -> Result<JobStatus<Vec<TradeHop>>> {
        self.spansh_job(job)
    }

    /// Queue a trade route and poll until it is done. Blocks (run it off the UI thread); past
    /// `polling.timeout` it gives up with [`Error::Timeout`].
    pub fn spansh_trade_route_wait(&self, r: &TradeRoute, polling: Polling) -> Result<Vec<TradeHop>> {
        let start = Instant::now();
        let job = self.spansh_trade_route(r)?;
        loop {
            if let JobStatus::Done(hops) = self.spansh_trade_result(&job.job)? {
                return Ok(hops);
            }
            if start.elapsed() + polling.every > polling.timeout {
                return Err(Error::Timeout { service: Service::Spansh });
            }
            std::thread::sleep(polling.every);
        }
    }

    /// Not cached: a pending answer must be asked again, and a finished one is the caller's.
    fn spansh_job<T: DeserializeOwned>(&self, job: &str) -> Result<JobStatus<T>> {
        let req = Request::get(format!("/api/results/{}", encode_segment(job)));
        Ok(self.fetch_with(Service::Spansh, &req, None, parse_job_status)?.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_body_uses_verified_shapes() {
        let b = Search::near(Reference::System("Sol".into()))
            .service("Interstellar Factors Contact")
            .module(&["Frame Shift Drive"], Some(5), Some("A"))
            .ship("Mandalay")
            .supply("Gold", 1000)
            .within(40.0)
            .size(3)
            .body();
        assert_eq!(b["reference_system"], "Sol");
        assert_eq!(b["filters"]["services"][0]["name"][0], "Interstellar Factors Contact");
        assert_eq!(b["filters"]["modules"][0]["class"][0], "5");
        assert_eq!(b["filters"]["ships"][0]["name"][0], "Mandalay");
        assert_eq!(b["filters"]["market"][0]["supply"]["value"][0], "1000");
        assert_eq!(b["filters"]["distance"]["max"], "40");
        assert_eq!(b["size"], 3);
        let s = Search::near(Reference::System("Sol".into()))
            .material_trader("Raw")
            .technology_broker("Guardian")
            .types(&["Outpost"])
            .large_pad()
            .max_arrival(1000.0)
            .updated_since("outfitting_updated_at", "2026-10-15T00:00:00Z")
            .body();
        let f = &s["filters"];
        assert_eq!(f["material_trader"]["value"][0], "Raw");
        assert_eq!(f["technology_broker"]["value"][0], "Guardian");
        assert_eq!(f["type"]["value"][0], "Outpost");
        assert_eq!(f["has_large_pad"]["value"], true);
        assert_eq!(f["distance_to_arrival"], json!({ "value": ["0", "1000"], "comparison": "<=>" }));
        assert_eq!(f["outfitting_updated_at"]["value"][0], "2026-10-15T00:00:00Z");
        assert_eq!(f["outfitting_updated_at"]["comparison"], "<=>");
        let p = Search::near(Reference::System("Lembava".into()))
            .power_state(&["Unoccupied"])
            .power(&["Li Yong-Rui"])
            .controlling_power(&["Pranav Antal", "Yuri Grom"])
            .body();
        assert_eq!(p["filters"]["power_state"]["value"][0], "Unoccupied");
        assert_eq!(p["filters"]["power"]["value"][0], "Li Yong-Rui");
        assert_eq!(p["filters"]["controlling_power"]["value"][1], "Yuri Grom");
        let c = Search::near(Reference::Coords([1.0, 2.0, 3.0])).body();
        assert_eq!(c["reference_coords"]["z"], 3.0);
        assert!(c.get("reference_system").is_none());
    }
}
