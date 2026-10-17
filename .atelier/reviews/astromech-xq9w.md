---
schema: atelier.review
schema_version: 1
id: astromech-xq9w
title: 'Pre-existing, not worsened: Rule::replayed still resets ''spoken'' (and re-derives ''armed'' from the replayed state), so after a summary is said, a mid-run replay whose state ends logged in (e.g. re-reading an older journal that ends in a LoadGame, as after a crash) followed by the live Shutdown of the same session says the summary a second time, breaking ''a Shutdown after a return to the menu says nothing new''. Reproduced with a scratch test on this branch (LOAD_GAME, MENU, tick -> said; replay LOAD_GAME; live SHUTDOWN, tick -> said again); main behaves the same. Suggest a follow-up to keep ''spoken'' across post-live replays (or ignore replays once live) with a test.'
target: astromech-hk7e
identity: review-agent (opus, independent)
commit: 1baa007d76cef6bfbad14b771352d4bf94480782
created_at: 2026-10-17T06:25:25.445742951Z
event:
  event: finding
  recommendation: defer
---

Pre-existing, not worsened: Rule::replayed still resets 'spoken' (and re-derives 'armed' from the replayed state), so after a summary is said, a mid-run replay whose state ends logged in (e.g. re-reading an older journal that ends in a LoadGame, as after a crash) followed by the live Shutdown of the same session says the summary a second time, breaking 'a Shutdown after a return to the menu says nothing new'. Reproduced with a scratch test on this branch (LOAD_GAME, MENU, tick -> said; replay LOAD_GAME; live SHUTDOWN, tick -> said again); main behaves the same. Suggest a follow-up to keep 'spoken' across post-live replays (or ignore replays once live) with a test.
