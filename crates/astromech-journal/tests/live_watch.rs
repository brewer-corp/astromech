//! astromech-ztrb: a page that caches what it reads from the journals must see an event appended
//! to the journal the game still holds open, without relying on directory metadata or file
//! notifications, and must not re-read when the game only appended events nothing models.

use astromech_journal::JournalWatch;
use std::io::Write;

/// What a reader of the Engineering page looks at.
const WATCHED: &[&str] = &["\"Loadout\"", "\"EngineerCraft\"", "\"EngineerProgress\""];

const HEADER: &str = r#"{ "timestamp":"2026-10-16T12:00:00Z", "event":"Fileheader", "part":1 }"#;

/// Frequent events, some of them typed by the parser, that no Engineering reader looks at.
const NOISE: &[&str] = &[
    r#"{ "timestamp":"2026-10-16T12:00:30Z", "event":"Music", "MusicTrack":"NoTrack" }"#,
    r#"{ "timestamp":"2026-10-16T12:00:31Z", "event":"ReceiveText", "From":"Cmdr X", "Message":"o7", "Channel":"local" }"#,
    r#"{ "timestamp":"2026-10-16T12:00:32Z", "event":"FuelScoop", "Scooped":1.5, "Total":16.0 }"#,
    r#"{ "timestamp":"2026-10-16T12:00:33Z", "event":"Scan", "BodyName":"Sol 1", "ScanType":"Detailed" }"#,
    r#"{ "timestamp":"2026-10-16T12:00:34Z", "event":"ShipTargeted", "TargetLocked":false }"#,
];

fn open(path: &std::path::Path) -> std::fs::File {
    std::fs::OpenOptions::new().create(true).append(true).open(path).unwrap()
}

#[test]
fn typed_appends_to_an_open_journal_change_the_generation() {
    let d = tempfile::tempdir().unwrap();
    let mut game = open(&d.path().join("Journal.2026-10-16T120000.01.log"));
    writeln!(game, "{HEADER}").unwrap();
    let mut w = JournalWatch::new(WATCHED);
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
        for noise in NOISE {
            writeln!(game, "{noise}").unwrap();
        }
        assert_eq!(w.poll(d.path()), seen, "an event no reader looks at changed the generation");
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
    let mut w = JournalWatch::new(WATCHED);
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
    let mut w = JournalWatch::new(WATCHED);
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
    let cache = JournalCache::<usize>::new(WATCHED);
    let builds = std::cell::Cell::new(0usize);
    let get = || {
        cache.get(d.path(), || {
            builds.set(builds.get() + 1);
            std::fs::read_to_string(d.path().join("Journal.2026-10-16T120000.01.log")).unwrap().lines().count()
        })
    };
    let first = get();
    assert_eq!((*first, builds.get()), (1, 1));
    assert!(Arc::ptr_eq(&first, &get()), "nothing changed");

    for noise in NOISE {
        writeln!(game, "{noise}").unwrap();
    }
    assert_eq!(builds.get(), 1);
    assert!(Arc::ptr_eq(&first, &get()), "an unmodelled event rebuilt the cache");

    writeln!(
        game,
        r#"{{ "timestamp":"2026-10-16T12:01:00Z", "event":"EngineerProgress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Unlocked" }}"#
    )
    .unwrap();
    assert_eq!(*get(), 2 + NOISE.len(), "the append to the open journal was not seen on the next call");
    assert_eq!(builds.get(), 2);
}

/// astromech-ztrb review: a new journal that cannot be opened yet (a virus scanner holds it) is
/// not forgotten; the switch is counted by the first poll that can read it.
#[cfg(unix)]
#[test]
fn a_new_journal_that_fails_to_open_is_counted_when_it_can_be_read() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("Journal.2026-10-16T120000.01.log"), format!("{HEADER}\n")).unwrap();
    let mut w = JournalWatch::new(WATCHED);
    let start = w.poll(d.path());
    use std::os::unix::fs::PermissionsExt;
    let second = d.path().join("Journal.2026-10-16T130000.01.log");
    std::fs::write(&second, format!("{HEADER}\n")).unwrap();
    std::fs::set_permissions(&second, std::fs::Permissions::from_mode(0o000)).unwrap();
    if std::fs::File::open(&second).is_ok() {
        return; // running as root: nothing can be locked
    }
    assert_eq!(w.poll(d.path()), start, "nothing readable yet");
    std::fs::set_permissions(&second, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert!(w.poll(d.path()) > start, "the switch to the new journal was lost");
}

#[test]
fn a_panicking_build_does_not_wedge_the_cache() {
    use astromech_journal::JournalCache;
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("Journal.2026-10-16T120000.01.log"), format!("{HEADER}\n")).unwrap();
    let cache = JournalCache::<u8>::new(WATCHED);
    let died = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cache.get(d.path(), || panic!("build"))));
    assert!(died.is_err());
    assert_eq!(*cache.get(d.path(), || 7), 7);
}

#[cfg(unix)]
#[test]
fn a_first_poll_that_cannot_open_the_journal_does_not_pin_the_build() {
    use astromech_journal::JournalCache;
    use std::os::unix::fs::PermissionsExt;
    let d = tempfile::tempdir().unwrap();
    let j = d.path().join("Journal.2026-10-16T120000.01.log");
    std::fs::write(&j, format!("{HEADER}\n")).unwrap();
    std::fs::set_permissions(&j, std::fs::Permissions::from_mode(0o000)).unwrap();
    if std::fs::File::open(&j).is_ok() {
        return; // running as root: nothing can be locked
    }
    let cache = JournalCache::<u8>::new(WATCHED);
    assert_eq!(*cache.get(d.path(), || 1), 1);
    std::fs::set_permissions(&j, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(*cache.get(d.path(), || 2), 2, "the build made while the journal was unreadable was kept");
}

#[test]
fn a_journal_that_shrinks_counts() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("Journal.2026-10-16T120000.01.log");
    std::fs::write(&p, format!("{HEADER}\n{HEADER}\n")).unwrap();
    let mut w = JournalWatch::new(WATCHED);
    let start = w.poll(d.path());
    std::fs::write(&p, format!("{HEADER}\n")).unwrap();
    assert!(w.poll(d.path()) > start);
}
