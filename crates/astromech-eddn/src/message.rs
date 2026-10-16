//! Building one EDDN message from journal-shaped JSON, following EDCD `docs/Developers.md`
//! (live branch). Each schema slice supplies a [`SchemaSpec`]; the rules every schema shares
//! (live-only, `*_Localised`, commander data, header, flags, augmentation cross-check) live here.

use crate::context::{GameContext, SystemFix};
use serde_json::{json, Map, Value};
use std::fmt;

/// EDDN rejects bodies over 1 MiB, measured after gzip.
pub const MAX_BODY: usize = 1024 * 1024;

/// One EDDN schema: its name and version, and what the schema README says to remove or add.
#[derive(Debug)]
pub struct SchemaSpec {
    /// `"commodity"`, `"journal"`, …
    pub name: &'static str,
    pub version: u32,
    /// Removals on top of the global ones ([`COMMANDER_KEYS`], `*_Localised`).
    pub elide: &'static [Elision],
    /// System fields the schema requires the sender to add if the source lacks them.
    pub augment: &'static [Augment],
    /// The schema has `horizons`/`odyssey` keys.
    pub flags: bool,
}

/// Remove `path` from the message. `path` is dot-separated; a segment ending in `[]` is an array
/// whose every element gets the rest of the path, e.g. `"Factions[].MyReputation"`.
#[derive(Debug, Clone, Copy)]
pub struct Elision {
    /// Only for this journal `event` (the message's `event` key); `None` for every message.
    pub event: Option<&'static str>,
    pub path: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Augment {
    pub event: Option<&'static str>,
    pub field: SystemField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemField {
    StarSystem,
    SystemAddress,
    StarPos,
}

impl SystemField {
    fn key(self) -> &'static str {
        match self {
            SystemField::StarSystem => "StarSystem",
            SystemField::SystemAddress => "SystemAddress",
            SystemField::StarPos => "StarPos",
        }
    }
}

/// Commander-specific keys that no EDDN schema accepts, whatever the event.
/// (Per-schema ones, such as `Latitude` in journal/1 but not approachsettlement/1, go in the spec.)
pub const COMMANDER_KEYS: &[&str] = &[
    "FID",
    "Commander",
    "Wanted",
    "ActiveFine",
    "CockpitBreach",
    "BoostUsed",
    "FuelLevel",
    "FuelUsed",
    "JumpDist",
    "IsNewEntry",
    "NewTraitsDiscovered",
    "Factions[].HappiestSystem",
    "Factions[].HomeSystem",
    "Factions[].MyReputation",
    "Factions[].SquadronFaction",
];

/// The sender's identity in every header.
#[derive(Debug, Clone)]
pub struct Software {
    pub name: String,
    pub version: String,
}

/// Why a message was not queued. Every drop is written to the send log.
#[derive(Debug, Clone, PartialEq)]
pub enum Dropped {
    /// From a beta or alpha client: never sent to EDDN.
    Beta(String),
    /// No `Fileheader`/`LoadGame` yet, so it can't be shown to be live data.
    UnknownGameVersion,
    NoCommander,
    NotAnObject,
    /// Augmentation needed but there's no last-location fix, or nothing in the message to check
    /// it against.
    NoLocation,
    /// The message names a different system than the last location event.
    LocationMismatch(String),
    /// Gzipped body over [`MAX_BODY`] bytes.
    TooLarge(usize),
}

impl fmt::Display for Dropped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dropped::Beta(v) => write!(f, "beta game version {v:?}"),
            Dropped::UnknownGameVersion => write!(f, "game version not known yet"),
            Dropped::NoCommander => write!(f, "commander not known yet"),
            Dropped::NotAnObject => write!(f, "message is not a JSON object"),
            Dropped::NoLocation => write!(f, "no location to cross-check the system against"),
            Dropped::LocationMismatch(m) => write!(f, "location mismatch: {m}"),
            Dropped::TooLarge(n) => write!(f, "{n} bytes gzipped, over the {MAX_BODY}-byte limit"),
        }
    }
}

