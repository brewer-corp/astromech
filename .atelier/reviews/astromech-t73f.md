---
schema: atelier.review
schema_version: 1
id: astromech-t73f
title: 'At y296''s closing commit 004ec10b (ancestor of main; contains 3764f8a6, a17364cf, c0cdbf39): h9kb holds, normalise joins ''i''+''ll'' to ''iwill'' (lib.rs:470), MISHEARD iwill->all capped by min(MISHEARD_CAP=0.80) (lib.rs:808/812); all_stop_mishearings_ask_and_never_execute asserts score <= MISHEARD_CAP for I''ll/I’ll stop and I''ll stop here/now/please; the 3764f8a6..004ec10b changes (astromech-1c66 learning guard, ''pays''->''paste'' MISHEARD entry, extra ''Ill stop'' alias tests) keep and extend the cap, they don''t undo it. cargo test -p astromech-intent at 004ec10b in a scratch worktree: 23 passed, 0 failed (main: 29 passed). zdy2 holds: docs/evidence/astromech-y296/{report,beam-search-report,hotwords-report}.md present at 004ec10b and ARCHITECTURE.md links the directory. y296 issue diff 3764f8a6..004ec10b is only the close (status done, close reason).'
target: astromech-y296
identity: review-agent-y296-3
commit: 004ec10bff0e3355fe9e8258c22ff82b4fe8b156
created_at: 2026-10-22T23:13:30.432321716Z
event:
  event: approval
---

At y296's closing commit 004ec10b (ancestor of main; contains 3764f8a6, a17364cf, c0cdbf39): h9kb holds, normalise joins 'i'+'ll' to 'iwill' (lib.rs:470), MISHEARD iwill->all capped by min(MISHEARD_CAP=0.80) (lib.rs:808/812); all_stop_mishearings_ask_and_never_execute asserts score <= MISHEARD_CAP for I'll/I’ll stop and I'll stop here/now/please; the 3764f8a6..004ec10b changes (astromech-1c66 learning guard, 'pays'->'paste' MISHEARD entry, extra 'Ill stop' alias tests) keep and extend the cap, they don't undo it. cargo test -p astromech-intent at 004ec10b in a scratch worktree: 23 passed, 0 failed (main: 29 passed). zdy2 holds: docs/evidence/astromech-y296/{report,beam-search-report,hotwords-report}.md present at 004ec10b and ARCHITECTURE.md links the directory. y296 issue diff 3764f8a6..004ec10b is only the close (status done, close reason).
