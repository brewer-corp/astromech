---
created_at: "2026-11-04T21:48:48.614549177+00:00"
id: "astromech-e0im"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-e0im
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "review"
    id: "astromech-mbgc"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Voice grammar: pin the everyday-question margin and 'how much is this worth'"
updated_at: "2026-11-05T00:51:42.603381619+00:00"
---

## Goal

The everyday-question rule in the voice grammar makes only the choices it means to, pinned by tests.

## Done when

- Whether EVERYDAY counts against every command's margin is decided: either drop the margin term or keep it, and a test pins near misses such as 'how much is my unsold cargo worth' (R2, astromech-mbgc).
- What 'how much is this worth' and 'what is this worth' should do (ask about system value, as before astromech-54k4, or 'Say again') is decided and pinned in a test (R3, astromech-eony).

## Context

Deferred review findings on task/astromech-54k4: crates/astromech-intent/src/lib.rs:972 and :467-470 with the filler words at :510. Read-only answers, so the cost is an extra 'yes' or a 'Say again', never a wrong action.

## Unknowns

- R3 (what a player most likely means by 'how much is this worth' in a system) was decided by an independent decider agent, under Sumeet's direction of 4 Nov 2026 that agents make these choices: it should ask about system value. See the decision note.
- Today both 'this worth' phrases give NoMatch, not Ask: 'my' is a filler word, so the everyday phrases 'how much is my ship worth' / 'what is my ship worth' score 0.900 / 0.875 against them, above the 0.833 / 0.800 system-value match. No EVERYDAY edit can separate them, so meeting R3 needs a change to the NoMatch rule.

## Notes

- 2026-11-04 [note] Decisions (R2, R3): Sumeet delegated both choices to an independent decider agent under his 4 Nov 2026 direction. R2 (astromech-mbgc): keep the EVERYDAY margin term (best.score - runner.max(everyday) >= MARGIN). Reason: an extra 'yes' costs less than confidently reading out the wrong number mid-flight. Reversible in one line. R3 (astromech-eony): 'how much is this worth' and 'what is this worth' should ask 'Did you mean what is this system worth?', with no new EVERYDAY entries. Reason: 'this' is ambiguous, system value is the only worth answer the app has, and a wrong guess costs one 'no'. Reversible by changing a test assert. The ship phrases ('how much is my ship worth', 'what is my ship worth') stay NoMatch.
- 2026-11-04 [note] Implementer report (partial; R3 is blocked on a scope call). Done: R2 is pinned by the new test everyday_margin_asks_for_near_misses in crates/astromech-intent/src/lib.rs, with no code change. Observed with Grammar::mvp: 'how much is my unsold cargo worth' -> exploration.unsold Ask 0.867 (everyday 0.833); 'how much is my dater worth' -> exploration.unsold Ask 0.920 (everyday 0.840); 'what is is in this system' -> exploration.biology Ask 0.857 (everyday 0.833); controls 'how much is my data worth' -> unsold Execute 1.000 and 'what biology is in this system' -> biology Execute 1.000. The test fails when the margin term is changed to 'best.score - runner'. Not done: R3. Today 'how much is this worth' gives NoMatch (system_value 0.833, everyday 'how much is my ship worth' 0.900) and so does 'what is this worth' (system_value 0.800, everyday 'what is my ship worth' 0.875), because 'my' is filler. No EVERYDAY edit keeps the ship phrases NoMatch while freeing these two, so R3 needs a change to the decision rule. A scratch candidate, NoMatch only when 'everyday >= EXECUTE && everyday - best.score >= MARGIN' (was 'everyday >= best.score'), passed all 35 existing tests and gave: both 'this worth' phrases -> system_value Ask; the ship phrases -> NoMatch (leads 0.200 / 0.300); 'how much is this system worth' -> Execute. Its leads are thin, 0.067 and 0.075 against a margin of 0.08. Reverted, not committed: it is outside the no-grammar-change brief. Proof: cargo test -p astromech-intent passes 36 tests; cargo fmt --all -- --check is clean; cargo clippy -p astromech-intent --all-targets -- -D warnings is clean.
