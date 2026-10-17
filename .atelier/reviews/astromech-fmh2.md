---
schema: atelier.review
schema_version: 1
id: astromech-fmh2
title: 'R1 [medium] The copy-budget clause is proven only with short fixture names. crates/astromech-core/tests/copy.rs the_cargo_answer_limits_its_partial_buyer_sentences checks each partial sentence against ANSWER_WORDS (16), but its stations are ''Other Port in Sol''. I measured with astromech_voice::speakable::normalise: ''Low Temperature Diamonds partly sells at Carvalho Engineering Foundry in Luyten''s Star: 1,073 of 1,100 tonnes, 259,950 credits each.'' is 19 spoken words, and the station is the one in the existing trade_voice fixture. check() also flags the apostrophe as a contraction. So the test passes because of its fixture names, and ''stays within the copy guide''s budget'' is not shown for real data. Lookup answers are otherwise outside the copy test (''built from live data''), and the main cargo sentence was already over 16 words. Fix: either tighten the sentence (e.g. drop ''in {system}'' or ''of N tonnes'') and test it with a realistic long name, or have the orchestrator state the budget this answer is held to (per sentence, names excepted) in the done-when and the test''s doc comment. The limit to two sentences and the dropped repeated station are correct.'
target: astromech-243w
identity: review-agent (opus, independent)
commit: 6a3910d6d36c5ba561d282fa63c537001b31e7cd
created_at: 2026-10-17T06:09:48.004403909Z
event:
  event: finding
  recommendation: fix_now
---

R1 [medium] The copy-budget clause is proven only with short fixture names. crates/astromech-core/tests/copy.rs the_cargo_answer_limits_its_partial_buyer_sentences checks each partial sentence against ANSWER_WORDS (16), but its stations are 'Other Port in Sol'. I measured with astromech_voice::speakable::normalise: 'Low Temperature Diamonds partly sells at Carvalho Engineering Foundry in Luyten's Star: 1,073 of 1,100 tonnes, 259,950 credits each.' is 19 spoken words, and the station is the one in the existing trade_voice fixture. check() also flags the apostrophe as a contraction. So the test passes because of its fixture names, and 'stays within the copy guide's budget' is not shown for real data. Lookup answers are otherwise outside the copy test ('built from live data'), and the main cargo sentence was already over 16 words. Fix: either tighten the sentence (e.g. drop 'in {system}' or 'of N tonnes') and test it with a realistic long name, or have the orchestrator state the budget this answer is held to (per sentence, names excepted) in the done-when and the test's doc comment. The limit to two sentences and the dropped repeated station are correct.
