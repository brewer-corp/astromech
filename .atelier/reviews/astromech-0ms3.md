---
schema: atelier.review
schema_version: 1
id: astromech-0ms3
title: 'Read the diff, issue, pipeline catch-up and replay paths, History::session and commander attribution. Engine change: replayed() is called only when not live, returns nothing so no rule can speak or start a cooldown in the catch-up; the default is a no-op and only session_summary overrides it; cost is one virtual call per enabled rule per replayed input, negligible. A simpler route was considered (arm on the first live input from state) but the state after a first live Shutdown is already logged out, so a hook with the catch-up state is justified; a single end-of-catch-up notification would be tighter (R3). Mid-session start with a live Shutdown speaks once (tested; mutating the arming to false fails the test). Ran astromech-announce and astromech-gamestate tests (110 pass) and the copy test (no new copy). Blocking: crash-then-relaunch regression (R1) and the untested Shutdown clear (R2); R4 doc. Not approved.'
target: astromech-627q
identity: review-agent (opus, independent)
commit: 4910e731b0539fd52abe0edc195f7fe40f8d6ebe
created_at: 2026-10-17T05:38:06.819864985Z
event:
  event: review
---

Read the diff, issue, pipeline catch-up and replay paths, History::session and commander attribution. Engine change: replayed() is called only when not live, returns nothing so no rule can speak or start a cooldown in the catch-up; the default is a no-op and only session_summary overrides it; cost is one virtual call per enabled rule per replayed input, negligible. A simpler route was considered (arm on the first live input from state) but the state after a first live Shutdown is already logged out, so a hook with the catch-up state is justified; a single end-of-catch-up notification would be tighter (R3). Mid-session start with a live Shutdown speaks once (tested; mutating the arming to false fails the test). Ran astromech-announce and astromech-gamestate tests (110 pass) and the copy test (no new copy). Blocking: crash-then-relaunch regression (R1) and the untested Shutdown clear (R2); R4 doc. Not approved.
