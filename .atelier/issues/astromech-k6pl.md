---
created_at: "2026-11-07T03:00:10.391746191+00:00"
id: "astromech-k6pl"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Scale the window with its height, and a stronger orange"
updated_at: "2026-11-10T06:03:07.884354190+00:00"
---

## Goal

Text is readable on a big screen. The game scales its UI with the screen's height; astromech's fixed pixel sizes leave a 1440p or 4K window's text about the size of the game's smallest. Sumeet, 6 Nov 2026: 'the text is just too small across the whole app', and the orange looks washed out.

## Done when

- [ ] At 2560 x 1440 the window renders at about 1.4x; at 1100 x 700 it is unchanged; at 3840 x 2160 it is capped at 1.8x. A tall, narrow window (1100 x 1400) stays at 1.0x.
- [ ] A Text size setting in Settings > General (Smaller, Normal, Larger: 0.85, 1.0, 1.15 times the height scale) changes it live and persists across restarts. It is stored in the core's config and read and set through commands in the command layer, as the voice settings are (voice_settings and its setter), with a fake handler in the preview and a test.
- [ ] DESIGN.md's Typography, Layout and Token map say the scale rule and the new values, and tokens.css has them.
- [ ] Every page checked in the preview at 1100 x 700 and 2560 x 1440; the design-system screenshots re-rendered.

## Context

Mission astromech-xajc. This item owns DESIGN.md's Typography sizes, colour values and Token map; astromech-b2k3 owns the other revision 4 amendments. The window has no settings store of its own: settings live in the core's config.

## Design

- The whole window scales with its height, as one root zoom so DESIGN.md's px tokens stay as they are: zoom = clamp(1, min(height / 1000, width / 1100), 1.8). The width term stops a tall, narrow window from squeezing the layout below its 1100 px minimum; the floor of 1 means text never shrinks, and a short window scrolls. The Text size setting multiplies the result.
- Orange text at weight 500, Orange Mid #d97a34, the base orange #ff7100, and fewer labels in Orange Mid. The before/after is in the Route mockup's toggles (astromech-ok29).
