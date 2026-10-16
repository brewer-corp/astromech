//! What stations sell: every module (with the classes, ratings and mounts it comes in) and
//! every ship, from `data/catalog/outfitting.json` (`scripts/gen-outfitting.py`, from
//! EDCD/FDevIDs), plus the names commanders say for them ("FSD", "multicannon").

use astromech_data::spansh;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// A hardpoint's mount.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mount {
    Fixed,
    Gimballed,
    Turreted,
}

impl Mount {
    pub fn word(self) -> &'static str {
        match self {
            Mount::Fixed => "fixed",
            Mount::Gimballed => "gimballed",
            Mount::Turreted => "turreted",
        }
    }

    /// The mount in a module's journal symbol (`Hpt_MultiCannon_Gimbal_Large`).
    pub fn of_symbol(symbol: &str) -> Option<Mount> {
        let s = symbol.to_ascii_lowercase();
        let has = |w: &str| s.split('_').any(|p| p == w);
        if has("fixed") {
            Some(Mount::Fixed)
        } else if has("gimbal") {
            Some(Mount::Gimballed)
        } else if has("turret") {
            Some(Mount::Turreted)
        } else {
            None
        }
    }

    /// The mount a spoken word names ("gimballed", "turret").
    pub fn from_word(w: &str) -> Option<Mount> {
        match w {
            "fixed" => Some(Mount::Fixed),
            "gimballed" | "gimbaled" | "gimbal" | "gimbals" => Some(Mount::Gimballed),
            "turreted" | "turret" | "turrets" => Some(Mount::Turreted),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub class: u8,
    pub rating: String,
    #[serde(default)]
    pub mount: Option<Mount>,
}

/// One module, by its name in game, and every size it is sold in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleKind {
    pub name: String,
    /// "standard", "internal", "hardpoint" or "utility".
    pub category: String,
    pub sizes: Vec<Size>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipKind {
    /// The journal symbol, lower case (`krait_mkii`).
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Catalog {
    pub source: String,
    pub modules: Vec<ModuleKind>,
    pub ships: Vec<ShipKind>,
}

pub fn catalog() -> &'static Catalog {
    static C: OnceLock<Catalog> = OnceLock::new();
    C.get_or_init(|| {
        serde_json::from_str(include_str!("../../../data/catalog/outfitting.json")).expect("valid outfitting.json")
    })
}

/// The catalogue's module called `name` (any case).
pub fn module(name: &str) -> Option<&'static ModuleKind> {
    catalog().modules.iter().find(|m| m.name.eq_ignore_ascii_case(name))
}

/// The catalogue's ship called `name` (any case).
pub fn ship(name: &str) -> Option<&'static ShipKind> {
    catalog().ships.iter().find(|s| s.name.eq_ignore_ascii_case(name))
}

/// The pad the ship called `name` needs.
pub fn ship_pad(name: &str) -> Option<crate::Pad> {
    ship(name).and_then(|s| crate::Pad::for_ship(&s.symbol))
}

/// Short names commanders use for ships, and the ship each stands for. Every ship's own name
/// is a way to say it too (see [`spoken_ships`]).
const SHIP_ALIASES: &[(&str, &str)] = &[
    ("krait", "Krait MkII"),
    ("phantom", "Krait Phantom"),
    ("conda", "Anaconda"),
    ("corvette", "Federal Corvette"),
    ("cutter", "Imperial Cutter"),
    ("clipper", "Imperial Clipper"),
    ("courier", "Imperial Courier"),
    ("asp", "Asp Explorer"),
    ("asp x", "Asp Explorer"),
    ("chieftain", "Alliance Chieftain"),
    ("challenger", "Alliance Challenger"),
    ("crusader", "Alliance Crusader"),
    ("fer de lance", "Fer-de-Lance"),
    ("fdl", "Fer-de-Lance"),
    ("cobra", "Cobra MkIII"),
    ("viper", "Viper MkIII"),
    ("dbx", "Diamondback Explorer"),
    ("diamondback", "Diamondback Explorer"),
    ("dbs", "Diamondback Scout"),
    ("type 6", "Type-6 Transporter"),
    ("type 7", "Type-7 Transporter"),
    ("type 8", "Type-8 Transporter"),
    ("type 9", "Type-9 Heavy"),
    ("type 10", "Type-10 Defender"),
    ("type 11", "Type-11 Prospector"),
    ("t9", "Type-9 Heavy"),
    ("t10", "Type-10 Defender"),
    ("beluga", "Beluga Liner"),
    ("panther", "Panther Clipper MkII"),
    ("panther clipper", "Panther Clipper MkII"),
    ("lynx", "Lynx Highliner"),
    ("kestrel", "Kestrel Mk II"),
    ("caspian", "Caspian Explorer"),
    ("assault ship", "Federal Assault Ship"),
    ("dropship", "Federal Dropship"),
    ("gunship", "Federal Gunship"),
];

