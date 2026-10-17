---
schema: atelier.review
schema_version: 1
id: astromech-ifqe
title: 'PR #126 at 810d8bd, read-only scratch worktree. Contract pass: with route_follow off a jump updates here and the route does not move, copy or speak; replays never move the route. Audited every Rule impl (18 in announce/src, test rules in tests/replay.rs, ScanRule in core pipeline.rs): only RouteFollow overrides while_off (default no-op); Engine::fire is the only dispatch site and the new loop runs only for disabled rules, before the live check. Ran cargo test -p astromech-announce -p astromech-core route (pass). Not rerun: fmt, clippy, full workspace, the fail-before checks, the gear_down flake. Three low findings, all no action.'
target: astromech-lo81
identity: review-agent (sonnet, independent)
commit: 810d8bd932424fffe8f71379eed3b8b68cb7e417
created_at: 2026-10-17T19:12:33.862021479Z
event:
  event: review
---

PR #126 at 810d8bd, read-only scratch worktree. Contract pass: with route_follow off a jump updates here and the route does not move, copy or speak; replays never move the route. Audited every Rule impl (18 in announce/src, test rules in tests/replay.rs, ScanRule in core pipeline.rs): only RouteFollow overrides while_off (default no-op); Engine::fire is the only dispatch site and the new loop runs only for disabled rules, before the live check. Ran cargo test -p astromech-announce -p astromech-core route (pass). Not rerun: fmt, clippy, full workspace, the fail-before checks, the gear_down flake. Three low findings, all no action.
