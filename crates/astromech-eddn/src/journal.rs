//! `journal/1`: the historical "all journal events" schema, for `Docked`, `FSDJump`, `Scan`,
//! `Location`, `SAASignalsFound` and `CarrierJump`. Rules from EDCD `schemas/journal-README.md`
//! and `journal-v1.0.json` (live branch, commit 4ad669bb, read 16 Oct 2026).
//!
//! `CodexEntry` is also allowed by the schema, but its README sends it on `codexentry/1` instead
//! ([`crate::events`]), so it is not sent here.

use crate::message::{Augment, Elision, SchemaSpec, SystemField};
use serde_json::Value;

const fn all(path: &'static str) -> Elision {
    Elision { event: None, path }
}

const fn on(event: &'static str, field: SystemField) -> Augment {
    Augment { event: Some(event), field }
}

pub const JOURNAL: SchemaSpec = SchemaSpec {
    name: "journal",
    version: 1,
    // The README lists these per event (Docked: Wanted, ActiveFine, CockpitBreach; FSDJump:
    // Wanted, BoostUsed, FuelLevel, FuelUsed, JumpDist and the faction keys; Location: Wanted,
    // Latitude, Longitude and the faction keys). The schema disallows every one of them on every
    // event, so they are removed from all of them, with the rest of its disallowed keys.
    elide: &[
        all("ActiveFine"),
        all("CockpitBreach"),
        all("BoostUsed"),
        all("FuelLevel"),
        all("FuelUsed"),
        all("JumpDist"),
        all("Latitude"),
        all("Longitude"),
        all("Wanted"),
        all("IsNewEntry"),
        all("NewTraitsDiscovered"),
        all("Traits"),
        all("VoucherAmount"),
        all("Factions[].HappiestSystem"),
        all("Factions[].HomeSystem"),
        all("Factions[].MyReputation"),
        all("Factions[].SquadronFaction"),
    ],
    // StarSystem only for SAASignalsFound; StarPos only for Docked, Scan and SAASignalsFound.
    augment: &[
        on("SAASignalsFound", SystemField::StarSystem),
        on("SAASignalsFound", SystemField::StarPos),
        on("Docked", SystemField::StarPos),
        on("Scan", SystemField::StarPos),
    ],
    flags: true,
};

/// The `journal/1` message for one journal event (its raw JSON), or `None` when the event isn't
/// sent on `journal/1`.
///
/// The schema requires `StarSystem`, `StarPos` and `SystemAddress`, and the README allows adding
/// only some of them, so an event missing one it can't be given is not sent: the gateway would
/// reject it.
pub fn message(ev: &Value) -> Option<(&'static SchemaSpec, Value)> {
    let needs: &[&str] = match ev.get("event")?.as_str()? {
        "FSDJump" | "Location" | "CarrierJump" => &["StarSystem", "SystemAddress", "StarPos"],
        "Docked" | "Scan" => &["StarSystem", "SystemAddress"],
        "SAASignalsFound" => &["SystemAddress"],
        _ => return None,
    };
    has_all(ev, needs).then(|| (&JOURNAL, ev.clone()))
}

