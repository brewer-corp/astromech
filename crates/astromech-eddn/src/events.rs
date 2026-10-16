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

#[cfg(test)]
mod schema_tests {
    // The event-specific schemas: a synthetic fixture for each, built by the sender and validated
    // against its live schema file, plus the `fsssignaldiscovered` batching and the `codexentry`
    // body rules.

    use crate::events::{self, EventSchemas, Outgoing};
    use crate::testutil::*;
    use crate::GameContext;
    use serde_json::{json, Value};

    const SOL: u64 = 10477373803;

    fn shinrarta_location() -> Value {
        json!({"timestamp":"2026-10-16T12:00:10Z","event":"Location","DistFromStarLS":0.0,"Docked":false,
            "StarSystem":"Shinrarta Dezhra","SystemAddress":SHINRARTA,"StarPos":SHINRARTA_POS,
            "Body":"Shinrarta Dezhra A","BodyID":1,"BodyType":"Star"})
    }

    /// Feed events the way the sender does (observe, then route) and collect what would be sent.
    fn feed(ctx: &mut GameContext, schemas: &mut EventSchemas, evs: &[Value]) -> Vec<Outgoing> {
        let mut out = Vec::new();
        for ev in evs {
            ctx.observe(ev);
            out.extend(schemas.on_event(ev, ctx));
        }
        out
    }

    /// One event in a live session, its single message validated and returned.
    fn one(ev: Value) -> Value {
        let mut ctx = live_context();
        let out = feed(&mut ctx, &mut EventSchemas::default(), &[ev]);
        assert_eq!(out.len(), 1, "{out:?}");
        validated(&ctx, &out[0])
    }

    fn validated(ctx: &GameContext, (spec, msg): &Outgoing) -> Value {
        let up = upload(ctx, spec, msg.clone());
        assert_valid(&schema_file(spec), &up);
        assert!(!up.to_string().contains("_Localised"), "{up:#}");
        assert_eq!(up["$schemaRef"], format!("https://eddn.edcd.io/schemas/{}/1", spec.name));
        up["message"].clone()
    }

    #[test]
    fn fssdiscoveryscan_drops_progress_and_adds_starpos() {
        let m = one(json!({"timestamp":"2026-10-16T12:01:00Z","event":"FSSDiscoveryScan","Progress":0.25,
            "BodyCount":12,"NonBodyCount":30,"SystemName":"Shinrarta Dezhra","SystemAddress":SHINRARTA}));
        assert_absent(&m, &["Progress"]);
        assert_eq!(m["StarPos"], json!(SHINRARTA_POS));
    }

    #[test]
    fn fssdiscoveryscan_naming_another_system_is_not_built() {
        let mut ctx = live_context();
        let ev = json!({"timestamp":"2026-10-16T12:01:00Z","event":"FSSDiscoveryScan","Progress":0.25,
            "BodyCount":12,"NonBodyCount":30,"SystemName":"Lave","SystemAddress":SHINRARTA});
        let out = feed(&mut ctx, &mut EventSchemas::default(), &[ev]);
        let sw = crate::Software { name: "a".into(), version: "1".into() };
        let r = crate::message::build(out[0].0, &ctx, &sw, false, out[0].1.clone());
        assert!(matches!(r, Err(crate::Dropped::LocationMismatch(_))), "{r:?}");
    }

    #[test]
    fn fssallbodiesfound_adds_starpos() {
        let m = one(json!({"timestamp":"2026-10-16T12:02:00Z","event":"FSSAllBodiesFound",
            "SystemName":"Shinrarta Dezhra","SystemAddress":SHINRARTA,"Count":12}));
        assert_eq!(m["StarPos"], json!(SHINRARTA_POS));
    }

    #[test]
    fn fssbodysignals_adds_starsystem_and_starpos() {
        let m = one(json!({"timestamp":"2026-10-16T12:03:00Z","event":"FSSBodySignals",
            "BodyName":"Shinrarta Dezhra A 1","BodyID":2,"SystemAddress":SHINRARTA,
            "Signals":[{"Type":"$SAA_SignalType_Geological;","Type_Localised":"Geological","Count":2}]}));
        assert_eq!((&m["StarSystem"], &m["StarPos"]), (&json!("Shinrarta Dezhra"), &json!(SHINRARTA_POS)));
    }

