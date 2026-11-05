---
schema: atelier.review
schema_version: 1
id: astromech-geeg
title: finding on astromech-51ue @0b6806b9
target: astromech-51ue
identity: reviewer-sonnet-f7i0
commit: 0b6806b99ee217651e9f848144dc2f608e9b5db2
created_at: 2026-11-05T09:13:09.745646937Z
event:
  event: finding
  recommendation: defer
---

low: astromech-f7i0 crates/astromech-core/tests/voice-eval-phrases.tsv, scripts/voice-eval.sh: the eval swap (trade.sell 'sell low temperature diamonds', find.service 'nearest universal cartographics' out; two mining.hotspots in) is acceptable under CONTRIBUTING (a 200-phrase set, enforced with every-command coverage by tests/voice_eval_phrases.rs, which passes) and follows e6ba5919's swap; the removed phrases' commands still have others. The new {mineral} slot words (void opal, benitoite, rhodplumsite, serendibite, LTD) have never been synthesised or transcribed, so recognition of the slot is unmeasured. Not rerun here: it needs a release sherpa build plus 830 MB models (models are cached but the build is a multi-core job on the live host). Defer: run scripts/voice-eval.sh on boat before the next release and record the mining family's score in docs/evidence.
