---
schema: atelier.review
schema_version: 2
id: astromech-4wuj
title: finding on astromech-bp23 @1aa6880b
target: astromech-bp23
identity: claude-reviewer-se44
commit: 1aa6880b69307cd219877f7d80afa6bc5dda2431
created_at: 2026-11-07T09:21:25.010338196Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: claude-orchestrator-kjfs
    commit: 00ba544e1f14041bd705bb10cb6784212db9e5bc
    recorded_at: 2026-11-07T09:21:39.916588662Z
    reason: Bindings drift test catches a dropped key
---

info: crates/astromech-core/src/app.rs core_replies_keep_their_json (4bf9f29f) found["install"] / found["install_source"] are set through IndexMut without checking the key exists, so the mask would re-add either key if find_elite's reply ever dropped it (autostart's mask is guarded by v.get(k).is_some()). Today setup::Found always serialises both (Option, no skip), and removing the field would also change types/Found.ts and fail the bindings drift test, so nothing is hidden now. Optional hardening: mask only keys already present (if let Some(x) = found.get_mut("install")).
