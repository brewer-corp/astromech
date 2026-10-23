---
schema: atelier.review
schema_version: 1
id: astromech-cg58
title: 'Low, crates/astromech-tools/src/exploration.rs:73: a third spoken-credits formatter beside astromech_announce::session_summary::credits and discovery::credits. The new one says billions and ''1 million credits''; session_summary says ''1.0 million credits'' and ''1460.0 million credits'', so the same number reads differently across answers. Move billions/trim into session_summary::credits and reuse it.'
target: astromech-m3xd
identity: review-agent-m3xd-1
commit: a83e6a6a42217c36b9fb3348031207138ce1f7f9
created_at: 2026-10-23T00:28:22.003326966Z
event:
  event: finding
  recommendation: defer
---

Low, crates/astromech-tools/src/exploration.rs:73: a third spoken-credits formatter beside astromech_announce::session_summary::credits and discovery::credits. The new one says billions and '1 million credits'; session_summary says '1.0 million credits' and '1460.0 million credits', so the same number reads differently across answers. Move billions/trim into session_summary::credits and reuse it.
