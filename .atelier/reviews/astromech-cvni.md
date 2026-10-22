---
schema: atelier.review
schema_version: 1
id: astromech-cvni
title: 'R1 medium crates/astromech-voice/src/speakable.rs:70 (SCALES): B is still read as billion after a space, so catalogue star and body names are misread: ''LHS 3447 B'' -> ''L H S 3447 billion'', ''HIP 36601 B 1 a'' (a site lookup answer, galaxy/src/sites.rs:299 speaks the full body name) -> ''Hip 36601 billion 1 a'', ''Wolf 359 B'', ''Pad 5 B''. No producer writes a spaced B (grep finds only biology''s ''{} M''), and sdg7''s goal is exactly this class of context-free scale false positive. Fix: make B glued-only (false) and add negative rows such as ''HIP 36601 B 1 a'' and ''LHS 3447 B''. Spaced M stays for biology; consider whether any body names end in a spaced M.'
target: astromech-uuqt
identity: review-agent-uuqt-1
commit: 70f1895d6b71686794c8ef3f4fd2a6885d716b46
created_at: 2026-10-22T22:59:21.384217440Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium crates/astromech-voice/src/speakable.rs:70 (SCALES): B is still read as billion after a space, so catalogue star and body names are misread: 'LHS 3447 B' -> 'L H S 3447 billion', 'HIP 36601 B 1 a' (a site lookup answer, galaxy/src/sites.rs:299 speaks the full body name) -> 'Hip 36601 billion 1 a', 'Wolf 359 B', 'Pad 5 B'. No producer writes a spaced B (grep finds only biology's '{} M'), and sdg7's goal is exactly this class of context-free scale false positive. Fix: make B glued-only (false) and add negative rows such as 'HIP 36601 B 1 a' and 'LHS 3447 B'. Spaced M stays for biology; consider whether any body names end in a spaced M.
