//! Exploration from the history: the data the commander hasn't sold yet
//! (`astromech_exploration::unsold`, folded over the stored journal lines since the last death),
//! and where the commander has landed before.
//!
//! A **landing** on a body is a `Touchdown` the commander flew (`PlayerControlled`, not on a
//! station's pad), or a `Disembark` from a taxi or dropship onto the surface (`Taxi`,
//! `OnPlanet`), which lands without a `Touchdown` of the commander's ship. Leaving the ship or
//! the SRV after a touchdown is the same landing and isn't counted again. Bodies are matched by
//! `BodyID`, else by name.

use crate::ledgers::Commander;
use astromech_exploration::system::SystemView;
use astromech_exploration::unsold::{self, Unsold};
use astromech_journal::{ExtraEvent, JournalEvent, JournalLine};
use astromech_store::rusqlite::{params, params_from_iter};
use serde::Serialize;

/// A commander's unsold data.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UnsoldData {
    pub commander: Commander,
    #[serde(flatten)]
    pub unsold: Unsold,
}

/// The commander's landings on one body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Landing {
    pub body: String,
    pub body_id: Option<i64>,
    pub count: u32,
    /// The latest (journal timestamp).
    pub last: String,
}

/// The body a line lands on, if it is a landing (see the module docs).
fn landed_on(l: &JournalLine) -> Option<(Option<i64>, Option<String>)> {
    match &l.event {
        JournalEvent::Extra(ExtraEvent::Touchdown(t)) if t.player_controlled && t.on_station != Some(true) => {
            Some((t.body_id, t.body.clone()))
        }
        JournalEvent::Extra(ExtraEvent::Disembark(d)) if d.taxi && d.on_planet && !d.on_station => {
            Some((d.body_id, d.body.clone()))
        }
        _ => None,
    }
}

