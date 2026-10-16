//! # astromech-galaxy
//!
//! Where things are and how many jumps away. A local SQLite star map, seeded from a bundled
//! preload (500 ly around Sol, `scripts/gen-starmap.py`) and filled in from EDSM as the
//! commander travels; live faction states from Spansh's system search; jump counts by A* over
//! the known stars at the ship's range.

pub mod filters;
pub mod map;
pub mod remote;
pub mod store;

pub use filters::{Facts, Filters, Pad};
pub use map::{estimate, Hop, StarMap};
pub use remote::{query, Faction, Hit, Query, QUERIES};
pub use store::{dist, Sphere, Star, Store};

use anyhow::Result;
use astromech_journal::JournalEvent;
use serde::Serialize;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

/// Radius fetched from EDSM when the commander is outside covered space.
pub const FILL_RADIUS: f64 = 50.0;
/// How long a faction-state search is reused.
pub const STATE_TTL_S: i64 = 3600;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Here {
    pub system: String,
    pub id64: Option<i64>,
    pub pos: [f64; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NearbyHit {
    pub name: String,
    pub id64: i64,
    #[serde(flatten)]
    pub hop: Hop,
    pub population: i64,
    pub updated_at: Option<String>,
    /// Only the factions that match the query.
    pub factions: Vec<Faction>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Nearby {
    pub query: Query,
    pub here: Here,
    pub range: f64,
    /// When the search was made (cached up to [`STATE_TTL_S`]).
    pub fetched: String,
    pub hits: Vec<NearbyHit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

pub struct Galaxy {
    store: Mutex<Store>,
    remote: astromech_data::Client,
    map: RwLock<Option<Arc<StarMap>>>,
    /// Last failed EDSM fill-in: don't retry for [`FILL_RETRY_S`].
    fill_failed: Mutex<Option<i64>>,
}

/// After an EDSM failure, wait this long before trying again.
pub const FILL_RETRY_S: i64 = 600;

fn now_s() -> i64 {
    chrono::Utc::now().timestamp()
}

fn iso(t: i64) -> String {
    chrono::DateTime::from_timestamp(t, 0).map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default()
}

impl Galaxy {
    /// Open (or create) the store at `path`, seeding it from `preload` once.
    pub fn open(path: &Path, preload: Option<&Path>) -> Result<Self> {
        let mut s = Store::open(path)?;
        if let Some(p) = preload {
            s.seed_from(p)?;
        }
        Ok(Self::with_store(s))
    }

    pub fn with_store(s: Store) -> Self {
        Self::with_remote(s, astromech_data::Client::new(astromech_data::Cache::in_memory().expect("in-memory SQLite")))
    }

    /// `remote` answers the Spansh and EDSM requests (tests pass a recorded transport).
    pub fn with_remote(s: Store, remote: astromech_data::Client) -> Self {
        Galaxy { store: Mutex::new(s), remote, map: RwLock::new(None), fill_failed: Mutex::new(None) }
    }

    fn store(&self) -> std::sync::MutexGuard<'_, Store> {
        self.store.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn map(&self) -> Result<Arc<StarMap>> {
        if let Some(m) = self.map.read().unwrap_or_else(|e| e.into_inner()).as_ref() {
            return Ok(m.clone());
        }
        let (stars, cov) = {
            let s = self.store();
            (s.stars()?, s.coverage()?)
        };
        let m = Arc::new(StarMap::new(stars, cov));
        *self.map.write().unwrap_or_else(|e| e.into_inner()) = Some(m.clone());
        Ok(m)
    }

    fn invalidate(&self) {
        *self.map.write().unwrap_or_else(|e| e.into_inner()) = None;
    }

    pub fn summary(&self) -> Result<(i64, Option<String>)> {
        let s = self.store();
        Ok((s.count()?, s.meta("preload")?))
    }

    pub fn position(&self, system: &str) -> Result<Option<[f64; 3]>> {
        Ok(self.store().system(system)?.map(|x| x.2))
    }

    /// Up to a handful of system names starting with `prefix` (Spansh's autocomplete), for
    /// typing a search origin.
    pub fn system_names(&self, prefix: &str) -> Result<Vec<String>> {
        Ok(self.remote.spansh_system_names(prefix)?.value.into_iter().map(|s| s.name).collect())
    }

    /// Where a typed system is: the local map first, else Spansh (exact name, any case).
    pub fn locate(&self, name: &str) -> Result<Here> {
        let name = name.trim();
        if let Some((id64, system, pos)) = self.store().system(name)? {
            return Ok(Here { system, id64: Some(id64), pos });
        }
        self.remote
            .spansh_system_names(name)?
            .value
            .into_iter()
            .find(|s| s.name.eq_ignore_ascii_case(name))
            .map(|s| Here { pos: s.pos(), system: s.name, id64: Some(s.id64) })
            .ok_or_else(|| anyhow::anyhow!("Spansh doesn't know a system called {name}."))
    }

    /// Fetch the stars around `p` from EDSM if `p` is outside covered space. Returns whether a
    /// fetch happened.
    pub fn ensure_covered(&self, p: [f64; 3]) -> Result<bool> {
        if self.map()?.covered(p) {
            return Ok(false);
        }
        let now = now_s();
        if self.fill_failed.lock().unwrap_or_else(|e| e.into_inner()).is_some_and(|t| now - t < FILL_RETRY_S) {
            return Ok(false);
        }
        let rows = match remote::edsm_sphere(&self.remote, p, FILL_RADIUS) {
            Ok(r) => r,
            Err(e) => {
                *self.fill_failed.lock().unwrap_or_else(|e| e.into_inner()) = Some(now);
                return Err(e);
            }
        };
        // An empty answer is not coverage: EDSM may simply not know the area yet.
        if rows.is_empty() {
            *self.fill_failed.lock().unwrap_or_else(|e| e.into_inner()) = Some(now);
            return Ok(false);
        }
        {
            let mut s = self.store();
            s.insert_systems(rows.iter().map(|(id, n, q)| (*id, n.as_str(), *q)))?;
            s.add_coverage(Sphere { center: p, radius: FILL_RADIUS }, "edsm", &iso(now))?;
        }
        self.invalidate();
        Ok(true)
    }

    pub fn hop(&self, a: [f64; 3], b: [f64; 3], range: f64) -> Result<Hop> {
        Ok(self.map()?.hop(a, b, range))
    }

    /// Systems near `here` matching `query_id` and `filters`, nearest first, with jump counts
    /// at `range`.
    pub fn nearby(&self, query_id: &str, here: &Here, range: f64, limit: usize, filters: &Filters) -> Result<Nearby> {
        let q = query(query_id).ok_or_else(|| anyhow::anyhow!("unknown lookup {query_id}"))?;
        // Fill-in is best effort: without it the jump counts are estimates.
        let _ = self.ensure_covered(here.pos);
        let key = format!(
            "spansh:{}:{:.0}:{:.0}:{:.0}",
            q.id,
            (here.pos[0] / 10.0).round() + 0.0,
            (here.pos[1] / 10.0).round() + 0.0,
            (here.pos[2] / 10.0).round() + 0.0
        );
        let now = now_s();
        let cached = self.store().cache_get(&key, now, STATE_TTL_S)?;
        let (fetched, body) = match cached {
            Some(c) => c,
            None => {
                let b = remote::spansh_search(&self.remote, q, here.pos, 50)?;
                remote::parse_search(&b)?;
                self.store().cache_put(&key, now, &b)?;
                (now, b)
            }
        };
        let hits = remote::parse_search(&body)?
            .into_iter()
            .filter(|h| filters.keeps(&Facts { updated_at: h.updated_at.as_deref(), ..Facts::default() }, now))
            .collect();
        Ok(self.rank(q, here, range, limit, hits, iso(fetched)))
    }

    fn rank(&self, q: &Query, here: &Here, range: f64, limit: usize, hits: Vec<Hit>, fetched: String) -> Nearby {
        let n = hits.len();
        let matched: Vec<Hit> = hits.into_iter().filter(|h| q.matches(h)).collect();
        let warning = (n > 0 && matched.is_empty() && q.min_population == 0).then(|| {
            "Spansh returned systems without the requested state; its search filter may have changed.".to_string()
        });
        {
            let mut s = self.store();
            let _ = s.insert_systems(matched.iter().map(|h| (h.id64, h.name.as_str(), h.pos())));
        }
        let map = self.map().ok();
        let mut out: Vec<NearbyHit> = matched
            .into_iter()
            .take(limit.max(1) * 2)
            .map(|h| {
                let hop = match &map {
                    Some(m) => m.hop(here.pos, h.pos(), range),
                    None => {
                        let ly = dist(here.pos, h.pos());
                        Hop { ly, jumps: estimate(ly, range), exact: false }
                    }
                };
                NearbyHit {
                    name: h.name.clone(),
                    id64: h.id64,
                    hop,
                    population: h.population,
                    updated_at: h.updated_at.clone(),
                    factions: h.minor_faction_presences.iter().filter(|f| q.faction_matches(f)).cloned().collect(),
                }
            })
            .collect();
        out.sort_by(|a, b| a.hop.jumps.cmp(&b.hop.jumps).then(a.hop.ly.total_cmp(&b.hop.ly)));
        out.truncate(limit.max(1));
        Nearby { query: q.clone(), here: here.clone(), range, fetched, hits: out, warning }
    }
}

const POSITION_EVENTS: &[&str] = &["\"Location\"", "\"FSDJump\"", "\"CarrierJump\""];

/// Newest `Location` / `FSDJump` / `CarrierJump` with `StarPos`, newest journal first.
pub fn here_from_journal_dir(dir: &Path) -> Option<Here> {
    let mut logs = astromech_journal::journal_logs(dir);
    logs.reverse();
    for p in logs {
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        for line in text.lines().rev().filter(|l| POSITION_EVENTS.iter().any(|e| l.contains(e))) {
            let Ok(l) = astromech_journal::parse_line(line.trim()) else { continue };
            match l.event {
                JournalEvent::Location { star_system, system_address, star_pos: Some(pos), .. }
                | JournalEvent::FsdJump { star_system, system_address, star_pos: Some(pos), .. } => {
                    return Some(Here { system: star_system, id64: system_address, pos });
                }
                _ => {}
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};

    /// Answers EDSM sphere and Spansh systems search with recorded bodies; logs request URLs.
    struct Recorded {
        sphere: String,
        log: Mutex<Vec<String>>,
    }

    impl Transport for Recorded {
        fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
            self.log.lock().unwrap().push(url.to_string());
            let body = if url.ends_with("/api-v1/sphere-systems") {
                self.sphere.clone()
            } else if url.ends_with("/api/systems/search") {
                include_str!("../tests/fixtures/spansh-famine.json").to_string()
            } else if url.ends_with("/api/systems/field_values/system_names") {
                SHIN.to_string()
            } else {
                panic!("unexpected request {url}")
            };
            Ok(Response { status: 200, body })
        }
    }

    #[test]
    fn nearby_fetches_through_the_data_client_then_reuses_the_cache() {
        // EDSM knows a dense line of stars from Sol towards the fixture systems.
        let sphere: Vec<serde_json::Value> = (0..40)
            .map(|i| {
                serde_json::json!({
                    "name": format!("S{i}"), "id64": 1000 + i,
                    "coords": { "x": i as f64 * 0.5, "y": -(i as f64) * 0.1, "z": i as f64 * 0.3 },
                })
            })
            .collect();
        let rec = Arc::new(Recorded { sphere: serde_json::Value::from(sphere).to_string(), log: Mutex::new(vec![]) });
        let client = Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none());
        let g = Galaxy::with_remote(Store::in_memory().unwrap(), client);
        let here = Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] };

        let n = g.nearby("famine", &here, 5.0, 2, &Filters::default()).unwrap();
        {
            let log = rec.log.lock().unwrap();
            assert_eq!(log.len(), 2, "one EDSM fill-in and one Spansh search: {log:?}");
            assert!(log[0].starts_with("https://www.edsm.net/api-v1/sphere-systems"));
            assert_eq!(log[1], "https://spansh.co.uk/api/systems/search");
        }
        assert_eq!(n.hits.len(), 2);
        assert_eq!(n.hits[0].name, "WISE 0855-0714");
        assert!(n.hits[0].hop.exact, "jump counts use the filled-in map");
        assert!(n.hits[0].hop.jumps >= 2);
        assert!(g.position("S10").unwrap().is_some(), "EDSM systems are stored");

        let again = g.nearby("famine", &here, 5.0, 2, &Filters::default()).unwrap();
        assert_eq!(rec.log.lock().unwrap().len(), 2, "covered and cached: no requests");
        assert_eq!(again.hits, n.hits);
        assert_eq!(again.fetched, n.fetched);
    }

    /// Spansh's autocomplete for "Shin" (recorded 16 Oct 2026, trimmed).
    const SHIN: &str = r#"{"min_max":[{"id64":1733321298650,"name":"Shingon","x":121.6875,"y":-3.09375,"z":47.1875},
        {"id64":3932277478106,"name":"Shinrarta Dezhra","x":55.71875,"y":17.59375,"z":27.15625}],
        "values":["Shingon","Shinrarta Dezhra"]}"#;

    #[test]
    fn typed_origin_is_found_in_the_map_then_on_spansh() {
        let rec = Arc::new(Recorded { sphere: "[]".into(), log: Mutex::new(vec![]) });
        let client = Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none());
        let mut s = Store::in_memory().unwrap();
        s.insert_systems([(10477373803, "Sol", [0.0, 0.0, 0.0])]).unwrap();
        let g = Galaxy::with_remote(s, client);

        let sol = g.locate(" sol ").unwrap();
        assert_eq!((sol.system.as_str(), sol.id64, sol.pos), ("Sol", Some(10477373803), [0.0; 3]));
        assert!(rec.log.lock().unwrap().is_empty(), "a known system needs no request");

        let names = g.system_names("Shin").unwrap();
        assert_eq!(names, ["Shingon", "Shinrarta Dezhra"]);
        let sd = g.locate("shinrarta dezhra").unwrap();
        assert_eq!(sd.system, "Shinrarta Dezhra", "Spansh's spelling");
        assert_eq!(sd.id64, Some(3932277478106));
        assert_eq!(sd.pos, [55.71875, 17.59375, 27.15625]);

        let err = g.locate("Shin").unwrap_err().to_string();
        assert_eq!(err, "Spansh doesn't know a system called Shin.", "a prefix is not a system");
    }

    #[test]
    fn nearby_ranks_cached_hits_by_jumps() {
        let mut s = Store::in_memory().unwrap();
        // a dense line of stars from Sol towards the fixture systems
        let line: Vec<(i64, String, [f64; 3])> =
            (0..40).map(|i| (1000 + i, format!("S{i}"), [i as f64 * 0.5, -(i as f64) * 0.1, i as f64 * 0.3])).collect();
        s.insert_systems(line.iter().map(|(a, b, c)| (*a, b.as_str(), *c))).unwrap();
        s.add_coverage(Sphere { center: [0.0; 3], radius: 500.0 }, "test", "t").unwrap();
        let body = include_str!("../tests/fixtures/spansh-famine.json");
        let here = Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] };
        let key = "spansh:famine:0:0:0";
        s.cache_put(key, now_s(), body).unwrap();
        let g = Galaxy::with_store(s);
        // covered → no EDSM call; cached → no Spansh call
        let n = g.nearby("famine", &here, 5.0, 2, &Filters::default()).unwrap();
        assert_eq!(n.hits.len(), 2);
        assert_eq!(n.hits[0].name, "WISE 0855-0714");
        assert!(n.hits[0].hop.jumps >= 2);
        assert!(n.hits.iter().all(|h| h
            .factions
            .iter()
            .all(|f| f.state.as_deref() == Some("Famine") || f.active_states.iter().any(|s| s == "Famine"))));
        assert!(n.warning.is_none());
        // Spansh hits are added to the map
        assert!(g.position("Tau Ceti").unwrap().is_some());
        // the lookup's filters apply to the cached answer: every fixture hit is older than a day
        let fresh = Filters { max_age_days: Some(1), ..Filters::default() };
        let none = g.nearby("famine", &here, 5.0, 2, &fresh).unwrap();
        assert!(none.hits.is_empty() && none.warning.is_none(), "{none:?}");
    }

    #[test]
    fn farming_notes_name_known_lookups() {
        for spec in astromech_materials::catalog::catalog() {
            for n in astromech_materials::hge::sources_for(spec) {
                if let Some(q) = n.query.as_deref() {
                    assert!(query(q).is_some(), "{} names unknown lookup {q}", spec.id);
                }
            }
        }
        assert!(
            astromech_materials::hge::sources_for(astromech_materials::lookup("dataminedwake").unwrap())[0]
                .query
                .as_deref()
                == Some("famine")
        );
    }

    #[test]
    fn here_from_journals() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(
            d.path().join("Journal.2026-10-01T000000.01.log"),
            "{\"timestamp\":\"t\",\"event\":\"FSDJump\",\"StarSystem\":\"Sol\",\"SystemAddress\":10477373803,\"StarPos\":[0,0,0]}\n{\"timestamp\":\"t\",\"event\":\"FSDJump\",\"StarSystem\":\"Deciat\",\"SystemAddress\":6681123623626,\"StarPos\":[122.625,-0.8125,-47.28125]}\n{\"timestamp\":\"t\",\"event\":\"Docked\",\"StationName\":\"X\"}\n",
        )
        .unwrap();
        let h = here_from_journal_dir(d.path()).unwrap();
        assert_eq!(h.system, "Deciat");
        assert_eq!(h.pos[0], 122.625);
    }
}
