---
schema: atelier.review
schema_version: 3
round: 1
commit: 9ddec52b6b245b949dd5222b858fe6e0f487f416
reviewers:
- thread:delegated-task:command%3Amcp%3A760cbda8-2996-4833-b25f-dbb34a010847%3Adelegate-task%3A6308cd87-ad6d-482a-b01e-634ee630fc33
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T12:52:37.855169799Z
---

## Summary

Complexity: medium. The page moves to the shared Board and Tiles layouts, and a new pure merge combines two sources of biology data; the core and its contracts stay unchanged.

Value: medium. Biology is an existing exploration page, and the cards make its bodies, sampling progress and other players' reports readable together.

Risk of merging: medium. This changes a user-visible page and adds a shared Tiles option, but the presentation is reversible and does not change stored data or send anything outside the app.

The value warrants the change. The merge is isolated, the existing Tiles default is unchanged, and the documented wide option is a reasonable way to fit genus lines while keeping one or two columns at the minimum window size. I found no supported problem requiring a change or deferral.

I reviewed `git diff main...9ddec52b6b245b949dd5222b858fe6e0f487f416` against every Done when line, DESIGN.md and CONTRIBUTING.md. I read the page, merge function and table cases, Samples, Tiles, layout and tooltip components, lint enrollment, guide changes and the separate page-sizing test repair. I compared the data handling with the recorded exobiology reply and the Rust producers for bodies, known reports and sampling. The merge preserves commander order, appends reported-only bodies, retains species values and differing reported body ranges, avoids matching unrelated null IDs, and does not mutate the input genera or species. I also checked the genus-name matching rule against every species in the repository's 22-genus catalogue; each resolves to its actual genus, including names with the genus at the end.

I independently ran:

- The biology table and browser files: 13 tests passed.
- `pnpm check`: no errors or warnings.
- `pnpm lint`: passed.
- `pnpm test`: 97 files and 901 tests passed, with no overlapping full test run observed when starting it.
- `ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core node --test 'journeys/*.journey.ts'`: all seven passed.
- `git diff --check main...9ddec52b` and `python3 scripts/check-guide.py`: passed; 23 guide pages checked.

For visual evidence I inspected the committed before-and-after sampling captures at 1100 x 700, the after capture at 2560 x 1440, both plans and the complete audit report. Sampling was measured at all three required sizes and Text size steps; loading and initial failure were measured at 1100 and 1920. All seven after captures have empty sideways, clipped, overlaps, small, contrast and far lists. The exemptions concern the existing frame, and the bright list's luminance ordering is explained in the Report. The sampling distance has the page's only Orange Lit main value, with remaining distance beside it; the cards use the stated grey roles.

I did not open a new interactive browser session or rerun the screenshot audit. The saved report identifies an earlier HEAD because it was captured from the working tree; I cannot independently establish that capture provenance from the JSON alone. As disclosed in the Report, no-sampling and empty-system states have no captures. I inspected their branches, along with sampling completion, missing clearance, far-enough and stale-poll handling, in the code. There is no core change; the journeys used the existing debug core rather than a new build in this worktree.
