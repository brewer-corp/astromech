//! What the Route page says about an inhabited stop: allegiance and government, security,
//! population, and the controlling faction and its state, from Spansh's systems search. A
//! route's stops are asked for by name, up to [`CHUNK`] in one request; a system Spansh doesn't
//! know, or one with no population, has no details.

use astromech_data::spansh::{Reference, Search};
use astromech_data::Client;
use serde::Serialize;
use serde_json::json;

/// Names asked for in one request.
pub const CHUNK: usize = 50;

#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[ts(rename = "StopDetails")]
pub struct Details {
    pub population: i64,
    pub allegiance: Option<String>,
    pub government: Option<String>,
    pub security: Option<String>,
    /// The controlling faction.
    pub faction: Option<String>,
    /// Its state; None when it has none (Spansh's "None").
    pub faction_state: Option<String>,
}

/// The details of each of `names` that Spansh has and that is inhabited, by the name as given
/// (matched in any case). One request per [`CHUNK`] names; the first failure is the error.
pub fn details(remote: &Client, names: &[String]) -> astromech_data::Result<Vec<(String, Details)>> {
    let mut out = vec![];
    for chunk in names.chunks(CHUNK) {
        let search = Search::near(Reference::System(chunk[0].clone()))
            .size(chunk.len())
            .filter("name", json!({ "value": chunk }));
        for s in remote.spansh_systems(&search)?.value.results {
            let Some(name) = chunk.iter().find(|n| n.trim().eq_ignore_ascii_case(&s.name)) else { continue };
            if s.population <= 0 {
                continue;
            }
            let some = |v: Option<String>| v.filter(|v| !v.is_empty() && v != "None");
            out.push((
                name.clone(),
                Details {
                    population: s.population,
                    allegiance: some(s.allegiance),
                    government: some(s.government),
                    security: some(s.security),
                    faction: some(s.controlling_minor_faction),
                    faction_state: some(s.controlling_minor_faction_state),
                },
            ));
        }
    }
    Ok(out)
}
