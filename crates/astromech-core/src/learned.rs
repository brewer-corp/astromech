//! The phrases astromech learned from "Did you mean ...?" answered yes (`astromech-store`
//! aliases), for the Voice tester page: list them, and forget one that was taught wrongly.

use crate::assistant::Assistant;
use astromech_api::Answer;
use serde::Serialize;
use std::path::Path;

fn open(data: &Path) -> Result<astromech_store::Store, String> {
    std::fs::create_dir_all(data).map_err(|e| format!("creating {}: {e}", data.display()))?;
    astromech_store::Store::open(&data.join("astromech.sqlite")).map_err(|e| format!("opening the database: {e}"))
}

/// The learned phrases, the most used first.
#[derive(Debug, Serialize, ts_rs::TS)]
pub struct Phrases {
    pub phrases: Vec<Phrase>,
}

/// What was heard, the command it now runs and that command's usual phrase.
#[derive(Debug, Serialize, ts_rs::TS)]
pub struct Phrase {
    pub heard: String,
    pub command: String,
    pub does: String,
}

/// `{"ok": true, "phrases": [{"heard", "command", "does"}]}`. Only phrases that run without
/// asking are listed (`assistant::alias_runs`): not one confirmed once for a key-pressing
/// command, nor an ask-only one.
pub fn list(data: &Path) -> Answer<Phrases> {
    let rows = open(data).and_then(|st| st.aliases().map_err(|e| format!("reading the learned phrases: {e}")));
    rows.map(|rows| {
        let g = astromech_intent::Grammar::mvp(&[]);
        let phrases = rows
            .iter()
            .filter(|(heard, command, count)| crate::assistant::alias_runs(heard, command, *count))
            .map(|(heard, command, _)| Phrase {
                heard: heard.clone(),
                command: command.clone(),
                does: g.canonical(command).unwrap_or(command).to_string(),
            })
            .collect();
        Phrases { phrases }
    })
    .into()
}

/// Forget `heard`, in the running assistant too when there is one, and answer with the list.
pub fn forget(data: &Path, assistant: Option<&Assistant>, heard: &str) -> Answer<Phrases> {
    let gone = match assistant {
        Some(a) => Ok(a.forget(heard)),
        None => open(data).and_then(|st| st.forget_alias(heard).map_err(|e| format!("forgetting the phrase: {e}"))),
    };
    match gone {
        Ok(_) => list(data),
        Err(e) => Answer::failed(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn list(data: &Path) -> Value {
        serde_json::to_value(super::list(data)).unwrap()
    }

    fn forget(data: &Path, a: Option<&Assistant>, heard: &str) -> Value {
        serde_json::to_value(super::forget(data, a, heard)).unwrap()
    }

    #[test]
    fn learned_phrases_are_listed_and_forgotten() {
        let d = tempfile::tempdir().unwrap();
        let data = d.path().join("not-yet");
        // A fresh machine: nothing learned, and no error about the missing folder.
        assert_eq!(list(&data), json!({"ok": true, "phrases": []}));
        let st = open(&data).unwrap();
        st.learn_alias("Definitely hard points.", "hardpoints.deploy").unwrap();
        st.learn_alias("Definitely hard points.", "hardpoints.deploy").unwrap();
        st.learn_alias("fuel stats", "status.fuel").unwrap();
        // Not listed: a key-pressing command confirmed once, and ask-only phrases.
        st.learn_alias("lamps", "lights.on").unwrap();
        st.learn_alias("You're down.", "landing_gear.down").unwrap();
        st.learn_alias("You're down.", "landing_gear.down").unwrap();
        drop(st);
        let v = list(&data);
        assert_eq!(
            v["phrases"][0],
            json!({"heard": "Definitely hard points.", "command": "hardpoints.deploy", "does": "deploy hardpoints"})
        );
        assert_eq!(v["phrases"].as_array().unwrap().len(), 2);
        let v = forget(&data, None, "Definitely hard points.");
        let heard: Vec<&str> = v["phrases"].as_array().unwrap().iter().map(|p| p["heard"].as_str().unwrap()).collect();
        assert_eq!(heard, ["fuel stats"]);
        assert_eq!(forget(&data, None, "never learned")["phrases"].as_array().unwrap().len(), 1);
    }
}
