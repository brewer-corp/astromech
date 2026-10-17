//! # astromech-colonisation
//!
//! Construction tracker (journal only). Folds `ColonisationConstructionDepot`,
//! `ColonisationContribution`, `ColonisationSystemClaim`, `ColonisationSystemClaimRelease` and
//! `ColonisationBeaconDeployed` into per-project state, keyed by the construction depot's
//! `MarketID`, and persists it in `astromech-store` so it survives a restart.
//!
//! State is kept per commander (the key `astromech_journal::commander` gives a line: the `FID`,
//! `/legacy` for Legacy mode), so two commanders sharing a journal folder never see each other's
//! projects, claims or beacons. The accessors show the commander the newest line belonged to.
//! A stored record's key is `<commander>|<id>`; events before a file names its commander (none
//! of the colonisation ones) go under the empty commander.
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

use astromech_journal::commander::{self, FileCommander};
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

/// One commander's state.
#[derive(Debug, Clone, Default)]
struct Books {
    projects: BTreeMap<i64, Project>,
    /// Keyed by system address, or by name when the address is missing.
    claims: BTreeMap<String, Claim>,
    beacons: BTreeMap<String, Beacon>,
}

#[derive(Debug, Clone, Default)]
pub struct Tracker {
    books: BTreeMap<String, Books>,
    /// The commander the newest line belonged to.
    cmdr: FileCommander,
    docked: Option<(i64, String, String)>,
    /// System of the last `Location`, `FSDJump` or `Docked`.
    here: (Option<String>, Option<i64>),
    /// (kind, `<commander>|<id>` record key) written since the last save.
    dirty: BTreeSet<(&'static str, String)>,
}

/// What [`Tracker::load`] found.
#[derive(Debug)]
pub struct Loaded {
    pub tracker: Tracker,
    /// Records that could not be read and were left out. They stay in the store untouched.
    pub skipped: Vec<Error>,
}

fn record_key(cmdr: &str, id: &str) -> String {
    format!("{cmdr}|{id}")
}

/// (commander, id) of a record key. A key without a commander belongs to the empty one.
fn split_key(key: &str) -> (&str, &str) {
    key.split_once('|').unwrap_or(("", key))
}

impl Tracker {
    pub fn new() -> Self {
        Self::default()
    }

    fn cmdr_key(&self) -> &str {
        self.cmdr.cmdr.as_deref().unwrap_or("")
    }

    fn books(&self) -> Option<&Books> {
        self.books.get(self.cmdr_key())
    }

    fn books_mut(&mut self) -> &mut Books {
        let key = self.cmdr_key().to_string();
        self.books.entry(key).or_default()
    }

    /// Restore state saved by [`Tracker::save`]. A record that cannot be read is skipped, not
    /// fatal: it stays in the store (a later save only touches the records it changes), the rest
    /// of the state loads, and the journal replays what the tracker then lacks. The caller logs
    /// `skipped`. Only a store that cannot be read at all is an error.
    pub fn load(store: &Store) -> Result<Loaded, Error> {
        let mut t = Tracker::new();
        let mut skipped = vec![];
        for (key, body) in store.records(PROJECT)? {
            match serde_json::from_str::<Project>(&body) {
                Ok(p) => {
                    t.books.entry(split_key(&key).0.to_string()).or_default().projects.insert(p.market_id, p);
                }
                Err(source) => skipped.push(Error::Corrupt { kind: PROJECT, key, source }),
            }
        }
        for (key, body) in store.records(CLAIM)? {
            match serde_json::from_str(&body) {
                Ok(c) => {
                    let (cmdr, id) = split_key(&key);
                    t.books.entry(cmdr.to_string()).or_default().claims.insert(id.to_string(), c);
                }
                Err(source) => skipped.push(Error::Corrupt { kind: CLAIM, key, source }),
            }
        }
        for (key, body) in store.records(BEACON)? {
            match serde_json::from_str(&body) {
                Ok(b) => {
                    let (cmdr, id) = split_key(&key);
                    t.books.entry(cmdr.to_string()).or_default().beacons.insert(id.to_string(), b);
                }
                Err(source) => skipped.push(Error::Corrupt { kind: BEACON, key, source }),
            }
        }
        Ok(Loaded { tracker: t, skipped })
    }

    /// Write everything changed since the last save.
    pub fn save(&mut self, store: &Store) -> Result<(), Error> {
        for (kind, key) in std::mem::take(&mut self.dirty) {
            let (cmdr, id) = split_key(&key);
            let Some(b) = self.books.get(cmdr) else { continue };
            let body = match kind {
                PROJECT => id.parse().ok().and_then(|id: i64| b.projects.get(&id)).map(serde_json::to_string),
                CLAIM => b.claims.get(id).map(serde_json::to_string),
                _ => b.beacons.get(id).map(serde_json::to_string),
            };
            if let Some(body) = body {
                store.put_record(kind, &key, &body.expect("plain structs always serialise"))?;
            }
        }
        Ok(())
    }

    /// The current commander's projects.
    pub fn projects(&self) -> impl Iterator<Item = &Project> {
        self.books().into_iter().flat_map(|b| b.projects.values())
    }

