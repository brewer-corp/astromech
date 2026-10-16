---
schema: atelier.review
schema_version: 1
id: astromech-5kdt
title: '[low] crates/astromech-galaxy/src/sites.rs:266 (spoken) for raw_farms: the voice answer to ''nearest raw farm'' names the body but not what it is farmed for. For example ''Nearest raw material farm: HIP 36601 C 1 a, in this system, 153,095 light seconds from the arrival star.'' says nothing about polonium or crystalline shards. The UI row carries it in detail. Without it a commander can''t tell the four HIP 36601 bodies apart, or tell whether the farm gives the element they need. Fix: for Source::Farms, put the material and site in the spoken line, e.g. '': polonium, crystalline shards, on HIP 36601 C 1 a, ...''.'
target: astromech-sfys
identity: review-agent (sfys review)
commit: 8c44864ffb8b65cc78872de4fd3c564004816b0a
created_at: 2026-10-16T22:31:57.982011370Z
event:
  event: finding
  recommendation: defer
---

[low] crates/astromech-galaxy/src/sites.rs:266 (spoken) for raw_farms: the voice answer to 'nearest raw farm' names the body but not what it is farmed for. For example 'Nearest raw material farm: HIP 36601 C 1 a, in this system, 153,095 light seconds from the arrival star.' says nothing about polonium or crystalline shards. The UI row carries it in detail. Without it a commander can't tell the four HIP 36601 bodies apart, or tell whether the farm gives the element they need. Fix: for Source::Farms, put the material and site in the spoken line, e.g. ': polonium, crystalline shards, on HIP 36601 C 1 a, ...'.
