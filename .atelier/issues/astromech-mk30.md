---
created_at: "2026-10-22T19:44:28.676083239+00:00"
id: "astromech-mk30"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-mk30
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Run atelier doctor in CI"
updated_at: "2026-10-22T23:05:41.552135298+00:00"
---

## Goal

Every PR and every push to `main` runs Atelier's full check (`atelier doctor`) on a fresh clone with full history, so the whole tracker history is verified in one place. The local Stop hook's `atelier land` then only has to check what a session changed, and trusting what is already on origin becomes something CI verified rather than something assumed.

## Done when

- A workflow (its own file, so `ci.yml` can keep ignoring `.atelier/**` for the build jobs) runs on pull requests and on pushes to `main`, including record-only commits, with `fetch-depth: 0`.
- It installs Atelier at a pinned commit of spadaval/atelier (no releases are published; `cargo install --git ... --rev <sha> atelier-cli`, cached), runs `atelier doctor`, and fails the job when it fails.
- It passes on `main` once atelier-as7i is fixed and the pin is moved to include the fix; before that it is shown failing on `main` with "review event history missing", which is the expected result.
- CONTRIBUTING.md says the check exists and how to run it locally (`atelier doctor`).

## Scope

New workflow file and CONTRIBUTING.md. This is a CI change, so it goes through a work branch, review and `atelier merge`, not straight to `main`.

## Context

Discussed with Sumeet on 22 Oct 2026. Atelier already splits the checks (atelier-37uq in spadaval/atelier): `land` checks only changed claims; `doctor` checks every done claim and the whole review history and was meant for CI. On 649dc8ec, `atelier doctor` fails here in about 8 s on the merge-commit review records described in astromech-sx84.

## Unknowns

How long a cached `cargo install` of the pinned Atelier takes on the hosted runner; if it is slow, build the binary once per pin and cache it by the pin.
