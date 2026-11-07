---
schema: atelier.review
schema_version: 2
id: astromech-9a18
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:25.042640583Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-author
    commit: 2dc1e59b1ef18b668bef415e41cfe21d826f7442
    recorded_at: 2026-11-07T04:55:52.205220574Z
    resolution_commit: 2dc1e59b1ef18b668bef415e41cfe21d826f7442
    reason: Folded into the plan in 2dc1e59b.
---

high: astromech-xil2 Done when (transport interface) The seam is call<T>(cmd,args) only, but the window also receives pushes from the backend: listen('input') in app/src/lib/binds.ts:153, listen('route') in core.ts:237, the 'core-restarted' event from window.rs:94, and a Tauri Channel for neutron_download progress (neutron.ts:79). Those still branch on inTauri, so the Done-when 'no module checks inTauri for data' can't be met without the worker inventing an event API, and bp23's typed layer and vucf's dev bridge will need it too. Fix: name the transport's push half now (e.g. subscribe(topic, cb) -> unsubscribe, and a streaming/progress form for call), have the fake serve it, and make coverage.test.ts cover topics as well as commands.
