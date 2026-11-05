---
schema: atelier.review
schema_version: 1
id: astromech-5rp7
title: finding on astromech-51ue @bee10fc9
target: astromech-51ue
identity: reviewer-sonnet-51ue-plan2
commit: bee10fc948d88fb61d41a941fff7e6a45fd1769a
created_at: 2026-11-05T06:29:58.754223431Z
event:
  event: finding
  recommendation: fix_now
---

medium: astromech-sgwi Done when has 'Find near this' from a Material traders or Mining hotspots row (and recipe 2 is a mining-hotspot recipe), but sgwi is blocked only by nxz9 and xlbz, not by astromech-f7i0 (the Mining hotspots finder). An implementer starting sgwi after nxz9 has no hotspots row to pin to. Fix: add f7i0 as blocked_by on sgwi, or drop Mining hotspots from sgwi's Done when. Graph stays acyclic; nothing in 51ue's Done when or kyqw is affected.
