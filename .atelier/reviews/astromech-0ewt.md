---
schema: atelier.review
schema_version: 1
id: astromech-0ewt
title: 'R2 [low] crates/astromech-colonisation/src/lib.rs:362-378 with app/src-tauri/src/main.rs:591-604: colonisation_projects re-parses every journal file whenever the newest journal grows (journal_version). Measured on a 96 MB synthetic folder of 400 files: 0.73 s per replay in a release build. It runs on a blocking thread and matches the existing engineering and powerplay replays, but adds another full pass, and polling for R1 would repeat it during play. Fix: an incremental fold (keep Tracker and Hold, apply only new lines) when the tracker is wired to the live journal; astromech-0x68 currently covers keying and storage, not this.'
target: astromech-k3wp
identity: review-agent (k3wp review)
commit: f5e62c69c573b451172bf1cd2e16cce3b6519e75
created_at: 2026-10-16T23:10:37.405627013Z
event:
  event: finding
  recommendation: defer
---

R2 [low] crates/astromech-colonisation/src/lib.rs:362-378 with app/src-tauri/src/main.rs:591-604: colonisation_projects re-parses every journal file whenever the newest journal grows (journal_version). Measured on a 96 MB synthetic folder of 400 files: 0.73 s per replay in a release build. It runs on a blocking thread and matches the existing engineering and powerplay replays, but adds another full pass, and polling for R1 would repeat it during play. Fix: an incremental fold (keep Tracker and Hold, apply only new lines) when the tracker is wired to the live journal; astromech-0x68 currently covers keying and storage, not this.
