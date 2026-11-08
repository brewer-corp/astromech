---
schema: atelier.review
schema_version: 2
id: astromech-ipgq
title: finding on astromech-h95t @f19cd827
target: astromech-h95t
identity: claude-reviewer-h95t
commit: f19cd827b023ba7d64a0e93f3cf1bc7c8ce070f8
created_at: 2026-11-08T16:01:09.772002132Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.5.md:23 'the other search caches' are the page caches built from the journal (materials, engineering, exploration, Powerplay, colonisation) and the star map's in-memory map (astromech-k6da report), and only the neutron file is reloaded by 'the next plot'. Fix: 'The file (about 80 MB in memory), the star map and the pages' journal caches are dropped after 10 minutes unused and load again when next needed.'