    pub fn project(&self, market_id: i64) -> Option<&Project> {
        self.books()?.projects.get(&market_id)
    }

    pub fn claims(&self) -> impl Iterator<Item = &Claim> {
        self.books().into_iter().flat_map(|b| b.claims.values())
    }

    pub fn beacons(&self) -> impl Iterator<Item = &Beacon> {
        self.books().into_iter().flat_map(|b| b.beacons.values())
    }

    /// Fold one parsed journal line in. Unrelated events are ignored.
    pub fn apply(&mut self, line: &JournalLine) {
        let ts = line.timestamp.as_str();
        // A new file names its commander again; a continuation part carries on with the last.
        if matches!(line.event, JournalEvent::FileHeader { .. }) && !commander::continues(line) {
            self.cmdr.cmdr = None;
        }
        let before = self.cmdr.cmdr.clone();
        self.cmdr.observe(line);
        if self.cmdr.cmdr != before {
            // Where the last commander was docked or flying is not where this one is.
            self.docked = None;
            self.here = (None, None);
        }
        match &line.event {
            JournalEvent::Docked { station_name, star_system, market_id, .. } => {
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
                let key = record_key(self.cmdr_key(), &id.to_string());
                let docked = self.docked.clone();
                let p = self.books_mut().projects.entry(id).or_insert_with(|| Project::new(id, ts));
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
                if let Some((m, station, system)) = &docked {
                    if *m == id {
                        p.station_name.get_or_insert_with(|| station.clone());
                        p.system_name.get_or_insert_with(|| system.clone());
                    }
                }
                self.dirty.insert((PROJECT, key));
            }
            JournalEvent::Extra(ExtraEvent::ColonisationContribution { market_id: Some(id), contributions }) => {
                let key = record_key(self.cmdr_key(), &id.to_string());
                let p = self.books_mut().projects.entry(*id).or_insert_with(|| Project::new(*id, ts));
                let c = Contribution { timestamp: ts.to_string(), items: contributions.clone() };
                if !p.contributions.contains(&c) {
                    p.contributions.push(c);
                }
                self.dirty.insert((PROJECT, key));
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
        if self.books().and_then(|b| b.claims.get(&key)).is_some_and(|c| ts <= c.newest()) {
            return;
        }
        self.books_mut().claims.insert(
            key.clone(),
            Claim { system_name: name, system_address: addr, claimed_at: ts.to_string(), released_at: None },
        );
        self.dirty.insert((CLAIM, record_key(self.cmdr_key(), &key)));
    }

    fn release(&mut self, ts: &str, raw: &Value) {
        let (name, addr) = system_of(raw);
        let Some(key) = system_key(&name, addr) else { return };
        let record = record_key(self.cmdr_key(), &key);
        match self.books_mut().claims.get_mut(&key) {
            Some(c) if ts > c.newest() => c.released_at = Some(ts.to_string()),
            Some(_) => return,
            None => {
                // Released a claim made before the journal we have: keep the fact.
                self.books_mut().claims.insert(
                    key,
                    Claim {
                        system_name: name,
                        system_address: addr,
                        claimed_at: String::new(),
                        released_at: Some(ts.to_string()),
                    },
                );
            }
        }
        self.dirty.insert((CLAIM, record));
    }

    fn beacon(&mut self, ts: &str) {
        let (name, addr) = self.here.clone();
        let key = format!("{ts}|{}", system_key(&name, addr).unwrap_or_default());
        if self.books().is_some_and(|b| b.beacons.contains_key(&key)) {
            return;
        }
        self.books_mut()
            .beacons
            .insert(key.clone(), Beacon { timestamp: ts.to_string(), system_name: name, system_address: addr });
        self.dirty.insert((BEACON, record_key(self.cmdr_key(), &key)));
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

/// A project and what is left to buy for it, for the app.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProjectView {
    #[serde(flatten)]
    pub project: Project,
    pub shopping: shopping::ShoppingList,
}

/// Every project in the journals in `dir`, replayed from the start (nothing is stored), with
/// its shopping list against the ship's hold: `Cargo.json` when it is there, else the hold the
/// journal events leave. Projects still being built come first, the most recently updated first.
pub fn from_journal_dir(dir: &std::path::Path) -> Vec<ProjectView> {
    let mut t = Tracker::new();
    let mut hold = shopping::Hold::default();
    for f in astromech_journal::journal_logs(dir) {
        for line in astromech_journal::parse_file(&f) {
            t.apply(&line);
            hold.apply(&line);
        }
    }
    if let Ok(b) = std::fs::read(dir.join("Cargo.json")) {
        hold.apply_cargo_json(&b);
    }
    let mut out: Vec<ProjectView> =
        t.projects().map(|p| ProjectView { project: p.clone(), shopping: shopping::shopping_list(p, &hold) }).collect();
    out.sort_by(|a, b| {
        let done = |v: &ProjectView| v.project.complete || v.project.failed;
        done(a).cmp(&done(b)).then_with(|| b.project.depot_updated.cmp(&a.project.depot_updated))
    });
    out
}

pub mod shopping;

#[cfg(test)]
mod tests;
