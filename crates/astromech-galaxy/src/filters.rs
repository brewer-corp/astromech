//! Search filters for the "search near me" kit, remembered per lookup in `astromech-store`
//! settings (`search.filters.<lookup>`). The landing pad follows the current ship until the
//! commander picks one.

use serde::{Deserialize, Serialize};

/// Landing pad size, smallest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Pad {
    S,
    M,
    L,
}

impl Pad {
    /// The pad a ship needs, by its journal symbol (`Loadout.Ship`); `None` if unknown.
    /// Player-visible game data (Elite Dangerous wiki, ship pages). The 2024-26 hulls
    /// (`explorer_nx`, `smallcombat01_nx`, `lakonminer`, `mediumtransport01`) were checked on
    /// 17 Oct 2026 against Coriolis' ship data (EDCD/coriolis-data, hull `class` 1/2/3 is the
    /// pad size) by symbol id from FDevIDs; the wiki itself was behind a bot wall.
    pub fn for_ship(symbol: &str) -> Option<Pad> {
        let s = symbol.to_ascii_lowercase();
        let pad = match s.as_str() {
            "sidewinder" | "eagle" | "hauler" | "adder" | "empire_eagle" | "viper" | "viper_mkiv" | "cobramkiii"
            | "cobramkiv" | "cobramkv" | "diamondback" | "diamondbackxl" | "dolphin" | "empire_courier" | "vulture"
            | "smallcombat01_nx" => Pad::S,
            "type6"
            | "type8"
            | "independant_trader"
            | "asp"
            | "asp_scout"
            | "federation_dropship"
            | "federation_dropship_mkii"
            | "federation_gunship"
            | "typex"
            | "typex_2"
            | "typex_3"
            | "krait_mkii"
            | "krait_light"
            | "ferdelance"
            | "mamba"
            | "python"
            | "python_nx"
            | "mandalay"
            | "corsair"
            | "lakonminer"
            | "mediumtransport01" => Pad::M,
            "type7"
            | "empire_trader"
            | "orca"
            | "type9"
            | "belugaliner"
            | "type9_military"
            | "anaconda"
            | "federation_corvette"
            | "cutter"
            | "panthermkii"
            | "explorer_nx" => Pad::L,
            _ => return None,
        };
        Some(pad)
    }
}

/// One lookup's filters. A filter a result can't be judged on (a system has no pad) keeps it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Filters {
    /// Hide stations without a pad at least this big. `None`: any pad.
    pub pad: Option<Pad>,
    /// The commander picked `pad`; otherwise it follows the current ship.
    pub pad_set: bool,
    pub exclude_carriers: bool,
    pub exclude_odyssey: bool,
    /// Hide data older than this many days.
    pub max_age_days: Option<u32>,
    /// Hide stations farther than this from the arrival star, in light seconds.
    pub max_arrival_ls: Option<u32>,
    /// Search radius in light years. `None`: the lookup's default.
    pub max_distance_ly: Option<u32>,
    /// Hide markets with less demand (selling) or stock (buying) than this, in tonnes.
    pub min_volume: Option<u32>,
}

/// What a result offers the filters. `None` / `false` where the source doesn't say.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Facts<'a> {
    /// The largest pad at the station.
    pub pad: Option<Pad>,
    pub arrival_ls: Option<f64>,
    pub fleet_carrier: bool,
    pub odyssey: bool,
    /// RFC 3339, as Spansh and Ardent send it.
    pub updated_at: Option<&'a str>,
    /// Light years from the search origin.
    pub ly: Option<f64>,
    /// Market demand or stock for the commodity searched, in tonnes.
    pub volume: Option<i64>,
}

impl Filters {
    /// With the pad taken from the current ship, unless the commander set it.
    pub fn for_ship(mut self, ship_pad: Option<Pad>) -> Self {
        if !self.pad_set {
            self.pad = ship_pad;
        }
        self
    }

    /// With the pad taken from the ship of journal type `ship_type` (the current ship, or none
    /// when unknown), unless the commander set it. Every search resolves this afresh.
    pub fn for_ship_type(self, ship_type: Option<&str>) -> Self {
        self.for_ship(ship_type.and_then(Pad::for_ship))
    }

