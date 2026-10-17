---
schema: atelier.review
schema_version: 1
id: astromech-aajo
title: Reviewed git diff main...HEAD at a22c52e against the issue and EDDI DataDefinitions/Body.cs estimateBodyValue (fetched from EDCD/EDDI develop). isPreOdysseyBubble = isFirstDiscoverer && !isFirstMapper matches Terms::pre_odyssey_bubble, including unknown WasDiscovered/WasMapped (EDDI ?? false == astromech == Some(false)). Mapping multiplier falls to 10/3 in both; the Odyssey 30% is skipped for bubble bodies; efficiency x1.25 kept; x2.6 skipped whether mapped or not; stars untouched in both. New test fails without the change by inspection (would add 30% and x2.6). cargo test -p astromech-history -p astromech-exploration -p astromech-journal pass, including the EDSM-checked values tests. Ring/belt doc comments are prose. One DEFER (pre-existing 500 cr floor order differs from EDDI for mapped small planets), one NO ACTION.
target: astromech-e8up
identity: review-agent (opus, independent)
commit: a22c52ee102a12166e52cd16373d6ec97cd996ce
created_at: 2026-10-17T08:00:25.666277763Z
event:
  event: approval
---

Reviewed git diff main...HEAD at a22c52e against the issue and EDDI DataDefinitions/Body.cs estimateBodyValue (fetched from EDCD/EDDI develop). isPreOdysseyBubble = isFirstDiscoverer && !isFirstMapper matches Terms::pre_odyssey_bubble, including unknown WasDiscovered/WasMapped (EDDI ?? false == astromech == Some(false)). Mapping multiplier falls to 10/3 in both; the Odyssey 30% is skipped for bubble bodies; efficiency x1.25 kept; x2.6 skipped whether mapped or not; stars untouched in both. New test fails without the change by inspection (would add 30% and x2.6). cargo test -p astromech-history -p astromech-exploration -p astromech-journal pass, including the EDSM-checked values tests. Ring/belt doc comments are prose. One DEFER (pre-existing 500 cr floor order differs from EDDI for mapped small planets), one NO ACTION.
