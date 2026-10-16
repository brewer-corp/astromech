---
created_at: "2026-10-16T22:10:17.282281285+00:00"
id: "astromech-lcsd"
evidence_type: "validation"
commit: "f8c900b66d63f1a7cbc6712e74b8ab92dd1b028c"
result: "pass"
captured_at: "2026-10-16T22:10:17.282220331+00:00"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-qhi4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qhi4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Spot-check of every ship engineer's top grade per module against the Elite Dangerous wiki Engineers table (fetched via the wiki API 16 Oct 2026): before corrections 135 agree, 8 differ (coriolis-data gives the manifest scanner the surface scanner's engineers: Bill Turner, Juri Ishmaak, Lori Jameson G5 vs G3, Hera Tani, Felicity Farseer, Lei Cheung listed though they don't offer it; Ram Tah hatch breaker G4 vs G3, confirmed on his own page). After the generator corrections: 139 agree, 1 differs (Lori Jameson kill warrant scanner G3: missing from the Engineers table but on her own wiki page, so ours is right). Repeat with scripts/check-engineer-grades.py."
updated_at: "2026-10-16T22:10:17.282596222+00:00"
---

Spot-check of every ship engineer's top grade per module against the Elite Dangerous wiki Engineers table (fetched via the wiki API 16 Oct 2026): before corrections 135 agree, 8 differ (coriolis-data gives the manifest scanner the surface scanner's engineers: Bill Turner, Juri Ishmaak, Lori Jameson G5 vs G3, Hera Tani, Felicity Farseer, Lei Cheung listed though they don't offer it; Ram Tah hatch breaker G4 vs G3, confirmed on his own page). After the generator corrections: 139 agree, 1 differs (Lori Jameson kill warrant scanner G3: missing from the Engineers table but on her own wiki page, so ours is right). Repeat with scripts/check-engineer-grades.py.
