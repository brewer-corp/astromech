---
created_at: "2026-10-17T02:47:04.970006119+00:00"
id: "astromech-dw8t"
evidence_type: "validation"
commit: "09fe9582eb59b2b43dd660eb6787cc5b134da783"
result: "pass"
captured_at: "2026-10-17T02:47:04.969900722+00:00"
target:
  kind: "issue"
  id: "astromech-20dy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-20dy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Spike, live Spansh 17 Oct 2026: systems search exposes is_colonised, is_being_colonised (usually omitted, not false; is_colonised:false matched nothing), population (<=> filter works), nearest_populated_distance/id64 (filter works, value lags new colonies: Greae Phoea KM-W b47-47 said 118.8 ly while 8.3 ly from populated colony MY-G d11-909; VS-M b39-29 said 27.4 ly, 9.5 ly from EM-K d9-675). Populated colony EM-K d9-373 has is_being_colonised and no is_colonised, so sources are selected by population only. All 2612 colonised pop-0 systems are still being colonised (claims). Bodies search system_id64 filter scopes exactly (rings, is_landable, is_main_star present). Page size caps at 500. Game rule: target unpopulated, within 15 ly of a populated system with a Colonisation Contact (elitedangerous.com System Colonisation Guide)."
updated_at: "2026-10-17T02:47:04.970730991+00:00"
---

Spike, live Spansh 17 Oct 2026: systems search exposes is_colonised, is_being_colonised (usually omitted, not false; is_colonised:false matched nothing), population (<=> filter works), nearest_populated_distance/id64 (filter works, value lags new colonies: Greae Phoea KM-W b47-47 said 118.8 ly while 8.3 ly from populated colony MY-G d11-909; VS-M b39-29 said 27.4 ly, 9.5 ly from EM-K d9-675). Populated colony EM-K d9-373 has is_being_colonised and no is_colonised, so sources are selected by population only. All 2612 colonised pop-0 systems are still being colonised (claims). Bodies search system_id64 filter scopes exactly (rings, is_landable, is_main_star present). Page size caps at 500. Game rule: target unpopulated, within 15 ly of a populated system with a Colonisation Contact (elitedangerous.com System Colonisation Guide).