/// Every key in `keys` is present and not `null`.
pub(crate) fn has_all(ev: &Value, keys: &[&str]) -> bool {
    keys.iter().all(|k| ev.get(*k).is_some_and(|v| !v.is_null()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn only_the_six_events_are_sent() {
        for e in ["Docked", "Scan"] {
            assert!(message(&json!({"event":e,"StarSystem":"Sol","SystemAddress":1})).is_some(), "{e}");
        }
        for e in ["FSDJump", "Location", "CarrierJump"] {
            let ev = json!({"event":e,"StarSystem":"Sol","SystemAddress":1,"StarPos":[0.0,0.0,0.0]});
            assert!(message(&ev).is_some(), "{e}");
        }
        assert!(message(&json!({"event":"SAASignalsFound","SystemAddress":1})).is_some());
        for e in ["CodexEntry", "Music", "FSSDiscoveryScan"] {
            assert!(message(&json!({"event":e,"StarSystem":"Sol","SystemAddress":1})).is_none(), "{e}");
        }
    }

    #[test]
    fn events_missing_a_field_that_cannot_be_added_are_not_sent() {
        assert!(message(&json!({"event":"Scan","SystemAddress":1})).is_none(), "StarSystem isn't added for Scan");
        assert!(message(&json!({"event":"FSDJump","StarSystem":"Sol","SystemAddress":1})).is_none());
        assert!(message(&json!({"event":"SAASignalsFound"})).is_none());
        assert!(message(&json!({"event":"Docked","StarSystem":"Sol","SystemAddress":null})).is_none());
    }
}

#[cfg(test)]
mod schema_tests {
    // journal/1: a synthetic fixture of each event, built by the sender and validated against the
    // live `journal-v1.0.json`, with the README's elisions asserted absent.

    use super::JOURNAL;
    use crate::journal;
    use crate::message::build;
    use crate::testutil::*;
    use crate::{Dropped, Software};
    use serde_json::{json, Value};

    /// Every key journal/1 disallows anywhere in a message.
    const DISALLOWED: &[&str] = &[
        "ActiveFine",
        "CockpitBreach",
        "BoostUsed",
        "FuelLevel",
        "FuelUsed",
        "JumpDist",
        "Latitude",
        "Longitude",
        "Wanted",
        "IsNewEntry",
        "NewTraitsDiscovered",
        "Traits",
        "VoucherAmount",
        "Factions[].HappiestSystem",
        "Factions[].HomeSystem",
        "Factions[].MyReputation",
        "Factions[].SquadronFaction",
    ];

    /// Route one journal event the way the sender does and validate the result.
    fn send(ctx: &crate::GameContext, ev: Value) -> Value {
        let (spec, msg) = journal::message(&ev).expect("journal/1 event");
        let up = upload(ctx, spec, msg);
        assert_valid(&schema_file(spec), &up);
        assert!(!up.to_string().contains("_Localised"), "{up:#}");
        assert_absent(&up["message"], DISALLOWED);
        assert!(!up.to_string().contains("F42"), "the FID never leaves");
        assert_eq!((&up["message"]["horizons"], &up["message"]["odyssey"]), (&json!(true), &json!(true)));
        up
    }

    fn factions() -> Value {
        json!([{"Name":"Pilots' Federation Local Branch","FactionState":"None","Government":"Democracy",
            "Influence":0.0,"Allegiance":"PilotsFederation","Happiness":"$Faction_HappinessBand2;",
            "Happiness_Localised":"Happy","MyReputation":42.0,"SquadronFaction":false,"HappiestSystem":true,
            "HomeSystem":true,"ActiveStates":[{"State":"Boom","State_Localised":"Boom"}]}])
    }

    #[test]
    fn docked_validates_with_starpos_added_and_personal_keys_gone() {
        let ev = json!({"timestamp":"2026-10-16T12:05:00Z","event":"Docked","StationName":"Jameson Memorial",
            "StationType":"Orbis","Taxi":false,"Multicrew":false,"StarSystem":"Shinrarta Dezhra",
            "SystemAddress":SHINRARTA,"MarketID":128666762,"StationFaction":{"Name":"Pilots' Federation Local Branch"},
            "StationGovernment":"$government_Democracy;","StationGovernment_Localised":"Democracy",
            "StationServices":["dock","autodock","commodities"],"StationEconomy":"$economy_HighTech;",
            "StationEconomy_Localised":"High Tech",
            "StationEconomies":[{"Name":"$economy_HighTech;","Name_Localised":"High Tech","Proportion":1.0}],
            "DistFromStarLS":350.0,"LandingPads":{"Small":17,"Medium":18,"Large":9},
            "Wanted":true,"ActiveFine":true,"CockpitBreach":true});
        let up = send(&live_context(), ev);
        assert_eq!(up["message"]["StarPos"], json!(SHINRARTA_POS));
        assert_eq!(up["message"]["StationEconomies"], json!([{"Name":"$economy_HighTech;","Proportion":1.0}]));
    }

    #[test]
    fn fsdjump_validates_with_jump_and_faction_personal_keys_gone() {
        let ev = json!({"timestamp":"2026-10-16T12:10:00Z","event":"FSDJump","Taxi":false,"Multicrew":false,
            "StarSystem":"Sol","SystemAddress":10477373803u64,"StarPos":[0.0,0.0,0.0],
            "SystemAllegiance":"Federation","SystemEconomy":"$economy_Refinery;","SystemEconomy_Localised":"Refinery",
            "SystemGovernment":"$government_Democracy;","SystemSecurity":"$SYSTEM_SECURITY_high;",
            "Population":22780919531u64,"Body":"Sol","BodyID":0,"BodyType":"Star",
            "JumpDist":8.2,"FuelUsed":1.2,"FuelLevel":30.5,"BoostUsed":1,"Wanted":true,
            "Factions":factions(),"SystemFaction":{"Name":"Mother Gaia","FactionState":"None"}});
        let mut ctx = live_context();
        ctx.observe(&ev);
        let up = send(&ctx, ev);
        assert_eq!(up["message"]["Factions"][0]["Influence"], 0.0, "public faction data stays");
    }

    #[test]
    fn scan_validates_with_starpos_added() {
        let ev = json!({"timestamp":"2026-10-16T12:06:00Z","event":"Scan","ScanType":"Detailed",
            "BodyName":"Shinrarta Dezhra A 1","BodyID":2,"Parents":[{"Star":1}],"StarSystem":"Shinrarta Dezhra",
            "SystemAddress":SHINRARTA,"DistanceFromArrivalLS":210.5,"TidalLock":false,"TerraformState":"",
            "PlanetClass":"Icy body","Atmosphere":"","Volcanism":"","MassEM":0.05,"Radius":2100000.0,
            "SurfaceGravity":1.2,"SurfaceTemperature":95.0,"SurfacePressure":0.0,"Landable":true,
            "Materials":[{"Name":"iron","Name_Localised":"Iron","Percent":18.5}],
            "Composition":{"Ice":0.8,"Rock":0.15,"Metal":0.05},"SemiMajorAxis":6.3e10,"Eccentricity":0.01,
            "OrbitalInclination":0.2,"Periapsis":12.0,"OrbitalPeriod":3.1e7,"AscendingNode":5.0,
            "MeanAnomaly":100.0,"RotationPeriod":90000.0,"AxialTilt":0.1,"WasDiscovered":true,"WasMapped":true});
        let up = send(&live_context(), ev);
        assert_eq!(up["message"]["StarPos"], json!(SHINRARTA_POS));
        assert_eq!(up["message"]["Materials"], json!([{"Name":"iron","Percent":18.5}]));
    }

    #[test]
    fn location_validates_with_latitude_longitude_and_wanted_gone() {
        let ev = json!({"timestamp":"2026-10-16T12:00:10Z","event":"Location","DistFromStarLS":350.0,
            "Docked":false,"Taxi":false,"Multicrew":false,"StarSystem":"Shinrarta Dezhra","SystemAddress":SHINRARTA,
            "StarPos":SHINRARTA_POS,"SystemAllegiance":"PilotsFederation","SystemEconomy":"$economy_HighTech;",
            "SystemEconomy_Localised":"High Tech","Population":85206935,"Body":"Shinrarta Dezhra A 1","BodyID":2,
            "BodyType":"Planet","Latitude":12.5,"Longitude":-45.25,"Wanted":true,"Factions":factions(),
            "SystemFaction":{"Name":"Pilots' Federation Local Branch"}});
        send(&live_context(), ev);
    }

    #[test]
    fn saasignalsfound_validates_with_starsystem_and_starpos_added() {
        let ev = json!({"timestamp":"2026-10-16T12:07:00Z","event":"SAASignalsFound",
            "BodyName":"Shinrarta Dezhra A 1","SystemAddress":SHINRARTA,"BodyID":2,
            "Signals":[{"Type":"$SAA_SignalType_Geological;","Type_Localised":"Geological","Count":3}],
            "Genuses":[{"Genus":"$Codex_Ent_Bacterial_Genus_Name;","Genus_Localised":"Bacterium"}]});
        let up = send(&live_context(), ev);
        assert_eq!(up["message"]["StarSystem"], "Shinrarta Dezhra");
        assert_eq!(up["message"]["StarPos"], json!(SHINRARTA_POS));
        assert_eq!(up["message"]["Signals"], json!([{"Type":"$SAA_SignalType_Geological;","Count":3}]));
    }

    #[test]
    fn carrierjump_validates_with_faction_personal_keys_gone() {
        let ev = json!({"timestamp":"2026-10-16T12:20:00Z","event":"CarrierJump","Docked":true,"OnFoot":false,
            "StationName":"X9X-9XX","StationType":"FleetCarrier","MarketID":3700000001u64,
            "StationFaction":{"Name":"FleetCarrier"},"StationGovernment":"$government_Carrier;",
            "StationGovernment_Localised":"Private Ownership","StationServices":["dock","commodities"],
            "StationEconomy":"$economy_Carrier;","StationEconomy_Localised":"Private Enterprise",
            "StationEconomies":[{"Name":"$economy_Carrier;","Name_Localised":"Private Enterprise","Proportion":1.0}],
            "Taxi":false,"Multicrew":false,"StarSystem":"Sol","SystemAddress":10477373803u64,"StarPos":[0.0,0.0,0.0],
            "SystemAllegiance":"Federation","Population":22780919531u64,"Body":"Sol","BodyID":0,"BodyType":"Star",
            "Factions":factions(),"SystemFaction":{"Name":"Mother Gaia"}});
        let mut ctx = live_context();
        ctx.observe(&ev);
        send(&ctx, ev);
    }

    #[test]
    fn the_schema_rejects_what_the_elisions_remove() {
        // Proof the validator is not vacuous: the raw event, unelided and without StarPos, fails.
        let raw = json!({"timestamp":"2026-10-16T12:05:00Z","event":"Docked","StarSystem":"Shinrarta Dezhra",
            "SystemAddress":SHINRARTA,"Wanted":true});
        let up = json!({"$schemaRef":"https://eddn.edcd.io/schemas/journal/1","header":{"uploaderID":"Jameson",
            "softwareName":"astromech","softwareVersion":"0.1.5"},"message":raw});
        let errs = schema_errors("journal-v1.0.json", &up);
        assert!(errs.iter().any(|e| e.contains("StarPos")), "{errs:?}");
        assert!(errs.iter().any(|e| e.contains("/message/Wanted")), "{errs:?}");
    }

    #[test]
    fn scan_from_another_system_is_dropped_not_augmented() {
        let ev = json!({"timestamp":"2026-10-16T12:06:00Z","event":"Scan","BodyName":"Sol 3","BodyID":3,
            "StarSystem":"Sol","SystemAddress":10477373803u64});
        let (spec, msg) = journal::message(&ev).unwrap();
        let sw = Software { name: "astromech".into(), version: "0.1.5".into() };
        assert!(matches!(build(spec, &live_context(), &sw, false, msg), Err(Dropped::LocationMismatch(_))));
        assert_eq!(spec.name, JOURNAL.name);
    }
}
