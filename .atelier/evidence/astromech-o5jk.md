---
created_at: "2026-11-05T16:01:42.156057846+00:00"
id: "astromech-o5jk"
evidence_type: "validation"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:01:42.156013012+00:00"
path: "docs/design/validation/finder-sell-1600.png"
agent_identity: "validator-fable-kyqw"
target:
  kind: "issue"
  id: "astromech-kyqw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kyqw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Claim 5: every finder shows the DESIGN.md Lookup result row and States. Observed on all ten finders at 1600 px: 44 px rows with the ranked value first (Value L, Orange Lit rgb(255,160,74): price on Sell/Buy, total on Sell my cargo, score on Faction states, hotspots on Mining hotspots, jumps elsewhere), the place with system and copy icon, fixed fact columns under Label heads over a hairline, then Add stop and Set destination; a one-line constraint summary above the heads ('Within 100 ly of Sol · medium or large pad · reports up to 30 days old'). Stale (Services, Data age Any): the Rescue Ship row (2,058 d) keeps its name Orange rgb(250,110,0), steps value and facts down to Orange Mid rgb(194,108,50) and shows its age in Orange; a fresh row's facts are Orange. Loading (?slow=never): one line 'Asking Spansh…'. Hard error (?fail=all): red '! preview: stations_nearby failed on purpose (?fail=)' with TRY AGAIN. Empty (Mining hotspots, Icy + Pristine): 'No rings within 100 ly.' with one action WIDEN TO 200 LY. At 1100 px Services and Mining hotspots have no horizontal overflow and no row taller than 44 px. Matches the ds-sell and ds-mining mockups' row grammar. Screenshots docs/design/validation/finder-*-1600.png, finder-{services,mining-hotspots}-1100.png, state-{loading,error,empty-hotspots}-*.png, mockup-ds-*.png. Tests: SearchKit.test.ts (13: row layout, compact row, stale, hidden-by-age, empty, loading-dim, failure) pass. Proof limits: the 'no position' failure (?core=empty) cannot be reached because that scenario shows onboarding; the dimmed old rows during a refresh were not captured (test only); the popover row measures 35 px against DESIGN's 34."
updated_at: "2026-11-05T16:01:42.156374898+00:00"
---

Claim 5: every finder shows the DESIGN.md Lookup result row and States. Observed on all ten finders at 1600 px: 44 px rows with the ranked value first (Value L, Orange Lit rgb(255,160,74): price on Sell/Buy, total on Sell my cargo, score on Faction states, hotspots on Mining hotspots, jumps elsewhere), the place with system and copy icon, fixed fact columns under Label heads over a hairline, then Add stop and Set destination; a one-line constraint summary above the heads ('Within 100 ly of Sol · medium or large pad · reports up to 30 days old'). Stale (Services, Data age Any): the Rescue Ship row (2,058 d) keeps its name Orange rgb(250,110,0), steps value and facts down to Orange Mid rgb(194,108,50) and shows its age in Orange; a fresh row's facts are Orange. Loading (?slow=never): one line 'Asking Spansh…'. Hard error (?fail=all): red '! preview: stations_nearby failed on purpose (?fail=)' with TRY AGAIN. Empty (Mining hotspots, Icy + Pristine): 'No rings within 100 ly.' with one action WIDEN TO 200 LY. At 1100 px Services and Mining hotspots have no horizontal overflow and no row taller than 44 px. Matches the ds-sell and ds-mining mockups' row grammar. Screenshots docs/design/validation/finder-*-1600.png, finder-{services,mining-hotspots}-1100.png, state-{loading,error,empty-hotspots}-*.png, mockup-ds-*.png. Tests: SearchKit.test.ts (13: row layout, compact row, stale, hidden-by-age, empty, loading-dim, failure) pass. Proof limits: the 'no position' failure (?core=empty) cannot be reached because that scenario shows onboarding; the dimmed old rows during a refresh were not captured (test only); the popover row measures 35 px against DESIGN's 34.
