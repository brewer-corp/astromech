---
created_at: "2026-10-16T20:29:47.924011626+00:00"
id: "astromech-0qxx"
evidence_type: "test"
commit: "bd05a76833eb46050ddd0cd11d67866e2a2d15f2"
result: "pass"
captured_at: "2026-10-16T20:29:43.813179780+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-store 2>&1 | grep -E \"^test \"; cargo test -p astromech-history 2>&1 | grep -E \"^test |test result: ok. 1[0-9]\"; cargo test -p astromech-core --lib history 2>&1 | grep -E \"^test \"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-i4gx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-i4gx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-store 2>&1 | grep -E \"^test \"; cargo test -p astromech-history 2>&1 | grep -E \"^test |test result: ok. 1[0-9]\"; cargo test -p astromech-core --lib history 2>&1 | grep -E \"^test \"'"
updated_at: "2026-10-16T20:29:47.924422033+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-store 2>&1 | grep -E "^test "; cargo test -p astromech-history 2>&1 | grep -E "^test |test result: ok. 1[0-9]"; cargo test -p astromech-core --lib history 2>&1 | grep -E "^test "'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-store 2>&1 | grep -E "^test "; cargo test -p astromech-history 2>&1 | grep -E "^test |test result: ok. 1[0-9]"; cargo test -p astromech-core --lib history 2>&1 | grep -E "^test "'
```

Exit status: 0

## Stdout

Bytes: 1628
Truncated: no

```text
test tests::migrates_and_stores ... ok
test tests::failed_migration_rolls_back_whole ... ok
test tests::learns_aliases_and_caps ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test commander::tests::keys ... ok
test tests::commanders_and_legacy_stay_apart ... ok
test tests::continuation_part_keeps_the_commander ... ok
test tests::scans_ledger ... ok
test tests::earnings_ledger ... ok
test tests::imports_every_line_once ... ok
test tests::session_summary_totals ... ok
test tests::import_reports_progress ... ok
test tests::read_only_open_reads_what_the_writer_wrote ... ok
test tests::rebuilt_ledgers_equal_imported_ones ... ok
test tests::logbook_search_finds_places_with_their_counts ... ok
test tests::ship_rows_ignore_older_events ... ok
test tests::unfinished_line_waits_for_its_newline ... ok
test tests::ships_ledger ... ok
test tests::visits_ledger ... ok
test tests::older_journal_imported_late_gives_the_same_ledgers ... ok
test tests::v1_database_is_migrated ... ok
test tests::import_killed_anywhere_then_resumed_equals_clean_import ... ok
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.06s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

