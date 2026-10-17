//! Raw-material sites near a point: crystalline shards, brain trees, crashed ships and
//! geological sites from Spansh's bodies search (codex landmarks and FSS signals, through
//! `astromech-data`), and the well-known grade 4 farms from `data/catalog/raw-farms.json`.
//! Spansh ignores filters it doesn't know, so every body is re-checked in [`SiteKind::matches`].

use crate::{Facts, Filters, Here, Hop};
use astromech_data::spansh::{Body, Reference, Search};
use astromech_materials::{catalog, Kind};
use serde::Serialize;
use serde_json::json;

/// Brain tree codex subtypes (Spansh `landmark_subtype` values, 16 Oct 2026).
const BRAIN_TREES: &[&str] = &[
    "Aureum Brain Tree",
    "Gypseeum Brain Tree",
    "Lindigoticum Brain Tree",
    "Lividum Brain Tree",
    "Ostrinum Brain Tree",
    "Puniceum Brain Tree",
    "Roseum Brain Tree",
    "Viride Brain Tree",
];

/// Landmark types that are geological sites (shown on a geological body's line).
const GEOLOGY: &[&str] = &["Fumarole", "Geyser", "Lava Spout", "Gas Vent"];

/// Where a kind of site comes from.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// Bodies with a codex landmark of one of these subtypes.
    Landmarks(&'static [&'static str]),
    /// Landable bodies with this FSS signal.
    Signal(&'static str),
    /// `data/catalog/raw-farms.json`.
    Farms,
}

/// One kind of site the lookup offers.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SiteKind {
    pub id: &'static str,
    pub label: &'static str,
    /// Spoken after "Nearest", e.g. "crystalline shards".
    pub noun: &'static str,
    /// What farming it gives, one line.
    pub yields: &'static str,
    #[serde(skip)]
    pub source: Source,
}

pub const SITE_KINDS: &[SiteKind] = &[
    SiteKind {
        id: "crystalline_shards",
        label: "Crystalline shards",
        noun: "crystalline shards",
        yields: "Grade 4 raw (polonium, ruthenium, tellurium, technetium, yttrium, antimony and more). Shoot the shards from the SRV and collect the fragments.",
        source: Source::Landmarks(&["Crystalline Shards"]),
    },
    SiteKind {
        id: "brain_trees",
        label: "Brain trees",
        noun: "brain trees",
        yields: "Grade 4 raw. Shoot them (turrets or flak), then collect with the SRV.",
        source: Source::Landmarks(BRAIN_TREES),
    },
    SiteKind {
        id: "crashed_ships",
        label: "Crashed ships",
        noun: "crashed ship",
        yields: "Encoded data points and salvage. Scan the data points from the SRV.",
        source: Source::Landmarks(&["Crashed Ship", "Wrecked Ship"]),
    },
    SiteKind {
        id: "geological",
        label: "Geological sites",
        noun: "geological sites",
        yields: "Grade 1 to 3 raw from the outcrops around geysers, fumaroles and vents.",
        source: Source::Signal("Geological"),
    },
    SiteKind {
        id: "raw_farms",
        label: "Known raw farms",
        noun: "raw material farm",
        yields: "The well-known grade 4 farms: one body for each element.",
        source: Source::Farms,
    },
];

pub fn site_kind(id: &str) -> Option<&'static SiteKind> {
    SITE_KINDS.iter().find(|k| k.id == id)
}

impl SiteKind {
    /// The bodies search for this kind, nearest to `origin` first. `None` for the farms list.
    pub fn search(&self, origin: [f64; 3], size: usize, filters: &Filters, now_s: i64) -> Option<Search> {
        let mut s = Search::near(Reference::Coords(origin)).size(size);
        if let Some(since) = filters.updated_since(now_s) {
            s = s.updated_since("updated_at", &since);
        }
        match self.source {
            Source::Landmarks(subtypes) => Some(s.filter("landmark_subtype", json!({ "value": subtypes }))),
            Source::Signal(name) => {
                Some(s.filter("signals", json!([{ "name": [name] }])).filter("is_landable", json!({ "value": true })))
            }
            Source::Farms => None,
        }
    }

    pub fn matches(&self, b: &Body) -> bool {
        match self.source {
            Source::Landmarks(subtypes) => b.landmarks.iter().any(|l| subtypes.contains(&l.subtype.as_str())),
            Source::Signal(name) => b.has_signal(name) && b.is_landable != Some(false),
            Source::Farms => false,
        }
    }

    /// The body's line: which sites (and where the first is), and its rarest raw materials.
    fn detail(&self, b: &Body) -> String {
        let mut parts = vec![];
        let lms: Vec<_> = match self.source {
            Source::Landmarks(subtypes) => {
                b.landmarks.iter().filter(|l| subtypes.contains(&l.subtype.as_str())).collect()
            }
            _ => b.landmarks.iter().filter(|l| GEOLOGY.contains(&l.landmark_type.as_str())).collect(),
        };
        if let Source::Signal(name) = self.source {
            let n = b.signals.iter().find(|s| s.name.eq_ignore_ascii_case(name)).map_or(0, |s| s.count);
            parts.push(format!("{n} {} signal{}", name.to_lowercase(), if n == 1 { "" } else { "s" }));
        }
        let mut names: Vec<&str> = vec![];
        for l in &lms {
            if !names.contains(&l.subtype.as_str()) {
                names.push(&l.subtype);
            }
        }
        if !names.is_empty() {
            parts.push(names.join(", "));
        }
        // Older codex entries come as 0, 0: no position.
        let placed = lms.iter().filter_map(|l| Some((l.latitude?, l.longitude?))).find(|p| *p != (0.0, 0.0));
        if let Some((lat, lon)) = placed {
            parts.push(format!("first at {lat:.1}, {lon:.1}"));
        }
        let rare = rare_raw(b);
        if !rare.is_empty() {
            parts.push(rare);
        }
        parts.join(" · ")
    }
}

