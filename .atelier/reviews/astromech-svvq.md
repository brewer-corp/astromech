---
schema: atelier.review
schema_version: 1
id: astromech-svvq
title: 'Read diff; replayed() keeps pending and spoken, resets armed to state.logged_in; a live LoadGame still resets all. New replay test fails with the fix reverted (12 pass, 1 fail) and passes with it. Existing guarantees (fresh launch silent, crashed session, mid-session start once, pending kept across replay) covered by passing tests; cargo test -p astromech-announce -p astromech-core green. Docs updated. PR #86 checks pass.'
target: astromech-4ed7
identity: review-agent (sonnet, independent)
commit: eaf0cacb11d0273623c652dee5c1e28eb7b723b5
created_at: 2026-10-17T07:01:37.880253239Z
event:
  event: approval
---

Read diff; replayed() keeps pending and spoken, resets armed to state.logged_in; a live LoadGame still resets all. New replay test fails with the fix reverted (12 pass, 1 fail) and passes with it. Existing guarantees (fresh launch silent, crashed session, mid-session start once, pending kept across replay) covered by passing tests; cargo test -p astromech-announce -p astromech-core green. Docs updated. PR #86 checks pass.
