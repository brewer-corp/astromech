---
schema: atelier.review
schema_version: 1
id: astromech-x81v
title: 'View.svelte poll sequencing can stall: it drops any answer whose number is not the newest asked (n !== asked). Polls fire every 2s, so if the command takes longer than 2s (cold journal replay on a big history, which also isn''t cached until the first replay finishes), every answer is superseded before it lands and the page never shows data or an error. Fix: keep a lastApplied counter and drop only answers with n < lastApplied, then set lastApplied = n on apply (same for the catch). The new test passes both ways, so add a case where a slow first poll is the only answer. The other parts (apply_cargo_json matches Hold::apply, Count>0 with no list kept; Cargo.json byte key cost is a few KB per 2s poll and a failed or partial read just costs one extra replay) are fine; tests 9293 and the late-poll test fail without their fixes.'
target: astromech-k3wp
identity: review-agent (k3wp fix review 2, sonnet)
commit: ed73181d0c0ea36c6e3e88a4ae9776db52ef28dc
created_at: 2026-10-16T23:21:41.359172503Z
event:
  event: finding
  recommendation: fix_now
---

View.svelte poll sequencing can stall: it drops any answer whose number is not the newest asked (n !== asked). Polls fire every 2s, so if the command takes longer than 2s (cold journal replay on a big history, which also isn't cached until the first replay finishes), every answer is superseded before it lands and the page never shows data or an error. Fix: keep a lastApplied counter and drop only answers with n < lastApplied, then set lastApplied = n on apply (same for the catch). The new test passes both ways, so add a case where a slow first poll is the only answer. The other parts (apply_cargo_json matches Hold::apply, Count>0 with no list kept; Cargo.json byte key cost is a few KB per 2s poll and a failed or partial read just costs one extra replay) are fine; tests 9293 and the late-poll test fail without their fixes.