/// The body's grade 3–4 raw materials, rarest first: "Polonium 0.5%, Tin 1.1%".
fn rare_raw(b: &Body) -> String {
    let mut found: Vec<(&'static catalog::Spec, f64)> = b
        .materials
        .iter()
        .filter_map(|m| {
            let s = catalog::lookup(&m.name.to_lowercase())?;
            (s.kind == Kind::Raw && s.grade >= 3).then_some((s, m.share))
        })
        .collect();
    found.sort_by(|a, b| b.0.grade.cmp(&a.0.grade).then(b.1.total_cmp(&a.1)));
    found.iter().take(3).map(|(s, share)| format!("{} {share:.1}%", s.name)).collect::<Vec<_>>().join(", ")
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SiteHit {
    pub system: String,
    pub id64: i64,
    /// Full body name, e.g. "HIP 36601 C 1 a".
    pub body: String,
    #[serde(flatten)]
    pub hop: Hop,
    /// Light seconds from the arrival star.
    pub arrival_ls: Option<f64>,
    pub updated_at: Option<String>,
    pub detail: String,
    /// Known farms: what the body is farmed for, spoken after its name ("polonium from
    /// crystalline shards"), so the four HIP 36601 farms sound different.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub farm: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Sites {
    pub kind: SiteKind,
    pub here: Here,
    pub range: f64,
    /// Who answered: "Spansh", or "astromech" for the bundled farms list.
    pub source: &'static str,
    /// When the source was asked (ISO 8601; may be a cached answer).
    pub fetched: String,
    pub hits: Vec<SiteHit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

/// A candidate before its jump count: system, id64, position, body, arrival, age, detail.
pub(crate) struct Candidate {
    pub system: String,
    pub id64: i64,
    pub pos: [f64; 3],
    pub body: String,
    pub arrival_ls: Option<f64>,
    pub updated_at: Option<String>,
    pub detail: String,
    pub farm: Option<String>,
}

impl Candidate {
    pub(crate) fn facts(&self) -> Facts<'_> {
        Facts { arrival_ls: self.arrival_ls, updated_at: self.updated_at.as_deref(), ..Facts::default() }
    }
}

/// Spansh bodies that really are `kind`, as candidates.
pub(crate) fn from_bodies(kind: &SiteKind, bodies: &[Body]) -> Vec<Candidate> {
    bodies
        .iter()
        .filter(|b| kind.matches(b))
        .map(|b| Candidate {
            system: b.system_name.clone(),
            id64: b.system_id64,
            pos: b.pos(),
            body: b.name.clone(),
            arrival_ls: b.distance_to_arrival,
            updated_at: b.updated_at.clone(),
            detail: kind.detail(b),
            farm: None,
        })
        .collect()
}

/// The bundled farms, nearest to `origin` first.
pub(crate) fn from_farms(origin: [f64; 3]) -> Vec<Candidate> {
    let mut out: Vec<Candidate> = astromech_materials::farm::farms()
        .iter()
        .map(|f| {
            let name = catalog::lookup(&f.material).map_or(f.material.as_str(), |s| s.name);
            Candidate {
                system: f.system.clone(),
                id64: f.id64,
                pos: f.pos,
                body: f.body_name(),
                arrival_ls: Some(f.arrival_ls),
                updated_at: Some(astromech_materials::farm::farms_checked().to_string()),
                detail: format!("{name} · {}", f.site.to_lowercase()),
                farm: Some(format!("{} from {}", name.to_lowercase(), f.site.to_lowercase())),
            }
        })
        .collect();
    out.sort_by(|a, b| crate::dist(origin, a.pos).total_cmp(&crate::dist(origin, b.pos)));
    out
}

/// Keep the candidates `filters` allow.
pub(crate) fn keep(c: Vec<Candidate>, filters: &Filters, now_s: i64) -> Vec<Candidate> {
    c.into_iter().filter(|c| filters.keeps(&c.facts(), now_s)).collect()
}

fn thousands(n: f64) -> String {
    let s = format!("{}", n.round() as i64);
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

/// The spoken answer: the nearest site, its distance, jumps and how far it is from the arrival
/// star (long supercruise is worth knowing before setting off).
pub fn spoken(s: &Sites) -> String {
    let Some(h) = s.hits.first() else {
        return format!("No {} found near {}.", s.kind.noun, s.here.system);
    };
    let jumps = format!(
        "{}{} jump{}",
        if h.hop.exact { "" } else { "about " },
        h.hop.jumps,
        if h.hop.jumps == 1 { "" } else { "s" }
    );
    let body = match &h.farm {
        Some(what) => format!("{}, {what}", h.body),
        None => h.body.clone(),
    };
    let mut line = if h.hop.ly < 0.05 {
        format!("Nearest {}: {body}, in this system", s.kind.noun)
    } else {
        let ly = if h.hop.ly < 100.0 { format!("{:.1}", h.hop.ly) } else { thousands(h.hop.ly) };
        format!("Nearest {}: {body}, {ly} light years, {jumps}", s.kind.noun)
    };
    if let Some(ls) = h.arrival_ls {
        line.push_str(&format!(", {} light seconds from the arrival star", thousands(ls)));
    }
    line.push('.');
    line
}