    #[test]
    fn navbeaconscan_adds_starsystem_and_starpos() {
        let m = one(
            json!({"timestamp":"2026-10-16T12:04:00Z","event":"NavBeaconScan","SystemAddress":SHINRARTA,"NumBodies":21}),
        );
        assert_eq!((&m["StarSystem"], &m["StarPos"]), (&json!("Shinrarta Dezhra"), &json!(SHINRARTA_POS)));
    }

    #[test]
    fn scanbarycentre_adds_starpos() {
        let m =
            one(json!({"timestamp":"2026-10-16T12:05:00Z","event":"ScanBaryCentre","StarSystem":"Shinrarta Dezhra",
            "SystemAddress":SHINRARTA,"BodyID":5,"SemiMajorAxis":1.2e9,"Eccentricity":0.02,"OrbitalInclination":0.5,
            "Periapsis":80.0,"OrbitalPeriod":4.0e6,"AscendingNode":12.0,"MeanAnomaly":200.0}));
        assert_eq!(m["StarPos"], json!(SHINRARTA_POS));
    }

    #[test]
    fn approachsettlement_keeps_latitude_and_longitude_and_adds_the_system() {
        let ev = json!({"timestamp":"2026-10-16T12:06:00Z","event":"ApproachSettlement","Name":"Hutton Research Base",
            "Name_Localised":"Hutton Research Base","MarketID":3510000001u64,
            "StationFaction":{"Name":"Pilots' Federation Local Branch","FactionState":"None"},
            "StationGovernment":"$government_Democracy;","StationGovernment_Localised":"Democracy",
            "StationAllegiance":"PilotsFederation","StationServices":["dock","commodities"],
            "StationEconomy":"$economy_HighTech;","StationEconomy_Localised":"High Tech",
            "StationEconomies":[{"Name":"$economy_HighTech;","Name_Localised":"High Tech","Proportion":1.0}],
            "SystemAddress":SHINRARTA,"BodyID":2,"BodyName":"Shinrarta Dezhra A 1","Latitude":12.5,"Longitude":-45.25});
        let m = one(ev.clone());
        assert_eq!(
            (&m["Latitude"], &m["Longitude"], &m["MarketID"]),
            (&json!(12.5), &json!(-45.25), &json!(3510000001u64))
        );
        assert_eq!((&m["StarSystem"], &m["StarPos"]), (&json!("Shinrarta Dezhra"), &json!(SHINRARTA_POS)));
        // Without a required field the gateway would reject it, so it isn't sent.
        let mut partial = ev;
        partial.as_object_mut().unwrap().remove("Latitude");
        assert!(EventSchemas::default().on_event(&partial, &live_context()).is_empty());
    }

    #[test]
    fn dockinggranted_and_dockingdenied_validate() {
        one(json!({"timestamp":"2026-10-16T12:07:00Z","event":"DockingGranted","LandingPad":41,
            "MarketID":128666762,"StationName":"Jameson Memorial","StationType":"Orbis"}));
        one(json!({"timestamp":"2026-10-16T12:08:00Z","event":"DockingDenied","Reason":"RestrictedAccess",
            "MarketID":3700000001u64,"StationName":"X9X-9XX","StationName_Localised":"X9X-9XX","StationType":"FleetCarrier"}));
    }

    fn codex_entry() -> Value {
        json!({"timestamp":"2026-10-16T12:09:00Z","event":"CodexEntry","EntryID":2420101,
            "Name":"$Codex_Ent_Bacterial_01_A_Name;","Name_Localised":"Bacterium Aurasus - Teal",
            "SubCategory":"$Codex_SubCategory_Organic_Structures;","SubCategory_Localised":"Organic structures",
            "Category":"$Codex_Category_Biology;","Category_Localised":"Biological and Geological",
            "Region":"$Codex_RegionName_18;","Region_Localised":"Inner Orion Spur",
            "System":"Shinrarta Dezhra","SystemAddress":SHINRARTA,"BodyID":99,"Latitude":12.5,"Longitude":-45.25,
            "IsNewEntry":true,"NewTraitsDiscovered":true,"VoucherAmount":50000,"Traits":["$Codex_Trait_Teal;"]})
    }

