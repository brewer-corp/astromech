---
schema: atelier.review
schema_version: 2
id: astromech-bnrg
title: finding on astromech-y6d6 @7441499d
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 7441499d59915f8285815090520428ce34af0146
created_at: 2026-11-07T20:42:17.443545760Z
event:
  event: finding
  recommendation: no_action
---

info: .github/workflows/one-process.yml:11-15 (7441499d). With scripts/one_process_probe.py dropped from its paths, the probe's spike-only paths (checks 2-3, before_build, baseline) and its --restart mode run on no PR. release.yml runs the probe (it lists the script in its paths), but only checks 1 and 4-7 against the installed app. astromech-fmub (Sumeet's PC: the spike exe plus --restart) and astromech-dl53's future job depend on --restart, so a probe change could break restart mode unnoticed until then. Acceptable: xz7i reduces the probe to its restart mode and deletes this workflow, and dl53 will add its own job. Until then, a probe edit that touches restart mode should be run by workflow_dispatch.
