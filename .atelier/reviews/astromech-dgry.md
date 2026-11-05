---
schema: atelier.review
schema_version: 1
id: astromech-dgry
title: finding on astromech-zirr @6b5f5330
target: astromech-zirr
identity: review-agent-zirr-1
commit: 6b5f5330b71aabb6a208d1b218c441b4d7fda5f9
created_at: 2026-11-04T23:34:14.138557466Z
event:
  event: finding
  recommendation: no_action
---

info: crates/astromech-store/src/paths.rs exe_name, the cfg(not(windows)) branch, reads /proc/<pid>/exe. On a non-Windows host without /proc, such as macOS, every note looks dead, so the window always uses the default config and ignores a live core's --config. On Windows the same thing happens if OpenProcess is refused. Either way the result is the same as before f2k1, not a crash or a wrong path. The product targets Windows, and this branch is labelled development only. No change needed unless macOS development becomes a target.
