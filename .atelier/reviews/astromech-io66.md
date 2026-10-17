---
schema: atelier.review
schema_version: 1
id: astromech-io66
title: 'Review of 3016a38: no FIX NOW; behaviour of tail and JournalWatch preserved. Nits accepted without change: a flaky core test gear_down.rs:545 (sleep-based, passes on main and the branch on rerun); shrink check compares against the raw offset, not the committed one (narrow window); lines re-read after a shrink are not double counted. Not covered by tests: committed() after a restart; shrink plus partial line in one poll.'
target: astromech-p4um
identity: p4um-reviewer-1
commit: 3016a38cbc2f5f1ad89e54e688a5e24cf30b272d
created_at: 2026-10-17T18:39:43.506296673Z
event:
  event: review
---

Review of 3016a38: no FIX NOW; behaviour of tail and JournalWatch preserved. Nits accepted without change: a flaky core test gear_down.rs:545 (sleep-based, passes on main and the branch on rerun); shrink check compares against the raw offset, not the committed one (narrow window); lines re-read after a shrink are not double counted. Not covered by tests: committed() after a restart; shrink plus partial line in one poll.
