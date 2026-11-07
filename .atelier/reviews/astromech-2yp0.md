---
schema: atelier.review
schema_version: 2
id: astromech-2yp0
title: finding on astromech-vucf @168fc814
target: astromech-vucf
identity: claude-reviewer-zgoo
commit: 168fc8140dcef80ec71579e18900b09ec4ba7d2c
created_at: 2026-11-07T21:46:40.684066930Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: claude-orchestrator-kjfs
    commit: 632406edc65aa0a963e2b23319c7c156d325bb69
    recorded_at: 2026-11-07T21:47:32.333210197Z
    reason: PollWatcher never reports read notifications; unused
---

clippy.toml's watcher ban: notify::PollWatcher::new is an inherent method, so it isn't caught by notify::Watcher::new (planted call: no error, while RecommendedWatcher::new, recommended_watcher and Receiver::recv_timeout each fail). The zgoo decision note's 'a new watcher can't skip the filter' overstates the ban slightly. No action: a PollWatcher compares metadata and never reports opens or read-closes, so may_change would pass every event it sends anyway; nothing in the tree uses it.
