---
schema: atelier.review
schema_version: 2
id: astromech-pscb
title: approval on astromech-h95t @88bf362a
target: astromech-h95t
identity: claude-reviewer-h95t-pin
commit: 88bf362acc8106f9d32aa067fa28ad30d5d9e16e
created_at: 2026-11-08T16:27:40.249779357Z
event:
  event: approval
---

Reviewed 18282b11..88bf362a. git diff 18282b11 88bf362a outside .atelier is only .github/workflows/atelier.yml:26, ATELIER_REV 12efdfd9 -> 4910be4563e9466e561af11029f46e8e8e1d5a3e; the other change is the new ticked Done when line. 4910be45 is on spadaval/atelier origin/master ('Merge atelier-stnv into master') and descends from 12efdfd9; its rust-toolchain.toml is still channel 1.95.0, matching ATELIER_TOOLCHAIN; the cache key (atelier-OS-ATELIER_REV-ATELIER_TOOLCHAIN) follows the rev. PR #225 run 42498130669 at 88bf362a missed the cache (key atelier-Linux-4910be45...-1.95.0), installed atelier-cli v0.3.1 at #4910be45, and doctor passed with 400 done claims. Locally atelier 0.3.1+4910be45: check passes, doctor 400 claims hold. The new Done when line is accurate. No other workflow pins atelier (ci.yml only path-filters .atelier/**; .claude/settings.json hooks call the local binary). Residual: main at 57ff1abe still pins 12efdfd9 while it already holds schema-4 review records (e.g. .atelier/reviews/astromech-6wwr.md), so main's Atelier job stays red until this branch merges.