    fn codex_with(status: Value, journal: &[Value]) -> Value {
        let mut ctx = live_context();
        let mut s = EventSchemas::default();
        feed(&mut ctx, &mut s, journal);
        s.observe_status(&status);
        let out = feed(&mut ctx, &mut s, &[codex_entry()]);
        assert_eq!(out.len(), 1);
        let m = validated(&ctx, &out[0]);
        assert_absent(&m, &["IsNewEntry", "NewTraitsDiscovered"]);
        assert_eq!(m["StarPos"], json!(SHINRARTA_POS));
        assert_eq!((&m["Latitude"], &m["VoucherAmount"]), (&json!(12.5), &json!(50000)), "codexentry keeps these");
        m
    }

    fn approach(name: &str, id: u64) -> Value {
        json!({"timestamp":"2026-10-16T12:08:30Z","event":"ApproachBody","StarSystem":"Shinrarta Dezhra",
            "SystemAddress":SHINRARTA,"Body":name,"BodyName":name,"BodyID":id})
    }

    #[test]
    fn codexentry_gets_bodyid_only_when_status_and_journal_agree() {
        let m = codex_with(json!({"BodyName":"Shinrarta Dezhra A 1"}), &[approach("Shinrarta Dezhra A 1", 2)]);
        assert_eq!((&m["BodyName"], &m["BodyID"]), (&json!("Shinrarta Dezhra A 1"), &json!(2)), "not the raw 99");

        // A close binary: the journal still has the first body, Status.json the second.
        let m = codex_with(json!({"BodyName":"Shinrarta Dezhra A 2"}), &[approach("Shinrarta Dezhra A 1", 2)]);
        assert_eq!(m["BodyName"], "Shinrarta Dezhra A 2");
        assert_absent(&m, &["BodyID"]);

        // No body in Status.json: neither key.
        let m = codex_with(json!({"Flags":0}), &[approach("Shinrarta Dezhra A 1", 2)]);
        assert_absent(&m, &["BodyName", "BodyID"]);

        // LeaveBody forgets the journal body.
        let leave =
            json!({"timestamp":"2026-10-16T12:08:40Z","event":"LeaveBody","Body":"Shinrarta Dezhra A 1","BodyID":2});
        let m = codex_with(json!({"BodyName":"Shinrarta Dezhra A 1"}), &[approach("Shinrarta Dezhra A 1", 2), leave]);
        assert_absent(&m, &["BodyID"]);

        // Location names the body as `Body`.
        let mut loc = shinrarta_location();
        loc["Body"] = json!("Shinrarta Dezhra A 1");
        loc["BodyID"] = json!(2);
        let m = codex_with(json!({"BodyName":"Shinrarta Dezhra A 1"}), &[loc]);
        assert_eq!(m["BodyID"], 2);
    }

    #[test]
    fn navroute_comes_from_the_announced_file() {
        let ev = json!({"timestamp":"2026-10-16T12:10:00Z","event":"NavRoute"});
        let file = json!({"timestamp":"2026-10-16T12:10:00Z","event":"NavRoute","Route":[
            {"StarSystem":"Shinrarta Dezhra","SystemAddress":SHINRARTA,"StarPos":SHINRARTA_POS,"StarClass":"K"},
            {"StarSystem":"Sol","SystemAddress":SOL,"StarPos":[0.0,0.0,0.0],"StarClass":"G"}]});
        let out = events::navroute(&ev, &file).unwrap();
        let m = validated(&live_context(), &out);
        assert_eq!(m["Route"], file["Route"]);
        // The journal event alone sends nothing.
        assert!(EventSchemas::default().on_event(&ev, &live_context()).is_empty());
        // A file from an earlier plot, or a cleared route, is not sent.
        let stale = json!({"timestamp":"2026-10-16T11:00:00Z","event":"NavRoute","Route":file["Route"]});
        assert!(events::navroute(&ev, &stale).is_none());
        let cleared = json!({"timestamp":"2026-10-16T12:10:00Z","event":"NavRoute","Route":[]});
        assert!(events::navroute(&ev, &cleared).is_none());
    }

