//! astromech-ztrb: a page that caches what it reads from the journals must see an event appended
//! to the journal the game still holds open, without relying on directory metadata or file
//! notifications, and must not re-read when the game only appended events nothing models.

use astromech_journal::JournalWatch;
use std::io::Write;

const HEADER: &str = r#"{ "timestamp":"2026-10-16T12:00:00Z", "event":"Fileheader", "part":1 }"#;

fn open(path: &std::path::Path) -> std::fs::File {
    std::fs::OpenOptions::new().create(true).append(true).open(path).unwrap()
}

#[test]
fn typed_appends_to_an_open_journal_change_the_generation() {
    let d = tempfile::tempdir().unwrap();
    let mut game = open(&d.path().join("Journal.2026-10-16T120000.01.log"));
    writeln!(game, "{HEADER}").unwrap();
    let mut w = JournalWatch::default();
    let start = w.poll(d.path());
    assert_eq!(w.poll(d.path()), start, "nothing was appended");

    let relevant = [
        r#"{ "timestamp":"2026-10-16T12:01:00Z", "event":"Loadout", "Ship":"python", "ShipID":1, "ShipName":"", "ShipIdent":"", "Modules":[] }"#,
        r#"{ "timestamp":"2026-10-16T12:02:00Z", "event":"EngineerCraft", "Engineer":"Felicity Farseer", "BlueprintName":"FSD_LongRange", "Level":1, "Ingredients":[] }"#,
        r#"{ "timestamp":"2026-10-16T12:03:00Z", "event":"EngineerProgress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Unlocked", "RankProgress":0, "Rank":1 }"#,
    ];
    let mut seen = start;
    for line in relevant {
        // An event nothing models comes first: it must not count.
        writeln!(game, r#"{{ "timestamp":"2026-10-16T12:00:30Z", "event":"Music", "MusicTrack":"NoTrack" }}"#).unwrap();
        assert_eq!(w.poll(d.path()), seen, "an unmodelled event changed the generation");
        writeln!(game, "{line}").unwrap();
        let now = w.poll(d.path());
        assert!(now > seen, "missed an append to the open journal: {line}");
        seen = now;
    }
}

#[test]
fn a_half_written_line_counts_once_it_is_finished() {
    let d = tempfile::tempdir().unwrap();
    let mut game = open(&d.path().join("Journal.2026-10-16T120000.01.log"));
    writeln!(game, "{HEADER}").unwrap();
    let mut w = JournalWatch::default();
    let start = w.poll(d.path());
    write!(game, r#"{{ "timestamp":"2026-10-16T12:01:00Z", "event":"Engineer"#).unwrap();
    assert_eq!(w.poll(d.path()), start);
    writeln!(game, r#"Progress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Unlocked" }}"#)
        .unwrap();
    assert!(w.poll(d.path()) > start);
}

#[test]
fn a_new_journal_file_counts() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("Journal.2026-10-16T120000.01.log"), format!("{HEADER}\n")).unwrap();
    let mut w = JournalWatch::default();
    let start = w.poll(d.path());
    std::fs::write(d.path().join("Journal.2026-10-16T130000.01.log"), format!("{HEADER}\n")).unwrap();
    assert!(w.poll(d.path()) > start);
}

#[test]
fn a_cache_rebuilds_for_modelled_events_only() {
    use astromech_journal::JournalCache;
    use std::sync::Arc;
    let d = tempfile::tempdir().unwrap();
    let mut game = open(&d.path().join("Journal.2026-10-16T120000.01.log"));
    writeln!(game, "{HEADER}").unwrap();
    let cache = JournalCache::<usize>::default();
    let builds = std::cell::Cell::new(0usize);
    let get = || {
        cache
            .get(d.path(), || {
                builds.set(builds.get() + 1);
                std::fs::read_to_string(d.path().join("Journal.2026-10-16T120000.01.log")).unwrap().lines().count()
            })
            .unwrap()
    };
    let first = get();
    assert_eq!((*first, builds.get()), (1, 1));
    assert!(Arc::ptr_eq(&first, &get()), "nothing changed");

    writeln!(game, r#"{{ "timestamp":"2026-10-16T12:00:30Z", "event":"Music", "MusicTrack":"NoTrack" }}"#).unwrap();
    assert_eq!(builds.get(), 1);
    assert!(Arc::ptr_eq(&first, &get()), "an unmodelled event rebuilt the cache");

    writeln!(
        game,
        r#"{{ "timestamp":"2026-10-16T12:01:00Z", "event":"EngineerProgress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Unlocked" }}"#
    )
    .unwrap();
    assert_eq!(*get(), 3, "the append to the open journal was not seen on the next call");
    assert_eq!(builds.get(), 2);
}
