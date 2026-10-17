---
schema: atelier.review
schema_version: 1
id: astromech-5nte
title: 'Process-global ''report each unknown mod once'' set (owned.rs report_unknown_mod SEEN): keep. The shell rebuilds JournalState from every journal whenever the journal version changes, so a per-Owned set would repeat each unknown mod in the log on every rebuild; the global set is what keeps it to once per process. Its cost is that a second caller with a different sink in the same process misses names already reported, and tests must use unique names (the new test does: suit_captured_by_test). There is one production caller (the shell). If a second caller appears, move the dedupe into the caller''s callback.'
target: astromech-arra
identity: review-agent (opus, independent)
commit: 69b89465f6cf049ee883b88cf8c38f81c05bc9aa
created_at: 2026-10-17T07:46:32.394987245Z
event:
  event: finding
  recommendation: no_action
---

Process-global 'report each unknown mod once' set (owned.rs report_unknown_mod SEEN): keep. The shell rebuilds JournalState from every journal whenever the journal version changes, so a per-Owned set would repeat each unknown mod in the log on every rebuild; the global set is what keeps it to once per process. Its cost is that a second caller with a different sink in the same process misses names already reported, and tests must use unique names (the new test does: suit_captured_by_test). There is one production caller (the shell). If a second caller appears, move the dedupe into the caller's callback.
