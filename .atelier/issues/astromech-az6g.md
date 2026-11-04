---
created_at: "2026-10-22T21:08:14.143466459+00:00"
id: "astromech-az6g"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-az6g
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T21:22:23.863838202+00:00"
status: "done"
title: "EDSM: drain refused events in batches after an outage and honour EDSM's rate-limit headers"
updated_at: "2026-11-04T21:22:23.863838202+00:00"
---

## Goal

After an EDSM outage (Cloudflare 502/503 count as refusals), the queue drains without one request per event, and the courier respects EDSM's X-Rate-Limit-Remaining and X-Rate-Limit-Reset headers as EDMC's EDSM plugin does.

## Done when

A test simulating a 60-minute 503 outage then recovery empties the queue in far fewer requests than events (refused events batched again after a pass that stored something, or 5xx refusals in a pass that stored nothing counted as outage tries), and a test shows the courier waiting when X-Rate-Limit-Remaining is 0 until X-Rate-Limit-Reset.

## Context

Review finding R2 (astromech-amp0) on feature/astromech-ux0h; existed before that branch. The reviewer's simulation: 171 requests to drain 201 events after recovery. crates/astromech-eddn/src/edsm/mod.rs, deliver().

## Notes

- 2026-11-04 [decision] Drain after an outage by batching refused events again once a request in the same pass has stored something (they go in batches of their own, after the never-refused ones); if such a batch fails as a whole, the rest of that pass sends refused events alone again, with no try counted for the batch failure. Chose this over counting 5xx refusals in a pass that stored nothing as outage tries.
  Reason: Counting 5xx alone-failures as outage tries would let a poisoned event that is the only one queued hold forever, against astromech-sklz (request-level 5xx is given up after the attempt limit). Batching only after a store keeps refusals as they are. Falling back to alone (not halving and stopping) keeps astromech-dxv1: good events between known-poisoned ones still all go in the same pass. Cost: one extra request per pass while poisoned events are due. Simulated 60-min 503 outage, 201 events: 168 recovery requests before, 10 after.
- 2026-11-04 [decision] X-Rate-Limit-Reset is read as seconds until EDSM's limit renews, capped at 3600 s; a missing or unreadable Reset with Remaining 0 waits 60 s. Only Remaining == 0 stops posting (in the pass and later passes, on the monotonic clock); Retry-After and X-Rate-Limit-Limit are ignored. Applied to journal posts, not to the daily discard-list GET.
  Reason: EDSM's API docs (api-v1, api-journal-v1, fetched 4 Nov 2026) don't document these headers. Observed EDSM headers (Frontier forum EDSM thread p.149, Jul 2026): x-rate-limit-limit 720, x-rate-limit-remaining 0, x-rate-limit-reset 3600, i.e. seconds in an hour window. The cap stops a bad value stalling delivery for longer than the window. The discard list is one GET a day.
- 2026-11-04 [decision] transport::Response gains headers: Vec<(String, String)> (lower-case names) with a header() lookup; only EDSM's HttpApi fills it, EDDN's HttpTransport leaves it empty.
  Reason: The EDSM Api trait returns the shared Response type; one field there is smaller than a second response type. EDDN reads no headers, so filling them would be unused code.

## Close Reason

Implementer report (commit 15f6a0bf). Changed: crates/astromech-eddn/src/edsm/mod.rs deliver(): once a request in the pass has stored something, refused events (refusals > 0, due after the others) go in batches of their own instead of one per request; if such a batch fails as a whole (Refused, n > 1), the rest of that pass sends refused events alone, with no try counted for the batch failure. New Courier.limited_until: after a journal post whose answer has X-Rate-Limit-Remaining 0, nothing is posted (this pass or later ones) until X-Rate-Limit-Reset seconds (capped 3600; 60 if missing) have passed; wait() sleeps until then. transport::Response gains headers (filled only by EDSM's HttpApi) and header(). Guide data-sharing.md and the module doc describe both. Proof: simulated 60-min 503 outage then recovery, 201 events, passes timed by the courier's own wait(): 168 recovery requests before, 10 after (test after_an_hour_of_503s_the_queue_drains_in_batches, fails without the fix); no_request_goes_while_edsms_rate_limit_is_used_up (fails without the fix). cargo test -p astromech-eddn 151+2 pass, clippy -D warnings and fmt clean, check-guide.py 0 problems. Choices: see the three decision notes (batch-after-store instead of counting 5xx as outage tries, because the latter would let a lone poisoned event hold forever against astromech-sklz; Reset read as seconds; headers on the shared Response). Deviations: none from the done-when; the drain test uses the option 'refused events batched again after a request in the same pass stored something', within the pass rather than the next pass. Surprises: EDSM's API docs don't document the rate-limit headers (the meaning comes from headers EDSM was observed sending: limit 720, remaining 0, reset 3600). Open items: the outage itself still costs about 168 requests an hour (OUTAGE_AFTER alone-failures per pass) and the rate limit is not read from the daily discard-list GET; Retry-After is ignored. Review outstanding: no independent review of this change yet.