    #[test]
    fn fcmaterials_journal_comes_from_the_announced_file() {
        let ev = json!({"timestamp":"2026-10-16T12:11:00Z","event":"FCMaterials","MarketID":3700000001u64,
            "CarrierName":"ASTROMECH","CarrierID":"X9X-9XX"});
        let file = json!({"timestamp":"2026-10-16T12:11:00Z","event":"FCMaterials","MarketID":3700000001u64,
            "CarrierName":"ASTROMECH","CarrierID":"X9X-9XX","Items":[
                {"id":128961524,"Name":"$aerogel_name;","Name_Localised":"Aerogel","Price":500,"Stock":0,"Demand":10},
                {"id":128972290,"Name":"$chemicalcatalyst_name;","Name_Localised":"Chemical Catalyst","Price":800,"Stock":5,"Demand":0}]});
        let m = validated(&live_context(), &events::fcmaterials(&ev, &file).unwrap());
        assert_eq!(m["Items"][0], json!({"id":128961524,"Name":"$aerogel_name;","Price":500,"Stock":0,"Demand":10}));
        let stale = json!({"timestamp":"2026-10-15T09:00:00Z","event":"FCMaterials","MarketID":1,"CarrierName":"A","CarrierID":"B","Items":[]});
        assert!(events::fcmaterials(&ev, &stale).is_none());
    }

    // fsssignaldiscovered batching.

    fn signal(ts: &str, address: u64, extra: Value) -> Value {
        let mut s = json!({"timestamp":ts,"event":"FSSSignalDiscovered","SystemAddress":address});
        s.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
        s
    }

    fn music(ts: &str) -> Value {
        json!({"timestamp":ts,"event":"Music","MusicTrack":"Exploration"})
    }

    fn station(ts: &str, address: u64) -> Value {
        signal(ts, address, json!({"SignalName":"Jameson Memorial","SignalType":"Outpost","IsStation":true}))
    }

    #[test]
    fn a_run_of_signals_is_one_message_ended_by_the_next_other_event() {
        let mut ctx = live_context();
        let mut s = EventSchemas::default();
        let run = [
            station("2026-10-16T12:00:11Z", SHINRARTA),
            signal(
                "2026-10-16T12:00:12Z",
                SHINRARTA,
                json!({"SignalName":"$USS_NonHumanSignalSource;",
                "SignalName_Localised":"Non-Human signal source","SignalType":"USS","USSType":"$USS_Type_NonHuman;",
                "USSType_Localised":"Non-Human signal source","SpawningState":"","SpawningFaction":"$faction_none;",
                "ThreatLevel":5,"TimeRemaining":1800.5}),
            ),
            signal(
                "2026-10-16T12:00:12Z",
                SHINRARTA,
                json!({"SignalName":"$USS_MissionTarget;",
                "SignalType":"USS","USSType":"$USS_Type_MissionTarget;","ThreatLevel":0,"TimeRemaining":900.0}),
            ),
            signal(
                "2026-10-16T12:00:13Z",
                SHINRARTA,
                json!({"SignalName":"X9X-9XX","SignalType":"FleetCarrier","IsStation":true}),
            ),
        ];
        assert!(feed(&mut ctx, &mut s, &run).is_empty(), "held until the run ends");
        let out = feed(&mut ctx, &mut s, &[music("2026-10-16T12:00:14Z")]);
        assert_eq!(out.len(), 1);
        let m = validated(&ctx, &out[0]);
        assert_eq!(m["timestamp"], "2026-10-16T12:00:11Z", "the first signal's timestamp");
        assert_eq!((&m["SystemAddress"], &m["StarSystem"]), (&json!(SHINRARTA), &json!("Shinrarta Dezhra")));
        assert_eq!(m["StarPos"], json!(SHINRARTA_POS));
        let signals = m["signals"].as_array().unwrap();
        let names: Vec<&str> = signals.iter().map(|s| s["SignalName"].as_str().unwrap()).collect();
        assert_eq!(names, ["Jameson Memorial", "$USS_NonHumanSignalSource;", "X9X-9XX"], "mission target dropped");
        assert_absent(&m, &["signals[].event", "signals[].SystemAddress", "signals[].TimeRemaining"]);
        assert!(signals.iter().all(|s| s.get("timestamp").is_some()), "each signal keeps its timestamp");
        // Nothing left over.
        assert!(s.flush(&ctx).is_none());
    }

