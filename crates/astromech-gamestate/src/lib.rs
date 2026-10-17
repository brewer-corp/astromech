//! # astromech-gamestate
//!
//! The reducer: folds journal events and Status.json snapshots into one [`GameState`]
//! (ARCHITECTURE.md §5.2). Pure and synchronous so it can be replay-tested.

pub mod mining;

use astromech_journal::companion::{
    CompanionFile, FcMaterialsSnap, InventorySnap, MarketSnap, ModulesInfoSnap, OutfittingSnap, ShipyardSnap,
};
use astromech_journal::{CargoSnap, ExtraEvent, JournalEvent, JournalLine, NavRouteSnap, RouteHop, StationInfo};
use astromech_materials::Ledger;
use astromech_status::{StatusSnap, FLAG_DOCKED, FLAG_GEAR, FLAG_SUPERCRUISE};
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;

/// A snapshot shared by every copy of the state. The hub clones and compares the whole state on
/// every update, so a 300-line market must not be copied or compared by value each time; two
/// are equal when they are the same read of the file.
#[derive(Debug)]
pub struct Shared<T>(pub Arc<T>);

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> std::ops::Deref for Shared<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// The latest fresh snapshot of each companion file (`Market.json`, ...). Not serialised to the
/// UI: they are large and consumers read them from the state directly.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Companions {
    pub market: Option<Shared<MarketSnap>>,
    pub outfitting: Option<Shared<OutfittingSnap>>,
    pub shipyard: Option<Shared<ShipyardSnap>>,
    pub modules_info: Option<Shared<ModulesInfoSnap>>,
    pub ship_locker: Option<Shared<InventorySnap>>,
    pub backpack: Option<Shared<InventorySnap>>,
    pub fc_materials: Option<Shared<FcMaterialsSnap>>,
}

/// An active mission, as far as the journal says.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Mission {
    pub id: i64,
    pub name: String,
    /// When it expires, in seconds since the Unix epoch (`None`: no expiry, or the event had
    /// no timestamp to date it by).
    pub expires: Option<i64>,
    pub destination_system: Option<String>,
    pub target_faction: Option<String>,
    pub kill_count: Option<i64>,
}

