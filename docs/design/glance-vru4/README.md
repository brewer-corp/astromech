# Glance test: Station after astromech-vru4

Station, docked, glance-test row 4. The answer sheet expects the main value to be the gain on
the hold's first line, +3,921 cr/t for Gold. Can change expects CHECK AGAIN. Needs you expects
none. Both earlier readers took For sale here's +52,506 cr/t on Tritium as the main value.

astromech-vru4 makes two changes. The first cargo gain is drawn in Value XL, Orange Lit, and
For sale here's ranked profits stay Orange at Value L. The Market section also moves up, right
under the station's name, and the fact list follows it under its own heading, "The station".

The captures were taken with `node scripts/browser/audit.mjs --plan
docs/design/glance-vru4/plan.json`. They use the glance test's query and clock, at 2560 x 1440
and 145 percent, and at 1100 x 700 and 100 percent. `before/` is main at e4d00425. `after/` is the
change. The audit finds nothing clipped, overlapping or sideways in any capture.

## Readers

Each reader was a fresh general-purpose agent (Claude Opus 5.5). It got the glance test's
round 2 prompt, written for one screenshot and without the route question, and the after
capture `station-2560x1440.png` copied alone as `01.png`. It got nothing else. The answers are
verbatim and are marked under the glance test's round 1 marking notes. M, C and N are the
marks for main value, can change and needs you.

| Reader | Answer (main value \| can change \| needs you) | M | C | N |
|---|---|---|---|---|
| 1 (ae2a3589592c379fc) | Test Orbital's market, led by "+3,921 cr/t" on your 20 t of Gold (the biggest figure) \| "CHECK AGAIN" re-asks for market data; the copy icons copy station and system names \| the market data is 4 days old ("Market as of 2026-10-16 … 4 d ago"), and the orange "CHECK AGAIN" next to it prompts a refresh | pass | pass | **fail** |
| 2 (a6775cf97d5e0dcde) | +3,921 cr/t (Gain on Gold, your cargo at Test Orbital) \| CHECK AGAIN (refreshes the market data) \| none | pass | pass | pass |

Both readers name the expected main value. Reader 1 names the market's age as needing
attention, where the sheet says none. Round 1's reader did the same, before this change.

Before the Market moved up, one reader (aab26d8de91da8ed5) saw only the size change. It split the
main value between the title "Test Orbital" and +3,921 cr/t, a fail. Its needs-you answer was
also the market's age. That is why the change also moves the Market up, as astromech-8jqg did
on Now.

astromech-crxa copies these answers into `docs/design/final/glance-test.md` when it resumes.
