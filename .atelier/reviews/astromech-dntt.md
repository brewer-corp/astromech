---
schema: atelier.review
schema_version: 1
id: astromech-dntt
title: 'Re-review of 4025b1c and merge 23e2be4 at HEAD. bo8g: refusals counted in their own column (v4), outages/203s no longer count; reverting to attempts makes outages_do_not_use_up_a_messages_refusals fail. nfuc: errors-cleared mark is a send_log row id; reverting to wall-1 makes a_key_that_is_still_wrong_shows_its_error_from_the_same_pass fail; app.rs test covers the status path. v4 is a plain ALTER ADD COLUMN with default, no BEGIN/COMMIT (v2''s COMMIT removed by the merge, matching main''s runner); harmless to EDDN (column unused there); v1 migration tests exist. errors_cleared_at never existed on main, so no legacy time-valued marks. Merge: only ARCHITECTURE.md conflicted, resolution keeps both sides; crate diffs vs each parent are only the other side''s work. API key: errors pass through hide() and tests assert it is absent; no new key paths. cargo test -p astromech-eddn -p astromech-core passes. Not checked: the full workspace, the UI. Minor note, no action: ARCHITECTURE.md EDSM line is very long.'
target: astromech-ky3d
identity: review-agent (sonnet, independent)
commit: 357397d78c8f5628f199da716280a7efd2122076
created_at: 2026-10-17T05:44:35.708282619Z
event:
  event: approval
---

Re-review of 4025b1c and merge 23e2be4 at HEAD. bo8g: refusals counted in their own column (v4), outages/203s no longer count; reverting to attempts makes outages_do_not_use_up_a_messages_refusals fail. nfuc: errors-cleared mark is a send_log row id; reverting to wall-1 makes a_key_that_is_still_wrong_shows_its_error_from_the_same_pass fail; app.rs test covers the status path. v4 is a plain ALTER ADD COLUMN with default, no BEGIN/COMMIT (v2's COMMIT removed by the merge, matching main's runner); harmless to EDDN (column unused there); v1 migration tests exist. errors_cleared_at never existed on main, so no legacy time-valued marks. Merge: only ARCHITECTURE.md conflicted, resolution keeps both sides; crate diffs vs each parent are only the other side's work. API key: errors pass through hide() and tests assert it is absent; no new key paths. cargo test -p astromech-eddn -p astromech-core passes. Not checked: the full workspace, the UI. Minor note, no action: ARCHITECTURE.md EDSM line is very long.
