---
schema: atelier.review
schema_version: 1
id: astromech-j3sk
title: 'The unknown-mod report (owned.rs report_unknown_mod) is eprintln!, and the release app is windows_subsystem = "windows" (app/src-tauri/src/main.rs:13), so on the commander''s PC stderr is not visible. astromech-d0zh''s done-when relies on seeing no ''odyssey: unknown ...'' line on stderr. Consistent with the repo''s other eprintln diagnostics, so not a defect here; d0zh should say the check needs a debug build or a console (cargo tauri dev), or the report should go to whatever log the app keeps.'
target: astromech-f1us
identity: review-agent (opus, independent)
commit: 9a75d654401c05ed91d14c49ec345b5ce3616a04
created_at: 2026-10-17T05:48:46.273898843Z
event:
  event: finding
  recommendation: defer
---

The unknown-mod report (owned.rs report_unknown_mod) is eprintln!, and the release app is windows_subsystem = "windows" (app/src-tauri/src/main.rs:13), so on the commander's PC stderr is not visible. astromech-d0zh's done-when relies on seeing no 'odyssey: unknown ...' line on stderr. Consistent with the repo's other eprintln diagnostics, so not a defect here; d0zh should say the check needs a debug build or a console (cargo tauri dev), or the report should go to whatever log the app keeps.
