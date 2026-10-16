//! The event-specific schemas: a synthetic fixture for each, built by the sender and validated
//! against its live schema file, plus the `fsssignaldiscovered` batching and the `codexentry`
//! body rules.

mod schema_check;

use astromech_eddn::events::{self, EventSchemas, Outgoing};
use astromech_eddn::GameContext;
use schema_check::*;
use serde_json::{json, Value};

const SHINRARTA_POS: [f64; 3] = [55.71875, 17.59375, 27.15625];
const SOL: u64 = 10477373803;

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
    let mut ctx = live();
    let out = feed(&mut ctx, &mut EventSchemas::default(), &[ev]);
    assert_eq!(out.len(), 1, "{out:?}");
    validated(&ctx, &out[0])
}

fn validated(ctx: &GameContext, (spec, msg): &Outgoing) -> Value {
    let up = upload(ctx, spec, msg.clone());
    assert_valid(spec, &up);
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
    let mut ctx = live();
    let ev = json!({"timestamp":"2026-10-16T12:01:00Z","event":"FSSDiscoveryScan","Progress":0.25,
        "BodyCount":12,"NonBodyCount":30,"SystemName":"Lave","SystemAddress":SHINRARTA});
    let out = feed(&mut ctx, &mut EventSchemas::default(), &[ev]);
    let sw = astromech_eddn::Software { name: "a".into(), version: "1".into() };
    let r = astromech_eddn::message::build(out[0].0, &ctx, &sw, false, out[0].1.clone());
    assert!(matches!(r, Err(astromech_eddn::Dropped::LocationMismatch(_))), "{r:?}");
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
    let m = one(json!({"timestamp":"2026-10-16T12:05:00Z","event":"ScanBaryCentre","StarSystem":"Shinrarta Dezhra",
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
    assert!(EventSchemas::default().on_event(&partial, &live()).is_empty());
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
    let mut ctx = live();
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
    let m = validated(&live(), &out);
    assert_eq!(m["Route"], file["Route"]);
    // The journal event alone sends nothing.
    assert!(EventSchemas::default().on_event(&ev, &live()).is_empty());
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
    let m = validated(&live(), &events::fcmaterials(&ev, &file).unwrap());
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
    let mut ctx = live();
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
    let mut ctx = live();
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
    let mut ctx = live();
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
    let mut ctx = live();
    let mut s = EventSchemas::default();
    let scan = json!({"timestamp":"2026-10-16T12:00:12Z","event":"FSSDiscoveryScan","Progress":0.1,
        "BodyCount":12,"NonBodyCount":30,"SystemName":"Shinrarta Dezhra","SystemAddress":SHINRARTA});
    let out = feed(&mut ctx, &mut s, &[station("2026-10-16T12:00:11Z", SHINRARTA), scan]);
    let names: Vec<&str> = out.iter().map(|(spec, _)| spec.name).collect();
    assert_eq!(names, ["fsssignaldiscovered", "fssdiscoveryscan"]);
}

#[test]
fn flush_sends_a_pending_run_and_drops_it_without_a_location() {
    let mut ctx = live();
    let mut s = EventSchemas::default();
    feed(&mut ctx, &mut s, &[station("2026-10-16T12:00:11Z", SHINRARTA)]);
    let out = s.flush(&ctx).expect("pending run");
    validated(&ctx, &out);
    assert!(s.flush(&ctx).is_none());

    // No location fix: nothing to cross-check against, so nothing is sent.
    let mut s = EventSchemas::default();
    let mut ctx = live();
    ctx.system = None;
    s.on_event(&station("2026-10-16T12:00:11Z", SHINRARTA), &ctx);
    assert!(s.flush(&ctx).is_none());
}

#[test]
fn a_run_with_only_other_systems_or_mission_targets_sends_nothing() {
    let mut ctx = live();
    let mut s = EventSchemas::default();
    let mission = signal(
        "2026-10-16T12:00:12Z",
        SHINRARTA,
        json!({"SignalName":"$USS_MissionTarget;",
        "USSType":"$USS_Type_MissionTarget;"}),
    );
    let out = feed(&mut ctx, &mut s, &[station("2026-10-16T12:00:11Z", SOL), mission, music("2026-10-16T12:00:13Z")]);
    assert!(out.is_empty(), "{out:?}");
}
