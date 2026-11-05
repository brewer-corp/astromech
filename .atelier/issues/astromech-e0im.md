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
closed_at: "2026-11-05T01:04:03.064133661+00:00"
status: "done"
title: "Voice grammar: pin the everyday-question margin and 'how much is this worth'"
updated_at: "2026-11-05T01:04:03.064133661+00:00"
---

## Goal

The everyday-question rule in the voice grammar makes only the choices it means to, pinned by tests.

## Done when

- Whether EVERYDAY counts against every command's margin is decided: either drop the margin term or keep it, and a test pins near misses such as 'how much is my unsold cargo worth' (R2, astromech-mbgc).
- What 'how much is this worth' and 'what is this worth' should do (ask about system value, as before astromech-54k4, or 'Say again') is decided and pinned in a test (R3, astromech-eony).

## Context

Deferred review findings on task/astromech-54k4: crates/astromech-intent/src/lib.rs:972 and :467-470 with the filler words at :510. Read-only answers, so the cost is an extra 'yes' or a 'Say again', never a wrong action.

## Unknowns

- R3 (what a player most likely means by 'how much is this worth' in a system) was decided by an independent decider agent, under Sumeet's direction of 4 Nov 2026 that agents make these choices. It stays 'Say again' (NoMatch); see the decision note. Revisit if real transcripts show players saying 'this worth' and meaning system value.

## Notes

- 2026-11-04 [note] Decisions (R2, R3): Sumeet delegated both choices to an independent decider agent under his 4 Nov 2026 direction. R2 (astromech-mbgc): keep the EVERYDAY margin term (best.score - runner.max(everyday) >= MARGIN). Reason: an extra 'yes' costs less than confidently reading out the wrong number mid-flight. Reversible in one line. R3 (astromech-eony): the first pick, ask 'Did you mean what is this system worth?', was withdrawn because its premise was measured wrong: it assumed these phrases still asked, but they already give NoMatch ('my' is filler, so 'how much is my ship worth' / 'what is my ship worth' score 0.900 / 0.875 against them, above the 0.833 / 0.800 system-value match), and making them ask would need a change to the NoMatch rule. Re-decided: 'how much is this worth' and 'what is this worth' stay NoMatch ('Say again'), with no code change. Reasons: changing the NoMatch rule for every command to rescue leads of 0.067 / 0.075 against a 0.08 margin is fragile; the bare 'this worth' is ambiguous in Elite; 'Say again' costs one rephrase. Reversible in one line, to NoMatch only when 'everyday - best.score >= MARGIN' (a scratch run of that change passed every existing test). Revisit if real transcripts show players meaning system value.
- 2026-11-04 [note] Implementer report: R2 and R3 are pinned by tests, with no grammar or decision-rule change. R2: the new test everyday_margin_asks_for_near_misses (crates/astromech-intent/src/lib.rs). Observed with Grammar::mvp: 'how much is my unsold cargo worth' -> exploration.unsold Ask 0.867 (best everyday 0.833); 'how much is my dater worth' -> exploration.unsold Ask 0.920 (everyday 0.840); 'what is is in this system' -> exploration.biology Ask 0.857 (everyday 0.833); controls 'how much is my data worth' -> unsold Execute 1.000 and 'what biology is in this system' -> biology Execute 1.000. The test fails when the margin term is changed to 'best.score - runner'. R3: everyday_questions_ask_no_exploration now asserts NoMatch for 'how much is this worth' (system_value 0.833, everyday 0.900), 'what is this worth' (system_value 0.800, everyday 0.875), 'how much is my ship worth' (unsold 0.800, everyday 1.000) and 'what is my ship worth' (system_value 0.700, everyday 1.000), with a comment that this is deliberate. It also asserts that 'how much is this system worth' and 'what is this system worth' execute exploration.system_value, and 'how much is my data worth' executes exploration.unsold. 'what is this worth' is out of the loose never-Execute list. Surprise: the first R3 pick assumed these phrases still asked about system value, but they already gave NoMatch, so R3 was re-decided (see the decision note). Proof: cargo test -p astromech-intent (36 passed), cargo fmt --all -- --check (clean), cargo clippy -p astromech-intent --all-targets -- -D warnings (clean). Review: outstanding.
- 2026-11-04 [decision] R2 (astromech-mbgc): EVERYDAY keeps counting against every command's margin; near misses such as 'how much is my unsold cargo worth' Ask, pinned by everyday_margin_asks_for_near_misses.
  Reason: An extra 'yes' costs less than confidently reading out the wrong number mid-flight; zero code change; reversible in one line. Decided by an independent decider agent under Sumeet's 4 Nov 2026 direction.
  Rejected: Drop the margin term (EVERYDAY as a veto only)
- 2026-11-04 [decision] R3 (astromech-eony): 'how much is this worth' and 'what is this worth' answer 'Say again' (NoMatch), pinned in everyday_questions_ask_no_exploration.
  Reason: Rescuing these via the NoMatch rule changes the veto for every command for near-tie leads (0.067/0.075 vs 0.08); bare 'this worth' is ambiguous in Elite; 'Say again' costs one rephrase; reversible in one line. Independent decider agent under Sumeet's 4 Nov 2026 direction; first pick (Ask) withdrawn because its premise was measured wrong. Revisit if real transcripts show players meaning system value.
  Rejected: Ask 'Did you mean what is this system worth?' by changing the NoMatch rule to everyday - best.score >= MARGIN

## Close Reason

Implementer report. Changed: tests only, crates/astromech-intent/src/lib.rs. The new test everyday_margin_asks_for_near_misses pins R2: EVERYDAY keeps counting against the margin, so 'how much is my unsold cargo worth' asks at 0.867 against everyday 0.833. everyday_questions_ask_no_exploration pins R3: 'how much is this worth' and 'what is this worth' get NoMatch ('Say again'). Decisions: an independent decider agent made both, under Sumeet's 4 Nov 2026 direction; they are recorded as decision notes. The first R3 pick, Ask, was withdrawn because the implementer measured its premise wrong. Proof: cargo test -p astromech-intent (36 passed), fmt and clippy clean. Mutations checked by the reviewer: dropping the margin term fails the R2 test, and the alternative NoMatch rule fails the R3 test. Review: approvals astromech-4yjx and astromech-hdry; finding astromech-u270 fixed (dae90f95). Deviations: none. Surprise: the bare 'this worth' phrases were already NoMatch, because 'my' is a filler word. Open items: revisit R3 if real transcripts show players meaning system value.
