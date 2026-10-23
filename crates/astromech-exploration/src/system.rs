//! The live system view: the current system's bodies as they are scanned, with what each is
//! worth ([`crate::value`]).
//!
//! Folded from the journal: a jump (`FSDJump`, `CarrierJump`) or a login (`Location`) into
//! another system starts afresh. Then `FSSDiscoveryScan` (the honk: how many bodies there are),
//! `Scan` (a star or planet, from the FSS, the DSS, a nav beacon or the automatic scans on
//! arrival; belt clusters are left out), `FSSBodySignals` and `SAASignalsFound` (biological
//! and geological signals), `SAAScanComplete` (mapped, and whether within the efficiency
//! target) and `FSSAllBodiesFound`. For the biology view ([`crate::biology`]) it also keeps the
//! DSS genus lists (`SAASignalsFound.Genuses`) and the species sampled on each body
//! (`ScanOrganic`), and whether all three samples were taken.
//!
//! "Worth mapping" is a planet not mapped yet whose value mapped efficiently is at least
//! [`WORTH_MAPPING`](crate::value::WORTH_MAPPING). "First footfall likely" is a landable body
//! nobody had discovered (`WasDiscovered: false`), in a system with no population, that the
//! scan doesn't say someone has walked on (`WasFootfalled`). The journal never says who walked
//! first, so it is only a hint.

use crate::value;
use astromech_journal::{ExtraEvent as X, JournalEvent as J, JournalLine, ScanBody};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

/// Standard gravity, m/s²: the journal's `SurfaceGravity` over this is in g.
const G: f64 = 9.80665;

/// One scanned body as the view shows it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BodyView {
    pub name: String,
    /// The name without the system's (`A 2` in `Sol A 2`); the whole name when it isn't prefixed.
    pub short_name: String,
    pub body_id: Option<i64>,
    pub star: bool,
    /// The journal's class (`StarType` or `PlanetClass`) and how it reads ("Earth-like world").
    pub class: String,
    pub class_name: String,
    pub terraformable: bool,
    pub landable: bool,
    /// Has rings. Asteroid belts, which the journal lists with the rings, are `belts`.
    pub rings: bool,
    pub belts: bool,
    /// Surface gravity in g (planets).
    pub gravity_g: Option<f64>,
    pub distance_ls: Option<f64>,
    /// Someone had discovered / mapped / walked on it before (`None`: the scan didn't say).
    pub was_discovered: Option<bool>,
    pub was_mapped: Option<bool>,
    pub was_footfalled: Option<bool>,
    /// The commander mapped it, and whether within the efficiency target.
    pub mapped: bool,
    pub mapped_efficiently: bool,
    pub bio_signals: i64,
    pub geo_signals: i64,
    /// Estimated value now (scanned, and mapped if it was) and mapped efficiently.
    pub value: i64,
    pub value_mapped: i64,
    pub first_discovery: bool,
    pub worth_mapping: bool,
    pub first_footfall_likely: bool,
    /// The commander's landings here before, from the history (the view's reader fills it in).
    pub landings: u32,
}

/// The current system and its bodies.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct SystemView {
    pub system: Option<String>,
    pub system_address: Option<i64>,
    pub population: i64,
    /// From the honk (`FSSDiscoveryScan` `BodyCount`), if the commander honked here.
    pub body_count: Option<i64>,
    /// `FSSAllBodiesFound`.
    pub all_found: bool,
    pub bodies: Vec<BodyView>,
    /// The bodies' value now, and with every planet worth mapping mapped efficiently.
    pub value: i64,
    pub value_if_mapped: i64,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Body {
    body_id: Option<i64>,
    scan: Option<ScanBody>,
    /// Mapped by the commander: within the efficiency target or not.
    mapped: Option<bool>,
    bio: i64,
    geo: i64,
    /// Genus symbols from the DSS.
    genuses: Vec<String>,
    /// Species sampled here, one per genus.
    sampled: Vec<Sampled>,
}

