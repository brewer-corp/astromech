---
schema: atelier.review
schema_version: 1
id: astromech-edaz
title: finding on astromech-f2k1 @47967160
target: astromech-f2k1
identity: review-agent-f2k1-1
commit: 47967160ec0492c800103d37c0319669e6210fb2
created_at: 2026-11-04T22:40:35.301473560Z
event:
  event: finding
  recommendation: no_action
---

info: crates/astromech-core/src/app.rs:1228 the note's lifetime in run relies on the named binding _published and on its declaration order relative to _guard. No test covers run itself, so rewriting it as let (config_path, _) = ... would drop the note at once with no compiler warning, because #[must_use] does not fire through a tuple pattern, and the tests would stay green. The comment documents this. Recording it for awareness; no change is needed now.
