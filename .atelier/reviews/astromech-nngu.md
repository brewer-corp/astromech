---
schema: atelier.review
schema_version: 2
id: astromech-nngu
title: approval on astromech-bp23 @1aa6880b
target: astromech-bp23
identity: claude-reviewer-se44
commit: 1aa6880b69307cd219877f7d80afa6bc5dda2431
created_at: 2026-11-07T09:21:25.511873638Z
event:
  event: approval
---

Re-review of the astromech-t9w7 fix (4bf9f29f), ff73c9da (se44 Done when reworded and ticked), d77370b0 (astromech-x6hc for cn58) and dispositions 39dabe77/1aa6880b. Fixture provenance: in a scratch worktree at 960792f2 (json! replies) with 4bf9f29f's test applied, ASTROMECH_UPDATE_REPLIES=1 under HOME=/tmp/... wrote a file byte-identical to HEAD's fixture. The HEAD test passes under that other HOME and under the normal one. Masks: paths masked on the parsed Value with / separators (so Windows-escaped JSON no longer matters); folders all under the temp dir; autostart masked only when present; install/install_source mask changes values only today (keys always serialised; info finding on the unguarded IndexMut); mic_list/mic_check left out on Windows only, from both sides. Fixture diff vs 791ff421 is only those five values; no home path left. Reworded Done when 2 is accurate: the core's bindings test covers every command (missing, differs, stale), -p astromech-api its own part. x6hc is in backlog with a matching Done when. Fresh: cargo test -p astromech-core 323 passed 0 failed; clippy -p astromech-core --all-targets -D warnings and fmt clean. Earlier pass (791ff421): workspace 1299 passed, deny/check-deps/check-guide, pnpm check/lint/test 528. Not run: Windows CI (the masked Windows paths and mic exclusion are verified by reading only).