    /// The oldest `updated_at` the data-age filter allows (RFC 3339, to the hour), to send to
    /// Spansh so its nearest matches are already fresh. `None` without an age filter.
    pub fn updated_since(&self, now_s: i64) -> Option<String> {
        let days = self.max_age_days?;
        Some(crate::iso((now_s - days as i64 * 86_400).div_euclid(3600) * 3600))
    }

    pub fn keeps(&self, f: &Facts, now_s: i64) -> bool {
        let pad = match (self.pad, f.pad) {
            (Some(need), Some(has)) => has >= need,
            _ => true,
        };
        let arrival = match (self.max_arrival_ls, f.arrival_ls) {
            (Some(max), Some(ls)) => ls <= max as f64,
            _ => true,
        };
        let age = match (self.max_age_days, f.updated_at.and_then(parse_time)) {
            (Some(days), Some(t)) => now_s - t <= days as i64 * 86_400,
            _ => true,
        };
        let near = match (self.max_distance_ly, f.ly) {
            (Some(max), Some(ly)) => ly <= max as f64 + 1e-9,
            _ => true,
        };
        let volume = match (self.min_volume, f.volume) {
            (Some(min), Some(v)) => v >= min as i64,
            _ => true,
        };
        pad && arrival
            && age
            && near
            && volume
            && !(self.exclude_carriers && f.fleet_carrier)
            && !(self.exclude_odyssey && f.odyssey)
    }
}

pub(crate) fn parse_time(s: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(s).ok().map(|t| t.timestamp())
}

fn key(lookup: &str) -> String {
    format!("search.filters.{lookup}")
}

/// `lookup`'s saved filters (defaults if none or unreadable), with the pad resolved for the
/// current ship.
pub fn load(store: &astromech_store::Store, lookup: &str, ship_pad: Option<Pad>) -> anyhow::Result<Filters> {
    let saved = store.get(&key(lookup))?.and_then(|v| serde_json::from_str::<Filters>(&v).ok()).unwrap_or_default();
    Ok(saved.for_ship(ship_pad))
}

