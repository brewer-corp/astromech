---
schema: atelier.review
schema_version: 1
id: astromech-molj
title: 'crates/astromech-core/src/speech.rs:513: interrupt() while Kokoro is loading clears the queued lines without printing them, and lines queued during the load are not printed until it ends. TtsSpeaker prints every line in say(), so a reply said just after the download that is followed by a PTT press within the load (seconds) is neither spoken nor printed. Low impact (the user is barging in); fix by printing cleared lines, or printing at queue time and having the start thread skip the print.'
target: astromech-mcdt
identity: review-agent (mcdt review)
commit: abf17ec4992104d88eff0a9ddfbc1e724bc1023d
created_at: 2026-10-17T00:02:18.301440237Z
event:
  event: finding
  recommendation: defer
---

crates/astromech-core/src/speech.rs:513: interrupt() while Kokoro is loading clears the queued lines without printing them, and lines queued during the load are not printed until it ends. TtsSpeaker prints every line in say(), so a reply said just after the download that is followed by a PTT press within the load (seconds) is neither spoken nor printed. Low impact (the user is barging in); fix by printing cleared lines, or printing at queue time and having the start thread skip the print.
