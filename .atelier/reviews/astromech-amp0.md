---
schema: atelier.review
schema_version: 1
id: astromech-amp0
title: 'Low, pre-existing, not introduced here: crates/astromech-eddn/src/edsm/mod.rs deliver(). A 5xx outage (Cloudflare 502/503 count as Refused) gives most queued events a refusal, and refused events always go one per request. After recovery, the queue drains one HTTP request per event. A 60-minute simulated outage with 201 queued took 171 requests in one pass, the same on main and HEAD. The courier ignores EDSM''s X-Rate-Limit-Remaining/Reset headers, which EDMC''s EDSM plugin honours. A 429 is Hold, so nothing is lost, but recovery bursts against EDSM''s limit. Suggest a follow-up: count refusals from a 5xx with nothing stored in the pass as outage tries, not refusals, or batch refused events again after a successful pass, and honour the rate-limit headers.'
target: astromech-ux0h
identity: review-agent-ux0h-1
commit: fa52c077d4e9350a5872eee56189099ec1c3ece3
created_at: 2026-10-22T21:07:45.910399904Z
event:
  event: finding
  recommendation: defer
---

Low, pre-existing, not introduced here: crates/astromech-eddn/src/edsm/mod.rs deliver(). A 5xx outage (Cloudflare 502/503 count as Refused) gives most queued events a refusal, and refused events always go one per request. After recovery, the queue drains one HTTP request per event. A 60-minute simulated outage with 201 queued took 171 requests in one pass, the same on main and HEAD. The courier ignores EDSM's X-Rate-Limit-Remaining/Reset headers, which EDMC's EDSM plugin honours. A 429 is Hold, so nothing is lost, but recovery bursts against EDSM's limit. Suggest a follow-up: count refusals from a 5xx with nothing stored in the pass as outage tries, not refusals, or batch refused events again after a successful pass, and honour the rate-limit headers.
