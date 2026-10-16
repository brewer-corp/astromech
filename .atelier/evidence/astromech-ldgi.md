---
created_at: "2026-10-16T21:13:41.108781505+00:00"
id: "astromech-ldgi"
evidence_type: "validation"
commit: "e8f5ccb7f708ca6627d0921f236e62dcb62f01ac"
result: "pass"
captured_at: "2026-10-16T21:13:41.108713188+00:00"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-fdyn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-fdyn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Discrimination check at e8f5ccb: with Module::engineer forced to ignore Modifiers, fer_de_lance_modifiers, anaconda_legacy and plan_legacy_fsd_long_range fail (legacy jump_max 67.89 vs Coriolis 71.16; FdL shield_thermal 42.0 vs 37.5), so the new tests exercise the Modifiers path. Expected values come from Coriolis 68c042ca / coriolis-data 0db9234b via scripts/coriolis-ref on the fixtures. Limit: fixtures are synthetic; label names and units follow coriolis-data's modifierActions and the game's documented journal format, not a real journal."
updated_at: "2026-10-16T21:13:41.109227989+00:00"
---

Discrimination check at e8f5ccb: with Module::engineer forced to ignore Modifiers, fer_de_lance_modifiers, anaconda_legacy and plan_legacy_fsd_long_range fail (legacy jump_max 67.89 vs Coriolis 71.16; FdL shield_thermal 42.0 vs 37.5), so the new tests exercise the Modifiers path. Expected values come from Coriolis 68c042ca / coriolis-data 0db9234b via scripts/coriolis-ref on the fixtures. Limit: fixtures are synthetic; label names and units follow coriolis-data's modifierActions and the game's documented journal format, not a real journal.