/// Every way to say a ship: (said, ship name), for the voice grammar's `{ship}` slot.
pub fn spoken_ships() -> Vec<(&'static str, &'static str)> {
    let names = catalog().ships.iter().map(|s| (s.name.as_str(), s.name.as_str()));
    names.chain(SHIP_ALIASES.iter().copied()).collect()
}

/// Short names commanders use, and the module each stands for. Every module's own name is a
/// way to say it too (see [`spoken_modules`]).
const MODULE_ALIASES: &[(&str, &str)] = &[
    ("fsd", "Frame Shift Drive"),
    ("frame shift", "Frame Shift Drive"),
    ("sco fsd", "Frame Shift Drive (SCO)"),
    ("sco frame shift drive", "Frame Shift Drive (SCO)"),
    ("powerplant", "Power Plant"),
    ("distributor", "Power Distributor"),
    ("shield gen", "Shield Generator"),
    ("bi weave", "Bi-Weave Shield Generator"),
    ("biweave shield generator", "Bi-Weave Shield Generator"),
    ("prismatic shields", "Prismatic Shield Generator"),
    ("engines", "Thrusters"),
    ("enhanced thrusters", "Enhanced Performance Thrusters"),
    ("scoop", "Fuel Scoop"),
    ("afmu", "Auto Field-Maintenance Unit"),
    ("dss", "Detailed Surface Scanner"),
    ("surface scanner", "Detailed Surface Scanner"),
    ("docking computer", "Standard Docking Computer"),
    ("adc", "Advanced Docking Computer"),
    ("sc assist", "Supercruise Assist"),
    ("hrp", "Hull Reinforcement Package"),
    ("hull reinforcement", "Hull Reinforcement Package"),
    ("mrp", "Module Reinforcement Package"),
    ("module reinforcement", "Module Reinforcement Package"),
    ("scb", "Shield Cell Bank"),
    ("shield cells", "Shield Cell Bank"),
    ("booster", "Shield Booster"),
    ("shield boosters", "Shield Booster"),
    ("fsd booster", "Guardian FSD Booster"),
    ("guardian booster", "Guardian FSD Booster"),
    ("cargo racks", "Cargo Rack"),
    ("srv hangar", "Planetary Vehicle Hangar"),
    ("fighter hangar", "Vessel Hangar"),
    ("collector limpets", "Collector Limpet Controller"),
    ("collectors", "Collector Limpet Controller"),
    ("prospector limpets", "Prospector Limpet Controller"),
    ("prospectors", "Prospector Limpet Controller"),
    ("repair limpets", "Repair Limpet Controller"),
    ("interdictor", "Frame Shift Drive Interdictor"),
    ("kws", "Kill Warrant Scanner"),
    ("wake scanner", "Frame Shift Wake Scanner"),
    ("chaff", "Chaff Launcher"),
    ("heat sink", "Heat Sink Launcher"),
    ("heatsink", "Heat Sink Launcher"),
    ("point defense", "Point Defence"),
    ("ecm", "Electronic Countermeasure"),
    ("multicannon", "Multi-Cannon"),
    ("multicannons", "Multi-Cannon"),
    ("railgun", "Rail Gun"),
    ("frag cannon", "Fragment Cannon"),
    ("missiles", "Missile Rack"),
    ("torpedo", "Torpedo Pylon"),
    ("torpedoes", "Torpedo Pylon"),
    ("mines", "Mine Launcher"),
    ("plasma", "Plasma Accelerator"),
    ("gauss cannon", "Guardian Gauss Cannon"),
    ("seismic charges", "Seismic Charge Launcher"),
    ("displacement missile", "Sub-Surface Displacement Missile"),
    ("economy cabin", "Economy Class Passenger Cabin"),
    ("business cabin", "Business Class Passenger Cabin"),
    ("first class cabin", "First Class Passenger Cabin"),
    ("luxury cabin", "Luxury Class Passenger Cabin"),
];

/// Every way to say a module: (said, module name), for the voice grammar's `{module}` slot.
pub fn spoken_modules() -> Vec<(&'static str, &'static str)> {
    let names = catalog().modules.iter().map(|m| (m.name.as_str(), m.name.as_str()));
    names.chain(MODULE_ALIASES.iter().copied()).collect()
}

/// "5a" → (5, 'A').
fn class_rating(w: &str) -> Option<(u8, char)> {
    match w.as_bytes() {
        [c @ b'1'..=b'8', r @ (b'a'..=b'i' | b'A'..=b'I')] => Some((c - b'0', (*r as char).to_ascii_uppercase())),
        _ => None,
    }
}

fn class_only(w: &str) -> Option<u8> {
    match w.as_bytes() {
        [c @ b'1'..=b'8'] => Some(c - b'0'),
        _ => None,
    }
}

/// Words a spoken module query may carry besides the module's name: a class and rating ("5a"),
/// a class ("class 5"), or a mount. The voice grammar's `{module}` slot doesn't match them
/// against module names.
pub fn is_modifier(w: &str) -> bool {
    class_rating(w).is_some()
        || class_only(w).is_some()
        || Mount::from_word(w).is_some()
        || matches!(w, "class" | "size" | "rated" | "grade")
}

