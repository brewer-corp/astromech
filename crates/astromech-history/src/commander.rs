//! Which commander a journal line belongs to.
//!
//! A journal file names its commander near the top (`Commander`, then `LoadGame`, both with
//! the Frontier id `FID`), and can name another one later after a return to the main menu.
//! Lines are attributed to the commander named last in the same file; lines before that
//! (`Fileheader`) have none.
//!
//! The key is the `FID` (or `name:<Commander>` in journals old enough to lack it). Legacy-mode
//! journals (game 3.x after Update 14 split the galaxies on 29 Nov 2022) get `/legacy` on
//! the key: the same commander there has a separate fleet and position.

use astromech_journal::{JournalEvent, JournalLine};

/// Live and Legacy split here (Update 14). Earlier 3.x journals are the live galaxy.
const LEGACY_FROM: &str = "2022-11-29";

/// Suffix on the key of a Legacy-mode commander.
pub(crate) const LEGACY_SUFFIX: &str = "/legacy";

/// The commander state of one journal file at some line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct FileCommander {
    pub cmdr: Option<String>,
    pub legacy: bool,
}

impl FileCommander {
    /// Follow one line.
    pub fn observe(&mut self, line: &JournalLine) {
        let (fid, name) = match &line.event {
            JournalEvent::FileHeader { game_version, .. } => {
                self.legacy = game_version.starts_with("3.") && line.timestamp.as_str() >= LEGACY_FROM;
                return;
            }
            JournalEvent::LoadGame { commander, .. } => (fid(&line.raw_json), commander.clone()),
            JournalEvent::Other { event, raw } if event == "Commander" => (
                raw.get("FID").and_then(|v| v.as_str()).map(str::to_string),
                raw.get("Name").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            ),
            _ => return,
        };
        let base = match (fid, name.is_empty()) {
            (Some(f), _) if !f.is_empty() => f,
            (_, false) => format!("name:{name}"),
            _ => return,
        };
        self.cmdr = Some(if self.legacy { format!("{base}{LEGACY_SUFFIX}") } else { base });
    }
}

/// `FID` from a `LoadGame` line (the typed event doesn't keep it).
fn fid(raw_json: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(raw_json).ok()?;
    v.get("FID").and_then(|x| x.as_str()).map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_journal::parse_line;

    fn run(lines: &[&str]) -> FileCommander {
        let mut c = FileCommander::default();
        for l in lines {
            c.observe(&parse_line(l).unwrap());
        }
        c
    }

    #[test]
    fn keys() {
        let head = |v: &str, ts: &str| format!(r#"{{"timestamp":"{ts}","event":"Fileheader","gameversion":"{v}"}}"#);
        let cmdr = r#"{"timestamp":"2023-01-01T00:00:00Z","event":"Commander","FID":"F1","Name":"A"}"#;
        assert_eq!(run(&[&head("4.0.0.1450", "2023-01-01T00:00:00Z"), cmdr]).cmdr.as_deref(), Some("F1"));
        assert_eq!(run(&[&head("3.8.0.407", "2023-01-01T00:00:00Z"), cmdr]).cmdr.as_deref(), Some("F1/legacy"));
        assert_eq!(run(&[&head("3.8.0.407", "2021-01-01T00:00:00Z"), cmdr]).cmdr.as_deref(), Some("F1"));
        let old = r#"{"timestamp":"2016-01-01T00:00:00Z","event":"LoadGame","Commander":"Old"}"#;
        assert_eq!(run(&[old]).cmdr.as_deref(), Some("name:Old"));
        let load = r#"{"timestamp":"2023-01-01T00:00:01Z","event":"LoadGame","FID":"F2","Commander":"B"}"#;
        assert_eq!(run(&[cmdr, load]).cmdr.as_deref(), Some("F2"));
    }
}
