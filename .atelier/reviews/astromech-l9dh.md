---
schema: atelier.review
schema_version: 1
id: astromech-l9dh
title: 'crates/astromech-announce/src/fuel.rs:144-152 and route.rs:105-113 (nwoo fix): the route briefing''s fuel line moved into fuel_range, so every fuel line now shares one 10 min per-rule cooldown that only a higher priority breaks (lib.rs:325), and all of them are High. Before, the route rule had no cooldown. That swallows distinct, safety-relevant warnings in both directions: (a) after an arrival ''Fuel too low to reach a scoopable star.'', replotting within 10 min gives no ''Fuel gap: 18 jumps between scoopable stars.'' for the new route; (b) a plot-time ''Fuel gap: N jumps ...'' (a full tank won''t cross some later stretch) silences the arrival ''Fuel too low to reach a scoopable star.'' for 10 min even though the tank is short right now. The nwoo finding only asked that the same warning not be said twice seconds apart. Fix: suppress only a repeat of the same text (e.g. fuel_range skips an arrival line identical to the last fuel line said within its cooldown, and a new route''s fuel line always speaks), with tests for (a) and (b).'
target: astromech-77a1
identity: review-agent (77a1 fix review, sonnet)
commit: 6256a28557b84d63e9e7158bc0d6f516b24e2647
created_at: 2026-10-17T02:18:08.511532983Z
event:
  event: finding
  recommendation: fix_now
---

crates/astromech-announce/src/fuel.rs:144-152 and route.rs:105-113 (nwoo fix): the route briefing's fuel line moved into fuel_range, so every fuel line now shares one 10 min per-rule cooldown that only a higher priority breaks (lib.rs:325), and all of them are High. Before, the route rule had no cooldown. That swallows distinct, safety-relevant warnings in both directions: (a) after an arrival 'Fuel too low to reach a scoopable star.', replotting within 10 min gives no 'Fuel gap: 18 jumps between scoopable stars.' for the new route; (b) a plot-time 'Fuel gap: N jumps ...' (a full tank won't cross some later stretch) silences the arrival 'Fuel too low to reach a scoopable star.' for 10 min even though the tank is short right now. The nwoo finding only asked that the same warning not be said twice seconds apart. Fix: suppress only a repeat of the same text (e.g. fuel_range skips an arrival line identical to the last fuel line said within its cooldown, and a new route's fuel line always speaks), with tests for (a) and (b).
