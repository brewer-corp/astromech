---
schema: atelier.review
schema_version: 1
id: astromech-ehud
title: approval on astromech-az6g @32ef569f
target: astromech-az6g
identity: review-agent-az6g-1
commit: 32ef569f95400a7642fd26e6baa8d5c17e458182
created_at: 2026-11-04T21:29:29.932723806Z
event:
  event: approval
---

Reviewed a61ca2b9..32ef569f against the done-when, astromech-sklz, the strategy's EDDN rules and CONTRIBUTING's guide rule. Ran cargo test -p astromech-eddn: 151+2 pass, including after_an_hour_of_503s_the_queue_drains_in_batches and no_request_goes_while_edsms_rate_limit_is_used_up. clippy -D warnings, fmt --check and check-guide.py (0 problems) are clean, and cargo check -p astromech-core compiles. Batching: no event is lost, duplicated or retried forever. A refused batch's whole failure counts no try, but it can happen only once per pass before refusals go alone and count, so astromech-sklz's give-up still applies. Rate limit: bad or missing headers fall back to 60 s or an hour at most, or are ignored. If the header meanings differ from the forum post, the worst case is a stall of an hour or less, or a 429 that is held, never lost data. transport::Response.headers is used only in astromech-eddn; the other senders use astromech_data::Response. The guide was updated. Not done: confirming the tests fail without the fix (code is read-only here). Deferred findings: astromech-wesu, ys8x, kusu.
