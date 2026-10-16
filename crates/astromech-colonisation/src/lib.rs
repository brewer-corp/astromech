//! # astromech-colonisation
//!
//! Construction tracker (journal only). Folds `ColonisationConstructionDepot`,
//! `ColonisationContribution`, `ColonisationSystemClaim`, `ColonisationSystemClaimRelease` and
//! `ColonisationBeaconDeployed` into per-project state, keyed by the construction depot's
//! `MarketID`, and persists it in `astromech-store` so it survives a restart.
//!
//! Field names for the depot and contribution events are the typed ones in `astromech-journal`
//! (post-v35 community docs, flagged uncertain there). Claims and beacons are not typed in the
//! journal crate yet, so they are read from the raw JSON of `Other`. Claim and release lines are
//! read the way EDDI reads them (`StarSystem`, `SystemAddress`, with `SystemName` as a
//! fallback). `ColonisationBeaconDeployed` carries no keys beyond the timestamp, so a beacon is
//! placed in the system of the last `Location`, `FSDJump` or `Docked` seen.
//!
//! Applying is idempotent: a journal line replayed after a restore (older than what the state
//! already holds, or an identical contribution) changes nothing.

use astromech_journal::{ExtraEvent, JournalEvent, JournalLine};
use astromech_store::{Store, StoreError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

const PROJECT: &str = "colonisation.project";
const CLAIM: &str = "colonisation.claim";
const BEACON: &str = "colonisation.beacon";

#[derive(Debug)]
pub enum Error {
    Store(StoreError),
    Corrupt { kind: &'static str, key: String, source: serde_json::Error },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Store(e) => write!(f, "store: {e}"),
            Error::Corrupt { kind, key, source } => write!(f, "unreadable {kind} record {key}: {source}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<StoreError> for Error {
    fn from(e: StoreError) -> Self {
        Error::Store(e)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub name_localised: Option<String>,
    pub required: i64,
    pub provided: i64,
    pub payment: i64,
}

impl Resource {
    pub fn remaining(&self) -> i64 {
        (self.required - self.provided).max(0)
    }
}

/// One `ColonisationContribution` event: what the commander delivered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contribution {
    pub timestamp: String,
    pub items: Vec<(String, i64)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub market_id: i64,
    /// From the `Docked` at this market, when seen. The depot event itself has no names.
    pub station_name: Option<String>,
    pub system_name: Option<String>,
    /// 0.0 to 1.0, as the game last reported it.
    pub progress: f64,
    pub complete: bool,
    pub failed: bool,
    pub resources: Vec<Resource>,
    pub first_seen: String,
    /// Timestamp of the newest depot event applied (empty until a depot event is seen).
    pub depot_updated: String,
    pub contributions: Vec<Contribution>,
}

impl Project {
    fn new(market_id: i64, ts: &str) -> Self {
        Project {
            market_id,
            station_name: None,
            system_name: None,
            progress: 0.0,
            complete: false,
            failed: false,
            resources: Vec::new(),
            first_seen: ts.to_string(),
            depot_updated: String::new(),
            contributions: Vec::new(),
        }
    }

    /// Units still to deliver across all commodities.
    pub fn remaining_total(&self) -> i64 {
        self.resources.iter().map(Resource::remaining).sum()
    }

    /// Units this commander has delivered of `name` (journal name, e.g. `$steel_name;`).
    pub fn contributed(&self, name: &str) -> i64 {
        self.contributions.iter().flat_map(|c| &c.items).filter(|(n, _)| n == name).map(|(_, a)| a).sum()
    }

    pub fn contributed_total(&self) -> i64 {
        self.contributions.iter().flat_map(|c| &c.items).map(|(_, a)| a).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    pub system_name: Option<String>,
    pub system_address: Option<i64>,
    pub claimed_at: String,
    pub released_at: Option<String>,
}

impl Claim {
    pub fn active(&self) -> bool {
        self.released_at.is_none()
    }
    fn newest(&self) -> &str {
        self.released_at.as_deref().unwrap_or(&self.claimed_at)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beacon {
    pub timestamp: String,
    pub system_name: Option<String>,
    pub system_address: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct Tracker {
    projects: BTreeMap<i64, Project>,
    /// Keyed by system address, or by name when the address is missing.
    claims: BTreeMap<String, Claim>,
    beacons: BTreeMap<String, Beacon>,
    docked: Option<(i64, String, String)>,
    /// System of the last `Location`, `FSDJump` or `Docked`.
    here: (Option<String>, Option<i64>),
    dirty: BTreeSet<(&'static str, String)>,
}

impl Tracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Restore state saved by [`Tracker::save`].
    pub fn load(store: &Store) -> Result<Self, Error> {
        let mut t = Tracker::new();
        for (key, body) in store.records(PROJECT)? {
            let p: Project =
                serde_json::from_str(&body).map_err(|source| Error::Corrupt { kind: PROJECT, key, source })?;
            t.projects.insert(p.market_id, p);
        }
        for (key, body) in store.records(CLAIM)? {
            let c = serde_json::from_str(&body).map_err(|source| Error::Corrupt {
                kind: CLAIM,
                key: key.clone(),
                source,
            })?;
            t.claims.insert(key, c);
        }
        for (key, body) in store.records(BEACON)? {
            let b = serde_json::from_str(&body).map_err(|source| Error::Corrupt {
                kind: BEACON,
                key: key.clone(),
                source,
            })?;
            t.beacons.insert(key, b);
        }
        Ok(t)
    }

    /// Write everything changed since the last save.
    pub fn save(&mut self, store: &Store) -> Result<(), Error> {
        for (kind, key) in std::mem::take(&mut self.dirty) {
            let body = match kind {
                PROJECT => key.parse().ok().and_then(|id: i64| self.projects.get(&id)).map(serde_json::to_string),
                CLAIM => self.claims.get(&key).map(serde_json::to_string),
                _ => self.beacons.get(&key).map(serde_json::to_string),
            };
            if let Some(body) = body {
                store.put_record(kind, &key, &body.expect("plain structs always serialise"))?;
            }
        }
        Ok(())
    }

    pub fn projects(&self) -> impl Iterator<Item = &Project> {
        self.projects.values()
    }

    pub fn project(&self, market_id: i64) -> Option<&Project> {
        self.projects.get(&market_id)
    }

    pub fn claims(&self) -> impl Iterator<Item = &Claim> {
        self.claims.values()
    }

    pub fn beacons(&self) -> impl Iterator<Item = &Beacon> {
        self.beacons.values()
    }

    /// Fold one parsed journal line in. Unrelated events are ignored.
    pub fn apply(&mut self, line: &JournalLine) {
        let ts = line.timestamp.as_str();
        match &line.event {
            JournalEvent::Docked { station_name, star_system, market_id } => {
                if self.here.0.as_deref() != Some(star_system.as_str()) {
                    self.here = (Some(star_system.clone()), None);
                }
                if let Some(id) = market_id {
                    self.docked = Some((*id, station_name.clone(), star_system.clone()));
                }
            }
            JournalEvent::Location { star_system, system_address, .. }
            | JournalEvent::FsdJump { star_system, system_address, .. } => {
                self.here = (Some(star_system.clone()), *system_address);
            }
            JournalEvent::Extra(ExtraEvent::ColonisationConstructionDepot(d)) => {
                let Some(id) = d.market_id else { return };
                let p = self.projects.entry(id).or_insert_with(|| Project::new(id, ts));
                if ts < p.depot_updated.as_str() {
                    return;
                }
                p.progress = d.progress;
                p.complete = d.complete;
                p.failed = d.failed;
                p.depot_updated = ts.to_string();
                p.resources = d
                    .resources
                    .iter()
                    .map(|r| Resource {
                        name: r.name.clone(),
                        name_localised: r.name_localised.clone(),
                        required: r.required,
                        provided: r.provided,
                        payment: r.payment,
                    })
                    .collect();
                if let Some((m, station, system)) = &self.docked {
                    if *m == id {
                        p.station_name.get_or_insert_with(|| station.clone());
                        p.system_name.get_or_insert_with(|| system.clone());
                    }
                }
                self.dirty.insert((PROJECT, id.to_string()));
            }
            JournalEvent::Extra(ExtraEvent::ColonisationContribution { market_id: Some(id), contributions }) => {
                let p = self.projects.entry(*id).or_insert_with(|| Project::new(*id, ts));
                let c = Contribution { timestamp: ts.to_string(), items: contributions.clone() };
                if !p.contributions.contains(&c) {
                    p.contributions.push(c);
                }
                self.dirty.insert((PROJECT, id.to_string()));
            }
            JournalEvent::Other { event, raw } => match event.as_str() {
                "ColonisationSystemClaim" => self.claim(ts, raw),
                "ColonisationSystemClaimRelease" => self.release(ts, raw),
                "ColonisationBeaconDeployed" => self.beacon(ts),
                _ => {}
            },
            _ => {}
        }
    }

    fn claim(&mut self, ts: &str, raw: &Value) {
        let (name, addr) = system_of(raw);
        let Some(key) = system_key(&name, addr) else { return };
        if self.claims.get(&key).is_some_and(|c| ts <= c.newest()) {
            return;
        }
        self.claims.insert(
            key.clone(),
            Claim { system_name: name, system_address: addr, claimed_at: ts.to_string(), released_at: None },
        );
        self.dirty.insert((CLAIM, key));
    }

    fn release(&mut self, ts: &str, raw: &Value) {
        let (name, addr) = system_of(raw);
        let Some(key) = system_key(&name, addr) else { return };
        match self.claims.get_mut(&key) {
            Some(c) if ts > c.newest() => c.released_at = Some(ts.to_string()),
            Some(_) => return,
            None => {
                // Released a claim made before the journal we have: keep the fact.
                self.claims.insert(
                    key.clone(),
                    Claim {
                        system_name: name,
                        system_address: addr,
                        claimed_at: String::new(),
                        released_at: Some(ts.to_string()),
                    },
                );
            }
        }
        self.dirty.insert((CLAIM, key));
    }

    fn beacon(&mut self, ts: &str) {
        let (name, addr) = self.here.clone();
        let key = format!("{ts}|{}", system_key(&name, addr).unwrap_or_default());
        if self.beacons.contains_key(&key) {
            return;
        }
        self.beacons.insert(key.clone(), Beacon { timestamp: ts.to_string(), system_name: name, system_address: addr });
        self.dirty.insert((BEACON, key));
    }
}

fn system_of(raw: &Value) -> (Option<String>, Option<i64>) {
    let name = ["StarSystem", "SystemName"]
        .iter()
        .find_map(|k| raw.get(*k).and_then(Value::as_str))
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    (name, raw.get("SystemAddress").and_then(Value::as_i64))
}

fn system_key(name: &Option<String>, addr: Option<i64>) -> Option<String> {
    addr.map(|a| a.to_string()).or_else(|| name.clone())
}

/// Apply every parsable line of an NDJSON journal, in order.
pub fn replay_lines<'a>(t: &mut Tracker, lines: impl IntoIterator<Item = &'a str>) {
    for line in lines {
        if let Ok(j) = astromech_journal::parse_line(line.trim()) {
            t.apply(&j);
        }
    }
}

#[cfg(test)]
mod tests;