    #[test]
    fn signals_before_an_fsdjump_are_checked_against_the_new_system() {
        // Odyssey writes the arrival system's signals before the FSDJump.
        let mut ctx = live_context();
        let mut s = EventSchemas::default();
        let jump = json!({"timestamp":"2026-10-16T12:30:01Z","event":"FSDJump","StarSystem":"Sol",
            "SystemAddress":SOL,"StarPos":[0.0,0.0,0.0],"Body":"Sol","BodyID":0,"BodyType":"Star"});
        let out = feed(
            &mut ctx,
            &mut s,
            &[station("2026-10-16T12:30:00Z", SOL), station("2026-10-16T12:30:00Z", SHINRARTA), jump],
        );
        assert_eq!(out.len(), 1);
        let m = validated(&ctx, &out[0]);
        assert_eq!((&m["StarSystem"], &m["SystemAddress"]), (&json!("Sol"), &json!(SOL)));
        assert_eq!(m["signals"].as_array().unwrap().len(), 1, "the Shinrarta signal is dropped");
    }

    #[test]
    fn separate_runs_are_separate_messages() {
        let mut ctx = live_context();
        let mut s = EventSchemas::default();
        let out = feed(
            &mut ctx,
            &mut s,
            &[
                station("2026-10-16T12:00:11Z", SHINRARTA),
                music("2026-10-16T12:00:12Z"),
                station("2026-10-16T12:00:20Z", SHINRARTA),
                music("2026-10-16T12:00:21Z"),
            ],
        );
        let ts: Vec<&Value> = out.iter().map(|(_, m)| &m["timestamp"]).collect();
        assert_eq!(ts, [&json!("2026-10-16T12:00:11Z"), &json!("2026-10-16T12:00:20Z")]);
    }

    #[test]
    fn the_batch_comes_before_the_ending_events_own_message() {
        let mut ctx = live_context();
        let mut s = EventSchemas::default();
        let scan = json!({"timestamp":"2026-10-16T12:00:12Z","event":"FSSDiscoveryScan","Progress":0.1,
            "BodyCount":12,"NonBodyCount":30,"SystemName":"Shinrarta Dezhra","SystemAddress":SHINRARTA});
        let out = feed(&mut ctx, &mut s, &[station("2026-10-16T12:00:11Z", SHINRARTA), scan]);
        let names: Vec<&str> = out.iter().map(|(spec, _)| spec.name).collect();
        assert_eq!(names, ["fsssignaldiscovered", "fssdiscoveryscan"]);
    }

    #[test]
    fn flush_sends_a_pending_run_and_drops_it_without_a_location() {
        let mut ctx = live_context();
        let mut s = EventSchemas::default();
        feed(&mut ctx, &mut s, &[station("2026-10-16T12:00:11Z", SHINRARTA)]);
        let out = s.flush(&ctx).expect("pending run");
        validated(&ctx, &out);
        assert!(s.flush(&ctx).is_none());

        // No location fix: nothing to cross-check against, so nothing is sent.
        let mut s = EventSchemas::default();
        let mut ctx = live_context();
        ctx.system = None;
        s.on_event(&station("2026-10-16T12:00:11Z", SHINRARTA), &ctx);
        assert!(s.flush(&ctx).is_none());
    }

    #[test]
    fn a_run_with_only_other_systems_or_mission_targets_sends_nothing() {
        let mut ctx = live_context();
        let mut s = EventSchemas::default();
        let mission = signal(
            "2026-10-16T12:00:12Z",
            SHINRARTA,
            json!({"SignalName":"$USS_MissionTarget;",
            "USSType":"$USS_Type_MissionTarget;"}),
        );
        let out =
            feed(&mut ctx, &mut s, &[station("2026-10-16T12:00:11Z", SOL), mission, music("2026-10-16T12:00:13Z")]);
        assert!(out.is_empty(), "{out:?}");
    }
}
