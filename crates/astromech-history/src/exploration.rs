//! Exploration from the history: the data the commander hasn't sold yet
//! (`astromech_exploration::unsold`, folded over the stored journal lines since the last death).

use crate::ledgers::Commander;
use astromech_exploration::unsold::{self, Unsold};
use astromech_store::rusqlite::params_from_iter;
use serde::Serialize;

/// A commander's unsold data.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UnsoldData {
    pub commander: Commander,
    #[serde(flatten)]
    pub unsold: Unsold,
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
    fn lines_since_last_reset_are_in_game_time_order() {
        let (_d, h) = imported();
        let lines = h.lines_since_last("F1", &["Scan", "Died"], "Died").unwrap();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains(r#""event":"Died""#));
        assert!(lines[2].contains("Beta"));
        // No reset event: from the start.
        assert_eq!(h.lines_since_last("F1", &["Scan"], "Nothing").unwrap().len(), 3);
    }
}
