---
schema: atelier.review
schema_version: 1
id: astromech-m7to
title: '[low] crates/astromech-galaxy/src/lib.rs:255 (astromech-tr4g fix): in Galaxy::nearby the population-filtered page is fetched with ''?'', so if that second Spansh request fails or times out (it is the heavier filtered query) the whole HGE lookup returns an error even though the nearest page is already fetched, parsed and cached. Before the fix the same lookup answered from the nearest page. Scenario: Spansh answers the plain search but the population search times out; the commander gets an error instead of the previous ranking. Fix: treat the population page as best effort (fall back to the nearest page alone, as ensure_covered is). Also the ''fetched'' age shown comes from the nearest page only. No test covers one request failing. Not blocking: both requests normally succeed or fail together.'
target: astromech-sfys
identity: review-agent (sfys fix review, sonnet)
commit: e4a8266d52ce3118d507b47dc12e4b51aafd200e
created_at: 2026-10-16T22:40:45.144859793Z
event:
  event: finding
  recommendation: defer
---

[low] crates/astromech-galaxy/src/lib.rs:255 (astromech-tr4g fix): in Galaxy::nearby the population-filtered page is fetched with '?', so if that second Spansh request fails or times out (it is the heavier filtered query) the whole HGE lookup returns an error even though the nearest page is already fetched, parsed and cached. Before the fix the same lookup answered from the nearest page. Scenario: Spansh answers the plain search but the population search times out; the commander gets an error instead of the previous ranking. Fix: treat the population page as best effort (fall back to the nearest page alone, as ensure_covered is). Also the 'fetched' age shown comes from the nearest page only. No test covers one request failing. Not blocking: both requests normally succeed or fail together.
