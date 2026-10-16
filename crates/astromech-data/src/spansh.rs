//! Spansh: systems, stations and bodies search, and the neutron route plotter.
//!
//! These APIs are undocumented. Request shapes were checked against spansh.co.uk on
//! 16 Oct 2026. Spansh silently ignores a filter it doesn't recognise (and a recognised name
//! with the wrong shape), returning unfiltered results, so callers should re-check hits with
//! the helpers on the result types ([`Station::has_service`], ...). Verified filter shapes:
//!
//! - `services`, `ships`: `[{"name": [..]}]` (`{"value": [..]}` is ignored);
//! - `modules`: `[{"name": [..], "class": [..], "rating": [..]}]`;
//! - `market`: `[{"name": "Gold", "supply"|"demand": {"value": [min, max], "comparison": "<=>"}}]`;
//! - `distance`: `{"min": "0", "max": "40"}`;
//! - `minor_faction_presences` (systems): `[{"allegiance": {"value": [..]}, "state": {"value": [..]}}]`;
//! - `subtype` (bodies): `{"value": [..]}`.
//!
//! An unknown reference system is HTTP 400 `{"error": "Invalid request"}`. `count` is capped
//! at 10000.

use crate::{encode_segment, parse_json, ttl, Client, Error, Fetched, Request, Result, Service};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

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

    /// Stations offering a service, by Spansh's name (e.g. "Interstellar Factors Contact").
    pub fn service(self, name: &str) -> Self {
        self.push("services", json!({ "name": [name] }))
    }

    /// Stations selling a module. `class` 1–8; `rating` "A"–"I".
    pub fn module(self, name: &str, class: Option<u8>, rating: Option<&str>) -> Self {
        let mut m = json!({ "name": [name] });
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
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub minor_faction_presences: Vec<Faction>,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    pub name: String,
    #[serde(default)]
    pub price: Option<i64>,
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
    pub is_planetary: Option<bool>,
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
    pub estimated_mapping_value: Option<i64>,
    #[serde(default)]
    pub estimated_scan_value: Option<i64>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum RouteStatus {
    /// Still being plotted: ask again in a few seconds.
    Pending,
    Done(NeutronPlan),
}

#[derive(Deserialize)]
struct ResultEnvelope {
    status: String,
    #[serde(default)]
    result: Option<Value>,
}

fn parse_route_status(body: &str) -> Result<RouteStatus> {
    let s = Service::Spansh;
    let env: ResultEnvelope = parse_json(s, body)?;
    match (env.status.as_str(), env.result) {
        ("queued", _) => Ok(RouteStatus::Pending),
        ("ok", Some(r)) => serde_json::from_value(r)
            .map(RouteStatus::Done)
            .map_err(|e| Error::Schema { service: s, detail: e.to_string() }),
        (other, _) => Err(Error::Schema { service: s, detail: format!("unexpected route status {other:?}") }),
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
        let req = Request::get(format!("/api/results/{}", encode_segment(job)));
        Ok(self.fetch_with(Service::Spansh, &req, None, parse_route_status)?.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_body_uses_verified_shapes() {
        let b = Search::near(Reference::System("Sol".into()))
            .service("Interstellar Factors Contact")
            .module("Frame Shift Drive", Some(5), Some("A"))
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
        let c = Search::near(Reference::Coords([1.0, 2.0, 3.0])).body();
        assert_eq!(c["reference_coords"]["z"], 3.0);
        assert!(c.get("reference_system").is_none());
    }
}
