//! Spansh system search (faction states near a point) and EDSM sphere queries (fill-in).
//! Neither API is formally documented; the Spansh filter shape was found by trial, and unknown
//! filter names are silently ignored, so every hit is re-checked in [`Query::matches`].

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const USER_AGENT: &str = concat!("astromech/", env!("CARGO_PKG_VERSION"), " (+https://github.com/spadaval/astromech)");

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new().timeout(Duration::from_secs(30)).user_agent(USER_AGENT).build()
}

/// What to look for: systems with one faction matching an allegiance (any if empty) and a
/// state (any if empty).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Query {
    pub id: &'static str,
    pub label: &'static str,
    pub allegiances: &'static [&'static str],
    pub states: &'static [&'static str],
    /// Drop systems below this population (Spansh's population filter is not used).
    pub min_population: i64,
}

/// Lookups the app offers: wake scanning (Famine) and the High Grade Emissions pools
/// (ids match `astromech_materials::hge` rules).
pub const QUERIES: &[Query] = &[
    Query {
        id: "famine",
        label: "Famine (distribution centres)",
        allegiances: &[],
        states: &["Famine"],
        min_population: 0,
    },
    Query {
        id: "fed_composites",
        label: "Federation faction",
        allegiances: &["Federation"],
        states: &[],
        min_population: 1_000_000,
    },
    Query {
        id: "empire_shielding",
        label: "Empire faction",
        allegiances: &["Empire"],
        states: &[],
        min_population: 1_000_000,
    },
    Query {
        id: "proto",
        label: "Independent or Alliance faction in Boom",
        allegiances: &["Independent", "Alliance"],
        states: &["Boom"],
        min_population: 0,
    },
    Query {
        id: "military",
        label: "Faction in War or Civil War",
        allegiances: &[],
        states: &["War", "Civil War"],
        min_population: 0,
    },
    Query {
        id: "improvised",
        label: "Independent or Alliance faction in Civil Unrest",
        allegiances: &["Independent", "Alliance"],
        states: &["Civil Unrest"],
        min_population: 0,
    },
    Query {
        id: "pharma",
        label: "Independent or Alliance faction in Outbreak",
        allegiances: &["Independent", "Alliance"],
        states: &["Outbreak"],
        min_population: 1_000_000,
    },
];

pub fn query(id: &str) -> Option<&'static Query> {
    QUERIES.iter().find(|q| q.id == id)
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
pub struct Hit {
    pub id64: i64,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[serde(default)]
    pub population: i64,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default, rename = "minor_faction_presences")]
    pub factions: Vec<Faction>,
}

impl Hit {
    pub fn pos(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl Query {
    pub fn faction_matches(&self, f: &Faction) -> bool {
        let al = self.allegiances.is_empty() || self.allegiances.iter().any(|a| a.eq_ignore_ascii_case(&f.allegiance));
        let st = self.states.is_empty()
            || self.states.iter().any(|s| {
                f.state.as_deref().is_some_and(|x| x.eq_ignore_ascii_case(s))
                    || f.active_states.iter().any(|x| x.eq_ignore_ascii_case(s))
            });
        al && st
    }

    pub fn matches(&self, h: &Hit) -> bool {
        h.population >= self.min_population && h.factions.iter().any(|f| self.faction_matches(f))
    }

    pub fn body(&self, origin: [f64; 3], size: usize) -> serde_json::Value {
        let mut presence = serde_json::Map::new();
        if !self.allegiances.is_empty() {
            presence.insert("allegiance".into(), serde_json::json!({ "value": self.allegiances }));
        }
        if !self.states.is_empty() {
            presence.insert("state".into(), serde_json::json!({ "value": self.states }));
        }
        serde_json::json!({
            "filters": { "minor_faction_presences": [presence] },
            "sort": [{ "distance": { "direction": "asc" } }],
            "size": size,
            "page": 0,
            "reference_coords": { "x": origin[0], "y": origin[1], "z": origin[2] },
        })
    }
}

#[derive(Deserialize)]
struct SearchResponse {
    #[serde(default)]
    results: Vec<Hit>,
    #[serde(default)]
    error: Option<String>,
}

pub fn parse_search(body: &str) -> Result<Vec<Hit>> {
    let r: SearchResponse = serde_json::from_str(body).context("Spansh search response")?;
    if let Some(e) = r.error {
        anyhow::bail!("Spansh: {e}");
    }
    Ok(r.results)
}

/// Raw response body (cached as-is).
pub fn spansh_search(q: &Query, origin: [f64; 3], size: usize) -> Result<String> {
    let resp = agent()
        .post("https://spansh.co.uk/api/systems/search")
        .send_json(q.body(origin, size))
        .context("Spansh search")?;
    Ok(resp.into_string()?)
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EdsmSystem {
    pub name: String,
    pub id64: Option<i64>,
    pub coords: Option<EdsmCoords>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct EdsmCoords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn parse_sphere(body: &str) -> Result<Vec<(i64, String, [f64; 3])>> {
    // EDSM answers `{}` or `[]` when it has nothing.
    let v: serde_json::Value = serde_json::from_str(body).context("EDSM sphere response")?;
    let Some(a) = v.as_array() else { return Ok(vec![]) };
    Ok(a.iter()
        .filter_map(|x| serde_json::from_value::<EdsmSystem>(x.clone()).ok())
        .filter_map(|s| Some((s.id64?, s.name, s.coords.map(|c| [c.x, c.y, c.z])?)))
        .collect())
}

/// Every system EDSM knows within `radius` ly (max 100) of `p`.
pub fn edsm_sphere(p: [f64; 3], radius: f64) -> Result<String> {
    let resp = agent()
        .get("https://www.edsm.net/api-v1/sphere-systems")
        .query("x", &p[0].to_string())
        .query("y", &p[1].to_string())
        .query("z", &p[2].to_string())
        .query("radius", &radius.min(100.0).to_string())
        .query("showCoordinates", "1")
        .query("showId", "1")
        .call()
        .context("EDSM sphere")?;
    Ok(resp.into_string()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_match_fixtures() {
        let hits = parse_search(include_str!("../tests/fixtures/spansh-famine.json")).unwrap();
        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].name, "WISE 0855-0714");
        let famine = query("famine").unwrap();
        assert!(hits.iter().all(|h| famine.matches(h)));
        assert!(
            !query("proto").unwrap().matches(&hits[1])
                || hits[1].factions.iter().any(|f| f.state.as_deref() == Some("Boom"))
        );
        let sphere = parse_sphere(include_str!("../tests/fixtures/edsm-sphere.json")).unwrap();
        assert!(sphere.iter().any(|(_, n, _)| n == "Alpha Centauri"));
        assert!(parse_sphere("{}").unwrap().is_empty());
        assert!(parse_search(r#"{"error":"bad filter"}"#).is_err());
        let b = query("proto").unwrap().body([1.0, 2.0, 3.0], 20);
        assert_eq!(b["filters"]["minor_faction_presences"][0]["state"]["value"][0], "Boom");
        assert_eq!(b["reference_coords"]["z"], 3.0);
    }
}
