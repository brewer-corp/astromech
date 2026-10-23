---
created_at: "2026-10-15T20:36:13.666255952+00:00"
id: "astromech-e8ux"
issue_type: "validation"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: validation/astromech-e8ux
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-y5so"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-3ez9"
    type: "related"
  - kind: "issue"
    id: "astromech-6sf7"
    type: "related"
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-8joi"
    type: "related"
  - kind: "issue"
    id: "astromech-dlrb"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-m3xd"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Validate the toolbox criteria S1–S8"
updated_at: "2026-10-22T23:51:36.692720641+00:00"
---

## Goal

Check the toolbox mission's S1–S8 independently against the evidence on each item.

## Done when

S1–S8 each have a result and an evidence receipt.

## Context

[Strategy](docs/strategies/replace-the-toolbox.md).

## Notes

- 2026-10-22 [note] 2026-10-22 [handoff] Independent validation of S1–S8 at 327c059 (main 8866d1f), validator-e8ux. Results: S1 pass (astromech-beyd), S2 pass (astromech-pdhh), S3 pass (astromech-ue93), S4 pass (astromech-r5aw), S5 pass (astromech-40m9, replaces the 17 Oct fail astromech-hlta), S6 fail (astromech-t33g: outfitting/2 is gated off, SEND_OUTFITTING=false, 111 Outfitting visits skipped in the real-journal dry run; live acceptance blocked on astromech-uxyq and astromech-shi5), S7 blocked (astromech-aqfc: replay passes; play sessions astromech-505u, astromech-6920), S8 fail (astromech-ciqa: no voice phrase templates for the §18.7 M7 queries). Filed: astromech-dlrb (send outfitting/2), astromech-m3xd (exploration and biology voice queries), astromech-j75v (saying 'near Sol' misroutes a voice lookup to another commodity/module; also asks Sumeet whether voice must search from a named system). Proof run fresh: cargo test --workspace 1124 pass / 0 fail / 1 ignored; app pnpm test 380/380; real_journals harness on a copy of ~/edjournal, 0 problems; live Spansh/Ardent lookups through the core Assistant (scratch harness outside the repo, about 30 requests) for every S1–S5 voice query; browser preview of the outfitting, shipyard, services, trade, materials, powerplay, colonisation, exploration, exobiology and datasharing pages. Limits: the Tauri shell isn't buildable here (main CI green); app pages checked on preview fixtures only; the real journals have no colonisation project (synthetic fixture used for S5 replay); voice searches from the current system only, read as meeting 'a typed one' via the app's typed origin; the needs-human in-game checks (4su3, 1per, fipx, w1q2, 66pq, xf8j) remain open but are not the criteria's stated proof for S1–S5. Close is gated by the S6 and S8 fails; requested review instead.
- 2026-10-22 [note] 2026-10-22: S6 and S8 failed; re-validate both after astromech-dlrb and astromech-m3xd merge (S7 stays blocked on the play sessions). The validation records are on main; the branch validation/astromech-e8ux is kept for its ownership.
