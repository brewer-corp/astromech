---
schema: atelier.review
schema_version: 1
id: astromech-pyfv
title: 'R4 [info] crates/astromech-core/src/pipeline.rs:601 / evidence astromech-fmkt: the 5gxp test passes against main''s rule (later() || classify == Game): classify already judges the new-scheme journal Game by its name stamp. So the test fails only for later() alone, and this branch fixes no live bug for 5gxp. Removing later() is behaviour-equivalent, because apply_class ignores game_live for Gated and Foreign. Keep the test as a regression guard. When closing 5gxp, say the bug was already fixed on main (classify) rather than citing this branch''s change.'
target: astromech-g3qt
identity: review-agent-g3qt-1
commit: 98e6827717c46e9692d70b475f2300e4eb9f4b51
created_at: 2026-10-22T21:06:26.686771342Z
event:
  event: finding
  recommendation: no_action
---

R4 [info] crates/astromech-core/src/pipeline.rs:601 / evidence astromech-fmkt: the 5gxp test passes against main's rule (later() || classify == Game): classify already judges the new-scheme journal Game by its name stamp. So the test fails only for later() alone, and this branch fixes no live bug for 5gxp. Removing later() is behaviour-equivalent, because apply_class ignores game_live for Gated and Foreign. Keep the test as a regression guard. When closing 5gxp, say the bug was already fixed on main (classify) rather than citing this branch's change.
