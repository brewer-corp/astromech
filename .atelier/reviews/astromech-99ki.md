---
schema: atelier.review
schema_version: 1
id: astromech-99ki
title: '[low] Stale doc comments after the fb2w fix: crates/astromech-journal/src/detail.rs:114 (ScanBody.ring_count) and crates/astromech-exploration/src/system.rs:39 (BodyView.rings) still say the field covers ''Rings, and asteroid belts'', but since f46e66f ring_count/rings exclude belts and belt_count/belts hold them. Rustdoc attaches each comment to the rings field only, so a reader of either struct is told the opposite of the behaviour. Fix: document rings as rings only and give belts its own line. Doc-only; fix when the file is next touched.'
target: astromech-wu30
identity: review-agent (opus, independent)
commit: 7d9ff52f2cb25136d95d2e07e4279564cbc2fdf2
created_at: 2026-10-17T05:29:07.844956979Z
event:
  event: finding
  recommendation: defer
---

[low] Stale doc comments after the fb2w fix: crates/astromech-journal/src/detail.rs:114 (ScanBody.ring_count) and crates/astromech-exploration/src/system.rs:39 (BodyView.rings) still say the field covers 'Rings, and asteroid belts', but since f46e66f ring_count/rings exclude belts and belt_count/belts hold them. Rustdoc attaches each comment to the rings field only, so a reader of either struct is told the opposite of the behaviour. Fix: document rings as rings only and give belts its own line. Doc-only; fix when the file is next touched.
