//! Companion files the game writes next to the journal (`Market.json`, `Outfitting.json`, ...).
//!
//! The journal event (`Market`, `Outfitting`, ...) only says the file changed; the data is in
//! the file. The game rewrites the file around its event, so we read after the event, try
//! again when the game next writes the file, and refuse a file that is missing or whose `timestamp` is older than the event: that
//! is last visit's data, and sending it on as this visit's would be wrong. Each snapshot keeps
//! the file's text as read (`raw_json`) for the EDDN sender.
//!
//! Field names follow the Journal manual v35; `Backpack.json` and `FCMaterials.json` follow the
//! community docs (uncertain: both are newer than the manual).

use super::{arr, flag, loc, micro_resources, opt_b, opt_f, opt_i, opt_s, s, JournalLine, MicroResource};
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompanionKind {
    Market,
    Outfitting,
    Shipyard,
    ModulesInfo,
    ShipLocker,
    Backpack,
    FcMaterials,
}

impl CompanionKind {
    pub const ALL: [CompanionKind; 7] = [
        Self::Market,
        Self::Outfitting,
        Self::Shipyard,
        Self::ModulesInfo,
        Self::ShipLocker,
        Self::Backpack,
        Self::FcMaterials,
    ];

    pub fn file_name(self) -> &'static str {
        match self {
            Self::Market => "Market.json",
            Self::Outfitting => "Outfitting.json",
            Self::Shipyard => "Shipyard.json",
            Self::ModulesInfo => "ModulesInfo.json",
            Self::ShipLocker => "ShipLocker.json",
            Self::Backpack => "Backpack.json",
            Self::FcMaterials => "FCMaterials.json",
        }
    }

    /// The kind whose file this file name is, if any.
    pub fn from_file_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.file_name() == name)
    }

    /// The kind a journal event of this name announces, if any. Note the journal's event is
    /// `ModuleInfo` but the file is `ModulesInfo.json`, and that `Backpack.json` is also
    /// rewritten with each `BackpackChange`, which has no `Backpack` event of its own.
    pub fn for_event(name: &str) -> Option<Self> {
        Some(match name {
            "Market" => Self::Market,
            "Outfitting" => Self::Outfitting,
            "Shipyard" => Self::Shipyard,
            "ModuleInfo" => Self::ModulesInfo,
            "ShipLocker" => Self::ShipLocker,
            "Backpack" | "BackpackChange" => Self::Backpack,
            "FCMaterials" => Self::FcMaterials,
            _ => return None,
        })
    }
}

/// Why a companion file was not used.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CompanionIssue {
    #[error("file is missing")]
    Missing,
    #[error("file is unreadable: {0}")]
    Unreadable(String),
    #[error("file is older than its event (file {file}, event {event})")]
    Stale { file: String, event: String },
}

/// A companion event seen in the journal: the file to read and the event's own timestamp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trigger {
    pub kind: CompanionKind,
    pub event_ts: String,
}

/// A companion file that was not used, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionReport {
    pub kind: CompanionKind,
    pub event_ts: String,
    pub issue: CompanionIssue,
}

