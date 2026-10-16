---
created_at: "2026-10-16T19:22:32.797052786+00:00"
id: "astromech-tc3d"
evidence_type: "validation"
commit: "318f1bf8937e3c832a2cc9a86e36311141ff6ae1"
result: "pass"
captured_at: "2026-10-16T19:22:32.796948871+00:00"
target:
  kind: "issue"
  id: "astromech-3njr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3njr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Coriolis references for the plan cases: scripts/coriolis-ref (built fresh in /tmp/3njr-coriolis against /tmp/coriolis-ref at coriolis 68c042ca / coriolis-data 0db9234b, the commits tests/coriolis.rs names) run with RUST=1 on crates/astromech-ship/tests/fixtures/krait_mkii_combat.json and plans/*.json. Every variant imported cleanly (no rebuild/blueprint/experimental errors); the base reproduced the committed krait_mkii_combat block exactly; the five plan blocks are pasted into tests/coriolis.rs as plan_*. Changes against current: Cluster Capacitors WEP cap 92.2->105.4, recharge 9->8.6, drain 1:10->1:01; Super Conduits at 100% 93.7 / 9.1 / 1:18; FSD Faster boot max jump 24.87->18.71; 3 thermal boosters thermal 36.2->43.9 % and shield 1428->1197 MJ; 4 boosters 49.5 % and 967 MJ."
updated_at: "2026-10-16T19:22:32.797566235+00:00"
---

Coriolis references for the plan cases: scripts/coriolis-ref (built fresh in /tmp/3njr-coriolis against /tmp/coriolis-ref at coriolis 68c042ca / coriolis-data 0db9234b, the commits tests/coriolis.rs names) run with RUST=1 on crates/astromech-ship/tests/fixtures/krait_mkii_combat.json and plans/*.json. Every variant imported cleanly (no rebuild/blueprint/experimental errors); the base reproduced the committed krait_mkii_combat block exactly; the five plan blocks are pasted into tests/coriolis.rs as plan_*. Changes against current: Cluster Capacitors WEP cap 92.2->105.4, recharge 9->8.6, drain 1:10->1:01; Super Conduits at 100% 93.7 / 9.1 / 1:18; FSD Faster boot max jump 24.87->18.71; 3 thermal boosters thermal 36.2->43.9 % and shield 1428->1197 MJ; 4 boosters 49.5 % and 967 MJ.
