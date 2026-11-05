---
schema: atelier.review
schema_version: 1
id: astromech-6ems
title: approval on astromech-1n9b @1a25cada
target: astromech-1n9b
identity: review-agent-1n9b-1
commit: 1a25cada2df231c244763fd65bbc7293ddc93667
created_at: 2026-11-04T23:26:37.217796267Z
event:
  event: approval
---

Reviewed f7388d79..1a25cada (2882f5ff code, 1a25cada evidence/close) against Done when R4-R8, the toolbox strategy, V1-DESIGN 6.5, ARCHITECTURE 5.12 and the guide rule. R4: in/around/'<name> system'/'the <name> system' give the origin; 'from a station', 'near my carrier', 'near the station', 'in a hurry' search from here. R5: colonisation.buy takes an origin; 'From Sol.' lead and unknown name tested; progress/remaining keep none. R6: copy.rs runs named variants, the 'From X.' lead and the unknown line on a 12-word name. R7: re-ran my sweep of every non-lookup MVP phrase plus ' near Sol', ' from Sol', ' near mi' against assistant::grammar() and compared with the j75v head: every borrowed 'Did you mean famine near Sol?' (about 60) is now NoMatch, and no non-lookup gains an origin. Ask on a literal 'near me' is unchanged, as before j75v. R8: near mi/you/my current location/my ship/by search from here; 'Nemi' and 'Min' stay names. UnknownSystem keeps the app's Display text; named_origin maps it to the new line and anything else to 'Could not look that up.'. No slot value in commodities/outfitting/ships contains an origin word. Ran after touching every crate's src/lib.rs: cargo test -p astromech-intent -p astromech-core -p astromech-tools -p astromech-galaxy all pass, new tests seen running; clippy -D warnings, fmt, check-guide and check-deps clean. One minor deferred finding: 'in bulk' read as a name.
