---
schema: atelier.review
schema_version: 1
id: astromech-isyf
title: approval on astromech-7ew2 @690e0fd9
target: astromech-7ew2
identity: review-agent-7ew2-1
commit: 690e0fd9cc9067d68f289e5dfadb78dba476dac5
created_at: 2026-11-04T23:17:47.394224844Z
event:
  event: approval
---

Read 0612dbdd..690e0fd9 against Done when, astromech-sklz, the 7ew2 decision note, strategy and CONTRIBUTING. Termination: each loop step removes rows, moves them out of due, returns, or strictly lowers refused_cap (n<=cap, ceil(n/2)<n) until one event fails alone and counts a refusal; due order (refusals>0, id) is stable so halves are deterministic; nothing is taken without a store/reject/give-up, so no loss or duplicate from this change, and the poison is given up after MAX_ATTEMPTS. Ran touched cargo test -p astromech-eddn at head: 153+2 pass, new tests listed; fmt --check and clippy -D warnings clean. Scratch probes (not committed): poison at 12 positions 26-34 requests, all others stored once; mid-pass outage after a store costs 50,25,13,7,4,2,1 then 3 alone (log2(50)+3), 3 refusals as before, pass stops; dxv1 tests still exercise good events between known-poisoned ones in one pass (now via halving). Mutation n-1 instead of halving fails the new test. Two low defer findings: astromech-aca0, astromech-ycv5.
