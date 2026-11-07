---
schema: atelier.review
schema_version: 2
id: astromech-0jlg
title: finding on astromech-bp23 @791ff421
target: astromech-bp23
identity: claude-reviewer-se44
commit: 791ff421536f6dbcc4eac9a98c4a0ad985c55af0
created_at: 2026-11-07T09:14:41.863162706Z
event:
  event: finding
  recommendation: no_action
---

low: .atelier/issues/astromech-se44.md Done when 2 (decision note) The whole-set drift test lives in astromech-core (bindings_are_the_checked_in_typescript: missing, differs and stale), while cargo test -p astromech-api checks only its own type files and commands.ts lines (check_part). Seen failing here: a hand edit of types/ShellInfo.ts, a stale types/Stale.ts and an edited quit line in commands.ts fail the core test (all three named); the api test catches ShellInfo.ts. Both run in cargo test --workspace on CI, and check-deps forbids astromech-api depending on astromech-core (dev-deps included), so the deviation meets the requirement's intent and the only literal fix would move the core's reply types into astromech-api. Accept; the plan owner should reword the Done when line to name cargo test -p astromech-core --lib bindings, as CONTRIBUTING already does.