/// A species sampled on a body (`ScanOrganic`).
#[derive(Debug, Clone, PartialEq)]
pub struct Sampled {
    /// The journal's `Genus` symbol (`$Codex_Ent_Stratum_Genus_Name;`).
    pub genus: String,
    /// The journal's `Species` symbol (`$Codex_Ent_Stratum_07_Name;`), the same in every language.
    pub species: String,
    /// `Species_Localised`: the name in the client's language.
    pub name: Option<String>,
    /// All three samples taken (`ScanType: Analyse`).
    pub analysed: bool,
}

/// One body as the fold knows it, for readers beyond the system view (the biology view).
#[derive(Debug, Clone, Copy)]
pub struct BodyRecord<'a> {
    pub name: &'a str,
    pub body_id: Option<i64>,
    pub scan: Option<&'a ScanBody>,
    pub bio_signals: i64,
    /// Genus symbols from the DSS (`$Codex_Ent_Stratum_Genus_Name;`).
    pub genuses: &'a [String],
    /// Species sampled here, one per genus.
    pub sampled: &'a [Sampled],
}

/// The fold behind [`SystemView`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SystemFold {
    system: Option<String>,
    address: Option<i64>,
    population: i64,
    body_count: Option<i64>,
    all_found: bool,
    bodies: BTreeMap<String, Body>,
}

impl SystemFold {
    fn enter(&mut self, name: Option<&str>, address: Option<i64>, population: Option<i64>) {
        let same = match (address, self.address) {
            (Some(a), Some(b)) => a == b,
            _ => name.is_some() && name == self.system.as_deref(),
        };
        if !same {
            *self = Self::default();
        }
        if let Some(n) = name {
            self.system = Some(n.to_string());
        }
        self.address = address.or(self.address);
        if let Some(p) = population {
            self.population = p;
        }
    }

    /// An event of the current system, or of one the fold hasn't seen arriving in.
    fn here(&mut self, address: Option<i64>) {
        if address.is_some() && self.address.is_some() && address != self.address {
            *self = Self::default();
        }
        self.address = self.address.or(address);
    }

    /// The body `name`, taking over what was kept under its id before a line named it.
    fn body(&mut self, name: &str, id: Option<i64>) -> &mut Body {
        if let Some(early) = id.and_then(|id| self.bodies.remove(&format!("#{id}"))) {
            let b = self.bodies.entry(name.to_string()).or_default();
            b.sampled.extend(early.sampled);
        }
        let b = self.bodies.entry(name.to_string()).or_default();
        b.body_id = id.or(b.body_id);
        b
    }

    pub fn apply(&mut self, line: &JournalLine) {
        self.apply_event(&line.event);
    }