pub fn save(store: &astromech_store::Store, lookup: &str, filters: &Filters) -> anyhow::Result<()> {
    store.set(&key(lookup), &serde_json::to_string(filters)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_survive_a_restart_for_their_lookup_only() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("astromech.sqlite");
        {
            let st = astromech_store::Store::open(&db).unwrap();
            let f =
                Filters { max_age_days: Some(7), exclude_carriers: true, ..load(&st, "faction.famine", None).unwrap() };
            save(&st, "faction.famine", &f).unwrap();
        }
        // the app closes and opens again
        let st = astromech_store::Store::open(&db).unwrap();
        let famine = load(&st, "faction.famine", None).unwrap();
        assert_eq!(famine.max_age_days, Some(7));
        assert!(famine.exclude_carriers);
        assert_eq!(load(&st, "faction.proto", None).unwrap(), Filters::default(), "other lookups are untouched");
    }

    /// What the shell does for every search: the page sends the filters it loaded (with the pad
    /// of the ship it knew), and a ship change since must still change the pad searched with.
    #[test]
    fn a_search_resolves_the_pad_from_the_current_ship_not_the_filters_sent() {
        let sent = Filters { pad: Some(Pad::M), pad_set: false, ..Filters::default() }; // loaded in a Krait
        assert_eq!(sent.clone().for_ship_type(Some("krait_mkii")).pad, Some(Pad::M));
        assert_eq!(sent.clone().for_ship_type(Some("anaconda")).pad, Some(Pad::L), "now in an Anaconda");
        assert_eq!(sent.clone().for_ship_type(Some("sidewinder")).pad, Some(Pad::S));
        assert_eq!(sent.for_ship_type(None).pad, None, "no known ship: any pad");
        let chosen = Filters { pad: Some(Pad::S), pad_set: true, ..Filters::default() };
        assert_eq!(chosen.for_ship_type(Some("anaconda")).pad, Some(Pad::S), "the commander's pick stays");
    }

    #[test]
    fn pad_follows_the_ship_unless_the_commander_set_it() {
        let st = astromech_store::Store::in_memory().unwrap();
        let krait = Pad::for_ship("krait_mkii");
        let anaconda = Pad::for_ship("Anaconda");
        assert_eq!((krait, anaconda), (Some(Pad::M), Some(Pad::L)));

        // not set: follows whatever ship is current, even after saving other filters
        save(&st, "outfitting", &Filters { max_age_days: Some(30), ..load(&st, "outfitting", krait).unwrap() })
            .unwrap();
        assert_eq!(load(&st, "outfitting", krait).unwrap().pad, Some(Pad::M));
        assert_eq!(load(&st, "outfitting", anaconda).unwrap().pad, Some(Pad::L), "follows a ship change");
        assert_eq!(load(&st, "outfitting", None).unwrap().pad, None, "unknown ship: any pad");

        // set by the commander: kept across ship changes, including "any pad"
        save(&st, "outfitting", &Filters { pad: Some(Pad::S), pad_set: true, ..Filters::default() }).unwrap();
        assert_eq!(load(&st, "outfitting", anaconda).unwrap().pad, Some(Pad::S));
        save(&st, "outfitting", &Filters { pad: None, pad_set: true, ..Filters::default() }).unwrap();
        assert_eq!(load(&st, "outfitting", anaconda).unwrap().pad, None);
        assert_eq!(load(&st, "trade", anaconda).unwrap().pad, Some(Pad::L), "another lookup still follows the ship");
    }

    /// Pads of the hulls added from memory, checked against Coriolis' data (see `for_ship`).
    #[test]
    fn newer_hulls_have_their_checked_pads() {
        for (symbol, pad) in [
            ("MediumTransport01", Pad::M), // Lynx Highliner
            ("Explorer_NX", Pad::L),       // Caspian Explorer
            ("LakonMiner", Pad::M),        // Type-11 Prospector
            ("SmallCombat01_NX", Pad::S),  // Kestrel Mk II
            ("PantherMkII", Pad::L),
        ] {
            assert_eq!(Pad::for_ship(symbol), Some(pad), "{symbol}");
        }
    }

    #[test]
    fn updated_since_rounds_down_to_the_hour() {
        let now = parse_time("2026-10-17T12:34:56Z").unwrap();
        assert_eq!(Filters::default().updated_since(now), None);
        let f = Filters { max_age_days: Some(2), ..Filters::default() };
        assert_eq!(f.updated_since(now).as_deref(), Some("2026-10-15T12:00:00Z"));
    }

    #[test]
    fn keeps_applies_each_filter() {
        let now = parse_time("2026-10-16T12:00:00Z").unwrap();
        let station = Facts {
            pad: Some(Pad::M),
            arrival_ls: Some(1200.0),
            fleet_carrier: false,
            odyssey: false,
            updated_at: Some("2026-10-14T18:08:31Z"),
            ly: Some(42.5),
            volume: Some(300),
        };
        let all = Filters::default();
        assert!(!Filters { max_distance_ly: Some(40), ..all.clone() }.keeps(&station, now));
        assert!(Filters { max_distance_ly: Some(50), ..all.clone() }.keeps(&station, now));
        assert!(!Filters { min_volume: Some(301), ..all.clone() }.keeps(&station, now));
        assert!(Filters { min_volume: Some(300), ..all.clone() }.keeps(&station, now));
        assert!(all.keeps(&station, now));
        assert!(!Filters { pad: Some(Pad::L), ..all.clone() }.keeps(&station, now));
        assert!(Filters { pad: Some(Pad::S), ..all.clone() }.keeps(&station, now));
        assert!(!Filters { max_arrival_ls: Some(1000), ..all.clone() }.keeps(&station, now));
        assert!(!Filters { max_age_days: Some(1), ..all.clone() }.keeps(&station, now));
        assert!(Filters { max_age_days: Some(2), ..all.clone() }.keeps(&station, now));
        let carrier = Facts { fleet_carrier: true, ..station };
        assert!(!Filters { exclude_carriers: true, ..all.clone() }.keeps(&carrier, now));
        let settlement = Facts { odyssey: true, ..station };
        assert!(!Filters { exclude_odyssey: true, ..all.clone() }.keeps(&settlement, now));
        // a system has no pad, arrival or carrier facts: only its age counts
        let strict = Filters {
            pad: Some(Pad::L),
            max_arrival_ls: Some(10),
            exclude_carriers: true,
            max_distance_ly: Some(1),
            min_volume: Some(1000),
            ..all
        };
        assert!(strict.keeps(&Facts { updated_at: Some("2026-10-16T11:00:00Z"), ..Facts::default() }, now));
    }
}
