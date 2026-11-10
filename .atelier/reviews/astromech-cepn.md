---
schema: atelier.review
schema_version: 2
id: astromech-cepn
title: finding on astromech-xajc @148c7445
target: astromech-xajc
identity: GPT-6 Astra (round 4)
commit: 148c74450f482a93c1e4d2813fbfd83d530f2572
created_at: 2026-11-10T06:19:34.468271796Z
event:
  event: finding
  recommendation: fix_now
---

P2: 148c7445 removes the module-wide enrollment deadlock from 0e3x, but irtr Done when 2 still gives a validator conflicting instructions for Places home (.atelier/issues/astromech-irtr.md:59). Its first sentence requires font-size checking for the files z5oh enrolled; z5oh Done when 4 explicitly requires Places home among those files (.atelier/issues/astromech-z5oh.md:41). The new last sentence instead says files left for later items, explicitly including Places home, are expected to be off the list. A correct increment-1 implementation has places/Home.svelte enrolled for font-size but not width: its font declarations are removed by z5oh, while its max-width remains until gvy0, which waits on irtr. A validator enforcing the last sentence can reject that correct state or demand undoing required enrollment. Fix only that sentence to say the deferred PART remains disabled, explicitly naming Places home as width off and font-size on. The strategy Risks change and the underlying per-file/per-part ownership are sound. No other blocking finding from the full Ready Work pass. The scale formula agrees in k6pl and the strategy; Normal gives 1, 1, 1.44 and 1.8 at the four specified sizes. The explicit post-clamp Text size multiplier makes Smaller/Larger intentional overrides of those base values (0.85/1.15 at 1100x700 and 1.53/2.07 at 4K), so the default-size acceptance is meetable. Review covers the strategy, mission brief, all requested issues, prior findings, relevant Places/Settings files and ESLint configuration; no implementation, mockup or Windows runtime validation.
