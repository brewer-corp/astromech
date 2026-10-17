//! The synthetic voice-eval phrase set (`voice-eval-phrases.tsv`, scored by
//! `examples/voice-eval.rs`) stays a 200-phrase set that covers the whole grammar.

use astromech_core::eval::load_phrases;
use astromech_intent::MVP;
use std::collections::BTreeSet;
use std::path::Path;

#[test]
fn phrase_set_covers_every_command() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/voice-eval-phrases.tsv");
    let phrases = load_phrases(&path).unwrap();
    assert_eq!(phrases.len(), 200, "ARCHITECTURE.md §14 asks for a 200-phrase set");
    let commands: BTreeSet<&str> = MVP.iter().map(|(c, _)| *c).collect();
    for p in &phrases {
        assert!(p.label == "nomatch" || commands.contains(p.label.as_str()), "unknown command {:?}", p.label);
    }
    let covered: BTreeSet<&str> = phrases.iter().map(|p| p.label.as_str()).collect();
    let missing: Vec<&&str> = commands.iter().filter(|c| !covered.contains(**c)).collect();
    assert!(missing.is_empty(), "grammar commands with no phrase: {missing:?}");
    let ids: BTreeSet<&str> = phrases.iter().map(|p| p.id.as_str()).collect();
    assert_eq!(ids.len(), phrases.len());
}
