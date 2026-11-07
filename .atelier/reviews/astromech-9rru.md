---
schema: atelier.review
schema_version: 2
id: astromech-9rru
title: finding on astromech-vucf @86b4d3e8
target: astromech-vucf
identity: claude-reviewer-xk0n
commit: 86b4d3e81bb36e6f278617b25d59ae0d32c6e4b9
created_at: 2026-11-07T17:13:08.847327513Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-data/src/replay.rs:584-604 (Tape::record) and CONTRIBUTING.md:253-258. Record mode redacts only the key-named fields and the literal secret values. A recorded flight log still holds the commander's dated travel history verbatim, which is personal data, and recording one means putting a real key in a world's secrets.json under worlds/. Only the prose rule in CONTRIBUTING.md and world.rs keeps either out of a commit. Two smaller gaps: the literal replace misses an echo that EDSM's PHP JSON escapes (\\uXXXX, \\/), and a short or common secret value (a commander named 'Sol') rewrites unrelated text in the body, so the replay no longer matches the real answer. Done when 4 holds as written, since it asks only for the key and commander. Suggested follow-up: a check (scripts/ or a test over worlds/) that fails a committed secrets.json whose key isn't an obvious placeholder, and a net.json holding a get-logs answer with a non-empty logs list. Or have record mode write EDSM's credentialed reads only with an explicit flag. Defer to astromech-vucf.
