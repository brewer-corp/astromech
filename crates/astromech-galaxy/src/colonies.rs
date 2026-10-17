//! Colonisation candidates near a point: unpopulated systems nobody has claimed that lie within
//! [`CLAIM_RANGE_LY`] of a populated system (the game's rule: a claim is made at a populated
//! system's Colonisation Contact for a target within 15 ly), with what their bodies offer.
//!
//! Spansh's systems search (filter shapes in `astromech_data::spansh`) gives the unpopulated
//! systems and their claim flags. Its `nearest_populated_distance` lags new colonies, so it only
//! settles a system when it is within range; any other system is measured against the populated
//! systems around the search by coordinates. Rings, landable bodies and the arrival star come
//! from the bodies search for the nearest few. Every hit is re-checked (Spansh ignores what it
//! doesn't know).

use crate::{dist, Facts, Filters, Galaxy, Here, Hop};
use anyhow::Result;
use astromech_data::spansh::{Body, Page, Reference, Search, System};
use serde::{Deserialize, Serialize};

/// How far from a populated system a colony may be claimed, in light years.
pub const CLAIM_RANGE_LY: f64 = 15.0;
/// Search radius when the commander hasn't picked one, in light years.
pub const DEFAULT_RADIUS_LY: u32 = 30;
/// Unpopulated systems asked of Spansh, nearest first.
const CANDIDATES: usize = 100;
/// Populated systems asked of Spansh (its largest page).
const POPULATED: usize = 500;
/// Systems whose bodies are looked up (nearest first); the interest filter picks from these.
const DETAILED: usize = 40;
/// Pages of bodies asked for at most.
const BODY_PAGES: usize = 3;
const MAX_POPULATION: i64 = 100_000_000_000;

/// What the commander wants in the system.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Interest {
    #[default]
    Any,
    /// A ringed body.
    Rings,
    /// An Earth-like world.
    EarthLike,
    /// A landable body.
    Landable,
}

impl Interest {
    pub fn matches(self, b: &Bodies) -> bool {
        match self {
            Interest::Any => true,
            Interest::Rings => b.ringed > 0,
            Interest::EarthLike => b.earth_like > 0,
            Interest::Landable => b.landable > 0,
        }
    }
}

/// The populated system that puts a candidate in range.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Populated {
    /// `None` when only Spansh's precomputed distance is known.
    pub name: Option<String>,
    pub id64: Option<i64>,
    pub ly: f64,
}

/// What a system's bodies offer, as far as Spansh knows them.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Bodies {
    /// Bodies Spansh knows (0: the system hasn't been scanned).
    pub known: u32,
    /// The arrival star's class, e.g. "K (Yellow-Orange) star".
    pub main_star: Option<String>,
    pub earth_like: u32,
    pub water_worlds: u32,
    pub ammonia_worlds: u32,
    /// Bodies with rings.
    pub ringed: u32,
    pub landable: u32,
    pub terraformable: u32,
}

impl Bodies {
    pub fn of(bodies: &[&Body]) -> Self {
        let mut s = Bodies { known: bodies.len() as u32, ..Bodies::default() };
        for b in bodies {
            let sub = b.subtype.as_deref().unwrap_or("");
            if b.body_type == "Star" && b.is_main_star == Some(true) {
                s.main_star = Some(match sub.strip_suffix(" Star") {
                    Some(class) => format!("{class} star"),
                    None => sub.to_string(),
                });
            }
            match sub {
                "Earth-like world" => s.earth_like += 1,
                "Water world" => s.water_worlds += 1,
                "Ammonia world" => s.ammonia_worlds += 1,
                _ => {}
            }
            s.ringed += u32::from(!b.rings.is_empty());
            s.landable += u32::from(b.is_landable == Some(true));
            s.terraformable += u32::from(b.terraforming_state.as_deref() == Some("Candidate for terraforming"));
        }
        s
    }

