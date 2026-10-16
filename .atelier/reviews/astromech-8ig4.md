---
schema: atelier.review
schema_version: 1
id: astromech-8ig4
title: 'R5 [low] crates/astromech-market/tests/list.rs:49-52: now() is 2026-10-16T21:00Z, called ''the evening the fixtures were captured'', but the fixtures hold updatedAt values up to 22:42Z (Miller Depot, Boston Base, Polyakov Station), so some listings are in the future relative to now. Harmless: the age filter test uses a 7-day cut-off. Only the comment is misleading.'
target: astromech-k3wp
identity: review-agent (k3wp review)
commit: f5e62c69c573b451172bf1cd2e16cce3b6519e75
created_at: 2026-10-16T23:10:37.571331338Z
event:
  event: finding
  recommendation: no_action
---

R5 [low] crates/astromech-market/tests/list.rs:49-52: now() is 2026-10-16T21:00Z, called 'the evening the fixtures were captured', but the fixtures hold updatedAt values up to 22:42Z (Miller Depot, Boston Base, Polyakov Station), so some listings are in the future relative to now. Harmless: the age filter test uses a 7-day cut-off. Only the comment is misleading.
