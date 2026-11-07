---
schema: atelier.review
schema_version: 2
id: astromech-zum0
title: review on astromech-y6d6 @107e5c5d
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 107e5c5d56fe9802323304beffa0ca83e69e3698
created_at: 2026-11-07T19:15:22.076217511Z
event:
  event: review
---

Re-reviewed 6eae89ce..107e5c5d56fe9802323304beffa0ca83e69e3698. 0891652b fixes 2wu4: with --expect-recovery check 6 requires, before the close, ui_state open and 'window: reloading the page' (renderer) or 'window: rebuilt' plus a new page load (browser); Release passes it, and run 42401381264 at 0891652b shows both recovered in place (kind 1 reload, kind 0 'rebuilt at keybinds') and both kill-switch runs with the WM_CLOSE tap. zo5n's comments fixed. 107e5c5d (Status.json idle re-read): judged a bug fix, not a reserved in-play change: it changes no feature, input, voice, kill switch, tray or restart behaviour, only makes the existing Status.json read catch a lost notification or failed read within 250 ms, which serves the strategy's reliability-while-playing Shape; no Sumeet sign-off needed. It is outside v3u4's Scope list, recorded only as a [result] note on v3u4; acceptable on the shared y6d6 branch. Duplicate reads are harmless (StateHub::update compares state before bumping the generation and notifying; the reaction engine ignores an unchanged status; EDDN gets the same BodyName). A partial write isn't recorded, so the next idle pass retries it; the stamp is taken before the read, so a write during the read is read again. The cost is one metadata call per 250 ms idle pass. hub verify waits only gain. Same-length, same-mtime rewrites with a lost notification are still missed, as before. Ran cargo test -p astromech-core --lib pipeline:: (67/67, both new tests included) and cargo test -p astromech-mockgame. Not seen: CI on 107e5c5d (no runs yet), in particular the MVP criteria rerun; keys.log write_all change read only.
