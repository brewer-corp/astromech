---
schema: atelier.review
schema_version: 1
id: astromech-0nz3
title: '[low] scripts/gen-exobiology.py star_by_name: when the name rule finds no star it returns None instead of the search''s immediate parent, and a single-letter component with no ''Sys X'' star returns None. In named systems whose stars carry other catalogue names this drops known stars: over the 24,756 cached bodies, 38 have an immediate parent star yet get None (LFT 1504 B 3 with stars {''LFT 1504'',''Gl 773 B''}; LFT 601 A 4; Gilgamesh B 8), and about 84 more are moons like ''Eme A 1 a'' (stars {''Eme'',''Gliese 425 B''}). About 0.5 % of bodies, so the stated figures hold. Fix: fall back to o[''star''] when the name rule gives nothing, and map ''Sys A'' to the star named as the system when no ''Sys A'' exists. Otherwise the rule checks out: longest-prefix match picks brown dwarfs (''AB 7'', ''A 5'') for their moons; multi-letter barycentres get None, as the runtime''s first-Star-in-Parents does; the name and parent agree on 6948/6948; SYSTEM_MASK (low 55 bits) is right; the held-out split is applied after star assignment and conditions come from train only.'
target: astromech-ys47
identity: review-agent (opus, independent)
commit: 918807c043016e7baad156f8bd7982eddeb732a5
created_at: 2026-10-17T08:13:32.421226184Z
event:
  event: finding
  recommendation: defer
---

[low] scripts/gen-exobiology.py star_by_name: when the name rule finds no star it returns None instead of the search's immediate parent, and a single-letter component with no 'Sys X' star returns None. In named systems whose stars carry other catalogue names this drops known stars: over the 24,756 cached bodies, 38 have an immediate parent star yet get None (LFT 1504 B 3 with stars {'LFT 1504','Gl 773 B'}; LFT 601 A 4; Gilgamesh B 8), and about 84 more are moons like 'Eme A 1 a' (stars {'Eme','Gliese 425 B'}). About 0.5 % of bodies, so the stated figures hold. Fix: fall back to o['star'] when the name rule gives nothing, and map 'Sys A' to the star named as the system when no 'Sys A' exists. Otherwise the rule checks out: longest-prefix match picks brown dwarfs ('AB 7', 'A 5') for their moons; multi-letter barycentres get None, as the runtime's first-Star-in-Parents does; the name and parent agree on 6948/6948; SYSTEM_MASK (low 55 bits) is right; the held-out split is applied after star assignment and conditions come from train only.
