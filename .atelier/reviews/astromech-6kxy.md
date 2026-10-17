---
schema: atelier.review
schema_version: 1
id: astromech-6kxy
title: 'medium: Signals::react (crates/astromech-announce/src/signals.rs) records a line as said before the engine''s combat/docking gate runs (lib.rs fire(): speech cleared when quiet and priority < Urgent). A conflict zone, Thargoid or megaship signal discovered while in danger, interdicted or docking is dropped and never repeated in that system, because the dedupe already holds it. Fix: only record a line when it can be spoken, or mark the callouts so a dropped line can be retried. Defer is fair: rare, and a HUD-adjacent honk during combat. Also low: ''Titan'' => Thargoid is unverified, as the module comment says; the SystemAddress-change reset is never tested apart from FSDJump.'
target: astromech-ew3x
identity: review-agent (sonnet, independent)
commit: 033ff69ae9c102b6e7923c256a242471305788b3
created_at: 2026-10-17T03:13:35.833390174Z
event:
  event: finding
  recommendation: defer
---

medium: Signals::react (crates/astromech-announce/src/signals.rs) records a line as said before the engine's combat/docking gate runs (lib.rs fire(): speech cleared when quiet and priority < Urgent). A conflict zone, Thargoid or megaship signal discovered while in danger, interdicted or docking is dropped and never repeated in that system, because the dedupe already holds it. Fix: only record a line when it can be spoken, or mark the callouts so a dropped line can be retried. Defer is fair: rare, and a HUD-adjacent honk during combat. Also low: 'Titan' => Thargoid is unverified, as the module comment says; the SystemAddress-change reset is never tested apart from FSDJump.
