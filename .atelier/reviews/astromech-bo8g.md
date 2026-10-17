---
schema: atelier.review
schema_version: 1
id: astromech-bo8g
title: 'R1 [high] edsm/mod.rs Answer::Refused arm: the give-up test ''q.attempts + 1 >= MAX_ATTEMPTS'' counts tries from Hold too (Hold bumps attempts on every outage/203 retry and never gives up). After 7 held tries (a few hours of 503s or a wrong key), the FIRST refusal (e.g. a Cloudflare 403 or an HTML 200 page as EDSM comes back) gives up the whole batch of up to 50 events and logs ''50 events refused 8 times''. Reproduced in a scratch test: 7x HTTP 503 then one 403 -> gave_up = all queued. Data loss and a false log line. Fix: count refusals separately from held tries (own column or reset/cap attempts when a Hold settles), and add this scenario as a test.'
target: astromech-ky3d
identity: review-agent (opus, independent)
commit: 769a9dea096cd5fb96e00ce173c87c92fc0134b2
created_at: 2026-10-17T05:37:39.329175606Z
event:
  event: finding
  recommendation: fix_now
---

R1 [high] edsm/mod.rs Answer::Refused arm: the give-up test 'q.attempts + 1 >= MAX_ATTEMPTS' counts tries from Hold too (Hold bumps attempts on every outage/203 retry and never gives up). After 7 held tries (a few hours of 503s or a wrong key), the FIRST refusal (e.g. a Cloudflare 403 or an HTML 200 page as EDSM comes back) gives up the whole batch of up to 50 events and logs '50 events refused 8 times'. Reproduced in a scratch test: 7x HTTP 503 then one 403 -> gave_up = all queued. Data loss and a false log line. Fix: count refusals separately from held tries (own column or reset/cap attempts when a Hold settles), and add this scenario as a test.
