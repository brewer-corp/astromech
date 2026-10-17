---
schema: atelier.review
schema_version: 1
id: astromech-rtq0
title: 'R3 [medium, test gap] crates/astromech-announce/src/biology.rs:174-176: mutation test. Replacing the per-system reset (*said = Said{..}) with only said.system = ... passes the whole astromech-announce suite. Without the reset, said.summary and said.mapped carry into the next system, so an identical summary or DSS line there (e.g. ''A 2: Stratum, up to 19 M.'' after a jump) is suppressed, and waiting_until from the old system persists. Need a test in tests/biology.rs: same lines in system 1, jump to system 2, same lines are said again; and a stale late answer for system 1 is not said after jumping to system 2. All other mutations tried (summary dedupe, DSS dedupe, late-answer expiry) were caught.'
target: astromech-a63r
identity: 'review-agent (sonnet, independent, part B: callouts, state, UI)'
commit: dcd988bc1ffbf9a6c4171078e4b65c1ee60fa48f
created_at: 2026-10-17T05:53:47.002269722Z
event:
  event: finding
  recommendation: fix_now
---

R3 [medium, test gap] crates/astromech-announce/src/biology.rs:174-176: mutation test. Replacing the per-system reset (*said = Said{..}) with only said.system = ... passes the whole astromech-announce suite. Without the reset, said.summary and said.mapped carry into the next system, so an identical summary or DSS line there (e.g. 'A 2: Stratum, up to 19 M.' after a jump) is suppressed, and waiting_until from the old system persists. Need a test in tests/biology.rs: same lines in system 1, jump to system 2, same lines are said again; and a stale late answer for system 1 is not said after jumping to system 2. All other mutations tried (summary dedupe, DSS dedupe, late-answer expiry) were caught.
