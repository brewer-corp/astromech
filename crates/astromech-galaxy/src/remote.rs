//! Faction-state lookups (which systems near a point have a faction in a given state) over
//! Spansh's systems search, through `astromech-data`. Spansh ignores filter names it doesn't
//! know, so every hit is re-checked in [`Query::matches`].

use anyhow::Result;
use astromech_data::{spansh, Client, Request, Service};
use serde::Serialize;

pub use astromech_data::spansh::{Faction, System as Hit};

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
    /// Rank by [`crate::hge_score`] (population and distance) instead of jumps: High Grade
    /// Emissions spawn reliably only in systems over a billion people.
    pub by_population: bool,
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
        by_population: false,
    },
    Query {
        id: "fed_composites",
        label: "Federation faction",
        allegiances: &["Federation"],
        states: &[],
        min_population: 1_000_000,
        by_population: true,
    },
    Query {
        id: "empire_shielding",
        label: "Empire faction",
        allegiances: &["Empire"],
        states: &[],
        min_population: 1_000_000,
        by_population: true,
    },
    Query {
        id: "proto",
        label: "Independent or Alliance faction in Boom",
        allegiances: &["Independent", "Alliance"],
        states: &["Boom"],
        min_population: 0,
        by_population: true,
    },
    Query {
        id: "military",
        label: "Faction in War or Civil War",
        allegiances: &[],
        states: &["War", "Civil War"],
        min_population: 0,
        by_population: true,
    },
    Query {
        id: "improvised",
        label: "Independent or Alliance faction in Civil Unrest",
        allegiances: &["Independent", "Alliance"],
        states: &["Civil Unrest"],
        min_population: 0,
        by_population: true,
    },
    Query {
        id: "pharma",
        label: "Independent or Alliance faction in Outbreak",
        allegiances: &["Independent", "Alliance"],
        states: &["Outbreak"],
        min_population: 1_000_000,
        by_population: true,
    },
];

pub fn query(id: &str) -> Option<&'static Query> {
    QUERIES.iter().find(|q| q.id == id)
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
        h.population >= self.min_population && h.minor_faction_presences.iter().any(|f| self.faction_matches(f))
    }

    /// The search Spansh's site posts, nearest first. `min_population`: only systems with at
    /// least this many people (checked 17 Oct 2026: from Deciat the Federation search went from
    /// 10000 matches to 2117, all 50 returned over a billion).
    /// `updated_since` (RFC 3339): only systems updated at or after it (checked 17 Oct 2026:
    /// from Sol, 10000 matches became 562 for the last 40 minutes).
    pub fn body(
        &self,
        origin: [f64; 3],
        size: usize,
        min_population: Option<i64>,
        updated_since: Option<&str>,
    ) -> serde_json::Value {
        let mut presence = serde_json::Map::new();
        if !self.allegiances.is_empty() {
            presence.insert("allegiance".into(), serde_json::json!({ "value": self.allegiances }));
        }
        if !self.states.is_empty() {
            presence.insert("state".into(), serde_json::json!({ "value": self.states }));
        }
        let mut filters = serde_json::json!({ "minor_faction_presences": [presence] });
        if let Some(min) = min_population {
            filters["population"] =
                serde_json::json!({ "comparison": "<=>", "value": [min.to_string(), "100000000000"] });
        }
        if let Some(since) = updated_since {
            filters["updated_at"] =
                serde_json::json!({ "comparison": "<=>", "value": [since, "2999-12-31T23:59:59Z"] });
        }
        serde_json::json!({
            "filters": filters,
            "sort": [{ "distance": { "direction": "asc" } }],
            "size": size,
            "page": 0,
            "reference_coords": { "x": origin[0], "y": origin[1], "z": origin[2] },
        })
    }
}

pub fn parse_search(body: &str) -> Result<Vec<Hit>> {
    Ok(astromech_data::parse_json::<spansh::Page<Hit>>(Service::Spansh, body)?.results)
}

/// Raw response body (cached as-is by the caller).
pub fn spansh_search(
    client: &Client,
    q: &Query,
    origin: [f64; 3],
    size: usize,
    min_population: Option<i64>,
    updated_since: Option<&str>,
) -> Result<String> {
    Ok(client.send(
        Service::Spansh,
        &Request::post_json("/api/systems/search", q.body(origin, size, min_population, updated_since)),
    )?)
}

/// Every system EDSM knows within `radius` ly (max 100) of `p`, as `(id64, name, position)`.
pub fn edsm_sphere(client: &Client, p: [f64; 3], radius: f64) -> Result<Vec<(i64, String, [f64; 3])>> {
    Ok(client
        .edsm_sphere(p, radius)?
        .value
        .into_iter()
        .filter_map(|s| Some((s.id64?, s.name, s.coords?.pos())))
        .collect())
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
                || hits[1].minor_faction_presences.iter().any(|f| f.state.as_deref() == Some("Boom"))
        );
        assert!(parse_search(r#"{"error":"bad filter"}"#).is_err());
        let b = query("proto").unwrap().body([1.0, 2.0, 3.0], 20, None, None);
        assert!(b["filters"].get("population").is_none());
        let big = query("fed_composites").unwrap().body([0.0; 3], 50, Some(1_000_000_000), None);
        assert_eq!(big["filters"]["population"]["value"][0], "1000000000");
        assert_eq!(big["filters"]["population"]["comparison"], "<=>");
        assert_eq!(b["filters"]["minor_faction_presences"][0]["state"]["value"][0], "Boom");
        assert_eq!(b["reference_coords"]["z"], 3.0);
    }
}
