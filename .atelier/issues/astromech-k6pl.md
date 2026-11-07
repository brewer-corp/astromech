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
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Scale the window with its height, and a stronger orange"
updated_at: "2026-11-07T03:00:10.391746191+00:00"
---

## Goal

Text is readable on a big screen. The game scales its UI with the screen's height; astromech's fixed pixel sizes leave a 1440p or 4K window's text about the size of the game's smallest. Sumeet, 6 Nov 2026: 'the text is just too small across the whole app', and the orange looks washed out.

## Done when

- [ ] At 2560 x 1440 the window renders at about 1.4x; at 1100 x 700 it is unchanged; the Text size setting changes it live and persists.
- [ ] DESIGN.md's Typography, Layout and Token map say the scale rule and the new values, and tokens.css has them.
- [ ] Every page checked in the preview at 1100 px and 2560 px wide; the design-system screenshots re-rendered.

## Design

- The whole window scales with its height: 1.0 at 1000 px tall, capped near 1.8, as one root zoom so DESIGN.md's px tokens stay as they are; a Text size setting multiplies it.
- Orange text at weight 500, Orange Mid #d97a34, the base orange #ff7100, and fewer labels in Orange Mid. The before/after is in the Route mockup's toggles (astromech-ok29).