impl crate::History {
    /// The journal lines of commander `cmdr` among `events`, from their last `reset` event on
    /// (or from the start if there is none), in game-time order, as the game wrote them.
    pub fn lines_since_last(&self, cmdr: &str, events: &[&str], reset: &str) -> crate::Result<Vec<String>> {
        // ?1 is the commander, ?2 the reset event, ?3... the events.
        let marks: Vec<String> = (3..3 + events.len()).map(|i| format!("?{i}")).collect();
        let sql = format!(
            "SELECT json FROM events
             WHERE cmdr = ?1 AND event IN ({})
               AND ts >= coalesce((SELECT max(ts) FROM events WHERE cmdr = ?1 AND event = ?2), '')
             ORDER BY ts, id",
            marks.join(", ")
        );
        let args = [cmdr, reset].into_iter().chain(events.iter().copied());
        let mut st = self.conn.prepare(&sql)?;
        let rows = st.query_map(params_from_iter(args), |r| r.get::<_, String>(0))?;
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    /// The unsold cartographic and organic data of `cmdr` (the most recently played commander if
    /// `None`); `None` if there is no such commander.
    pub fn unsold(&self, cmdr: Option<&str>) -> crate::Result<Option<UnsoldData>> {
        let commanders = self.commanders()?;
        let Some(commander) = (match cmdr {
            Some(k) => commanders.into_iter().find(|c| c.cmdr == k),
            None => commanders.into_iter().next(),
        }) else {
            return Ok(None);
        };
        let lines = self.lines_since_last(&commander.cmdr, unsold::EVENTS, "Died")?;
        Ok(Some(UnsoldData { commander, unsold: unsold::from_json_lines(&lines) }))
    }

    /// [`unsold`](Self::unsold) from the database at `db`, opened read only. No database yet
    /// (before the core's first import) is the same as no commander: `None`, not an error.
    pub fn unsold_at(db: &std::path::Path, cmdr: Option<&str>) -> crate::Result<Option<UnsoldData>> {
        if !db.is_file() {
            return Ok(None);
        }
        Self::open_read_only(db)?.unsold(cmdr)
    }

    /// The landings of the commander called `name` in the system `address` before `before` (a
    /// journal timestamp, not included), by body, most first; Legacy-mode journals are ignored.
    pub fn landings_before(&self, name: &str, address: i64, before: &str) -> crate::Result<Vec<Landing>> {
        let mut st = self.conn.prepare_cached(
            "SELECT ts, json FROM events
             WHERE event IN ('Touchdown', 'Disembark') AND ts < ?3
               AND cmdr IN (SELECT cmdr FROM commanders WHERE name = ?2 AND legacy = 0)
               AND json_extract(json, '$.SystemAddress') = ?1
             ORDER BY ts, id",
        )?;
        let rows =
            st.query_map(params![address, name, before], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        let mut out: Vec<Landing> = vec![];
        for row in rows {
            let (ts, json) = row?;
            let Ok(line) = astromech_journal::parse_line(&json) else { continue };
            let Some((body_id, body)) = landed_on(&line) else { continue };
            let found = out.iter_mut().find(|l| match (body_id, l.body_id) {
                (Some(a), Some(b)) => a == b,
                _ => body.as_deref() == Some(l.body.as_str()),
            });
            match found {
                Some(l) => {
                    l.count += 1;
                    l.last = ts;
                    if l.body_id.is_none() {
                        l.body_id = body_id;
                    }
                }
                None => out.push(Landing { body: body.unwrap_or_default(), body_id, count: 1, last: ts }),
            }
        }
        out.sort_by(|a, b| b.count.cmp(&a.count).then(b.last.cmp(&a.last)));
        Ok(out)
    }

    /// Fill in the most recently played commander's landings on the view's bodies.
    pub fn add_landings(&self, view: &mut SystemView) -> crate::Result<()> {
        let (Some(address), Some(commander)) = (view.system_address, self.commanders()?.into_iter().next()) else {
            return Ok(());
        };
        let landings = self.landings_before(&commander.name, address, "9999")?;
        for b in &mut view.bodies {
            let n: u32 = landings
                .iter()
                .filter(|l| match (l.body_id, b.body_id) {
                    (Some(x), Some(y)) => x == y,
                    _ => l.body == b.name,
                })
                .map(|l| l.count)
                .sum();
            b.landings = n;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{History, ImportOptions};

    fn load(ts: &str, fid: &str, name: &str) -> String {
        format!(r#"{{"timestamp":"{ts}","event":"LoadGame","FID":"{fid}","Commander":"{name}","Ship":"krait_mkii"}}"#)
    }

    fn scan(ts: &str, system: &str, body: &str) -> String {
        format!(
            r#"{{"timestamp":"{ts}","event":"Scan","ScanType":"Detailed","BodyName":"{system} {body}","BodyID":1,"StarSystem":"{system}","SystemAddress":{},"PlanetClass":"Icy body","MassEM":0.5,"WasDiscovered":true,"WasMapped":true}}"#,
            system.len()
        )
    }

    /// One commander explores, dies, explores again and sells one of two systems; another
    /// commander's scans are not theirs.
    fn imported() -> (tempfile::TempDir, History) {
        let d = tempfile::tempdir().unwrap();
        let mine = [
            load("2026-10-01T09:00:00Z", "F1", "Test"),
            scan("2026-10-01T10:00:00Z", "Lost", "1"),
            r#"{"timestamp":"2026-10-01T11:00:00Z","event":"Died"}"#.to_string(),
            scan("2026-10-02T10:00:00Z", "Alpha", "1"),
            scan("2026-10-02T10:05:00Z", "Beta", "1"),
            r#"{"timestamp":"2026-10-02T10:10:00Z","event":"ScanOrganic","ScanType":"Analyse","Genus":"$Codex_Ent_Bacterial_Genus_Name;","Species":"$Codex_Ent_Bacterial_01_Name;","SystemAddress":5,"Body":1}"#.to_string(),
            r#"{"timestamp":"2026-10-02T12:00:00Z","event":"SellExplorationData","Systems":["Alpha"],"Discovered":[],"BaseValue":10,"Bonus":0,"TotalEarnings":10}"#.to_string(),
        ];
        std::fs::write(d.path().join("Journal.2026-10-01T090000.01.log"), mine.join("\n") + "\n").unwrap();
        let theirs = [load("2026-10-03T09:00:00Z", "F2", "Other"), scan("2026-10-03T10:00:00Z", "Gamma", "1")];
        std::fs::write(d.path().join("Journal.2026-10-03T090000.01.log"), theirs.join("\n") + "\n").unwrap();
        let mut h = History::in_memory().unwrap();
        h.import(d.path(), &ImportOptions::default()).unwrap();
        (d, h)
    }

    #[test]
    fn unsold_is_since_the_last_death_less_what_was_sold() {
        let (_d, h) = imported();
        let u = h.unsold(Some("F1")).unwrap().unwrap();
        assert_eq!(u.commander.name, "Test");
        // Beta's body and the sample; Lost went with the death, Alpha was sold.
        assert_eq!((u.unsold.systems, u.unsold.bodies, u.unsold.samples), (1, 1, 1));
        assert_eq!(u.unsold.organic, 1_000_000);
        assert_eq!(u.unsold.since.as_deref(), Some("2026-10-02T10:05:00Z"));
        // The most recently played commander by default.
        let other = h.unsold(None).unwrap().unwrap();
        assert_eq!((other.commander.name.as_str(), other.unsold.bodies), ("Other", 1));
        assert!(h.unsold(Some("nobody")).unwrap().is_none());
    }

    #[test]
    fn no_database_yet_is_no_commander_not_an_error() {
        let d = tempfile::tempdir().unwrap();
        assert_eq!(History::unsold_at(&d.path().join("history.sqlite"), None).unwrap(), None);
        // Once there, it answers.
        let (_j, h) = imported();
        let db = d.path().join("copy.sqlite");
        h.conn().execute("VACUUM INTO ?1", [db.to_str().unwrap()]).unwrap();
        assert_eq!(History::unsold_at(&db, Some("F1")).unwrap().unwrap().unsold.bodies, 1);
    }

    #[test]
    fn lines_since_last_reset_are_in_game_time_order() {
        let (_d, h) = imported();
        let lines = h.lines_since_last("F1", &["Scan", "Died"], "Died").unwrap();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains(r#""event":"Died""#));
        assert!(lines[2].contains("Beta"));
        // No reset event: from the start.
        assert_eq!(h.lines_since_last("F1", &["Scan"], "Nothing").unwrap().len(), 3);
    }

    fn jump(ts: &str, name: &str, address: i64, population: i64) -> String {
        format!(
            r#"{{"timestamp":"{ts}","event":"FSDJump","StarSystem":"{name}","SystemAddress":{address},"StarPos":[0,0,0],"Population":{population},"JumpDist":9.5}}"#
        )
    }

    fn touchdown(ts: &str, address: i64, body: &str, id: i64, extra: &str) -> String {
        format!(
            r#"{{"timestamp":"{ts}","event":"Touchdown","PlayerControlled":true,"Latitude":1.0,"Longitude":2.0,"NearestDestination":"","StarSystem":"S{address}","SystemAddress":{address},"Body":"{body}","BodyID":{id},"OnStation":false,"OnPlanet":true{extra}}}"#
        )
    }

    /// Two landings on Explo A 2 (with a walk and an SRV trip in between, and an automatic
    /// touchdown of the recalled ship, none of which count), a station pad, a dropship to A 3,
    /// and a landing in another system on a body with the same id.
    fn landed() -> (tempfile::TempDir, History, Vec<String>) {
        let d = tempfile::tempdir().unwrap();
        let scan = |ts: &str, address: i64, system: &str, body: &str, id: i64| {
            format!(
                r#"{{"timestamp":"{ts}","event":"Scan","ScanType":"Detailed","BodyName":"{system} {body}","BodyID":{id},"StarSystem":"{system}","SystemAddress":{address},"PlanetClass":"Rocky body","MassEM":0.05,"SurfaceGravity":1.5,"Landable":true,"WasDiscovered":false,"WasMapped":false}}"#
            )
        };
        let lines = vec![
            load("2026-10-01T09:00:00Z", "F1", "Test"),
            jump("2026-10-01T09:10:00Z", "Explo", 42, 0),
            scan("2026-10-01T09:11:00Z", 42, "Explo", "A 2", 7),
            scan("2026-10-01T09:12:00Z", 42, "Explo", "A 3", 8),
            touchdown("2026-10-01T09:20:00Z", 42, "Explo A 2", 7, ""),
            r#"{"timestamp":"2026-10-01T09:21:00Z","event":"Disembark","SRV":false,"Taxi":false,"Multicrew":false,"StarSystem":"Explo","SystemAddress":42,"Body":"Explo A 2","BodyID":7,"OnStation":false,"OnPlanet":true}"#.to_string(),
            r#"{"timestamp":"2026-10-01T09:30:00Z","event":"Disembark","SRV":true,"Taxi":false,"Multicrew":false,"StarSystem":"Explo","SystemAddress":42,"Body":"Explo A 2","BodyID":7,"OnStation":false,"OnPlanet":true}"#.to_string(),
            touchdown("2026-10-01T09:40:00Z", 42, "Explo A 2", 7, r#","PlayerControlled":false"#).replacen(r#""PlayerControlled":true,"#, "", 1),
            touchdown("2026-10-02T09:20:00Z", 42, "Explo A 2", 7, ""),
            touchdown("2026-10-02T09:30:00Z", 42, "Explo Port", 9, "").replace(r#""OnStation":false"#, r#""OnStation":true"#),
            r#"{"timestamp":"2026-10-02T10:00:00Z","event":"Disembark","SRV":false,"Taxi":true,"Multicrew":false,"StarSystem":"Explo","SystemAddress":42,"Body":"Explo A 3","BodyID":8,"OnStation":false,"OnPlanet":true}"#.to_string(),
            jump("2026-10-03T09:00:00Z", "Busy", 43, 5_000_000),
            touchdown("2026-10-03T09:20:00Z", 43, "Busy A 2", 7, ""),
        ];
        std::fs::write(d.path().join("Journal.2026-10-01T090000.01.log"), lines.join("\n") + "\n").unwrap();
        let mut h = History::in_memory().unwrap();
        h.import(d.path(), &ImportOptions::default()).unwrap();
        (d, h, lines)
    }

    #[test]
    fn landings_count_flown_touchdowns_and_dropships_per_body() {
        let (_d, h, _) = landed();
        let l = h.landings_before("Test", 42, "2026-10-04T09:00:00Z").unwrap();
        let got: Vec<(&str, u32, &str)> = l.iter().map(|l| (l.body.as_str(), l.count, l.last.as_str())).collect();
        assert_eq!(got, [("Explo A 2", 2, "2026-10-02T09:20:00Z"), ("Explo A 3", 1, "2026-10-02T10:00:00Z")]);
        // Before the second landing there was one; another commander has none; the other system its own.
        assert_eq!(h.landings_before("Test", 42, "2026-10-02T00:00:00Z").unwrap()[0].count, 1);
        assert!(h.landings_before("Other", 42, "9999").unwrap().is_empty());
        assert_eq!(h.landings_before("Test", 43, "9999").unwrap().len(), 1);
    }

    #[test]
    fn the_system_view_shows_landings_and_the_first_footfall_hint() {
        use astromech_exploration::SystemFold;
        let (_d, h, lines) = landed();
        // The view of Explo, as it was before leaving it.
        let mut f = SystemFold::default();
        for l in &lines[..11] {
            f.apply(&astromech_journal::parse_line(l).unwrap());
        }
        let mut v = f.view();
        h.add_landings(&mut v).unwrap();
        let by: Vec<(&str, u32, bool)> =
            v.bodies.iter().map(|b| (b.short_name.as_str(), b.landings, b.first_footfall_likely)).collect();
        // Undiscovered and landable in an unpopulated system: first footfall likely.
        assert_eq!(by, [("A 2", 2, true), ("A 3", 1, true)]);
        // The same body in a populated system gets no hint.
        let mut f = SystemFold::default();
        for l in
            [jump("2026-10-05T09:00:00Z", "Busy", 43, 5_000_000), lines[2].replace("Explo", "Busy").replace("42", "43")]
        {
            f.apply(&astromech_journal::parse_line(&l).unwrap());
        }
        let v = f.view();
        assert_eq!(v.bodies.len(), 1);
        assert!(v.bodies[0].landable && !v.bodies[0].first_footfall_likely);
    }
}
