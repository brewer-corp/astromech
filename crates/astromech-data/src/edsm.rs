//! EDSM: system position, bodies, traffic and the sphere of systems around a point. Shapes
//! checked against www.edsm.net on 16 Oct 2026. EDSM answers an unknown system with `[]` or
//! `{}` and HTTP 200; that becomes [`Error::NotFound`]. The commander flight log needs a
//! per-commander API key and is not wrapped yet.

use crate::{parse_json, ttl, Client, Error, Fetched, Request, Result, Service};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// EDSM's sphere search caps the radius at 100 ly.
pub const MAX_SPHERE_LY: f64 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coords {
    pub fn pos(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub name: String,
    pub id64: i64,
    pub coords: Coords,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub id64: Option<i64>,
    pub body_id: Option<i64>,
    pub name: String,
    #[serde(rename = "type")]
    pub body_type: String,
    pub sub_type: Option<String>,
    pub distance_to_arrival: f64,
    #[serde(default)]
    pub is_landable: Option<bool>,
    #[serde(default)]
    pub gravity: Option<f64>,
    #[serde(default)]
    pub terraforming_state: Option<String>,
    #[serde(default)]
    pub is_scoopable: Option<bool>,
    #[serde(default)]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bodies {
    pub id64: i64,
    pub name: String,
    #[serde(default)]
    pub body_count: Option<u32>,
    pub bodies: Vec<Body>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TrafficCounts {
    pub total: u64,
    pub week: u64,
    pub day: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Traffic {
    pub id64: i64,
    pub name: String,
    pub traffic: TrafficCounts,
    /// Ships seen in the last day, by type.
    #[serde(default)]
    pub breakdown: std::collections::BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SphereSystem {
    pub name: String,
    pub id64: Option<i64>,
    pub coords: Option<Coords>,
}

/// EDSM's "nothing": `[]`, `{}`, or `null`.
fn is_empty(v: &serde_json::Value) -> bool {
    v.is_null() || v.as_array().is_some_and(|a| a.is_empty()) || v.as_object().is_some_and(|o| o.is_empty())
}

fn parse_known<T: DeserializeOwned>(body: &str, system: &str) -> Result<T> {
    let v: serde_json::Value = parse_json(Service::Edsm, body)?;
    if is_empty(&v) {
        return Err(Error::NotFound { service: Service::Edsm, message: format!("no system named {system}") });
    }
    serde_json::from_value(v).map_err(|e| Error::Schema { service: Service::Edsm, detail: e.to_string() })
}

impl Client {
    /// A system's id64 and position.
    pub fn edsm_system(&self, name: &str) -> Result<Fetched<System>> {
        let req =
            Request::get("/api-v1/system").query("systemName", name).query("showCoordinates", 1).query("showId", 1);
        self.fetch_with(Service::Edsm, &req, Some(ttl::SYSTEM), |b| parse_known(b, name))
    }

    pub fn edsm_bodies(&self, system: &str) -> Result<Fetched<Bodies>> {
        let req = Request::get("/api-system-v1/bodies").query("systemName", system);
        self.fetch_with(Service::Edsm, &req, Some(ttl::SCANNED), |b| parse_known(b, system))
    }

    pub fn edsm_traffic(&self, system: &str) -> Result<Fetched<Traffic>> {
        let req = Request::get("/api-system-v1/traffic").query("systemName", system);
        self.fetch_with(Service::Edsm, &req, Some(ttl::TRAFFIC), |b| parse_known(b, system))
    }

    /// Every system EDSM knows within `radius` ly (capped at [`MAX_SPHERE_LY`]) of `p`. Systems
    /// EDSM lists without an id64 or position are kept as `None`s for the caller to skip. Not
    /// cached: callers store the systems themselves, and an empty answer should be retried.
    pub fn edsm_sphere(&self, p: [f64; 3], radius: f64) -> Result<Fetched<Vec<SphereSystem>>> {
        let req = Request::get("/api-v1/sphere-systems")
            .query("x", p[0])
            .query("y", p[1])
            .query("z", p[2])
            .query("radius", radius.clamp(0.0, MAX_SPHERE_LY))
            .query("showCoordinates", 1)
            .query("showId", 1);
        self.fetch_with(Service::Edsm, &req, None, |b| {
            let v: serde_json::Value = parse_json(Service::Edsm, b)?;
            if is_empty(&v) {
                return Ok(vec![]);
            }
            serde_json::from_value(v).map_err(|e| Error::Schema { service: Service::Edsm, detail: e.to_string() })
        })
    }
}
