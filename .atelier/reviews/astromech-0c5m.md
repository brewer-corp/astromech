---
schema: atelier.review
schema_version: 1
id: astromech-0c5m
title: finding on astromech-ja5a @8fd31c63
target: astromech-ja5a
identity: claude-reviewer-ja5a
commit: 8fd31c6366b5b5ca540fcacf3dd0315bcf6a0288
created_at: 2026-11-07T00:07:25.387285604Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.4.md:13 lists 'Material traders, Raw sites and Faction states (moved from Materials)'. Only Raw sites and Faction states came from Materials (kwoi d72c4098); Material traders is a new finder (nxz9 326fd18e) for what was a service on the old Galaxy Services page, and Services, Outfitting and Shipyard were the old Galaxy pages. Fix: 'Places has a tile for each finder: Sell, Buy and Sell my cargo (moved from Trade); Services, Outfitting and Shipyard (the old Galaxy pages) and a new Material traders finder; Raw sites and Faction states (moved from Materials); and Mining hotspots.'
