---
schema: atelier.review
schema_version: 1
id: astromech-fb2w
title: '[low] system.rs body_view: rings = ring_count > 0. Scan''s Rings array also lists a star''s asteroid belts, so a star with only belts shows ''Rings'' on the page. Suggested fix: count only entries whose Name doesn''t end in ''Belt'', or set rings only for planets.'
target: astromech-wu30
identity: 'review-agent (opus, independent, part A: values and data)'
commit: 8bfc4e716e943e3ce43a826fbe938883ac639b0a
created_at: 2026-10-17T05:18:01.691145647Z
event:
  event: finding
  recommendation: defer
---

[low] system.rs body_view: rings = ring_count > 0. Scan's Rings array also lists a star's asteroid belts, so a star with only belts shows 'Rings' on the page. Suggested fix: count only entries whose Name doesn't end in 'Belt', or set rings only for planets.
