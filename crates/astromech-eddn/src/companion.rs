//! The station schemas built from companion files: `commodity/3` from `Market.json`,
//! `outfitting/2` from `Outfitting.json` and `shipyard/2` from `Shipyard.json`.
//!
//! Written from EDCD/EDDN `schemas/<schema>-README.md` and the schema files on the `live` branch
//! (commit `4ad669bb`, read 16 Oct 2026). The journal event (`Market`, `Outfitting`,
//! `Shipyard`) says the file was written; the file is the data, and is treated as the event.
//!
//! Each message is built from an explicit list of the schema's keys rather than by renaming the
//! file and eliding what's left: every schema here has `additionalProperties: false`, so this is
//! the README's elision list (`StationType`, `Producer`, `Rare`, `id`, `Category`, prices,
//! `Horizons` from the file, ...) applied by construction. `*_Localised`, commander keys and the
//! `horizons`/`odyssey` flags are then handled by [`crate::message::build`] as for every schema.
//!
//! The caller passes the file as JSON (e.g. `astromech_journal` snapshot's `raw_json`), so this
//! crate needs nothing from the journal crate.

use crate::message::SchemaSpec;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::HashSet;
use std::fmt;
use std::sync::OnceLock;

pub const COMMODITY: SchemaSpec = SchemaSpec { name: "commodity", version: 3, elide: &[], augment: &[], flags: true };
pub const OUTFITTING: SchemaSpec = SchemaSpec { name: "outfitting", version: 2, elide: &[], augment: &[], flags: true };
pub const SHIPYARD: SchemaSpec = SchemaSpec { name: "shipyard", version: 2, elide: &[], augment: &[], flags: true };

/// Why a companion file gave no message. "No data is better than bad data": a file that can't
/// be shown to belong to its event, or that is malformed anywhere, is not sent at all.
#[derive(Debug, Clone, PartialEq)]
pub enum Unusable {
    /// Not a `Market`, `Outfitting` or `Shipyard` event.
    NotStationEvent(String),
    /// The file is not the one this event announced (other event, other station, or older).
    Mismatch(String),
    /// A required top-level key is missing or of the wrong type.
    Missing(&'static str),
    /// An item is malformed; a partial list would read as the station's full list.
    BadItem(String),
    /// A module astromech can't place as the station's or the commander's (see [`modules`]):
    /// sending the rest would read as the station's full list.
    UnknownModule(String),
    /// Nothing left to send after the README's filters (the schema needs at least one).
    Empty,
}

impl fmt::Display for Unusable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unusable::NotStationEvent(e) => write!(f, "{e:?} is not a station companion event"),
            Unusable::Mismatch(m) => write!(f, "companion file doesn't match its event: {m}"),
            Unusable::Missing(k) => write!(f, "companion file has no usable {k}"),
            Unusable::BadItem(m) => write!(f, "malformed item: {m}"),
            Unusable::UnknownModule(m) => {
                write!(f, "module {m} isn't in astromech's module list, so it may be commander-specific")
            }
            Unusable::Empty => write!(f, "nothing to send after filtering"),
        }
    }
}

