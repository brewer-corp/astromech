---
schema: atelier.review
schema_version: 3
round: 1
commit: 95e9c0b37c0b5370b3dfd25326208928a7fcc0b4
reviewers:
- thread:delegated-task:command%3Amcp%3A0e8d811f-87e0-4f49-9833-ec0e0b4769d1%3Adelegate-task%3A0ae1af71-c61e-4407-81c8-f70ed9a2b749
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T06:10:08.313736356Z
---

## Summary

Complexity: high. This is the binding design contract for several later items, with colour,
type, row, navigation and frame rules that must agree across the document.

Value: high. The approved revision needs one written reference before the implementation
items start.

Risk of merging: low. This changes documentation and reference captures, is easy to revert,
and makes no runtime change; an inconsistent rule can still send a later item the wrong way.

The value warrants the scope. Updating PRODUCT.md's three conflicting colour/selection
phrases is justified, and keeping that correction in its own commit makes it easy to undo.
A smaller edit would leave binding rules in conflict.

I reviewed `git diff main...95e9c0b37c0b5370b3dfd25326208928a7fcc0b4`, read DESIGN.md and
PRODUCT.md, and checked the Done when lines against the mission, its 17 Nov decision notes,
and the related text-size, palette and result-row items. The requested three grey steps,
semantic colour list, two reds, selection treatment, grouped rows, game symbols, frame,
ten tabs, rem sizing and input value weight are present. The Buy exception in Finding 1
needs correcting before this becomes the shared rulebook.

I inspected the Sell captures at both sizes, Engineers at rest and with a search, Suits,
the collapsed Variant B frame and the narrow route overlay, and compared their rules with
the spike's Frame, RouteSide, tone.css, Sell5 and Engineers5 sources. The stated five whole
rows at 1100 x 700 match the capture. The larger capture contains eight rows with spare
space; the document correctly labels the eleven-row figure an estimate. The old colours,
Variant A overlay and one-action no-route reference are identified as older examples rather
than silently presented as the final rule.

Validation:

- `python3 scripts/check-guide.py` passes: 22 guide pages, zero problems. It does not scan
  DESIGN.md, so I separately ran its link/anchor checks over DESIGN.md and PRODUCT.md:
  zero problems.
- Recalculated WCAG contrast from the hex values. The readable grey steps meet 4.5:1 on
  Void, Hull and Hull Raised. Red Low is 5.1379 / 4.8486 / 4.5721, so the change from the
  spike's red protects selected rows as reported. The dim tokens are explicitly limited
  to state rather than necessary reading.
- All 14 imported nav-spike captures match their source Git blobs; all eight imported
  glow-up captures match theirs. PNG chunk CRCs pass. No capture cited by the Atelier
  records is missing, and glow-up-icons.png is unchanged from main.
- No component or tokens.css implementation is included, consistent with the scope.

I could not independently locate or check the spike's conversational final report; the
committed mission decision, source mockups and captures supply the approval evidence used
here. I did not test runtime changes or remeasure future rem breakpoints because those
implementations belong to the subsequent items, as the document and Report state.

## Finding 1: Keep Buy supply warnings when the hold is empty
Recommendation: fix now

DESIGN.md:555 says “Demand or supply with nothing aboard” uses Prose Grey bars with no
warning colour. That contradicts the preceding Buy rule at DESIGN.md:554, which measures
supply against free cargo space. It also contradicts astromech-3ca6's explicit requirement
that Buy use that comparison “including with an empty hold.”

For example, an empty 256 t hold and a seller with 96 t supply must show short supply, not
neutral bars. Following this exception would remove the warning on the normal starting
state for buying cargo. The exception also omits unknown capacity, although that means
Buy has no known denominator even when the cargo count is known.

Split the exception by finder: Sell is neutral when none of the searched commodity is
aboard or the sellable amount is unknown; Buy is neutral when free cargo space cannot be
known (cargo or capacity unknown). State what Buy does at known zero free space separately.
Keep an empty hold with known positive capacity in Buy's normal supply comparison.