fn epoch(ts: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(ts).ok().map(|t| t.timestamp())
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct GameState {
    pub commander: Option<String>,
    pub ship: Option<String>,
    pub ship_name: Option<String>,
    pub ship_ident: Option<String>,
    pub system: Option<String>,
    /// The current system's position in ly (`StarPos` of the last `Location` / `FSDJump`).
    pub star_pos: Option<[f64; 3]>,
    pub body: Option<String>,
    pub station: Option<String>,
    /// `MarketID` of the station docked at; ties `Market.json` and friends to this dock.
    pub station_market_id: Option<i64>,
    /// Services, economies, faction and pads of the station docked at, from `Docked` (or the
    /// `Location` of a login while docked). Cleared with `station_market_id`.
    pub station_info: Option<StationInfo>,
    pub docked: bool,
    pub gear_down: bool,
    pub supercruise: bool,
    pub status_flags: u64,
    /// False until a Status.json with Flags has been read (main menu / game not running).
    pub status_seen: bool,
    pub gui_focus: Option<u64>,
    pub fuel_main: Option<f64>,
    pub fuel_reservoir: Option<f64>,
    pub fuel_capacity_main: Option<f64>,
    pub cargo_tonnes: Option<f64>,
    pub cargo_capacity: Option<i64>,
    pub cargo_items: Vec<(String, i64)>,
    /// Stolen tonnes aboard (`Cargo.json`).
    pub cargo_stolen: i64,
    /// `Cargo.json`'s lines with their symbols (for "where should I sell my cargo?").
    pub cargo_inventory: Vec<astromech_journal::CargoItem>,
    pub legal_state: Option<String>,
    pub credits: Option<i64>,
    pub rebuy: Option<i64>,
    pub hull_health: Option<f64>,
    pub max_jump_range: Option<f64>,
    pub route: Vec<String>,
    /// The current system's minor factions and population (`Location` / `FSDJump`).
    pub system_factions: Vec<astromech_journal::FactionSnap>,
    pub system_population: i64,
    /// `NavRoute.json`'s systems with star class and position (the route as plotted: the first
    /// is where it was plotted from).
    pub route_hops: Vec<RouteHop>,
    /// The last ordinary hyperspace jump of this ship: (distance in ly, tonnes of fuel it used).
    /// Boosted jumps (neutron, white dwarf, FSD injection) don't count: their distance is
    /// boosted, their fuel isn't. Cleared when the loadout is for another ship.
    pub last_jump: Option<(f64, f64)>,
    /// `ShipID` of the last loadout.
    pub ship_id: Option<i64>,
    /// Frame Shift Drive size from the loadout (2 to 8), for fuel per jump.
    pub fsd_size: Option<u8>,
    /// Journal file currently followed.
    pub journal_file: Option<String>,
    /// Ship engineering materials (login snapshot + later deltas).
    pub materials: Ledger,
    /// Powerplay standing (since the core started: the week total needs older journals, which
    /// the app window replays itself).
    pub powerplay: astromech_powerplay::Standing,
    /// The current system's Powerplay state (`None`: no Powerplay activity, or not known yet).
    pub system_power: Option<astromech_powerplay::SystemPower>,
    /// Colonisation projects still being built, by the `MarketID` of the construction site: the
    /// commodities (lower-case journal symbols, `$steel_name;`) each still needs delivered, from
    /// the last `ColonisationConstructionDepot` seen (this journal; older ones are not read).
    /// A project that is complete or failed is dropped.
    #[serde(skip)]
    pub project_needs: BTreeMap<i64, Vec<String>>,
    /// Active missions (`Missions` at login, then `MissionAccepted` and the ends).
    pub missions: Vec<Mission>,
    /// The sampling in progress.
    pub exobiology: astromech_exobiology::Exobiology,
    /// The current system's bodies (scans, signals, genus lists, samples), for the biology
    /// view (`astromech_exploration::biology`).
    #[serde(skip)]
    pub system_bodies: astromech_exploration::SystemFold,
    #[serde(skip)]
    pub companions: Companions,
    /// In a logged-in session: set by `LoadGame`, cleared by `Shutdown` and the `Music`
    /// `MainMenu` that follows a logout. False at a fresh launch until the login.
    #[serde(skip)]
    pub logged_in: bool,
}

impl GameState {
    /// Flags for planning, `None` if no Status.json has been seen.
    pub fn flags(&self) -> Option<u64> {
        self.status_seen.then_some(self.status_flags)
    }

    /// Fold a journal line (its timestamp dates Powerplay merits into their weekly cycle).
    pub fn apply_line(&mut self, l: &JournalLine) {
        self.apply_at(&l.timestamp, &l.event);
    }

    /// Fold an event without its timestamp.
    pub fn apply_journal(&mut self, ev: &JournalEvent) {
        self.apply_at("", ev);
    }

    fn apply_at(&mut self, ts: &str, ev: &JournalEvent) {
        self.powerplay.apply(ts, ev);
        self.exobiology.apply(ev);
        self.system_bodies.apply_event(ev);
        if let Some(s) = astromech_powerplay::SystemPower::from_event(ev) {
            self.system_power = s;
        }
        // A new journal file that is not the continuation of one (part 2 and up) is a new run of
        // the game: a crash leaves no logout behind.
        if matches!(ev, JournalEvent::FileHeader { part, .. } if part.unwrap_or(1) <= 1) {
            self.logged_in = false;
        }
        if let JournalEvent::Other { event, raw } = ev {
            if event == "Shutdown"
                || (event == "Music" && raw.get("MusicTrack").and_then(|t| t.as_str()) == Some("MainMenu"))
            {
                self.logged_in = false;
            }
        }
        match ev {
            JournalEvent::LoadGame { commander, ship, ship_name, ship_ident, credits } => {
                self.logged_in = true;
                self.commander = Some(commander.clone());
                self.ship = ship.clone();
                if ship_name.is_some() {
                    self.ship_name = ship_name.clone();
                }
                if ship_ident.is_some() {
                    self.ship_ident = ship_ident.clone();
                }
                self.credits = *credits;
            }
            JournalEvent::Loadout {
                ship,
                ship_name,
                ship_ident,
                hull_health,
                cargo_capacity,
                max_jump_range,
                fuel_capacity_main,
                rebuy,
                modules,
                ship_id,
                ..
            } => {
                if ship_id.is_some() && *ship_id != self.ship_id {
                    if self.ship_id.is_some() {
                        self.last_jump = None;
                    }
                    self.ship_id = *ship_id;
                }
                if ship.is_some() {
                    self.ship = ship.clone();
                }
                self.ship_name = ship_name.clone();
                self.ship_ident = ship_ident.clone();
                self.hull_health = *hull_health;
                self.cargo_capacity = *cargo_capacity;
                self.max_jump_range = *max_jump_range;
                self.fuel_capacity_main = *fuel_capacity_main;
                self.rebuy = *rebuy;
                self.fsd_size = modules
                    .iter()
                    .find(|m| m.slot.eq_ignore_ascii_case("FrameShiftDrive"))
                    .and_then(|m| m.item.split("_size").nth(1)?.chars().next()?.to_digit(10))
                    .map(|d| d as u8);
            }
            JournalEvent::Location {
                star_system,
                body,
                docked,
                station_name,
                market_id,
                star_pos,
                station,
                factions,
                population,
                ..
            } => {
                self.system_factions = factions.clone();
                self.system_population = *population;
                if self.system.as_ref() != Some(star_system) || star_pos.is_some() {
                    self.star_pos = *star_pos;
                }
                self.system = Some(star_system.clone());
                self.body = body.clone();
                self.docked = *docked;
                // Logging in docked: `Location` carries the station, there is no `Docked`.
                if *docked {
                    if station_name.is_some() {
                        self.station = station_name.clone();
                    }
                    self.station_market_id = *market_id;
                    self.station_info = station.clone();
                } else {
                    self.station = None;
                    self.station_market_id = None;
                    self.station_info = None;
                }
            }
            JournalEvent::ApproachBody { star_system, body } => {
                self.system = Some(star_system.clone());
                self.body = Some(body.clone());
            }
            JournalEvent::LeaveBody { .. } => self.body = None,
            JournalEvent::FsdJump {
                star_system,
                from_carrier,
                star_pos,
                jump_dist,
                fuel_used,
                fuel_level,
                factions,
                population,
                boost_used,
                ..
            } => {
                self.system_factions = factions.clone();
                self.system_population = *population;
                self.system = Some(star_system.clone());
                self.star_pos = *star_pos;
                if let Some(f) = fuel_used.filter(|_| !from_carrier && !boost_used && *jump_dist > 0.0) {
                    self.last_jump = Some((*jump_dist, f));
                }
                if fuel_level.is_some() {
                    self.fuel_main = *fuel_level;
                }
                if !from_carrier {
                    self.station = None;
                    self.station_market_id = None;
                    self.station_info = None;
                    self.body = None;
                }
            }
            JournalEvent::Docked { station_name, star_system, market_id, station, .. } => {
                self.station = Some(station_name.clone());
                self.station_market_id = *market_id;
                self.station_info = Some(station.clone());
                self.system = Some(star_system.clone());
                self.docked = true;
            }
            JournalEvent::Extra(ExtraEvent::ColonisationConstructionDepot(d)) => {
                if let Some(id) = d.market_id {
                    let needs: Vec<String> = d
                        .resources
                        .iter()
                        .filter(|r| r.required > r.provided)
                        .map(|r| r.name.to_ascii_lowercase())
                        .collect();
                    if d.complete || d.failed || needs.is_empty() {
                        self.project_needs.remove(&id);
                    } else {
                        self.project_needs.insert(id, needs);
                    }
                }
            }
            JournalEvent::Undocked { .. } => {
                self.docked = false;
                self.station = None;
                self.station_market_id = None;
                self.station_info = None;
            }
            JournalEvent::Extra(ExtraEvent::Missions { active, .. }) => {
                // The login list only dates expiry by seconds from its own timestamp; what an
                // acceptance told us (destination, target) is kept for the ids still active.
                let at = epoch(ts);
                let old = std::mem::take(&mut self.missions);
                self.missions = active
                    .iter()
                    .map(|m| {
                        let known = old.iter().find(|o| o.id == m.mission_id);
                        Mission {
                            id: m.mission_id,
                            name: m.name.clone(),
                            expires: at.zip(m.expires_secs).map(|(a, s)| a + s).or(known.and_then(|k| k.expires)),
                            destination_system: known.and_then(|k| k.destination_system.clone()),
                            target_faction: known.and_then(|k| k.target_faction.clone()),
                            kill_count: known.and_then(|k| k.kill_count),
                        }
                    })
                    .collect();
            }
            JournalEvent::Extra(ExtraEvent::MissionAccepted(m)) => {
                self.missions.retain(|o| o.id != m.mission_id);
                self.missions.push(Mission {
                    id: m.mission_id,
                    name: m.name.clone(),
                    expires: m.expiry.as_deref().and_then(epoch),
                    destination_system: m.destination_system.clone(),
                    target_faction: m.target_faction.clone(),
                    kill_count: m.kill_count,
                });
            }
            JournalEvent::Extra(ExtraEvent::MissionCompleted { mission_id: Some(id), .. })
            | JournalEvent::Extra(ExtraEvent::MissionEnded { mission_id: Some(id), .. }) => {
                self.missions.retain(|m| m.id != *id);
            }
            JournalEvent::Extra(ExtraEvent::MissionRedirected {
                mission_id: Some(id), new_destination_system, ..
            }) => {
                if let Some(m) = self.missions.iter_mut().find(|m| m.id == *id) {
                    if new_destination_system.is_some() {
                        m.destination_system = new_destination_system.clone();
                    }
                }
            }
            _ => {}
        }
        self.materials.apply("", ev);
    }
    pub fn apply_status(&mut self, s: &StatusSnap) {
        self.status_flags = s.flags;
        self.status_seen = true;
        self.gui_focus = s.gui_focus;
        self.gear_down = s.flags & FLAG_GEAR != 0;
        self.supercruise = s.flags & FLAG_SUPERCRUISE != 0;
        self.docked = s.flags & FLAG_DOCKED != 0;
        if s.fuel_main.is_some() {
            self.fuel_main = s.fuel_main;
        }
        if s.fuel_reservoir.is_some() {
            self.fuel_reservoir = s.fuel_reservoir;
        }
        if s.cargo.is_some() {
            self.cargo_tonnes = s.cargo;
        }
        if s.legal_state.is_some() {
            self.legal_state = s.legal_state.clone();
        }
        if s.body_name.is_some() {
            self.body = s.body_name.clone();
        }
        if s.balance.is_some() {
            self.credits = s.balance;
        }
        self.exobiology.apply_status(s);
    }

    pub fn apply_cargo(&mut self, c: &CargoSnap) {
        self.cargo_tonnes = Some(c.count as f64);
        self.cargo_items = c.items.clone();
        self.cargo_inventory = c.inventory.clone();
        self.cargo_stolen = c.inventory.iter().map(|i| i.stolen.max(0)).sum();
    }

    /// Keep a companion file that passed the freshness check (see `astromech_journal::companion`).
    pub fn apply_companion(&mut self, f: CompanionFile) {
        let c = &mut self.companions;
        match f {
            CompanionFile::Market(x) => c.market = Some(Shared(Arc::new(x))),
            CompanionFile::Outfitting(x) => c.outfitting = Some(Shared(Arc::new(x))),
            CompanionFile::Shipyard(x) => c.shipyard = Some(Shared(Arc::new(x))),
            CompanionFile::ModulesInfo(x) => c.modules_info = Some(Shared(Arc::new(x))),
            CompanionFile::ShipLocker(x) => c.ship_locker = Some(Shared(Arc::new(x))),
            CompanionFile::Backpack(x) => c.backpack = Some(Shared(Arc::new(x))),
            CompanionFile::FcMaterials(x) => c.fc_materials = Some(Shared(Arc::new(x))),
        }
    }

    pub fn apply_route(&mut self, r: &NavRouteSnap) {
        self.route = r.systems.clone();
        self.route_hops = r.hops.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn line(l: &str) -> astromech_journal::JournalLine {
        astromech_journal::parse_line(l).unwrap()
    }
    const FILEHEADER: &str = r#"{"timestamp":"t","event":"Fileheader","part":1,"gameversion":"4.0","build":"b"}"#;
    const ROLLOVER: &str = r#"{"timestamp":"t","event":"Fileheader","part":2,"gameversion":"4.0","build":"b"}"#;
    const LOAD: &str = r#"{"timestamp":"t","event":"LoadGame","Commander":"Test","Ship":"krait_mkii","Credits":50}"#;
    const SHUTDOWN: &str = r#"{"timestamp":"t","event":"Shutdown"}"#;
    const MENU: &str = r#"{"timestamp":"t","event":"Music","MusicTrack":"MainMenu"}"#;

    fn logged_in_after(lines: &[&str]) -> bool {
        let mut g = GameState::default();
        for l in lines {
            g.apply_line(&line(l));
        }
        g.logged_in
    }

    #[test]
    fn logged_in_follows_the_login_and_the_logout() {
        assert!(!logged_in_after(&[FILEHEADER]));
        assert!(logged_in_after(&[FILEHEADER, LOAD]));
        assert!(!logged_in_after(&[FILEHEADER, LOAD, SHUTDOWN]));
        assert!(!logged_in_after(&[FILEHEADER, LOAD, MENU]));
        assert!(logged_in_after(&[LOAD, SHUTDOWN, LOAD]), "a login after a logout");
        let other_track = r#"{"timestamp":"t","event":"Music","MusicTrack":"Exploration"}"#;
        assert!(logged_in_after(&[LOAD, other_track]));
    }

    #[test]
    fn a_new_journal_file_clears_a_crashed_session_but_a_rollover_keeps_it() {
        // No Shutdown or MainMenu: the game crashed, and the next run opens a new file.
        assert!(!logged_in_after(&[LOAD, FILEHEADER]));
        let headerless = r#"{"timestamp":"t","event":"Fileheader","gameversion":"4.0","build":"b"}"#;
        assert!(!logged_in_after(&[LOAD, headerless]));
        // Part 2 is the same run continued in a new file.
        assert!(logged_in_after(&[LOAD, ROLLOVER]));
    }

    #[test]
    fn folds_events() {
        let mut g = GameState::default();
        for l in [
            r#"{"timestamp":"t","event":"LoadGame","Commander":"Test","Ship":"krait_mkii","Credits":50}"#,
            r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Shinrarta Dezhra","JumpDist":12.0}"#,
            r#"{"timestamp":"t","event":"Docked","StationName":"Jameson Memorial","StarSystem":"Shinrarta Dezhra"}"#,
        ] {
            g.apply_journal(&astromech_journal::parse_line(l).unwrap().event);
        }
        assert_eq!(g.station.as_deref(), Some("Jameson Memorial"));
        assert!(g.docked);
        assert_eq!(g.credits, Some(50));
        g.apply_status(&StatusSnap { flags: FLAG_GEAR, gui_focus: None, ..Default::default() });
        assert!(g.gear_down && !g.docked);
        g.apply_route(&NavRouteSnap { systems: vec!["Shinrarta Dezhra".into(), "Sol".into()], hops: vec![] });
        assert_eq!(g.route.len(), 2);
        g.apply_journal(
            &astromech_journal::parse_line(
                r#"{"timestamp":"t","event":"Materials","Raw":[{"Name":"iron","Count":4}],"Manufactured":[],"Encoded":[]}"#,
            )
            .unwrap()
            .event,
        );
        assert_eq!(g.materials.count("iron"), 4);
        g.apply_journal(
            &astromech_journal::parse_line(
                r#"{"timestamp":"t","event":"CarrierJump","StarSystem":"Alioth","SystemAllegiance":"Alliance"}"#,
            )
            .unwrap()
            .event,
        );
        assert_eq!(g.system.as_deref(), Some("Alioth"));
        assert_eq!(g.station.as_deref(), Some("Jameson Memorial"));
    }

    #[test]
    fn missions_follow_the_journal() {
        let mut g = GameState::default();
        let apply = |g: &mut GameState, l: &str| g.apply_line(&astromech_journal::parse_line(l).unwrap());
        apply(
            &mut g,
            r#"{"timestamp":"2026-10-16T20:00:00Z","event":"MissionAccepted","MissionID":1,"Name":"Mission_Massacre","Faction":"A","DestinationSystem":"Sys","TargetFaction":"Bad","KillCount":10,"Expiry":"2026-10-16T22:00:00Z"}"#,
        );
        assert_eq!(g.missions[0].expires, Some(1_792_188_000));
        assert_eq!(g.missions[0].destination_system.as_deref(), Some("Sys"));
        // The login list re-dates by seconds left, keeping what the acceptance said.
        apply(
            &mut g,
            r#"{"timestamp":"2026-10-16T21:00:00Z","event":"Missions","Active":[{"MissionID":1,"Name":"Mission_Massacre","PassengerMission":false,"Expires":1800},{"MissionID":2,"Name":"Mission_Delivery","PassengerMission":false,"Expires":60}],"Failed":[],"Complete":[]}"#,
        );
        assert_eq!(g.missions.len(), 2);
        assert_eq!(g.missions[0].expires, Some(1_792_188_000 - 1800));
        assert_eq!(g.missions[0].target_faction.as_deref(), Some("Bad"));
        apply(
            &mut g,
            r#"{"timestamp":"2026-10-16T21:01:00Z","event":"MissionRedirected","MissionID":1,"NewDestinationSystem":"Elsewhere"}"#,
        );
        assert_eq!(g.missions[0].destination_system.as_deref(), Some("Elsewhere"));
        // A redirect to a station only (no new system) keeps the system.
        apply(
            &mut g,
            r#"{"timestamp":"2026-10-16T21:01:30Z","event":"MissionRedirected","MissionID":1,"NewDestinationStation":"Only A Station"}"#,
        );
        assert_eq!(g.missions[0].destination_system.as_deref(), Some("Elsewhere"));
        apply(
            &mut g,
            r#"{"timestamp":"2026-10-16T21:02:00Z","event":"MissionCompleted","MissionID":1,"Name":"Mission_Massacre"}"#,
        );
        apply(
            &mut g,
            r#"{"timestamp":"2026-10-16T21:02:00Z","event":"MissionAbandoned","MissionID":2,"Name":"Mission_Delivery"}"#,
        );
        assert!(g.missions.is_empty());
    }

    #[test]
    fn star_pos_follows_the_current_system() {
        let mut g = GameState::default();
        let apply = |g: &mut GameState, l: &str| g.apply_journal(&astromech_journal::parse_line(l).unwrap().event);
        apply(&mut g, r#"{"timestamp":"t","event":"Location","StarSystem":"Sol","StarPos":[0,0,0],"Docked":false}"#);
        apply(
            &mut g,
            r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Deciat","StarPos":[122.625,-0.8125,-47.28125]}"#,
        );
        apply(&mut g, r#"{"timestamp":"t","event":"Location","StarSystem":"Deciat","Docked":false}"#);
        assert_eq!(g.star_pos, Some([122.625, -0.8125, -47.28125]), "same system without StarPos: kept");
        apply(&mut g, r#"{"timestamp":"t","event":"Location","StarSystem":"Alioth","Docked":false}"#);
        assert_eq!(g.star_pos, None, "another system without StarPos: unknown, not stale");
    }
}
