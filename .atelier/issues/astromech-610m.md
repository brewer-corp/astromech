---
created_at: "2026-10-15T20:36:13.666944526+00:00"
id: "astromech-610m"
issue_type: "mission"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-0474"
    type: "advances"
  - kind: "issue"
    id: "astromech-7ck3"
    type: "advances"
  - kind: "issue"
    id: "astromech-a50f"
    type: "advances"
  - kind: "issue"
    id: "astromech-fdyn"
    type: "advances"
  - kind: "issue"
    id: "astromech-fgv5"
    type: "advances"
  - kind: "issue"
    id: "astromech-hnar"
    type: "advances"
  - kind: "issue"
    id: "astromech-huab"
    type: "advances"
  - kind: "issue"
    id: "astromech-lxma"
    type: "advances"
  - kind: "issue"
    id: "astromech-q318"
    type: "advances"
  - kind: "issue"
    id: "astromech-snqt"
    type: "advances"
  - kind: "issue"
    id: "astromech-txzq"
    type: "advances"
  - kind: "issue"
    id: "astromech-we0f"
    type: "advances"
  - kind: "issue"
    id: "astromech-y5mz"
    type: "advances"
  - kind: "issue"
    id: "astromech-z4jb"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ship v1.0"
updated_at: "2026-10-15T20:36:18.899608472+00:00"
---

## Outcome

astromech v1.0 is on GitHub Releases: voice replies sound like the in-game COVAS, a fresh install reaches a working voice command in minutes, and the README is up to date.

## Success criteria

The milestone exit tests from [V1-DESIGN §8](docs/V1-DESIGN.md). M0–M2 shipped (v0.1.5, #4).

- S1 **M3 voice:** at least 95% correct execute-or-ask on Sumeet's VR clip set (`--eval`),
  and Cockpit replies sit in the same register as in-game COVAS, judged by Sumeet in the
  headset.
- S2 **M4 onboarding:** a fresh install reaches a working "gear down" by voice in under
  5 minutes, through the onboarding wizard.
- S3 **MVP criteria** ([ARCHITECTURE §14](docs/ARCHITECTURE.md) Phase 1) measured and reported
  together: 200 mock-game trials per tool, a 4 h soak replay with zero unintended keypresses,
  kill switch ≤ 50 ms, action start ≤ 400 ms p95 after PTT release on each device class.
- S4 **M5 release:** v1.0 on GitHub Releases with an up-to-date README, signed if the
  signing decision says so.

The "callouts" part of M4 (V1-DESIGN §1.5 #5) is replaced by the reaction work under
"Replace the daily toolbox": low fuel, interdiction and heat are prominent on the HUD, so they
aren't spoken.
