---
schema: atelier.review
schema_version: 3
id: astromech-dyr0
title: approval on branch atelier-uqxh-agents @fb6bed36
branch: atelier-uqxh-agents
identity: independent reviewer subagent (claude-opus-5-5), atelier-uqxh
commit: fb6bed366844d5e9673b8dae18364c34e11c054d
created_at: 2026-11-07T16:25:06.230851718Z
event:
  event: approval
---

Checked fb6bed36 on origin/main e6ae38df (still the remote head): the only change is AGENTS.md:21-23, which now says only a code change makes an approval stale and a Done when line rewritten after start is shown in the review brief and by merge --check and refuses nothing. That matches atelier --help on spadaval/atelier branch atelier-uqxh, docs/product/reviews-and-merge.md (A Done when edited after the start: the note refuses nothing), and the agent-factory skill at 658dfe5 references/atelier.md:89-98, and drops the old 'still gets a fresh one' that contradicted 'only a code change makes an approval stale'. The rest of the paragraph still agrees with the skill's order and loose-end definition. No proof beyond reading applies to a one-line docs change.
