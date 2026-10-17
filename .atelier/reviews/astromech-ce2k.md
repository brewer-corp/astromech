---
schema: atelier.review
schema_version: 1
id: astromech-ce2k
title: 'Portable detection documented, not coded (issue''s second item): acceptable. The shell starts the core from its own folder (start_core: current_exe().with_file_name(CORE_EXE)), so both decide portable mode from the same folder by construction; only a hand-run core elsewhere diverges, and ARCHITECTURE.md now says so. The shell wiring still has no test (not compiled on Linux); the logic is in the tested Paths::carry_over and the call site is 4 lines.'
target: astromech-7w5d
identity: review-agent (opus, independent)
commit: 8a46cb6d7e122d233226efd204a31f8aee0f1735
created_at: 2026-10-17T07:46:22.180774711Z
event:
  event: finding
  recommendation: no_action
---

Portable detection documented, not coded (issue's second item): acceptable. The shell starts the core from its own folder (start_core: current_exe().with_file_name(CORE_EXE)), so both decide portable mode from the same folder by construction; only a hand-run core elsewhere diverges, and ARCHITECTURE.md now says so. The shell wiring still has no test (not compiled on Linux); the logic is in the tested Paths::carry_over and the call site is 4 lines.
