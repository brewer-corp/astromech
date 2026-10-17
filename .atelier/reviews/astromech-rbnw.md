---
schema: atelier.review
schema_version: 1
id: astromech-rbnw
title: 'Wrong/stale session concern checked: the kept pending entry carries the commander and journal timestamp captured from the live ending line, and the tick queries the history for exactly that (commander, at) and requires end >= ts, so a replay that switches commander or contains another LoadGame/Shutdown cannot redirect it. Scratch test (replay of another commander''s LoadGame+Shutdown during the wait) spoke once with every history query pinned to (Test, 21:00:05Z). The ''since'' wait clock rides inside pending, so the 30 s give-up still applies across a replay (scratch test confirmed). Optional: the branch''s test could replay another commander''s lines to pin this, but not required.'
target: astromech-hk7e
identity: review-agent (opus, independent)
commit: 1baa007d76cef6bfbad14b771352d4bf94480782
created_at: 2026-10-17T06:25:25.612887065Z
event:
  event: finding
  recommendation: no_action
---

Wrong/stale session concern checked: the kept pending entry carries the commander and journal timestamp captured from the live ending line, and the tick queries the history for exactly that (commander, at) and requires end >= ts, so a replay that switches commander or contains another LoadGame/Shutdown cannot redirect it. Scratch test (replay of another commander's LoadGame+Shutdown during the wait) spoke once with every history query pinned to (Test, 21:00:05Z). The 'since' wait clock rides inside pending, so the 30 s give-up still applies across a replay (scratch test confirmed). Optional: the branch's test could replay another commander's lines to pin this, but not required.
