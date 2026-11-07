---
created_at: "2026-11-07T04:50:17.662693859+00:00"
id: "astromech-1zhg"
issue_type: "decision"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T04:50:40.102708814+00:00"
status: "done"
title: "Testing: fake only true boundaries; UI tests behind one transport; a small journey suite end to end"
updated_at: "2026-11-07T04:50:40.102708814+00:00"
---

## Context

Today about 26 frontend modules branch on `inTauri` to a fake core with hand-written fixtures. Only the setup and calibration replies and mining.json are pinned to the real core. The 512 frontend tests run in jsdom, and no automated test crosses from the journal to the page. Reference: T3 Code's strategy (docs/orchestration-v2/testing-strategy.md and AGENTS.md there): doubles only at true process, network, clock, id and filesystem boundaries; recorded raw transcripts as fixtures; few high-value integration tests; UI tests in real Chromium faking only the socket; determinism enforced by lint; no retries.

## Question

Where do astromech's test seams go, and what kinds of tests does it keep, now that the window's hand-written fixtures drift from the real core and no test drives a user journey?

## Options

- Seam at the transport only: fast UI tests, but every fixture is a guess about the core.
- Seam at the world only (game files, network, clock): everything real, but heavy for UI behaviour tests and unusable by the static preview.
- Both, each for its own job, plus a small automated journey suite.

## Decision

Both seams plus a journey suite, decided by Sumeet on 6 Nov 2026 in conversation.
- Test doubles only at true boundaries: game files, network, clock, input devices, audio and the clipboard. Core behaviour always runs for real. Fixtures are recorded raw inputs (journals, Spansh and Ardent responses), dumped or recorded from the real thing, never invented replies.
- Each case is tested once, at the lowest layer that fails. No tests that only mirror wiring, copy text or static markup. Logic bugs get a failing test first. Visual changes are proven with before-and-after captures, not tests.
- Algorithm tests (parsers, jump range and fuel, plotters, engineering maths) stay in Rust, table-driven.
- UI behaviour tests run in real Chromium (Vitest browser mode) and fake only the window's one transport. The fake transport is also the static preview's backend and never ships in the release bundle.
- A small automated journey suite (about 5 to 10 user journeys) runs on every push against the real headless core and a scripted world in headless Chrome. Faults are inputs to that world: an HTTP replay answering 503 or timing out, a truncated neutron star file, a refused clipboard write. Sumeet rejected leaving end-to-end coverage to agent-driven passes: those cost orders of magnitude more per run and don't run on every change. Agent passes remain for judging new UI.
- A Windows smoke test of the packaged app runs at release: launch, window up, connected, quit.
- Determinism is enforced: the core reads time through a clock tests control; tests wait on events and never sleep, poll or retry; a flaky test is fixed or quarantined against an issue; lint enforces what it can.

Consequences: the rules go into AGENTS.md and CONTRIBUTING.md; existing frontend tests move to browser mode and are pruned case by case as journeys cover them, not deleted in one go.

## Close Reason

Decided by Sumeet on 6 Nov 2026: doubles only at true boundaries, UI behaviour tests behind one transport in real Chromium, and a small journey suite end to end in CI; rationale and consequences in the Decision section.
