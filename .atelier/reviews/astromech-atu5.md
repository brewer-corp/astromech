---
schema: atelier.review
schema_version: 1
id: astromech-atu5
title: R3 [low] app/src/lib/ui/SearchKit.svelte followShip makes a second load_filters IPC on every search and labels the pad from that separate resolution, not from the filters the search ran with. It agrees in practice, since both resolve from the current ship within milliseconds, but the result carrying the resolved pad would be a single source of truth for both SearchKit and Route. Also, main.rs:976 (search_filters) still inlines Pad::for_ship rather than the new for_ship_type. Optional tidy.
target: astromech-ssyo
identity: review-agent (opus, independent)
commit: 7d5fa6d67b38f916be0bf4bc76d95f87e7d884d0
created_at: 2026-10-17T06:09:22.880677820Z
event:
  event: finding
  recommendation: defer
---

R3 [low] app/src/lib/ui/SearchKit.svelte followShip makes a second load_filters IPC on every search and labels the pad from that separate resolution, not from the filters the search ran with. It agrees in practice, since both resolve from the current ship within milliseconds, but the result carrying the resolved pad would be a single source of truth for both SearchKit and Route. Also, main.rs:976 (search_filters) still inlines Pad::for_ship rather than the new for_ship_type. Optional tidy.