    /// Fold an event (the game state's reducer has no line for some).
    pub fn apply_event(&mut self, event: &J) {
        match event {
            J::FsdJump { star_system, system_address, population, .. } => {
                self.enter(Some(star_system), *system_address, Some(*population))
            }
            J::Location { star_system, system_address, population, .. } => {
                self.enter(Some(star_system), *system_address, Some(*population))
            }
            J::Extra(x) => match x {
                X::FssDiscoveryScan { body_count, system_address, system_name, .. } => {
                    self.here(*system_address);
                    if self.system.is_none() {
                        self.system = system_name.clone();
                    }
                    self.body_count = Some(*body_count);
                }
                X::FssAllBodiesFound { system_address, .. } => {
                    self.here(*system_address);
                    self.all_found = true;
                }
                X::Scan(b) => {
                    let is_body = b.star_type.is_some() || b.planet_class.as_deref().is_some_and(|c| !c.is_empty());
                    if !is_body {
                        return;
                    }
                    self.here(b.system_address);
                    if self.system.is_none() {
                        self.system = b.star_system.clone();
                    }
                    let body = self.body(&b.body_name, b.body_id);
                    body.scan = Some((**b).clone());
                }
                X::FssBodySignals(s) | X::SaaSignalsFound(s) => {
                    self.here(s.system_address);
                    let body = self.body(&s.body_name, s.body_id);
                    if !s.genuses.is_empty() {
                        body.genuses = s.genuses.clone();
                    }
                    let count = |kind: &str| s.signals.iter().filter(|c| c.kind.contains(kind)).map(|c| c.count).sum();
                    body.bio = count("Biological");
                    body.geo = count("Geological");
                }
                X::SaaScanComplete { body_name, system_address, probes_used, efficiency_target, .. } => {
                    self.here(*system_address);
                    let efficient = *efficiency_target > 0 && probes_used <= efficiency_target;
                    self.bodies.entry(body_name.clone()).or_default().mapped = Some(efficient);
                }
                X::ScanOrganic {
                    scan_type, genus, species, species_localised, body: Some(id), system_address, ..
                } => {
                    self.here(*system_address);
                    // The line names the body by id only; a body the fold hasn't seen is kept
                    // under its id until a scan or signal names it.
                    let key = self
                        .bodies
                        .iter()
                        .find(|(_, b)| b.body_id == Some(*id))
                        .map(|(k, _)| k.clone())
                        .unwrap_or_else(|| format!("#{id}"));
                    let body = self.bodies.entry(key).or_default();
                    body.body_id = Some(*id);
                    body.sampled.retain(|s| !s.genus.eq_ignore_ascii_case(genus));
                    body.sampled.push(Sampled {
                        genus: genus.clone(),
                        species: species.clone(),
                        name: species_localised.clone(),
                        analysed: scan_type == "Analyse",
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }

    /// Take in what an earlier visit to this system left (`earlier`: the stored lines of this
    /// system's address folded in game-time order; astromech-0n4h). The game doesn't scan a body
    /// again on a return, so the bodies this fold hasn't heard of come from there, and what it
    /// lacks of the ones it has (mapping, signals, samples). What this fold knows wins. Another
    /// system's fold is ignored.
    pub fn absorb(&mut self, earlier: SystemFold) {
        if earlier.address.is_none() || earlier.address != self.address {
            return;
        }
        self.body_count = self.body_count.or(earlier.body_count);
        self.all_found |= earlier.all_found;
        if self.system.is_none() {
            self.system = earlier.system;
        }
        for (name, old) in earlier.bodies {
            // Samples named only by body id never found their body: leave them.
            if name.starts_with('#') {
                continue;
            }
            let b = self.body(&name, old.body_id);
            b.scan = b.scan.take().or(old.scan);
            b.mapped = b.mapped.or(old.mapped);
            if b.bio == 0 && b.geo == 0 {
                (b.bio, b.geo) = (old.bio, old.geo);
            }
            if b.genuses.is_empty() {
                b.genuses = old.genuses;
            }
            for s in old.sampled {
                if !b.sampled.iter().any(|t| t.genus.eq_ignore_ascii_case(&s.genus)) {
                    b.sampled.push(s);
                }
            }
        }
    }

    pub fn system(&self) -> Option<&str> {
        self.system.as_deref()
    }

    pub fn system_address(&self) -> Option<i64> {
        self.address
    }

    /// Every body the fold has heard of, scanned or not.
    pub fn records(&self) -> impl Iterator<Item = BodyRecord<'_>> {
        self.bodies.iter().map(|(name, b)| BodyRecord {
            name: b.scan.as_ref().map_or(name.as_str(), |s| s.body_name.as_str()),
            body_id: b.body_id,
            scan: b.scan.as_ref(),
            bio_signals: b.bio,
            genuses: &b.genuses,
            sampled: &b.sampled,
        })
    }

    pub fn view(&self) -> SystemView {
        let mut bodies: Vec<BodyView> =
            self.bodies.values().filter_map(|b| body_view(b, self.system.as_deref(), self.population)).collect();
        bodies.sort_by(|a, b| {
            a.body_id.unwrap_or(i64::MAX).cmp(&b.body_id.unwrap_or(i64::MAX)).then(a.name.cmp(&b.name))
        });
        let value = bodies.iter().map(|b| b.value).sum();
        let value_if_mapped = bodies.iter().map(|b| if b.worth_mapping { b.value_mapped } else { b.value }).sum();
        SystemView {
            system: self.system.clone(),
            system_address: self.address,
            population: self.population,
            body_count: self.body_count,
            all_found: self.all_found,
            bodies,
            value,
            value_if_mapped,
        }
    }
}

fn body_view(b: &Body, system: Option<&str>, population: i64) -> Option<BodyView> {
    let s = b.scan.as_ref()?;
    let (value, value_mapped) = value::estimate(s, b.mapped)?;
    let star = s.star_type.is_some();
    let class = s.star_type.clone().or_else(|| s.planet_class.clone()).unwrap_or_default();
    let terraformable = !star && value::terraformable(s);
    let short_name = system
        .and_then(|sys| s.body_name.strip_prefix(sys))
        .and_then(|rest| rest.strip_prefix(' '))
        .filter(|r| !r.is_empty())
        .unwrap_or(&s.body_name)
        .to_string();
    let first_discovery = s.was_discovered == Some(false);
    Some(BodyView {
        name: s.body_name.clone(),
        short_name,
        body_id: s.body_id,
        star,
        class_name: class_name(&class, s.subclass),
        class,
        terraformable,
        landable: s.landable,
        rings: s.ring_count > 0,
        belts: s.belt_count > 0,
        gravity_g: if star { None } else { s.surface_gravity.map(|g| g / G) },
        distance_ls: s.distance_from_arrival_ls,
        was_discovered: s.was_discovered,
        was_mapped: s.was_mapped,
        was_footfalled: s.was_footfalled,
        mapped: b.mapped.is_some(),
        mapped_efficiently: b.mapped == Some(true),
        bio_signals: b.bio,
        geo_signals: b.geo,
        value,
        value_mapped,
        first_discovery,
        worth_mapping: !star && b.mapped.is_none() && value_mapped >= value::WORTH_MAPPING,
        first_footfall_likely: s.landable && first_discovery && population == 0 && s.was_footfalled != Some(true),
        landings: 0,
    })
}

/// How a journal class reads: "Earth-like world", "Class II gas giant", "K star", "White dwarf
/// (DA)". Unknown classes read as the journal wrote them.
pub fn class_name(class: &str, subclass: Option<i64>) -> String {
    let fixed = match class {
        "Metal rich body" => "Metal-rich body",
        "High metal content body" => "High metal content world",
        "Rocky ice body" => "Rocky ice world",
        "Earthlike body" => "Earth-like world",
        "Gas giant with water based life" => "Gas giant with water-based life",
        "Gas giant with ammonia based life" => "Gas giant with ammonia-based life",
        "Sudarsky class I gas giant" => "Class I gas giant",
        "Sudarsky class II gas giant" => "Class II gas giant",
        "Sudarsky class III gas giant" => "Class III gas giant",
        "Sudarsky class IV gas giant" => "Class IV gas giant",
        "Sudarsky class V gas giant" => "Class V gas giant",
        "Helium rich gas giant" => "Helium-rich gas giant",
        "N" => "Neutron star",
        "H" => "Black hole",
        "SupermassiveBlackHole" => "Supermassive black hole",
        "TTS" => "T Tauri star",
        "AeBe" => "Herbig Ae/Be star",
        "A_BlueWhiteSuperGiant" => "A supergiant",
        "B_BlueWhiteSuperGiant" => "B supergiant",
        "F_WhiteSuperGiant" => "F supergiant",
        "G_WhiteSuperGiant" => "G supergiant",
        "M_RedSuperGiant" => "M supergiant",
        "M_RedGiant" => "M giant",
        "K_OrangeGiant" => "K giant",
        "X" => "Exotic star",
        "RoguePlanet" => "Rogue planet",
        "Nebula" | "StellarRemnantNebula" => "Nebula",
        "L" | "T" | "Y" => return format!("Brown dwarf ({class})"),
        "O" | "B" | "A" | "F" | "G" | "K" | "M" => {
            return format!("{class}{} star", subclass.map(|s| s.to_string()).unwrap_or_default())
        }
        c if value::is_white_dwarf(c) => return format!("White dwarf ({c})"),
        // Star types are short codes; planet classes are words ("Water world").
        c if c.starts_with('W') && c.len() <= 3 => "Wolf-Rayet star",
        "MS" | "S" => "S-type star",
        c if c.starts_with('C') && c.len() <= 3 => "Carbon star",
        c => c,
    };
    fixed.to_string()
}

/// The two newest journals, which the system view is folded from (a continuation part may start
/// mid-system).
fn view_logs(dir: &Path) -> Vec<std::path::PathBuf> {
    let logs = astromech_journal::journal_logs(dir);
    let skip = logs.len().saturating_sub(2);
    logs.into_iter().skip(skip).collect()
}

/// The fold of the current system from the newest journal and the one before it.
pub fn fold_from_journal_dir(dir: &Path) -> SystemFold {
    let mut f = SystemFold::default();
    for p in view_logs(dir) {
        for l in astromech_journal::parse_file(&p) {
            f.apply(&l);
        }
    }
    f
}

/// The current system from the newest journal and the one before it.
pub fn from_journal_dir(dir: &Path) -> SystemView {
    fold_from_journal_dir(dir).view()
}

/// [`fold_from_journal_dir`] remembered between calls (astromech-dt5h): the Exploration page
/// polls every few seconds, and the journals only change while the game writes. The fold is
/// reused until one of the two journals is added, removed or grows. It is the fold, not its
/// view, that is kept: the earlier visits' bodies are added from the history on every poll
/// (`History::system_view`, astromech-25c5). Lengths are read through an open
/// handle because on Windows a directory listing can lag appends to a file the game holds open
/// (astromech-txzq).
#[derive(Debug, Default)]
pub struct SystemViewCache {
    last: Option<(Vec<(std::path::PathBuf, u64)>, SystemFold)>,
    parses: usize,
}

impl SystemViewCache {
    pub fn fold(&mut self, dir: &Path) -> SystemFold {
        let sig: Vec<(std::path::PathBuf, u64)> = view_logs(dir)
            .into_iter()
            .map(|p| {
                let len = std::fs::File::open(&p).and_then(|f| f.metadata()).map_or(0, |m| m.len());
                (p, len)
            })
            .collect();
        match &self.last {
            Some((s, v)) if *s == sig => v.clone(),
            _ => {
                self.parses += 1;
                let v = fold_from_journal_dir(dir);
                self.last = Some((sig, v.clone()));
                v
            }
        }
    }

    /// How many times the journals were parsed (for tests).
    pub fn parses(&self) -> usize {
        self.parses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unchanged_journal_is_not_parsed_again() {
        let d = std::env::temp_dir().join(format!("astromech-viewcache-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        let log = d.join("Journal.2026-10-01T090000.01.log");
        let jump = r#"{"timestamp":"2026-10-01T09:00:00Z","event":"FSDJump","StarSystem":"Alpha","SystemAddress":1,"StarPos":[0,0,0],"JumpDist":9.5}"#;
        std::fs::write(&log, format!("{jump}\n")).unwrap();
        let mut c = SystemViewCache::default();
        let first = c.fold(&d).view();
        assert_eq!(first.system.as_deref(), Some("Alpha"));
        assert_eq!(c.fold(&d).view(), first);
        assert_eq!(c.fold(&d).view(), first);
        assert_eq!(c.parses(), 1, "polling an unchanged journal re-parsed it");
        // Growth, or a newer file, is parsed.
        let jump2 = jump.replace("Alpha", "Beta").replace("09:00:00", "09:05:00");
        std::fs::write(&log, format!("{jump}\n{jump2}\n")).unwrap();
        assert_eq!(c.fold(&d).view().system.as_deref(), Some("Beta"));
        assert_eq!(c.parses(), 2);
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn class_names() {
        assert_eq!(class_name("Earthlike body", None), "Earth-like world");
        assert_eq!(class_name("Sudarsky class II gas giant", None), "Class II gas giant");
        assert_eq!(class_name("G", Some(2)), "G2 star");
        assert_eq!(class_name("DAB", Some(5)), "White dwarf (DAB)");
        assert_eq!(class_name("WNC", None), "Wolf-Rayet star");
        assert_eq!(class_name("CHd", None), "Carbon star");
        assert_eq!(class_name("Water world", None), "Water world");
        assert_eq!(class_name("Water giant", None), "Water giant");
        assert_eq!(class_name("Y", None), "Brown dwarf (Y)");
        assert_eq!(class_name("Something new", None), "Something new");
    }
}
