---
created_at: "2026-10-16T20:11:51.940108518+00:00"
id: "astromech-garq"
evidence_type: "validation"
commit: "27e5c4be4296ff0a3e2d702f5cd1049271a84e59"
result: "pass"
captured_at: "2026-10-16T20:11:51.940070156+00:00"
path: "docs/evidence/astromech-fxp0/logbook.png"
target:
  kind: "issue"
  id: "astromech-fxp0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-fxp0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "UI check at 27e5c4b: vite dev + scripts/ui (headless Chrome 1280x800) on #/logbook with the sample history (the Rust fixture set). docs/evidence/astromech-fxp0/logbook.png: CMDR Fixture, 3 systems, 3 stations, 5 arrivals, 4 dockings; places with SystemChip names, visits, first/last; timeline newest first with jump distances. docs/evidence/astromech-fxp0/search.png: typing 'hutton' leaves one place, Hutton Orbital in Alpha Centauri, 2 visits, 2026-09-03 10:10 to 10:25, and its two dockings in the timeline. pnpm lint, check (0 errors, 0 warnings), build and test (22 tests) pass. Search timing (release, import-history example) on the 400 MB synthetic history with 74,679 visits all at different places: empty search 243 ms, 'sys 12' 196 ms (1,301 matches), no match 113 ms. Not verified: the shell's logbook command compiles only on the Windows CI job (no webkit2gtk here); app/src-tauri/Cargo.lock passes the --locked check."
updated_at: "2026-10-16T20:11:51.940347264+00:00"
---

UI check at 27e5c4b: vite dev + scripts/ui (headless Chrome 1280x800) on #/logbook with the sample history (the Rust fixture set). docs/evidence/astromech-fxp0/logbook.png: CMDR Fixture, 3 systems, 3 stations, 5 arrivals, 4 dockings; places with SystemChip names, visits, first/last; timeline newest first with jump distances. docs/evidence/astromech-fxp0/search.png: typing 'hutton' leaves one place, Hutton Orbital in Alpha Centauri, 2 visits, 2026-09-03 10:10 to 10:25, and its two dockings in the timeline. pnpm lint, check (0 errors, 0 warnings), build and test (22 tests) pass. Search timing (release, import-history example) on the 400 MB synthetic history with 74,679 visits all at different places: empty search 243 ms, 'sys 12' 196 ms (1,301 matches), no match 113 ms. Not verified: the shell's logbook command compiles only on the Windows CI job (no webkit2gtk here); app/src-tauri/Cargo.lock passes the --locked check.
