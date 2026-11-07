---
schema: atelier.review
schema_version: 2
id: astromech-63ma
title: finding on astromech-y6d6 @f19adab5
target: astromech-y6d6
identity: claude-reviewer-7i2k
commit: f19adab5f35eae79f75396ae144710964cc6d453
created_at: 2026-11-07T17:15:13.794466080Z
event:
  event: finding
  recommendation: defer
---

low: .atelier/issues/astromech-fmub.md. The main copy (424b8a18) and the branch copy (2cb0a07f) differ in two places: the branch keeps the discovered_from link to evidence astromech-fqw9 and words Context differently. Both sides added the file, so 'git merge-tree main HEAD' reports an add/add CONFLICT on this file and nothing else. The next main merge or atelier merge will stop on it. Resolve to the branch copy, which is correct once fqw9 is on main, or make the two copies identical before merging.
