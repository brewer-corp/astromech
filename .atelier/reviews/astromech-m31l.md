---
schema: atelier.review
schema_version: 1
id: astromech-m31l
title: 'Reviewed git diff main...HEAD at 98e68277 (pipeline.rs, background.rs, core history.rs, astromech-history lib/tests, ARCHITECTURE.md, .atelier records) against the five child records and ARCHITECTURE''s replay/live rules. Reran: cargo test -p astromech-core --lib (171 pass), cargo test -p astromech-history (29 pass), cargo fmt --check, clippy -D warnings on core+history (clean), scripts/check-deps.py (OK). Mutation checks in a scratch copy: dropping note_startup_lengths fails the nrhv test; ignoring the remembered live flag, or retrying every pending trigger as replay, fails the 01c0 test; dropping the metadata fallback fails the 2h0q test; no expiry / 10 s back-off fails both background tests. Restoring main''s later()||classify rule does NOT fail the 5gxp test: the bug was already fixed by classify on main; the change is a behaviour-equivalent simplification. Limits: Linux only, no Windows run; Windows listing-lag and file-locking behaviour judged by reading; cargo test --workspace not rerun.'
target: astromech-g3qt
identity: review-agent-g3qt-1
commit: 98e6827717c46e9692d70b475f2300e4eb9f4b51
created_at: 2026-10-22T21:06:25.475739662Z
event:
  event: review
---

Reviewed git diff main...HEAD at 98e68277 (pipeline.rs, background.rs, core history.rs, astromech-history lib/tests, ARCHITECTURE.md, .atelier records) against the five child records and ARCHITECTURE's replay/live rules. Reran: cargo test -p astromech-core --lib (171 pass), cargo test -p astromech-history (29 pass), cargo fmt --check, clippy -D warnings on core+history (clean), scripts/check-deps.py (OK). Mutation checks in a scratch copy: dropping note_startup_lengths fails the nrhv test; ignoring the remembered live flag, or retrying every pending trigger as replay, fails the 01c0 test; dropping the metadata fallback fails the 2h0q test; no expiry / 10 s back-off fails both background tests. Restoring main's later()||classify rule does NOT fail the 5gxp test: the bug was already fixed by classify on main; the change is a behaviour-equivalent simplification. Limits: Linux only, no Windows run; Windows listing-lag and file-locking behaviour judged by reading; cargo test --workspace not rerun.
