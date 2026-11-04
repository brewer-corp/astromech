---
schema: atelier.review
schema_version: 1
id: astromech-9x8t
title: approval on astromech-zftk @b98a8650
target: astromech-zftk
identity: review-agent-zftk-1
commit: b98a8650910957a8ba247ee436f87d8ccc6b1244
created_at: 2026-11-04T23:08:39.999955254Z
event:
  event: approval
---

Re-reviewed c34aafb1..b98a8650910957a8ba247ee436f87d8ccc6b1244 (36f1203f fix for R1 astromech-6rig and R2 astromech-p58u, plus records). R1: the followed journal is measured only through a handle, else UNMEASURED (u64::MAX), so the catch-up replays all it reads (fail closed). R2: open() leaves an unreadable journal for later with tail None and its opened length kept, so follow_newest opens it with replay=true and the split at the startup length, or all history if UNMEASURED. Open item (tail None, follow_newest may follow a touched older journal until the game's is readable) is not a regression: before, the unreadable game's journal had no start_time, so begins_before was false and follow_newest switched to the touched journal too, and its whole history later went live; now it is only state staleness, fail closed. Ran as uid 1000 (chmod 000 confirmed unreadable on /tmp tmpfs): cargo test -p astromech-core -p astromech-journal, 191 core unit tests ok, including the three new chmod tests (two assert tail.is_none(), which only the unreadable path gives; the unmeasured one would send the post-measure line under the previous code). Clippy -D warnings and fmt are clean. One low no_action finding (rollover/flush bookkeeping on failed opens).
