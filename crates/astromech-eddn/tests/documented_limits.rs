//! The limits on what is shared, as the Data sharing page and the guide state them, checked
//! sentence by sentence against the sender's constants. The two age limits are different: a
//! live event is only handed to EDDN's queue if the game wrote it within `MAX_EVENT_AGE_SECS`
//! (EDSM has no such check), and EDDN gives a queued message up after `MAX_AGE_SECS` without
//! delivery or `MAX_ATTEMPTS` tries (EDSM has its own delivery: refusals, no age limit). Both
//! queues hold at most `MAX_QUEUED`.

use astromech_eddn::feed::MAX_EVENT_AGE_SECS;
use astromech_eddn::outbox::{MAX_AGE_SECS, MAX_ATTEMPTS, MAX_QUEUED};
use std::path::Path;

/// The file at `rel` from the repository root, with line breaks and runs of spaces as one space
/// so a wrapped sentence still matches.
fn read(rel: &str) -> String {
    let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(rel)).unwrap();
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `n` with a comma between thousands, as the guide writes numbers.
fn thousands(n: i64) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

fn assert_says(name: &str, text: &str, sentence: &str) {
    assert!(text.contains(sentence), "{name} should say: {sentence}");
}

#[test]
fn the_page_and_the_guide_state_the_senders_limits() {
    let event_min = MAX_EVENT_AGE_SECS / 60;
    let event = format!("{event_min} minutes");
    let queue = format!("{} minutes", MAX_AGE_SECS / 60);
    assert_ne!(event, queue, "two limits; the wording below must keep them apart");

    let page = read("app/src/modules/datasharing/View.svelte");
    let name = "the Data sharing page";
    assert_says(
        name,
        &page,
        &format!(
            "Only live play is sent: never the game's beta, never old journals when astromech starts, and \
             nothing the game wrote more than {event} ago."
        ),
    );
    assert_says(name, &page, &format!("A message that can't be delivered within {queue} is dropped."));
    // The old page text claimed one 30-minute limit on everything sent.
    assert!(!page.contains("nothing older than 30 minutes"));

    let guide = read("docs/guide/data-sharing.md");
    let name = "docs/guide/data-sharing.md";
    for sentence in [
        format!("For EDDN, an event more than **{event}** old by its timestamp is treated as a replay and not sent."),
        "EDSM has no such check: it keeps your own history, so an old event is still true there, and EDSM \
         ignores one it already has."
            .to_owned(),
        format!(
            "EDDN drops a message after {MAX_ATTEMPTS} tries or after {queue} in the queue; that is separate \
             from the {event_min}-minute check above, which happens before a message is queued."
        ),
        format!(
            "EDSM keeps an event until it is sent or EDSM refuses it ({MAX_ATTEMPTS} refusals, or \
             {MAX_ATTEMPTS} tries after a server error); it has no age limit."
        ),
        format!("At most {} wait at once.", thousands(MAX_QUEUED)),
    ] {
        assert_says(name, &guide, &sentence);
    }
}

#[test]
fn thousands_are_separated_by_commas() {
    assert_eq!([8, 999, 2000, 1_234_567].map(thousands), ["8", "999", "2,000", "1,234,567"].map(String::from));
}
