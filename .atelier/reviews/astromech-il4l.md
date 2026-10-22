---
schema: atelier.review
schema_version: 1
id: astromech-il4l
title: 'Reviewed git diff main...HEAD (fa52c077) against astromech-dxv1, astromech-st8o, astromech-otsm Done-when, ARCHITECTURE and the data-sharing guide. Reran cargo test -p astromech-eddn (146 lib + 2 documented_limits pass), clippy -D warnings on astromech-eddn, cargo fmt --check, on an exported copy of HEAD. Mutations on that copy: reverting Outbox::due to ORDER BY id fails three_poisoned_events_side_by_side_do_not_hold_the_good_ones_behind and the outage test; removing failed_alone = 0 fails an_event_stored_between_poisoned_ones_resets_the_outage_guard. Ran a 60-minute 503 outage at one pass a minute with 3 live events a minute, then recovery, on both main and HEAD: identical (0 given up, 165 of 201 refused, 171 requests to drain, no stored events out of id order). otsm test builds 8, 30, 5 and 2,000 from the constants and matches whole sentences; the page sentence is in the EDDN section. Limits: EDSM''s api-journal-v1 page is behind a Cloudflare challenge, so EDSM''s ordering and duplicate (101-103) rules were not read first-hand; EDMC''s EDSM plugin was read instead. check-deps and check-guide not rerun.'
target: astromech-ux0h
identity: review-agent-ux0h-1
commit: fa52c077d4e9350a5872eee56189099ec1c3ece3
created_at: 2026-10-22T21:07:45.516983206Z
event:
  event: review
---

Reviewed git diff main...HEAD (fa52c077) against astromech-dxv1, astromech-st8o, astromech-otsm Done-when, ARCHITECTURE and the data-sharing guide. Reran cargo test -p astromech-eddn (146 lib + 2 documented_limits pass), clippy -D warnings on astromech-eddn, cargo fmt --check, on an exported copy of HEAD. Mutations on that copy: reverting Outbox::due to ORDER BY id fails three_poisoned_events_side_by_side_do_not_hold_the_good_ones_behind and the outage test; removing failed_alone = 0 fails an_event_stored_between_poisoned_ones_resets_the_outage_guard. Ran a 60-minute 503 outage at one pass a minute with 3 live events a minute, then recovery, on both main and HEAD: identical (0 given up, 165 of 201 refused, 171 requests to drain, no stored events out of id order). otsm test builds 8, 30, 5 and 2,000 from the constants and matches whole sentences; the page sentence is in the EDDN section. Limits: EDSM's api-journal-v1 page is behind a Cloudflare challenge, so EDSM's ordering and duplicate (101-103) rules were not read first-hand; EDMC's EDSM plugin was read instead. check-deps and check-guide not rerun.