pub fn schema_ref(spec: &SchemaSpec, test: bool) -> String {
    format!("https://eddn.edcd.io/schemas/{}/{}{}", spec.name, spec.version, if test { "/test" } else { "" })
}

/// Build the full upload (`$schemaRef`, `header`, `message`) from `message`, which is the
/// journal event (or the schema-shaped object) after any schema-specific renames.
pub fn build(
    spec: &SchemaSpec,
    ctx: &GameContext,
    software: &Software,
    test: bool,
    message: Value,
) -> Result<Value, Dropped> {
    let Value::Object(mut msg) = message else {
        return Err(Dropped::NotAnObject);
    };
    let gameversion = ctx.gameversion.clone().ok_or(Dropped::UnknownGameVersion)?;
    if ctx.is_beta() {
        return Err(Dropped::Beta(gameversion));
    }
    let uploader = ctx.commander.clone().ok_or(Dropped::NoCommander)?;

    strip_localised(&mut msg);
    let event = msg.get("event").and_then(Value::as_str).map(str::to_owned);
    let applies = |e: Option<&str>| e.is_none() || e == event.as_deref();
    for path in COMMANDER_KEYS {
        remove_path(&mut msg, path);
    }
    for el in spec.elide.iter().filter(|el| applies(el.event)) {
        remove_path(&mut msg, el.path);
    }

    let wanted: Vec<SystemField> = spec.augment.iter().filter(|a| applies(a.event)).map(|a| a.field).collect();
    if wanted.iter().any(|f| !msg.contains_key(f.key())) {
        let fix = cross_check(&msg, ctx.system.as_ref())?;
        for f in wanted {
            msg.entry(f.key()).or_insert_with(|| match f {
                SystemField::StarSystem => json!(fix.name),
                SystemField::SystemAddress => json!(fix.address),
                SystemField::StarPos => json!(fix.pos),
            });
        }
    }

    // Never null; absent when unknown.
    msg.remove("horizons");
    msg.remove("odyssey");
    if spec.flags {
        if let Some(h) = ctx.horizons {
            msg.insert("horizons".into(), h.into());
        }
        if let Some(o) = ctx.odyssey {
            msg.insert("odyssey".into(), o.into());
        }
    }

    Ok(json!({
        "$schemaRef": schema_ref(spec, test),
        "header": {
            "uploaderID": uploader,
            "softwareName": software.name,
            "softwareVersion": software.version,
            "gameversion": gameversion,
            "gamebuild": ctx.gamebuild.clone().unwrap_or_default(),
        },
        "message": msg,
    }))
}

/// Before adding system data, check what the message already says about its system against the
/// last location event. Returns the fix to augment from, or why the message must be dropped.
pub fn cross_check<'a>(msg: &Map<String, Value>, fix: Option<&'a SystemFix>) -> Result<&'a SystemFix, Dropped> {
    let fix = fix.ok_or(Dropped::NoLocation)?;
    let address = msg.get("SystemAddress").map(|v| v.as_u64());
    let names: Vec<&str> = SYSTEM_NAME_KEYS.iter().copied().filter(|k| msg.contains_key(*k)).collect();
    if names.is_empty() && address.is_none() {
        return Err(Dropped::NoLocation);
    }
    if let Some(a) = address {
        if a != Some(fix.address) {
            return Err(Dropped::LocationMismatch(format!(
                "SystemAddress {} vs last location {}",
                msg["SystemAddress"], fix.address
            )));
        }
    }
    for k in names {
        if msg[k].as_str() != Some(fix.name.as_str()) {
            return Err(Dropped::LocationMismatch(format!("{k} {} vs last location {:?}", msg[k], fix.name)));
        }
    }
    Ok(fix)
}

/// Keys that name the message's system: `StarSystem` in most events, `SystemName` in
/// `FSSDiscoveryScan`/`FSSAllBodiesFound`, `System` in `CodexEntry`. Every one present is checked.
const SYSTEM_NAME_KEYS: &[&str] = &["StarSystem", "SystemName", "System"];

