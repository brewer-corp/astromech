---
created_at: "2026-10-16T20:01:27.410847511+00:00"
id: "astromech-2d1l"
evidence_type: "validation"
commit: "79fbb8e9f9c5633bfd19b8439e338ba698fe9708"
result: "pass"
captured_at: "2026-10-16T20:01:27.410745872+00:00"
target:
  kind: "issue"
  id: "astromech-z4jb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-z4jb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review R1/R2/R4 at 79fbb8e. R1: with the sample's fourth ship made a stored Imperial Cutter (no name, CU-01) 'In transit to Synuefe XR-H d11-102' (temporary fixture edit, restored), 4 cards at 1200x800: before the fix /tmp/3njr-r1-before.png shows the location over 'Imperial Cutter'; after, /tmp/3njr-r1-after.png shows 'In transit to' with the chip wrapped below and its name ellipsized, Copy intact. Measured in Chrome: on every card the type and location boxes don't intersect and the location ends inside the card (also on the normal sample, /tmp/3njr-cards-final.png). jsdom has no layout, so the component test checks the full system name is on hover instead. R2: View.test.ts shows the page note when loadHeadlines rejects. R4: links are named e.g. 'CU-01, Imperial Cutter', '#12, Asp Explorer' (component test and Chrome)."
updated_at: "2026-10-16T20:01:27.411481184+00:00"
---

Review R1/R2/R4 at 79fbb8e. R1: with the sample's fourth ship made a stored Imperial Cutter (no name, CU-01) 'In transit to Synuefe XR-H d11-102' (temporary fixture edit, restored), 4 cards at 1200x800: before the fix /tmp/3njr-r1-before.png shows the location over 'Imperial Cutter'; after, /tmp/3njr-r1-after.png shows 'In transit to' with the chip wrapped below and its name ellipsized, Copy intact. Measured in Chrome: on every card the type and location boxes don't intersect and the location ends inside the card (also on the normal sample, /tmp/3njr-cards-final.png). jsdom has no layout, so the component test checks the full system name is on hover instead. R2: View.test.ts shows the page note when loadHeadlines rejects. R4: links are named e.g. 'CU-01, Imperial Cutter', '#12, Asp Explorer' (component test and Chrome).
