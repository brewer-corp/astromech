---
schema: atelier.review
schema_version: 1
id: astromech-x9pk
title: 'Second review at HEAD. Checked the fix commit a7647ba9 against findings g12z (panel stays mounted mid-plot: close() keeps picked while Plot.plotting()), vkpl (Tauri plotRoute notifies watchers), 67t2 (fakecore route_set clears parked, test added), nf0a (comments/docs), fngg (resume() arrives at the current system first, test added), 5gjt (wording). Skimmed the whole main...HEAD diff in crates, app/src and docs. Proof: cargo test route:: (13 pass), cargo test -p astromech-announce (all pass), vitest src/modules/route (14 pass), pnpm check (0 errors/warnings). Limits: Linux only, no real game or Tauri run. Low note, not blocking: Desk::plotted''s auto-resume path (a galaxy-map plot to a parked waypoint) does not advance next past a waypoint reached while parked, unlike resume() after fngg; the shown next waypoint is stale until the next arrival, which corrects it, and nothing is copied.'
target: astromech-g0hm
identity: reviewer-agent-2
commit: c49ec75a2c779f146c54a111fbcc8a45dd000b74
created_at: 2026-10-23T01:21:03.856447367Z
event:
  event: approval
---

Second review at HEAD. Checked the fix commit a7647ba9 against findings g12z (panel stays mounted mid-plot: close() keeps picked while Plot.plotting()), vkpl (Tauri plotRoute notifies watchers), 67t2 (fakecore route_set clears parked, test added), nf0a (comments/docs), fngg (resume() arrives at the current system first, test added), 5gjt (wording). Skimmed the whole main...HEAD diff in crates, app/src and docs. Proof: cargo test route:: (13 pass), cargo test -p astromech-announce (all pass), vitest src/modules/route (14 pass), pnpm check (0 errors/warnings). Limits: Linux only, no real game or Tauri run. Low note, not blocking: Desk::plotted's auto-resume path (a galaxy-map plot to a parked waypoint) does not advance next past a waypoint reached while parked, unlike resume() after fngg; the shown next waypoint is stale until the next arrival, which corrects it, and nothing is copied.