/// Remove every key ending in `_Localised`, at any depth.
pub fn strip_localised(m: &mut Map<String, Value>) {
    m.retain(|k, _| !k.ends_with("_Localised"));
    for v in m.values_mut() {
        strip_localised_value(v);
    }
}

fn strip_localised_value(v: &mut Value) {
    match v {
        Value::Object(m) => strip_localised(m),
        Value::Array(a) => a.iter_mut().for_each(strip_localised_value),
        _ => {}
    }
}

fn remove_path(m: &mut Map<String, Value>, path: &str) {
    match path.split_once('.') {
        None => {
            m.remove(path);
        }
        Some((head, rest)) => {
            let (key, each) = match head.strip_suffix("[]") {
                Some(k) => (k, true),
                None => (head, false),
            };
            match m.get_mut(key) {
                Some(Value::Array(a)) if each => {
                    for item in a {
                        if let Value::Object(o) = item {
                            remove_path(o, rest);
                        }
                    }
                }
                Some(Value::Object(o)) if !each => remove_path(o, rest),
                _ => {}
            }
        }
    }
}

/// Serialise and gzip an upload, refusing anything EDDN would reject as too large.
pub fn encode(upload: &Value) -> Result<Vec<u8>, Dropped> {
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    // Writing to a Vec can't fail.
    serde_json::to_writer(&mut gz, upload).expect("serialise to memory");
    let body = gz.finish().expect("gzip to memory");
    if body.len() > MAX_BODY {
        return Err(Dropped::TooLarge(body.len()));
    }
    Ok(body)
}

