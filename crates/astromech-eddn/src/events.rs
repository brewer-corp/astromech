//! The event-specific schemas: `fssdiscoveryscan`, `fssallbodiesfound`, `fssbodysignals`,
//! `fsssignaldiscovered`, `navbeaconscan`, `navroute`, `scanbarycentre`, `codexentry`,
//! `approachsettlement`, `dockinggranted`, `dockingdenied` and `fcmaterials_journal`, all version 1.
//! Rules from each `<name>-README.md` and `<name>-v1.0.json` in EDCD `schemas/` (live branch,
//! commit 4ad669bb, read 16 Oct 2026).
//!
//! Driving it: for each journal event, first [`crate::Sender::observe`] it, then pass it to
//! [`EventSchemas::on_event`] with the sender's context and submit what comes back, in order.
//! Observing first matters for `fsssignaldiscovered`: an Odyssey client writes a system's
//! `FSSSignalDiscovered` events *before* the `FSDJump` that arrives there, so the batch they form
//! must be checked against the new location. Feed `Status.json` to
//! [`EventSchemas::observe_status`]. `navroute` and `fcmaterials_journal` are built from the
//! companion file the `NavRoute`/`FCMaterials` event announces ([`navroute`], [`fcmaterials`]).

use crate::context::GameContext;
use crate::journal::has_all;
use crate::message::{Augment, Elision, SchemaSpec, SystemField};
use serde_json::{json, Value};

const STAR_SYSTEM: Augment = Augment { event: None, field: SystemField::StarSystem };
const STAR_POS: Augment = Augment { event: None, field: SystemField::StarPos };

const fn spec(name: &'static str, elide: &'static [Elision], augment: &'static [Augment]) -> SchemaSpec {
    SchemaSpec { name, version: 1, elide, augment, flags: true }
}

const fn all(path: &'static str) -> Elision {
    Elision { event: None, path }
}

/// `Progress` is personal (the schema disallows it); `StarPos` is added.
pub const FSS_DISCOVERY_SCAN: SchemaSpec = spec("fssdiscoveryscan", &[all("Progress")], &[STAR_POS]);
pub const FSS_ALL_BODIES_FOUND: SchemaSpec = spec("fssallbodiesfound", &[], &[STAR_POS]);
pub const FSS_BODY_SIGNALS: SchemaSpec = spec("fssbodysignals", &[], &[STAR_SYSTEM, STAR_POS]);
/// Each signal loses `event` (redundant), `SystemAddress` (moved to the top level after the
/// cross-check) and `TimeRemaining` (ephemeral, slightly personal).
pub const FSS_SIGNAL_DISCOVERED: SchemaSpec = spec(
    "fsssignaldiscovered",
    &[all("signals[].event"), all("signals[].SystemAddress"), all("signals[].TimeRemaining")],
    &[STAR_SYSTEM, STAR_POS],
);
pub const NAV_BEACON_SCAN: SchemaSpec = spec("navbeaconscan", &[], &[STAR_SYSTEM, STAR_POS]);
pub const NAV_ROUTE: SchemaSpec = spec("navroute", &[], &[]);
pub const SCAN_BARY_CENTRE: SchemaSpec = spec("scanbarycentre", &[], &[STAR_POS]);
/// `Latitude`, `Longitude`, `Traits` and `VoucherAmount` are kept here (unlike journal/1).
pub const CODEX_ENTRY: SchemaSpec = spec("codexentry", &[all("IsNewEntry"), all("NewTraitsDiscovered")], &[STAR_POS]);
/// `Latitude` and `Longitude` are required here.
pub const APPROACH_SETTLEMENT: SchemaSpec = spec("approachsettlement", &[], &[STAR_SYSTEM, STAR_POS]);
pub const DOCKING_GRANTED: SchemaSpec = spec("dockinggranted", &[], &[]);
pub const DOCKING_DENIED: SchemaSpec = spec("dockingdenied", &[], &[]);
pub const FC_MATERIALS_JOURNAL: SchemaSpec = spec("fcmaterials_journal", &[], &[]);

/// The one signal type that must never be sent: only the commander with the mission can use it.
const MISSION_TARGET: &str = "$USS_Type_MissionTarget;";

