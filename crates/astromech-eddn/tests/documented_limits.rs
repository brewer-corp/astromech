//! The two age limits on what is shared, as the Data sharing page and the guide state them.
//! They are different: a live event is only handed to the queue if the game wrote it within
//! `MAX_EVENT_AGE_SECS` (EDDN only), and a queued message is given up after `MAX_AGE_SECS`
//! without delivery (EDDN's `deliver_due` only: EDSM has its own delivery with no age limit).
//! The page text and the guide must name the sender's numbers.

use astromech_eddn::feed::MAX_EVENT_AGE_SECS;
use astromech_eddn::outbox::MAX_AGE_SECS;
use std::path::Path;

fn read(rel: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(rel)).unwrap()
}

#[test]
fn the_page_and_the_guide_state_the_senders_age_limits() {
    let event = format!("{} minutes", MAX_EVENT_AGE_SECS / 60);
    let queue = format!("{} minutes", MAX_AGE_SECS / 60);
    assert_ne!(event, queue, "two limits; the wording below must keep them apart");
    // Normalise line breaks so a wrapped sentence still matches.
    let squash = |s: String| s.split_whitespace().collect::<Vec<_>>().join(" ");
    let page = squash(read("app/src/modules/datasharing/View.svelte"));
    let guide = squash(read("docs/guide/data-sharing.md"));
    for (name, text) in [("the Data sharing page", &page), ("docs/guide/data-sharing.md", &guide)] {
        assert!(
            text.contains(&format!("more than {event} ago")) || text.contains(&format!("more than **{event}** old")),
            "{name} should say an event more than {event} old is not sent"
        );
        assert!(text.contains(&queue), "{name} should say a queued message is dropped after {queue}");
    }
    assert!(
        guide.contains(&format!("EDDN drops a message after 8 tries or after {queue} in the queue")),
        "the guide should say the {queue} limit is EDDN's"
    );
    // The old page text claimed one 30-minute limit on everything sent.
    assert!(!page.contains("nothing older than 30 minutes"));
}
