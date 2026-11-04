---
schema: atelier.review
schema_version: 1
id: astromech-wlsx
title: finding on astromech-j75v @9f516d3e
target: astromech-j75v
identity: review-agent-j75v-1
commit: 9f516d3e479fae0aa610989b7b29db820e240589
created_at: 2026-11-04T21:25:05.132059195Z
event:
  event: finding
  recommendation: fix_now
---

major: crates/astromech-intent/src/lib.rs:979 The split is kept only when the head scores at least as well as the whole transcript, so the faction phrases that end in 'near me' (where is there a famine/war/boom/outbreak/civil unrest near me, famine near me) lose the named system. Probed against assistant::grammar() at 9f516d3e: 'where is there a famine near Sol' -> faction.famine origin=None Ask 'Did you mean where is there a famine near me?' (same for war, boom, outbreak; 'famine near Sol' 0.67 Ask), and 'where is there civil unrest near Sol' -> faction.improvised origin=None Execute 0.86, a silent search from the current system with no 'From Sol.'. A yes to the Ask also learns the transcript as an alias (assistant.rs:499: slot and from are both None), after which 'where is there a famine near Sol' executes from the current system every time (probe: add_alias then best -> Execute, origin None). This contradicts docs/guide/lookups.md:21 and :158 ('Any of these except colonisation can end with near <system>') and the strategy's 'every lookup works from ... a typed one ... as a voice query'. The new tests avoid it by using 'nearest famine system near ...'. Fix: compare the head against the phrase with its own trailing 'near me' removed (or drop 'near me' from those phrases and let HERE_WORDS cover it), and add the 'where is there a famine near Sol' / 'civil unrest near Sol' cases to a_named_system_is_the_origin_not_part_of_the_slot.