/// The schema-shaped message for a `Market`, `Outfitting` or `Shipyard` journal `event` and the
/// companion `file` read for it. Pass the result to [`crate::Sender::submit`] with the spec.
///
/// Trigger rule: the file must name the same event and `MarketID` as the journal event, and
/// its `timestamp` must not be older than the event's (an older file is the last visit's).
pub fn message(event: &Value, file: &Value) -> Result<(&'static SchemaSpec, Value), Unusable> {
    let name = event.get("event").and_then(Value::as_str).unwrap_or_default();
    let (spec, list): (_, fn(&Value) -> Result<(&'static str, Value), Unusable>) = match name {
        "Market" => (&COMMODITY, commodities),
        "Outfitting" => (&OUTFITTING, modules),
        "Shipyard" => (&SHIPYARD, ships),
        _ => return Err(Unusable::NotStationEvent(name.to_owned())),
    };
    check_trigger(name, event, file)?;
    let mut msg = station_keys(file)?;
    let (key, items) = list(file)?;
    msg.insert(key.into(), items);
    match name {
        "Market" => {
            // StationType is on the README's elision list. The journal Market.json carries no
            // `economies` or `prohibited`, and the README forbids sending them empty.
            if let Some(a) = file.get("CarrierDockingAccess").and_then(Value::as_str) {
                msg.insert("carrierDockingAccess".into(), a.into());
            }
        }
        "Shipyard" => {
            if let Some(b) = file.get("AllowCobraMkIV").and_then(Value::as_bool) {
                msg.insert("allowCobraMkIV".into(), b.into());
            }
        }
        _ => {}
    }
    Ok((spec, Value::Object(msg)))
}

/// The one trigger rule for every schema built from a companion file (commodity, outfitting,
/// shipyard, navroute, fcmaterials_journal): the file names the same event, the same `MarketID`
/// (or none, for `NavRoute`), and its `timestamp` is not older than the event's (an older file
/// is the last visit's). Callers use it only for events tailed live, never for a replay.
///
/// ⚠ Whether the game always stamps the file with exactly its event's `timestamp` is not yet
/// checked against real logs (astromech-uxyq). If it is, this should require equality, which
/// would also refuse a newer file written for a later event.
pub fn check_trigger(name: &str, event: &Value, file: &Value) -> Result<(), Unusable> {
    let file_event = file.get("event").and_then(Value::as_str);
    if file_event != Some(name) || event.get("event").and_then(Value::as_str) != Some(name) {
        return Err(Unusable::Mismatch(format!("file event {file_event:?}, journal event {name:?}")));
    }
    let (fm, em) = (file.get("MarketID").and_then(Value::as_u64), event.get("MarketID").and_then(Value::as_u64));
    if fm != em {
        return Err(Unusable::Mismatch(format!("file MarketID {fm:?}, event MarketID {em:?}")));
    }
    let (ft, et) = (game_timestamp(file), game_timestamp(event));
    match (ft, et) {
        // Same fixed-width UTC format, so text order is time order.
        (Some(f), Some(e)) if f >= e => Ok(()),
        _ => Err(Unusable::Mismatch(format!("file timestamp {ft:?}, event timestamp {et:?}"))),
    }
}

/// The game's `timestamp`, if it has the game's `YYYY-MM-DDTHH:MM:SSZ` shape.
pub(crate) fn game_timestamp(v: &Value) -> Option<&str> {
    let t = v.get("timestamp")?.as_str()?;
    let b = t.as_bytes();
    let shaped = b.len() == 20
        && b.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == b'-',
            10 => *c == b'T',
            13 | 16 => *c == b':',
            19 => *c == b'Z',
            _ => c.is_ascii_digit(),
        });
    shaped.then_some(t)
}

/// `systemName`, `stationName`, `marketId` and `timestamp`, renamed from the file's keys.
fn station_keys(file: &Value) -> Result<Map<String, Value>, Unusable> {
    let text = |k: &'static str| match file.get(k).and_then(Value::as_str) {
        Some(s) if !s.is_empty() => Ok(s.to_owned()),
        _ => Err(Unusable::Missing(k)),
    };
    let market_id = file.get("MarketID").and_then(Value::as_u64).ok_or(Unusable::Missing("MarketID"))?;
    let mut m = Map::new();
    m.insert("systemName".into(), text("StarSystem")?.into());
    m.insert("stationName".into(), text("StationName")?.into());
    m.insert("marketId".into(), market_id.into());
    m.insert("timestamp".into(), text("timestamp")?.into());
    Ok(m)
}

fn items<'a>(file: &'a Value, key: &'static str) -> Result<&'a [Value], Unusable> {
    file.get(key).and_then(Value::as_array).map(Vec::as_slice).ok_or(Unusable::Missing(key))
}

