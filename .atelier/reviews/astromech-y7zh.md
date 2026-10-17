---
schema: atelier.review
schema_version: 1
id: astromech-y7zh
title: 'Reviewed 16197f3 and the diff since e9945e7 (fix plus main merge, no oo8y code changes beyond the fix). Arming rule is correct: LoadGame resets to armed and unspoken, end triggers need armed and not spoken, so a real logout, Shutdown and menu still speak once, the next login re-arms, and the catch-up stays silent because no rule runs while not live. The test a_menu_before_the_login_of_a_fresh_launch_says_nothing would fail without the fix (MainMenu with commander in state would set pending and ask the history) and also checks the re-arm. The replay.rs change only prepends a LoadGame to the all-rules live line set; the catch-up assertions are untouched, so it does not weaken them. credits() is now one pub fn in astromech-announce::session_summary, used by session_summary and tools/lookup.rs (tools already depends on announce, so no new edge and check-deps rules are unaffected); no other copies remain; credits_round_into_millions_at_the_boundary covers 999,499/999,500/999,999. cargo test -p astromech-announce -p astromech-tools -p astromech-core all pass. One DEFER finding (mid-session app start leaves the summary unarmed). Not run: pnpm/clippy/check-deps (relied on the author''s post-merge gate evidence).'
target: astromech-oo8y
identity: review-agent (sonnet, independent)
commit: 9b1966aabd613a587ba08e605b06a50a2f416294
created_at: 2026-10-17T05:18:02.572648546Z
event:
  event: approval
---

Reviewed 16197f3 and the diff since e9945e7 (fix plus main merge, no oo8y code changes beyond the fix). Arming rule is correct: LoadGame resets to armed and unspoken, end triggers need armed and not spoken, so a real logout, Shutdown and menu still speak once, the next login re-arms, and the catch-up stays silent because no rule runs while not live. The test a_menu_before_the_login_of_a_fresh_launch_says_nothing would fail without the fix (MainMenu with commander in state would set pending and ask the history) and also checks the re-arm. The replay.rs change only prepends a LoadGame to the all-rules live line set; the catch-up assertions are untouched, so it does not weaken them. credits() is now one pub fn in astromech-announce::session_summary, used by session_summary and tools/lookup.rs (tools already depends on announce, so no new edge and check-deps rules are unaffected); no other copies remain; credits_round_into_millions_at_the_boundary covers 999,499/999,500/999,999. cargo test -p astromech-announce -p astromech-tools -p astromech-core all pass. One DEFER finding (mid-session app start leaves the summary unarmed). Not run: pnpm/clippy/check-deps (relied on the author's post-merge gate evidence).