/// A module to buy: its name, and the class, rating and mount when they matter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleWant {
    pub name: String,
    #[serde(default)]
    pub class: Option<u8>,
    /// "A" to "I".
    #[serde(default)]
    pub rating: Option<String>,
    #[serde(default)]
    pub mount: Option<Mount>,
}

impl ModuleWant {
    /// `name` with the class, rating and mount found in `words` (a spoken query: "5a",
    /// "class 5", "gimballed").
    pub fn from_words(name: &str, words: &str) -> ModuleWant {
        let mut w = ModuleWant { name: name.to_string(), class: None, rating: None, mount: None };
        for t in words.split_whitespace().map(str::to_ascii_lowercase) {
            if let Some((c, r)) = class_rating(&t) {
                w.class = Some(c);
                w.rating = Some(r.to_string());
            } else if let Some(c) = class_only(&t) {
                w.class = Some(c);
            } else if let Some(m) = Mount::from_word(&t) {
                w.mount = Some(m);
            }
        }
        w
    }

    /// "5A Frame Shift Drive", "3C Multi-Cannon (gimballed)", "class 5 Fuel Scoop".
    pub fn label(&self) -> String {
        let size = match (self.class, self.rating.as_deref()) {
            (Some(c), Some(r)) => format!("{c}{r} "),
            (Some(c), None) => format!("class {c} "),
            (None, Some(r)) => format!("{r}-rated "),
            (None, None) => String::new(),
        };
        let mount = self.mount.map(|m| format!(" ({})", m.word())).unwrap_or_default();
        format!("{size}{}{mount}", self.name)
    }

    /// Whether a module a station sells is this one.
    pub fn fits(&self, m: &spansh::Module) -> bool {
        m.name.eq_ignore_ascii_case(&self.name)
            && self.class.is_none_or(|c| m.class == c)
            && self.rating.as_deref().is_none_or(|r| m.rating.eq_ignore_ascii_case(r))
            && self.mount.is_none_or(|want| m.ed_symbol.as_deref().and_then(Mount::of_symbol) == Some(want))
    }
}

/// "5A", "3C (gimballed)" for one sold module.
pub fn size_label(m: &spansh::Module) -> String {
    let mount =
        m.ed_symbol.as_deref().and_then(Mount::of_symbol).map(|x| format!(" ({})", x.word())).unwrap_or_default();
    format!("{}{} {}{mount}", m.class, m.rating, m.name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_has_modules_and_ships() {
        let fsd = module("frame shift drive").unwrap();
        assert!(fsd.sizes.contains(&Size { class: 5, rating: "A".into(), mount: None }));
        let mc = module("Multi-Cannon").unwrap();
        assert!(mc.sizes.contains(&Size { class: 3, rating: "C".into(), mount: Some(Mount::Gimballed) }));
        assert!(catalog().ships.iter().any(|s| s.symbol == "krait_mkii" && s.name == "Krait MkII"));
        for (said, name) in MODULE_ALIASES {
            assert!(module(name).is_some(), "alias {said:?} names unknown module {name:?}");
        }
        for (said, name) in SHIP_ALIASES {
            assert!(ship(name).is_some(), "alias {said:?} names unknown ship {name:?}");
        }
        for s in &catalog().ships {
            assert!(ship_pad(&s.name).is_some(), "no pad size for {} ({})", s.name, s.symbol);
        }
        assert_eq!(ship_pad("Anaconda"), Some(crate::Pad::L));
        assert_eq!(ship_pad("krait mkii"), Some(crate::Pad::M));
    }

    #[test]
    fn spoken_words_give_class_rating_and_mount() {
        let w = ModuleWant::from_words("Multi-Cannon", "3c gimballed multi cannon");
        assert_eq!((w.class, w.rating.as_deref(), w.mount), (Some(3), Some("C"), Some(Mount::Gimballed)));
        assert_eq!(w.label(), "3C Multi-Cannon (gimballed)");
        let w = ModuleWant::from_words("Fuel Scoop", "class 5 scoop");
        assert_eq!(w.label(), "class 5 Fuel Scoop");
        assert_eq!(ModuleWant::from_words("Frame Shift Drive", "fsd").label(), "Frame Shift Drive");
        assert!(is_modifier("6a") && is_modifier("turret") && is_modifier("5") && !is_modifier("fsd"));
    }

    #[test]
    fn mount_comes_from_the_symbol() {
        assert_eq!(Mount::of_symbol("Hpt_MultiCannon_Gimbal_Large"), Some(Mount::Gimballed));
        assert_eq!(Mount::of_symbol("Hpt_ATMultiCannon_Turret_Medium_V2"), Some(Mount::Turreted));
        assert_eq!(Mount::of_symbol("Int_Hyperdrive_Size5_Class5"), None);
    }
}