/// commodity/3 `commodities` from `Items`: limpets and other `NonMarketable` goods skipped,
/// `$…_name;` stripped from `Name`, and only the schema's eight keys kept.
fn commodities(file: &Value) -> Result<(&'static str, Value), Unusable> {
    let mut out = Vec::new();
    for item in items(file, "Items")? {
        let category = item.get("Category").and_then(Value::as_str).unwrap_or_default();
        if category.eq_ignore_ascii_case("$MARKET_category_nonmarketable;") {
            continue;
        }
        let raw = item.get("Name").and_then(Value::as_str).ok_or_else(|| bad(item, "no Name"))?;
        let name = strip_symbol(raw);
        if name.is_empty() {
            return Err(bad(item, "empty Name"));
        }
        let int =
            |k: &str| item.get(k).and_then(Value::as_i64).ok_or_else(|| bad(item, &format!("{k} not an integer")));
        let bracket = |k: &str| match item.get(k) {
            Some(Value::Number(n)) if n.as_u64().is_some_and(|b| b <= 3) => Ok(Value::Number(n.clone())),
            Some(Value::String(s)) if s.is_empty() => Ok(json!("")),
            _ => Err(bad(item, &format!("{k} not 0-3 or \"\""))),
        };
        out.push(json!({
            "name": name,
            "meanPrice": int("MeanPrice")?,
            "buyPrice": int("BuyPrice")?,
            "stock": int("Stock")?,
            "stockBracket": bracket("StockBracket")?,
            "sellPrice": int("SellPrice")?,
            "demand": int("Demand")?,
            "demandBracket": bracket("DemandBracket")?,
        }));
    }
    Ok(("commodities", Value::Array(out)))
}

/// `$gold_name;` -> `gold`. The README asks only for the prefix and suffix to go.
fn strip_symbol(name: &str) -> &str {
    let name = name.strip_prefix('$').unwrap_or(name);
    let cut = name.len().saturating_sub("_name;".len());
    match name.get(cut..) {
        Some(tail) if tail.eq_ignore_ascii_case("_name;") => &name[..cut],
        _ => name,
    }
}

/// Which module symbols outfitting/2 may carry: `data/catalog/eddn-outfitting.json`, generated
/// by `scripts/gen-eddn-outfitting.py` from EDCD/FDevIDs `outfitting.csv` (its `entitlement`
/// column is the Frontier API's `sku`). Lowercased, as the journal writes them.
struct ModuleList {
    station: HashSet<String>,
    commander: HashSet<String>,
}

fn module_list() -> &'static ModuleList {
    #[derive(Deserialize)]
    struct File {
        station: Vec<String>,
        commander: Vec<String>,
    }
    static L: OnceLock<ModuleList> = OnceLock::new();
    L.get_or_init(|| {
        let f: File = serde_json::from_str(include_str!("../../../data/catalog/eddn-outfitting.json"))
            .expect("eddn-outfitting.json");
        ModuleList { station: f.station.into_iter().collect(), commander: f.commander.into_iter().collect() }
    })
}

/// outfitting/2 `modules` from `Items[].Name`: only weapons and utilities (`Hpt_`), standard and
/// internal modules (`Int_`) and armour (`_Armour_`), in the cases the schema's pattern allows;
/// not `Int_PlanetApproachSuite`; no commander-specific item. Duplicates are dropped (the schema
/// wants unique names).
///
/// The README drops "items that have a non-null `sku` property, unless it's
/// `ELITE_HORIZONS_V_PLANETARY_LANDINGS` (i.e. PowerPlay and tech broker items)": they're listed
/// because of the commander's Powerplay rank or tech-broker unlocks, not because the station
/// sells them. The journal's `Outfitting.json` has no `sku`, so each module's sku comes from
/// [`module_list`] instead (an item that does carry one, as the Frontier API's do, is judged by
/// it). A module in neither of its lists can't be shown to be the station's, so the visit sends
/// nothing ([`Unusable::UnknownModule`]): a list without it would also be wrong.
///
/// EDMC (EDCD/EDMarketConnector `plugins/eddn.py`) applies this sku rule only to Frontier API
/// data (`export_outfitting`); its journal path (`export_journal_outfitting`) drops only
/// `int_planetapproachsuite` and sends commander-specific modules as station stock.
fn modules(file: &Value) -> Result<(&'static str, Value), Unusable> {
    let list = module_list();
    let mut out: Vec<&str> = Vec::new();
    for item in items(file, "Items")? {
        let name = item.get("Name").and_then(Value::as_str).ok_or_else(|| bad(item, "no Name"))?;
        let station_item = ["Hpt_", "hpt_", "Int_", "int_"].iter().any(|p| name.starts_with(p))
            || ["_Armour_", "_armour_"].iter().any(|p| name.contains(p));
        if !station_item || name.eq_ignore_ascii_case("Int_PlanetApproachSuite") || out.contains(&name) {
            continue;
        }
        let symbol = name.to_ascii_lowercase();
        let cmdr_sku = match item.get("sku") {
            Some(s) if !s.is_null() => s.as_str() != Some("ELITE_HORIZONS_V_PLANETARY_LANDINGS"),
            _ if list.commander.contains(&symbol) => true,
            _ if list.station.contains(&symbol) => false,
            _ => return Err(Unusable::UnknownModule(name.to_owned())),
        };
        if !cmdr_sku {
            out.push(name);
        }
    }
    if out.is_empty() {
        return Err(Unusable::Empty);
    }
    Ok(("modules", json!(out)))
}