#[cfg(test)]
pub(crate) fn gunzip(b: &[u8]) -> Value {
    serde_json::from_reader(flate2::read::GzDecoder::new(b)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const SPEC: SchemaSpec = SchemaSpec {
        name: "journal",
        version: 1,
        elide: &[
            Elision { event: Some("Location"), path: "Latitude" },
            Elision { event: Some("Docked"), path: "Nested.Secret" },
        ],
        augment: &[
            Augment { event: Some("Scan"), field: SystemField::StarPos },
            Augment { event: Some("Scan"), field: SystemField::StarSystem },
        ],
        flags: true,
    };

    fn sw() -> Software {
        Software { name: "astromech".into(), version: "0.1.5".into() }
    }

    fn live() -> GameContext {
        let mut c = GameContext::default();
        c.observe(&json!({"event":"Fileheader","gameversion":"4.0.0.1904","build":"r308767/r0 "}));
        c.observe(&json!({"event":"LoadGame","Commander":"Jameson","FID":"F42","Horizons":true,"Odyssey":true}));
        c.observe(&json!({"event":"Location","StarSystem":"Shinrarta Dezhra","SystemAddress":3932277478106u64,"StarPos":[55.71875,17.59375,27.15625]}));
        c
    }

    #[test]
    fn localised_keys_are_stripped_at_every_depth() {
        let ev = json!({
            "timestamp":"2026-10-16T12:00:00Z","event":"Location",
            "StarSystem":"Shinrarta Dezhra","SystemAddress":3932277478106u64,
            "SystemEconomy":"$economy_HighTech;","SystemEconomy_Localised":"High Tech",
            "Factions":[{"Name":"Dark Wheel","FactionState_Localised":"Boom","ActiveStates":[{"State":"Boom","State_Localised":"Boom"}]}]
        });
        let up = build(&SPEC, &live(), &sw(), false, ev).unwrap();
        let text = up.to_string();
        assert!(!text.contains("_Localised"), "{text}");
        assert_eq!(up["message"]["SystemEconomy"], "$economy_HighTech;");
        assert_eq!(up["message"]["Factions"][0]["ActiveStates"][0]["State"], "Boom");
    }

    #[test]
    fn commander_data_is_stripped() {
        let ev = json!({
            "timestamp":"2026-10-16T12:00:00Z","event":"Location",
            "StarSystem":"Shinrarta Dezhra","SystemAddress":3932277478106u64,
            "FID":"F42","Commander":"Jameson","Wanted":true,"Latitude":1.5,"FuelLevel":3.2,
            "Factions":[{"Name":"Dark Wheel","MyReputation":100.0,"HomeSystem":true,"SquadronFaction":true,"HappiestSystem":true,"Influence":0.5}]
        });
        let up = build(&SPEC, &live(), &sw(), false, ev).unwrap();
        let m = up["message"].as_object().unwrap();
        for k in ["FID", "Commander", "Wanted", "FuelLevel", "Latitude"] {
            assert!(!m.contains_key(k), "{k} left in {m:?}");
        }
        let f = m["Factions"][0].as_object().unwrap();
        assert_eq!(f.keys().collect::<Vec<_>>(), ["Influence", "Name"]);
        assert!(!up.to_string().contains("F42"));
        assert_eq!(up["header"]["uploaderID"], "Jameson", "the commander name goes only in uploaderID");
    }

    #[test]
    fn per_event_elisions_apply_only_to_their_event() {
        let docked =
            json!({"event":"Docked","StarSystem":"Shinrarta Dezhra","Latitude":1.0,"Nested":{"Secret":1,"Keep":2}});
        let m = build(&SPEC, &live(), &sw(), false, docked).unwrap()["message"].clone();
        assert_eq!(m["Latitude"], 1.0, "Latitude is only elided for Location in this spec");
        assert_eq!(m["Nested"], json!({"Keep":2}));
    }

    #[test]
    fn header_comes_from_fileheader_and_is_sent_even_when_empty() {
        let up = build(&SPEC, &live(), &sw(), false, json!({"event":"Docked","StarSystem":"x"})).unwrap();
        assert_eq!(
            up["header"],
            json!({"uploaderID":"Jameson","softwareName":"astromech","softwareVersion":"0.1.5","gameversion":"4.0.0.1904","gamebuild":"r308767/r0 "})
        );
        let mut c = live();
        c.observe(&json!({"event":"Fileheader"}));
        c.observe(&json!({"event":"Commander","Name":"Jameson"}));
        let up = build(&SPEC, &c, &sw(), false, json!({"event":"Docked","StarSystem":"x"})).unwrap();
        assert_eq!((&up["header"]["gameversion"], &up["header"]["gamebuild"]), (&json!(""), &json!("")));
    }

    #[test]
    fn beta_game_versions_are_not_built() {
        let mut c = live();
        c.observe(&json!({"event":"Fileheader","gameversion":"4.1.0.100 BETA","build":"r1/r0"}));
        let r = build(&SPEC, &c, &sw(), false, json!({"event":"Docked","StarSystem":"x"}));
        assert_eq!(r, Err(Dropped::Beta("4.1.0.100 BETA".into())));
        // Not even to /test from here: the gate is before the endpoint choice.
        assert!(build(&SPEC, &c, &sw(), true, json!({"event":"Docked"})).is_err());
    }

    #[test]
    fn unknown_game_version_is_not_built() {
        let mut c = GameContext::default();
        c.observe(&json!({"event":"Commander","Name":"Jameson"}));
        assert_eq!(build(&SPEC, &c, &sw(), false, json!({"event":"Docked"})), Err(Dropped::UnknownGameVersion));
    }

    #[test]
    fn flags_are_never_null() {
        let mut c = live();
        c.observe(&json!({"event":"LoadGame","Commander":"Jameson","Horizons":true}));
        let up = build(&SPEC, &c, &sw(), false, json!({"event":"Docked","odyssey":null,"horizons":null})).unwrap();
        assert_eq!(up["message"]["horizons"], true);
        assert!(up["message"].get("odyssey").is_none(), "unknown odyssey is omitted, not false or null");
        let no_flags = SchemaSpec { flags: false, ..SPEC };
        let up = build(&no_flags, &live(), &sw(), false, json!({"event":"Docked"})).unwrap();
        assert!(up["message"].get("horizons").is_none());
    }

    #[test]
    fn test_mode_uses_the_test_schema() {
        let up = build(&SPEC, &live(), &sw(), true, json!({"event":"Docked"})).unwrap();
        assert_eq!(up["$schemaRef"], "https://eddn.edcd.io/schemas/journal/1/test");
        let up = build(&SPEC, &live(), &sw(), false, json!({"event":"Docked"})).unwrap();
        assert_eq!(up["$schemaRef"], "https://eddn.edcd.io/schemas/journal/1");
    }

    #[test]
    fn augmentation_adds_fields_after_a_matching_cross_check() {
        let scan = json!({"event":"Scan","BodyName":"A 1","SystemAddress":3932277478106u64});
        let m = build(&SPEC, &live(), &sw(), false, scan).unwrap()["message"].clone();
        assert_eq!(m["StarSystem"], "Shinrarta Dezhra");
        assert_eq!(m["StarPos"], json!([55.71875, 17.59375, 27.15625]));
    }

    #[test]
    fn augmentation_mismatch_drops_the_message() {
        let scan = json!({"event":"Scan","SystemAddress":1u64});
        assert!(matches!(build(&SPEC, &live(), &sw(), false, scan), Err(Dropped::LocationMismatch(_))));
        let scan = json!({"event":"Scan","StarSystem":"Lave"});
        assert!(matches!(build(&SPEC, &live(), &sw(), false, scan), Err(Dropped::LocationMismatch(_))));
        // Name matches, address doesn't: still a mismatch.
        let scan = json!({"event":"Scan","StarSystem":"Shinrarta Dezhra","SystemAddress":2u64});
        assert!(matches!(build(&SPEC, &live(), &sw(), false, scan), Err(Dropped::LocationMismatch(_))));
    }

    #[test]
    fn every_system_name_key_is_cross_checked() {
        let fix = live().system;
        for k in ["StarSystem", "SystemName", "System"] {
            let mut m = Map::new();
            m.insert(k.into(), json!("Lave"));
            assert!(matches!(cross_check(&m, fix.as_ref()), Err(Dropped::LocationMismatch(_))), "{k}");
            m.insert(k.into(), json!("Shinrarta Dezhra"));
            assert!(cross_check(&m, fix.as_ref()).is_ok(), "{k} alone is enough to check against");
        }
    }

    #[test]
    fn augmentation_without_anything_to_check_drops_the_message() {
        assert_eq!(build(&SPEC, &live(), &sw(), false, json!({"event":"Scan"})), Err(Dropped::NoLocation));
        let mut c = live();
        c.system = None;
        let scan = json!({"event":"Scan","SystemAddress":3932277478106u64});
        assert_eq!(build(&SPEC, &c, &sw(), false, scan), Err(Dropped::NoLocation));
    }

    #[test]
    fn augmentation_is_skipped_when_fields_are_present_or_not_asked_for() {
        // A Docked event names its own system; this spec augments only Scan, so no fix is needed.
        let mut c = live();
        c.system = None;
        let m = build(&SPEC, &c, &sw(), false, json!({"event":"Docked","StarSystem":"Lave"})).unwrap();
        assert!(m["message"].get("StarPos").is_none());
    }

    #[test]
    fn encode_gzips_and_refuses_bodies_over_one_mib() {
        let up = build(&SPEC, &live(), &sw(), false, json!({"event":"Docked"})).unwrap();
        assert_eq!(gunzip(&encode(&up).unwrap()), up);
        // Incompressible text: hex of a simple PRNG, ~2 MiB raw, well over 1 MiB gzipped.
        let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
        let noise: String = (0..2 * 1024 * 1024 / 16)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                format!("{x:016x}")
            })
            .collect();
        let big = build(&SPEC, &live(), &sw(), false, json!({"event":"Docked","Blob":noise})).unwrap();
        assert!(matches!(encode(&big), Err(Dropped::TooLarge(n)) if n > MAX_BODY));
    }
}
