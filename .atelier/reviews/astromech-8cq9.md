---
schema: atelier.review
schema_version: 1
id: astromech-8cq9
title: Shopping list entries carry 'until' (have + collect) and the rules filter by the materials ledger, so collected materials drop out with the app closed (test a_material_collected_up_to_its_count_drops_off_the_list, fails without the filter); empty plans already sent an empty list, and a ship that left the fleet now sends one too.
target: astromech-77a1
identity: orchestrator (Claude, not a code author)
commit: 9f945111d1193b4d9ec0b67802eba8dcc70b0c41
created_at: 2026-10-17T02:12:28.627652194Z
event:
  event: disposition
  finding: astromech-irtf
  decision: fix_now
  supersedes: null
  resolution_commit: 9f945111d1193b4d9ec0b67802eba8dcc70b0c41
  follow_up: null
---

Shopping list entries carry 'until' (have + collect) and the rules filter by the materials ledger, so collected materials drop out with the app closed (test a_material_collected_up_to_its_count_drops_off_the_list, fails without the filter); empty plans already sent an empty list, and a ship that left the fleet now sends one too.