/// shipyard/2 `ships` from `PriceList[].ShipType`, duplicates dropped.
fn ships(file: &Value) -> Result<(&'static str, Value), Unusable> {
    let mut out: Vec<&str> = Vec::new();
    for item in items(file, "PriceList")? {
        match item.get("ShipType").and_then(Value::as_str) {
            Some(s) if !s.is_empty() => {
                if !out.contains(&s) {
                    out.push(s);
                }
            }
            _ => return Err(bad(item, "no ShipType")),
        }
    }
    if out.is_empty() {
        return Err(Unusable::Empty);
    }
    Ok(("ships", json!(out)))
}

fn bad(item: &Value, why: &str) -> Unusable {
    let id = item.get("id").map(Value::to_string).unwrap_or_else(|| "?".into());
    Unusable::BadItem(format!("item id {id}: {why}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::build;
    use crate::testutil::{assert_valid, fixture, live_context, schema_errors, software};

    fn journal_event(file: &Value) -> Value {
        let mut ev = json!({"event": file["event"], "timestamp": file["timestamp"], "MarketID": file["MarketID"]});
        for k in ["StationName", "StationType", "StarSystem"] {
            if let Some(v) = file.get(k) {
                ev[k] = v.clone();
            }
        }
        ev
    }

    /// The full upload for a fixture file, as the sender would queue it.
    fn upload(file: &Value) -> Value {
        let (spec, msg) = message(&journal_event(file), file).unwrap();
        build(spec, &live_context(), &software(), false, msg).unwrap()
    }

    #[test]
    fn market_fixture_validates_against_commodity_v3() {
        let up = upload(&fixture("Market.json"));
        assert_eq!(up["$schemaRef"], "https://eddn.edcd.io/schemas/commodity/3");
        assert_valid("commodity-v3.0.json", &up);
    }

    #[test]
    fn outfitting_fixture_validates_against_outfitting_v2() {
        let up = upload(&fixture("Outfitting.json"));
        assert_eq!(up["$schemaRef"], "https://eddn.edcd.io/schemas/outfitting/2");
        assert_valid("outfitting-v2.0.json", &up);
    }

    #[test]
    fn shipyard_fixture_validates_against_shipyard_v2() {
        let up = upload(&fixture("Shipyard.json"));
        assert_eq!(up["$schemaRef"], "https://eddn.edcd.io/schemas/shipyard/2");
        assert_valid("shipyard-v2.0.json", &up);
    }

    #[test]
    fn commodity_follows_the_readme() {
        let m = upload(&fixture("Market.json"))["message"].clone();
        assert_eq!(
            (&m["systemName"], &m["stationName"], &m["marketId"], &m["timestamp"]),
            (&json!("Shinrarta Dezhra"), &json!("Jameson Memorial"), &json!(128666762), &json!("2026-10-16T12:00:00Z"))
        );
        let names: Vec<&str> =
            m["commodities"].as_array().unwrap().iter().map(|c| c["name"].as_str().unwrap()).collect();
        assert_eq!(names, ["platinum", "hydrogenfuel", "imperialslaves", "lavianbrandy"], "limpets are NonMarketable");
        assert_eq!(
            m["commodities"][0],
            json!({"name":"platinum","meanPrice":58105,"buyPrice":0,"stock":0,"stockBracket":0,
                   "sellPrice":58263,"demand":1650,"demandBracket":2}),
            "id, Category, Consumer, Producer, Rare and *_Localised are gone"
        );
        assert_eq!(m["commodities"][2]["demandBracket"], "", "an empty bracket is passed through");
        for k in ["stationType", "StationType", "economies", "prohibited"] {
            assert!(m.get(k).is_none(), "{k} must not be sent from Market.json");
        }
        assert_eq!((&m["horizons"], &m["odyssey"]), (&json!(true), &json!(true)), "flags come from LoadGame");
    }

    #[test]
    fn carrier_docking_access_is_renamed() {
        let mut f = fixture("Market.json");
        f["CarrierDockingAccess"] = json!("squadronfriends");
        let up = upload(&f);
        assert_eq!(up["message"]["carrierDockingAccess"], "squadronfriends");
        assert_valid("commodity-v3.0.json", &up);
    }

    #[test]
    fn symbol_stripping() {
        assert_eq!(strip_symbol("$gold_name;"), "gold");
        assert_eq!(strip_symbol("$Gold_Name;"), "Gold");
        assert_eq!(strip_symbol("gold"), "gold");
        assert_eq!(strip_symbol("$_name;"), "");
    }

    #[test]
    fn outfitting_keeps_only_station_modules() {
        let m = upload(&fixture("Outfitting.json"))["message"].clone();
        assert_eq!(
            m["modules"],
            json!(["hpt_chafflauncher_tiny", "int_engine_size3_class5_fast", "sidewinder_armour_grade1"]),
            "no planet approach suite, paintjob, bobblehead, decal, duplicate, PowerPlay or tech-broker module"
        );
        assert!(m.get("Horizons").is_none() && m.get("Items").is_none());
        assert_eq!(m["horizons"], true);
    }

    #[test]
    fn outfitting_skips_commander_sku_items_but_not_horizons_ones() {
        let mut f = fixture("Outfitting.json");
        f["Items"] = json!([
            {"id":1,"Name":"hpt_atdumbfiremissile_fixed_medium","sku":"ELITE_SPECIFIC_V_POWER_100500"},
            {"id":2,"Name":"int_buggybay_size2_class1","sku":"ELITE_HORIZONS_V_PLANETARY_LANDINGS"},
            {"id":3,"Name":"Int_PlanetApproachSuite","sku":null},
            {"id":4,"Name":"Hpt_ChaffLauncher_Tiny","sku":null}
        ]);
        let m = upload(&f)["message"].clone();
        assert_eq!(m["modules"], json!(["int_buggybay_size2_class1", "Hpt_ChaffLauncher_Tiny"]));
    }

    /// The README's commander-specific items: (the wiki's name, a journal symbol for it), from
    /// elite-dangerous.fandom.com's Powerplay modules table and Technology Broker "Unlocks"
    /// tables (read 22 Oct 2026).
    const POWERPLAY: &[(&str, &str)] = &[
        ("Advanced Plasma Accelerator", "hpt_plasmaaccelerator_fixed_large_advanced"),
        ("Concord Cannon", "hpt_cannon_gimbal_medium_burst"),
        ("Containment Missile", "hpt_dumbfiremissilerack_fixed_medium_lasso"),
        ("Cytoscrambler", "hpt_pulselaserburst_fixed_small_scatter"),
        ("Enforcer Cannon", "hpt_multicannon_fixed_small_strong"),
        ("Imperial Hammer", "hpt_railgun_fixed_medium_burst"),
        ("Mining Lance", "hpt_mininglaser_fixed_small_advanced"),
        ("Pacifier Frag-Cannon", "hpt_slugshot_fixed_large_range"),
        ("Pack-Hound Missile Rack", "hpt_drunkmissilerack_fixed_medium"),
        ("Prismatic Shield Generator (sizes 1-8)", "int_shieldgenerator_size4_class5_strong"),
        ("Pulse Disruptor", "hpt_pulselaser_fixed_medium_disruptor"),
        ("Retributor", "hpt_beamlaser_fixed_small_heat"),
    ];
    const TECH_BROKER: &[(&str, &str)] = &[
        ("Corrosion Resistant Cargo Rack (class 4)", "int_corrosionproofcargorack_size4_class1"),
        ("Enzyme Missile Rack", "hpt_causticmissile_fixed_medium"),
        ("Meta Alloy Hull Reinforcement", "int_metaalloyhullreinforcement_size3_class2"),
        ("Remote Release Flechette Launcher", "hpt_flechettelauncher_fixed_medium"),
        ("Remote Release Flechette Launcher", "hpt_flechettelauncher_turret_medium"),
        ("Shock Cannon (9 mounts and classes)", "hpt_plasmashockcannon_gimbal_small"),
        ("Guardian FSD Booster", "int_guardianfsdbooster_size5"),
        ("Guardian Gauss Cannon", "hpt_guardian_gausscannon_fixed_small"),
        ("Guardian Hull Reinforcement Package", "int_guardianhullreinforcement_size2_class1"),
        ("Guardian Hybrid Power Distributor", "int_guardianpowerdistributor_size6"),
        ("Guardian Hybrid Power Plant", "int_guardianpowerplant_size4"),
        ("Guardian Module Reinforcement Package", "int_guardianmodulereinforcement_size5_class2"),
        ("Guardian Plasma Charger", "hpt_guardian_plasmalauncher_turret_large"),
        ("Guardian Shard Cannon", "hpt_guardian_shardcannon_fixed_medium"),
        ("Guardian Shield Reinforcement Package", "int_guardianshieldreinforcement_size1_class1"),
        ("Caustic Sink Launcher (rescue megaship)", "hpt_causticsinklauncher_turret_tiny"),
    ];

    fn with_items(names: &[&str]) -> Value {
        let mut f = fixture("Outfitting.json");
        f["Items"] = names.iter().enumerate().map(|(i, n)| json!({"id": i, "Name": n, "BuyPrice": 1})).collect();
        f
    }

    #[test]
    fn outfitting_drops_every_powerplay_and_tech_broker_module() {
        let mut names = vec!["hpt_chafflauncher_tiny"];
        names.extend(POWERPLAY.iter().chain(TECH_BROKER).map(|(_, symbol)| *symbol));
        // The journal writes lowercase; the schema allows the capitalised form too.
        names.extend(["Hpt_DrunkMissileRack_Fixed_Medium", "Int_GuardianFSDBooster_Size1"]);
        let up = upload(&with_items(&names));
        assert_eq!(up["message"]["modules"], json!(["hpt_chafflauncher_tiny"]));
        assert_valid("outfitting-v2.0.json", &up);
    }

    #[test]
    fn outfitting_with_only_commander_modules_is_not_sent() {
        let names: Vec<&str> = POWERPLAY.iter().chain(TECH_BROKER).map(|(_, symbol)| *symbol).collect();
        let f = with_items(&names);
        assert_eq!(message(&journal_event(&f), &f).err(), Some(Unusable::Empty));
    }

    #[test]
    fn outfitting_keeps_horizons_modules() {
        // `ELITE_HORIZONS_V_PLANETARY_LANDINGS` items are the station's (the README's exception).
        let names = ["int_buggybay_size2_class1", "hpt_atmulticannon_fixed_large", "int_planetapproachsuite_advanced"];
        assert_eq!(upload(&with_items(&names))["message"]["modules"], json!(names));
    }

    #[test]
    fn outfitting_with_an_unknown_module_is_not_sent() {
        let f = with_items(&["hpt_chafflauncher_tiny", "hpt_newweapon_fixed_small"]);
        assert_eq!(
            message(&journal_event(&f), &f).err(),
            Some(Unusable::UnknownModule("hpt_newweapon_fixed_small".into()))
        );
        // Cosmetics aren't looked up: the schema's pattern drops them first.
        let g = with_items(&["hpt_chafflauncher_tiny", "paintjob_new_01"]);
        assert!(message(&journal_event(&g), &g).is_ok());
    }

    #[test]
    fn outfitting_with_nothing_left_is_not_sent() {
        let mut f = fixture("Outfitting.json");
        f["Items"] =
            json!([{"id":1,"Name":"paintjob_sidewinder_default_01"},{"id":2,"Name":"int_planetapproachsuite"}]);
        assert_eq!(message(&journal_event(&f), &f).err(), Some(Unusable::Empty));
    }

    #[test]
    fn shipyard_lists_ship_types_and_cobra_flag() {
        let m = upload(&fixture("Shipyard.json"))["message"].clone();
        assert_eq!(m["ships"], json!(["sidewinder", "adder", "empire_courier", "anaconda"]));
        assert_eq!(m["allowCobraMkIV"], false);
        assert!(m.get("PriceList").is_none() && m.get("AllowCobraMkIV").is_none());
        let mut f = fixture("Shipyard.json");
        f.as_object_mut().unwrap().remove("AllowCobraMkIV");
        assert!(upload(&f)["message"].get("allowCobraMkIV").is_none(), "absent, not guessed");
    }

    #[test]
    fn file_from_another_visit_or_event_is_refused() {
        let f = fixture("Market.json");
        let ev = journal_event(&f);
        let mut later = ev.clone();
        later["timestamp"] = json!("2026-10-16T12:00:01Z");
        assert!(matches!(message(&later, &f), Err(Unusable::Mismatch(_))), "file older than its event");
        let mut other = ev.clone();
        other["MarketID"] = json!(3228342528u64);
        assert!(matches!(message(&other, &f), Err(Unusable::Mismatch(_))), "another station's file");
        let mut wrong = ev.clone();
        wrong["event"] = json!("Outfitting");
        assert!(matches!(message(&wrong, &f), Err(Unusable::Mismatch(_))), "another companion file");
        let mut newer = ev;
        newer["timestamp"] = json!("2026-10-16T11:59:59Z");
        assert!(message(&newer, &f).is_ok(), "a file rewritten after its event is still this visit's");
        let mut odd = f.clone();
        odd["timestamp"] = json!("2026-10-16 12:00:00");
        assert!(matches!(message(&journal_event(&odd), &odd), Err(Unusable::Mismatch(_))));
    }

    #[test]
    fn other_events_are_not_station_events() {
        let f = fixture("Market.json");
        assert_eq!(message(&json!({"event":"Docked"}), &f).err(), Some(Unusable::NotStationEvent("Docked".into())));
    }

    #[test]
    fn malformed_files_send_nothing() {
        let f = fixture("Market.json");
        let mut g = f.clone();
        g["Items"][0]["SellPrice"] = json!("58263");
        assert!(matches!(message(&journal_event(&g), &g), Err(Unusable::BadItem(_))));
        let mut g = f.clone();
        g["Items"][1]["StockBracket"] = json!(4);
        assert!(matches!(message(&journal_event(&g), &g), Err(Unusable::BadItem(_))));
        let mut g = f.clone();
        g["StationName"] = json!("");
        assert_eq!(message(&journal_event(&g), &g).err(), Some(Unusable::Missing("StationName")));
        let mut g = f;
        g.as_object_mut().unwrap().remove("Items");
        assert_eq!(message(&journal_event(&g), &g).err(), Some(Unusable::Missing("Items")));
    }

    #[test]
    fn schema_rejects_what_the_readme_elides() {
        // The validator is not vacuous: the raw file's keys fail each schema.
        let mut up = upload(&fixture("Market.json"));
        up["message"]["commodities"][0]["Producer"] = json!(false);
        up["message"]["StationType"] = json!("Orbis");
        assert!(schema_errors("commodity-v3.0.json", &up).len() >= 2);
        let mut up = upload(&fixture("Outfitting.json"));
        up["message"]["modules"] = json!(["paintjob_sidewinder_default_01"]);
        assert!(!schema_errors("outfitting-v2.0.json", &up).is_empty());
        let mut up = upload(&fixture("Shipyard.json"));
        up["message"]["ships"] = json!([]);
        assert!(!schema_errors("shipyard-v2.0.json", &up).is_empty());
    }
}
