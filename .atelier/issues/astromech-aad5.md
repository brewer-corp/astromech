---
created_at: "2026-10-17T17:29:23.315127139+00:00"
id: "astromech-aad5"
issue_type: "task"
labels:
- "docs-found"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Callouts can only be turned off by editing astromech.toml"
updated_at: "2026-10-17T17:29:23.315127139+00:00"
---

## Goal

Each callout has [callouts.<id>] enabled and cooldown_s in the config, but Settings has no row for them (only the prospector thresholds), no master switch and no volume. V1-DESIGN 2.3 lists game event callouts and volume under Voice responses and a Debug section (save push-to-talk clips, input logging, bounce and minimum-clip thresholds); none are in Settings. Diagnostics also lacks the health report and device list the design puts there.

## Done when

Settings can turn each callout on or off (and the default-off Powerplay arrival on), and the guide's "More in the config file" table in docs/guide/settings.md shrinks accordingly. Found while writing the user guide.