pub type Outgoing = (&'static SchemaSpec, Value);

/// The state the event schemas need across events: the pending `FSSSignalDiscovered` run and the
/// body the commander is at (for `codexentry`).
#[derive(Debug, Default)]
pub struct EventSchemas {
    signals: Vec<Value>,
    /// `BodyName`/`BodyID` from the last `ApproachBody` or `Location`, cleared by `LeaveBody`
    /// and `FSDJump`.
    journal_body: Option<(String, u64)>,
    /// `BodyName` from the latest `Status.json`.
    status_body: Option<String>,
}

impl EventSchemas {
    /// Messages to submit for one journal event (raw JSON), observed by the sender first.
    ///
    /// `FSSSignalDiscovered` events are held; the next event of any other kind ends the run and
    /// sends it as one `fsssignaldiscovered` message, before that event's own message.
    pub fn on_event(&mut self, ev: &Value, ctx: &GameContext) -> Vec<Outgoing> {
        let Some(event) = ev.get("event").and_then(Value::as_str) else {
            return Vec::new();
        };
        if event == "FSSSignalDiscovered" {
            self.signals.push(ev.clone());
            return Vec::new();
        }
        let mut out: Vec<Outgoing> = self.flush(ctx).into_iter().collect();
        self.track_body(event, ev);
        let single = |spec: &'static SchemaSpec, needs: &[&str]| has_all(ev, needs).then(|| (spec, ev.clone()));
        let msg = match event {
            "FSSDiscoveryScan" => {
                single(&FSS_DISCOVERY_SCAN, &["timestamp", "SystemName", "SystemAddress", "BodyCount", "NonBodyCount"])
            }
            "FSSAllBodiesFound" => {
                single(&FSS_ALL_BODIES_FOUND, &["timestamp", "SystemName", "SystemAddress", "Count"])
            }
            "FSSBodySignals" => single(&FSS_BODY_SIGNALS, &["timestamp", "SystemAddress", "BodyID", "Signals"]),
            "NavBeaconScan" => single(&NAV_BEACON_SCAN, &["timestamp", "SystemAddress", "NumBodies"]),
            "ScanBaryCentre" => single(&SCAN_BARY_CENTRE, &["timestamp", "StarSystem", "SystemAddress", "BodyID"]),
            "CodexEntry" => single(&CODEX_ENTRY, &["timestamp", "System", "SystemAddress", "EntryID"])
                .map(|(spec, m)| (spec, self.codex_body(m))),
            "ApproachSettlement" => single(
                &APPROACH_SETTLEMENT,
                &["timestamp", "SystemAddress", "Name", "BodyID", "BodyName", "Latitude", "Longitude"],
            ),
            "DockingGranted" => single(&DOCKING_GRANTED, &["timestamp", "MarketID", "StationName"]),
            "DockingDenied" => single(&DOCKING_DENIED, &["timestamp", "MarketID", "StationName", "Reason"]),
            _ => None,
        };
        out.extend(msg);
        out
    }

    /// Send the pending `FSSSignalDiscovered` run now (at the end of a journal, or on shutdown).
    ///
    /// Each signal is checked against the last location: one from another system, or without a
    /// `SystemAddress`, is dropped, as is every mission-target USS. `None` if nothing is left.
    pub fn flush(&mut self, ctx: &GameContext) -> Option<Outgoing> {
        let batch = std::mem::take(&mut self.signals);
        let fix = ctx.system.as_ref()?;
        let signals: Vec<Value> = batch
            .into_iter()
            .filter(|s| {
                s.get("SystemAddress").and_then(Value::as_u64) == Some(fix.address)
                    && s.get("USSType").and_then(Value::as_str) != Some(MISSION_TARGET)
                    && has_all(s, &["timestamp", "SignalName"])
            })
            .collect();
        let timestamp = signals.first()?["timestamp"].clone();
        Some((
            &FSS_SIGNAL_DISCOVERED,
            json!({"timestamp":timestamp,"event":"FSSSignalDiscovered","SystemAddress":fix.address,"signals":signals}),
        ))
    }

    /// Feed every `Status.json` the game writes.
    pub fn observe_status(&mut self, status: &Value) {
        self.status_body = status.get("BodyName").and_then(Value::as_str).map(str::to_owned);
    }

    fn track_body(&mut self, event: &str, ev: &Value) {
        let body = |name_key: &str| Some((ev.get(name_key)?.as_str()?.to_owned(), ev.get("BodyID")?.as_u64()?));
        match event {
            "ApproachBody" => self.journal_body = body("BodyName"),
            // Location names its body `Body`.
            "Location" => self.journal_body = body("Body"),
            "LeaveBody" | "FSDJump" => self.journal_body = None,
            _ => {}
        }
    }

    /// `BodyName` only from `Status.json`, and `BodyID` only when the journal's body has the same
    /// name; otherwise the keys are left out altogether (codexentry README).
    fn codex_body(&self, mut msg: Value) -> Value {
        let m = msg.as_object_mut().expect("checked by has_all");
        m.remove("BodyName");
        m.remove("BodyID");
        if let Some(name) = &self.status_body {
            m.insert("BodyName".into(), json!(name));
            if let Some((_, id)) = self.journal_body.as_ref().filter(|(n, _)| n == name) {
                m.insert("BodyID".into(), json!(id));
            }
        }
        msg
    }
}

/// The `navroute` message from `NavRoute.json`, read when the journal's `NavRoute` event `ev`
/// announces it. `None` if the file isn't the one that event announced (its `timestamp` differs:
/// a stale file) or the route is empty (as after `NavRouteClear`).
pub fn navroute(ev: &Value, file: &Value) -> Option<Outgoing> {
    let route = file.get("Route")?.as_array()?;
    (fresh(ev, file, "NavRoute") && !route.is_empty()).then(|| (&NAV_ROUTE, file.clone()))
}

/// The `fcmaterials_journal` message from `FCMaterials.json`, read when the journal's
/// `FCMaterials` event `ev` announces it. `None` for a stale or incomplete file.
pub fn fcmaterials(ev: &Value, file: &Value) -> Option<Outgoing> {
    (fresh(ev, file, "FCMaterials") && has_all(file, &["MarketID", "CarrierName", "CarrierID", "Items"]))
        .then(|| (&FC_MATERIALS_JOURNAL, file.clone()))
}

fn fresh(ev: &Value, file: &Value, event: &str) -> bool {
    let ts = file.get("timestamp").and_then(Value::as_str);
    ev.get("event").and_then(Value::as_str) == Some(event)
        && file.get("event").and_then(Value::as_str) == Some(event)
        && ts.is_some()
        && ts == ev.get("timestamp").and_then(Value::as_str)
}
