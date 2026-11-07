---
schema: atelier.review
schema_version: 2
id: astromech-4xt6
title: finding on astromech-vucf @168fc814
target: astromech-vucf
identity: claude-reviewer-zgoo
commit: 168fc8140dcef80ec71579e18900b09ec4ba7d2c
created_at: 2026-11-07T21:46:41.053078378Z
event:
  event: finding
  recommendation: no_action
---

spikes/ (835020c8): the Report says every spike crate root carries the exemption, but four spike example and test crates that read time directly have none (spikes/s5-status-latency/examples/mockgame.rs, tests/mock_game.rs, tests/binds_and_keys.rs, spikes/s7-input-listener/examples/mockfocus.rs). No action: no workflow on this branch runs clippy on spikes (they're outside the workspace), so nothing fails; whoever lints a spike adds the line.
