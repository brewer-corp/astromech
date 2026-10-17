---
created_at: "2026-10-17T05:27:12.314263471+00:00"
id: "astromech-243w"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-243w
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T06:03:27.219659530+00:00"
status: "done"
title: "Trade lookups: blank age and distance selects, placeholder rows, shorter partial-buyer voice"
updated_at: "2026-10-17T06:03:27.219659530+00:00"
---

## Goal

Left over from the Trade near me review (astromech-zf3s, R6 and R7) and the astromech-proz review:
- R6: the age and distance dropdowns show blank.
- R7: placeholder rows can push real markets out of Ardent's 1000-row answer.
- astromech-2myn: voice adds a ~20-word partial-buyer sentence for every commodity no market takes whole, with no limit, often repeating the main answer's station. Limit it to the top one or two and drop the station when it repeats.

## Done when

Component test: the selects show their saved values. Recorded-response test: a 1000-row answer is flagged truncated and placeholder rows are never offered (R7 rescoped: placeholders do take slots, but a full answer has lost real markets whatever its placeholders, so the truncation flag is the right signal; Ardent has no filter to keep placeholders out, checked 17 Oct 2026). Copy test: a hold of five such commodities stays within the copy guide's budget.

## Notes

- 2026-10-17 [note] R7 not changed, needs a decision. truncated = (rows >= 1000) is the right test: Ardent cuts at 1000 rows and placeholders (high price, so first) use up slots, so a full answer has lost real markets whatever its placeholders. Not counting placeholders would clear the flag exactly when real markets were cut. Pinned by placeholders_use_up_ardents_rows_so_a_full_answer_is_truncated. Done-when wording ('placeholders don't count toward truncation') needs rewording or a different fix (Ardent has no maxVolume; checked 17 Oct 2026).
- 2026-10-17 [note] Orchestrator rescoped R7 to the existing truncation flag after the implementer showed the original fix (not counting placeholders toward truncation) would clear the flag exactly when real markets were cut.

## Close Reason

R6 and the partial-buyer voice fixed; R7 rescoped to the existing truncation flag with a pinning test.
