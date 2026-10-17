---
schema: atelier.review
schema_version: 1
id: astromech-bxjj
title: 'low: crates/astromech-announce/src/lib.rs module doc reads ''and [missions] and [missions]'' (duplicate). Also the module and Rule trait docs say rules are stateless, but Missions keeps a Mutex<HashMap> of warned levels; update the wording.'
target: astromech-rpsb
identity: review-agent (sonnet, independent)
commit: 5f02b645b125ba32a44502aa4c84b61edfa8acfc
created_at: 2026-10-17T03:14:52.025229471Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/astromech-announce/src/lib.rs module doc reads 'and [missions] and [missions]' (duplicate). Also the module and Rule trait docs say rules are stateless, but Missions keeps a Mutex<HashMap> of warned levels; update the wording.