impl std::fmt::Display for CompanionReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} not used for the {} event: {}", self.kind.file_name(), self.event_ts, self.issue)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarketItem {
    pub id: Option<i64>,
    /// Commodity symbol, e.g. `$gold_name;`.
    pub name: String,
    pub name_localised: Option<String>,
    pub category: Option<String>,
    pub buy_price: i64,
    pub sell_price: i64,
    pub mean_price: i64,
    pub stock_bracket: i64,
    pub demand_bracket: i64,
    pub stock: i64,
    pub demand: i64,
    pub consumer: bool,
    pub producer: bool,
    pub rare: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarketSnap {
    pub timestamp: String,
    pub market_id: Option<i64>,
    pub station_name: String,
    pub station_type: Option<String>,
    pub star_system: String,
    pub items: Vec<MarketItem>,
    pub raw_json: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutfittingItem {
    pub id: Option<i64>,
    pub name: String,
    pub buy_price: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutfittingSnap {
    pub timestamp: String,
    pub market_id: Option<i64>,
    pub station_name: String,
    pub star_system: String,
    pub horizons: Option<bool>,
    pub items: Vec<OutfittingItem>,
    pub raw_json: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShipyardItem {
    pub id: Option<i64>,
    pub ship_type: String,
    pub ship_type_localised: Option<String>,
    pub ship_price: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShipyardSnap {
    pub timestamp: String,
    pub market_id: Option<i64>,
    pub station_name: String,
    pub star_system: String,
    pub horizons: Option<bool>,
    pub allow_cobra_mk_iv: Option<bool>,
    pub price_list: Vec<ShipyardItem>,
    pub raw_json: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleInfo {
    pub slot: String,
    pub item: String,
    pub power: Option<f64>,
    pub priority: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModulesInfoSnap {
    pub timestamp: String,
    pub modules: Vec<ModuleInfo>,
    pub raw_json: String,
}

/// `ShipLocker.json` or `Backpack.json`: micro-resources by category.
#[derive(Debug, Clone, PartialEq)]
pub struct InventorySnap {
    pub timestamp: String,
    pub items: Vec<MicroResource>,
    pub raw_json: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FcMaterialItem {
    pub id: Option<i64>,
    pub name: String,
    pub name_localised: Option<String>,
    pub price: i64,
    pub stock: i64,
    pub demand: i64,
}

/// `FCMaterials.json`: what a fleet carrier's bartender buys and sells. community.
#[derive(Debug, Clone, PartialEq)]
pub struct FcMaterialsSnap {
    pub timestamp: String,
    pub market_id: Option<i64>,
    pub carrier_name: Option<String>,
    pub carrier_id: Option<String>,
    pub items: Vec<FcMaterialItem>,
    pub raw_json: String,
}

/// A parsed, fresh companion file.
#[derive(Debug, Clone, PartialEq)]
pub enum CompanionFile {
    Market(MarketSnap),
    Outfitting(OutfittingSnap),
    Shipyard(ShipyardSnap),
    ModulesInfo(ModulesInfoSnap),
    ShipLocker(InventorySnap),
    Backpack(InventorySnap),
    FcMaterials(FcMaterialsSnap),
}

fn parse_ts(t: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(t).ok().map(|d| d.to_utc())
}

/// Parse `bytes` as the file for `kind` and accept it only if it is not older than the event.
/// `bytes` is `None` when the file does not exist. Pure; no I/O and no waiting.
pub fn parse_fresh(kind: CompanionKind, bytes: Option<&[u8]>, event_ts: &str) -> Result<CompanionFile, CompanionIssue> {
    let bytes = bytes.ok_or(CompanionIssue::Missing)?;
    let text = std::str::from_utf8(bytes).map_err(|e| CompanionIssue::Unreadable(e.to_string()))?;
    // The game truncates and rewrites in place, so a read can catch it half-written.
    let v: Value = serde_json::from_str(text).map_err(|e| CompanionIssue::Unreadable(e.to_string()))?;
    let timestamp = opt_s(&v, "timestamp").ok_or_else(|| CompanionIssue::Unreadable("no timestamp".into()))?;
    match (parse_ts(&timestamp), parse_ts(event_ts)) {
        (Some(file), Some(event)) if file < event => {
            return Err(CompanionIssue::Stale { file: timestamp, event: event_ts.to_string() })
        }
        (None, _) => return Err(CompanionIssue::Unreadable(format!("bad timestamp `{timestamp}`"))),
        // An event timestamp we cannot read is not the file's fault; accept the file.
        _ => {}
    }
    let raw_json = text.to_string();
    Ok(match kind {
        CompanionKind::Market => CompanionFile::Market(MarketSnap {
            timestamp,
            market_id: opt_i(&v, "MarketID"),
            station_name: s(&v, "StationName"),
            station_type: opt_s(&v, "StationType"),
            star_system: s(&v, "StarSystem"),
            items: arr(&v, "Items")
                .map(|i| MarketItem {
                    id: opt_i(i, "id"),
                    name: s(i, "Name"),
                    name_localised: loc(i, "Name"),
                    category: opt_s(i, "Category"),
                    buy_price: opt_i(i, "BuyPrice").unwrap_or(0),
                    sell_price: opt_i(i, "SellPrice").unwrap_or(0),
                    mean_price: opt_i(i, "MeanPrice").unwrap_or(0),
                    stock_bracket: opt_i(i, "StockBracket").unwrap_or(0),
                    demand_bracket: opt_i(i, "DemandBracket").unwrap_or(0),
                    stock: opt_i(i, "Stock").unwrap_or(0),
                    demand: opt_i(i, "Demand").unwrap_or(0),
                    consumer: flag(i, "Consumer"),
                    producer: flag(i, "Producer"),
                    rare: flag(i, "Rare"),
                })
                .collect(),
            raw_json,
        }),
        CompanionKind::Outfitting => CompanionFile::Outfitting(OutfittingSnap {
            timestamp,
            market_id: opt_i(&v, "MarketID"),
            station_name: s(&v, "StationName"),
            star_system: s(&v, "StarSystem"),
            horizons: opt_b(&v, "Horizons"),
            items: arr(&v, "Items")
                .map(|i| OutfittingItem {
                    id: opt_i(i, "id"),
                    name: s(i, "Name"),
                    buy_price: opt_i(i, "BuyPrice").unwrap_or(0),
                })
                .collect(),
            raw_json,
        }),
        CompanionKind::Shipyard => CompanionFile::Shipyard(ShipyardSnap {
            timestamp,
            market_id: opt_i(&v, "MarketID"),
            station_name: s(&v, "StationName"),
            star_system: s(&v, "StarSystem"),
            horizons: opt_b(&v, "Horizons"),
            allow_cobra_mk_iv: opt_b(&v, "AllowCobraMkIV"),
            price_list: arr(&v, "PriceList")
                .map(|i| ShipyardItem {
                    id: opt_i(i, "id"),
                    ship_type: s(i, "ShipType"),
                    ship_type_localised: loc(i, "ShipType"),
                    ship_price: opt_i(i, "ShipPrice").unwrap_or(0),
                })
                .collect(),
            raw_json,
        }),
        CompanionKind::ModulesInfo => CompanionFile::ModulesInfo(ModulesInfoSnap {
            timestamp,
            modules: arr(&v, "Modules")
                .map(|m| ModuleInfo {
                    slot: s(m, "Slot"),
                    item: s(m, "Item"),
                    power: opt_f(m, "Power"),
                    priority: opt_i(m, "Priority"),
                })
                .collect(),
            raw_json,
        }),
        CompanionKind::ShipLocker => {
            CompanionFile::ShipLocker(InventorySnap { timestamp, items: micro_resources(&v), raw_json })
        }
        CompanionKind::Backpack => {
            CompanionFile::Backpack(InventorySnap { timestamp, items: micro_resources(&v), raw_json })
        }
        CompanionKind::FcMaterials => CompanionFile::FcMaterials(FcMaterialsSnap {
            timestamp,
            market_id: opt_i(&v, "MarketID"),
            carrier_name: opt_s(&v, "CarrierName"),
            // The id is a number on some builds and a string on others; keep the text.
            carrier_id: opt_s(&v, "CarrierID").or_else(|| opt_i(&v, "CarrierID").map(|n| n.to_string())),
            items: arr(&v, "Items")
                .map(|i| FcMaterialItem {
                    id: opt_i(i, "id"),
                    name: s(i, "Name"),
                    name_localised: loc(i, "Name"),
                    price: opt_i(i, "Price").unwrap_or(0),
                    stock: opt_i(i, "Stock").unwrap_or(0),
                    demand: opt_i(i, "Demand").unwrap_or(0),
                })
                .collect(),
            raw_json,
        }),
    })
}

/// Read `dir/<file>` for `kind` once. A file the game has not finished writing is reported; the
/// caller tries again when the game next touches the file (waiting here would stall the journal).
pub fn read_companion(dir: &Path, kind: CompanionKind, event_ts: &str) -> Result<CompanionFile, CompanionIssue> {
    let bytes = std::fs::read(dir.join(kind.file_name())).ok();
    parse_fresh(kind, bytes.as_deref(), event_ts)
}

/// The companion events in `lines`, newest per kind, in order of appearance. Earlier events of
/// the same kind are superseded: the file only ever holds the latest.
pub fn triggers(lines: &[JournalLine]) -> Vec<Trigger> {
    let mut out: Vec<Trigger> = vec![];
    for l in lines {
        if let Some(kind) = CompanionKind::for_event(&l.name) {
            out.retain(|t| t.kind != kind);
            out.push(Trigger { kind, event_ts: l.timestamp.clone() });
        }
    }
    out
}

/// Read the file for each trigger. Files that can't be used come back as reports.
pub fn load(dir: &Path, triggers: &[Trigger]) -> (Vec<CompanionFile>, Vec<CompanionReport>) {
    let mut files = vec![];
    let mut reports = vec![];
    for t in triggers {
        match read_companion(dir, t.kind, &t.event_ts) {
            Ok(f) => files.push(f),
            Err(issue) => reports.push(CompanionReport { kind: t.kind, event_ts: t.event_ts.clone(), issue }),
        }
    }
    (files, reports)
}