    /// "K (Yellow-Orange) star · 1 Earth-like world · 2 ringed · 9 landable".
    pub fn line(&self) -> String {
        if self.known == 0 {
            return "bodies not on Spansh".into();
        }
        let mut parts: Vec<String> = self.main_star.iter().cloned().collect();
        for (n, one, many) in [
            (self.earth_like, "Earth-like world", "Earth-like worlds"),
            (self.water_worlds, "water world", "water worlds"),
            (self.ammonia_worlds, "ammonia world", "ammonia worlds"),
        ] {
            if n > 0 {
                parts.push(format!("{n} {}", if n == 1 { one } else { many }));
            }
        }
        for (n, what) in [(self.ringed, "ringed"), (self.landable, "landable"), (self.terraformable, "terraformable")] {
            if n > 0 {
                parts.push(format!("{n} {what}"));
            }
        }
        if parts.is_empty() {
            parts.push(format!("{} bod{}", self.known, if self.known == 1 { "y" } else { "ies" }));
        }
        parts.join(" · ")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CandidateHit {
    pub name: String,
    pub id64: i64,
    #[serde(flatten)]
    pub hop: Hop,
    pub populated: Populated,
    pub bodies: Bodies,
    pub updated_at: Option<String>,
    /// The bodies' line, then the populated system in range.
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Candidates {
    pub interest: Interest,
    pub here: Here,
    pub range: f64,
    pub radius_ly: f64,
    /// When Spansh was asked (may be a cached answer).
    pub fetched: String,
    /// Nearest first.
    pub hits: Vec<CandidateHit>,
    /// Unclaimed systems that couldn't be checked against every populated system nearby.
    pub unchecked: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

/// Unpopulated systems within `radius` ly of `origin`, nearest first.
pub fn candidate_search(origin: [f64; 3], radius: f64) -> Search {
    Search::near(Reference::Coords(origin)).within(radius).population(0, 0).size(CANDIDATES)
}

/// Populated systems that could put a candidate within `radius` ly of `origin` in range.
pub fn populated_search(origin: [f64; 3], radius: f64) -> Search {
    Search::near(Reference::Coords(origin))
        .within(radius + CLAIM_RANGE_LY)
        .population(1, MAX_POPULATION)
        .size(POPULATED)
}

/// Unpopulated and unclaimed, as far as Spansh knows.
pub fn unclaimed(s: &System) -> bool {
    s.population == 0 && !s.is_claimed()
}

/// Spansh's own nearest populated system, when it is in range.
fn by_spansh(s: &System) -> Option<Populated> {
    let ly = s.nearest_populated_distance.filter(|ly| *ly <= CLAIM_RANGE_LY)?;
    Some(Populated { name: None, id64: s.nearest_populated_id64, ly })
}

/// The nearest of `populated` to `s` in range, or Spansh's own when nearer (or the only one).
pub fn in_range(s: &System, populated: &[System]) -> Option<Populated> {
    let measured = populated
        .iter()
        .filter(|p| p.population > 0)
        .map(|p| (dist(s.pos(), p.pos()), p))
        .filter(|(ly, _)| *ly <= CLAIM_RANGE_LY)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(ly, p)| Populated { name: Some(p.name.clone()), id64: Some(p.id64), ly });
    match (measured, by_spansh(s)) {
        (Some(m), Some(sp)) if sp.ly + 0.01 < m.ly => Some(sp),
        (Some(m), _) => Some(m),
        (None, sp) => sp,
    }
}

impl Galaxy {
    /// Unclaimed systems within [`CLAIM_RANGE_LY`] of a populated system and within the
    /// filters' radius (default [`DEFAULT_RADIUS_LY`]) of `here`, with a body of `interest`,
    /// nearest first, with jump counts at `range`.
    pub fn colony_candidates(
        &self,
        interest: Interest,
        here: &Here,
        range: f64,
        limit: usize,
        filters: &Filters,
    ) -> Result<Candidates> {
        // Fill-in is best effort: without it the jump counts are estimates.
        let _ = self.ensure_covered(here.pos);
        let radius = filters.max_distance_ly.unwrap_or(DEFAULT_RADIUS_LY) as f64;
        let page = self.remote.spansh_systems(&candidate_search(here.pos, radius))?;
        let fetched = page.fetched_at;
        // Every system claimed is an answer; no unpopulated system at all means the filter was ignored.
        let ignored = !page.value.results.is_empty() && page.value.results.iter().all(|s| s.population > 0);
        let open: Vec<System> = page.value.results.into_iter().filter(unclaimed).collect();
        let mut warning =
            ignored.then(|| "Spansh returned only populated systems; its search filter may have changed.".to_string());

        // Only fetch the populated systems when Spansh's distances don't settle every system.
        let populated: Option<Page<System>> = if open.iter().all(|s| by_spansh(s).is_some()) {
            None
        } else {
            Some(self.remote.spansh_systems(&populated_search(here.pos, radius))?.value)
        };
        let all_populated = populated.as_ref().is_none_or(|p| p.count as usize <= p.results.len());
        let mut unchecked = 0;
        let now = crate::now_s();
        let mut found: Vec<(System, Populated)> = vec![];
        for s in open {
            match in_range(&s, populated.as_ref().map_or(&[], |p| &p.results)) {
                Some(p) => found.push((s, p)),
                None if !all_populated => unchecked += 1,
                None => {}
            }
        }
        found.retain(|(s, _)| filters.keeps(&Facts { updated_at: s.updated_at.as_deref(), ..Facts::default() }, now));
        found.sort_by(|a, b| dist(here.pos, a.0.pos()).total_cmp(&dist(here.pos, b.0.pos())));
        found.truncate(DETAILED);
        if unchecked > 0 {
            warning = Some(format!(
                "More than {POPULATED} populated systems around: {unchecked} unclaimed system{} couldn't be checked. Try a smaller radius.",
                if unchecked == 1 { "" } else { "s" }
            ));
        }

        let bodies = self.bodies_of(here.pos, &found)?;
        {
            let mut st = self.store();
            let _ = st.insert_systems(found.iter().map(|(s, _)| (s.id64, s.name.as_str(), s.pos())));
        }
        let map = self.map().ok();
        let mut hits: Vec<CandidateHit> = found
            .into_iter()
            .filter_map(|(s, populated)| {
                let mine: Vec<&Body> = bodies.iter().filter(|b| b.system_id64 == s.id64).collect();
                let summary = Bodies::of(&mine);
                if !interest.matches(&summary) {
                    return None;
                }
                let from = match &populated.name {
                    Some(name) => format!("{:.1} ly from {name}", populated.ly),
                    None => format!("{:.1} ly from a populated system", populated.ly),
                };
                Some(CandidateHit {
                    hop: crate::hop_on(map.as_deref(), here.pos, s.pos(), range),
                    detail: format!("{} · {from}", summary.line()),
                    name: s.name,
                    id64: s.id64,
                    populated,
                    bodies: summary,
                    updated_at: s.updated_at,
                })
            })
            .collect();
        hits.sort_by(|a, b| a.hop.ly.total_cmp(&b.hop.ly));
        hits.truncate(limit.max(1));
        Ok(Candidates {
            interest,
            here: here.clone(),
            range,
            radius_ly: radius,
            fetched: crate::iso(fetched),
            hits,
            unchecked,
            warning,
        })
    }

    /// Every body Spansh knows in these systems (a few pages at most).
    fn bodies_of(&self, origin: [f64; 3], systems: &[(System, Populated)]) -> Result<Vec<Body>> {
        if systems.is_empty() {
            return Ok(vec![]);
        }
        let ids: Vec<i64> = systems.iter().map(|(s, _)| s.id64).collect();
        let mut out = vec![];
        for page in 0..BODY_PAGES {
            let q = Search::near(Reference::Coords(origin)).in_systems(&ids).size(POPULATED).page(page);
            let p = self.remote.spansh_bodies(&q)?.value;
            let done = p.results.len() < POPULATED || out.len() + p.results.len() >= p.count as usize;
            out.extend(p.results.into_iter().filter(|b| ids.contains(&b.system_id64)));
            if done {
                break;
            }
        }
        Ok(out)
    }
}
