---
schema: atelier.review
schema_version: 1
id: astromech-a400
title: finding on astromech-jp36 @e2a7b6bb
target: astromech-jp36
identity: reviewer-sonnet-jp36
commit: e2a7b6bb7dfdbdef096bb343d095fa592db019c8
created_at: 2026-11-05T08:24:35.252171619Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/stations.ts:112 and SearchKit.svelte template: at 1100 px Outfitting's Module and Faction text columns get ~40 px (docs/design/lookup-row-outfitting-1100-after.png shows '5A Fr…' on every row and 'Feder…'), so the Faction column is unreadable at the supported minimum width and Module repeats the same cell. Decision: leave the shrinking behaviour (fixed widths would push the facts out and a floor would trip the compact row, which loses distance, pad and arrival, worse than a truncated faction); follow-up: omit a text column whose cell is identical on every row (more than one row) in stations toResults, which covers Outfitting with class and rating chosen without coupling the adapter to the page's choices. Not blocking: no overflow at 1100 or 1600 on any lookup page (I measured 14 page/width cases), rows stay aligned, the full text is in the title tooltip.
