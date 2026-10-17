---
schema: atelier.review
schema_version: 1
id: astromech-gf51
title: 'Approved at 3dbca50. DockedHere::new now keeps state.system when undocked. Its only consumers are the docked_here shell command and the Station page, which branches on here.docked before reading anything else, so no code took ''has a system'' to mean ''docked'' (grep over Rust, TS and Svelte). The Station page shows the system chip and ExtLinks when not docked; Powerplay links ov.here or the Powerplay system; Materials links inv.here.system on the manufactured tab. The done-when page test exists and is real. Mutation-checked: deleting the Station View''s undocked line fails ''links the current system when no station is docked'', and reverting DockedHere::new to Self::default() fails dock.rs no_facts_when_not_docked. cargo test -p astromech-station passes, and vitest for station, powerplay and materials passes 23/23. Findings: R1, a doc sentence that misattributes the Materials and Powerplay links to DockedHere (DEFER), and R2, an unrestored spy (NO ACTION). Materials has no test of its own, which the done-when does not ask for. CI on PR #78: Linux and Windows build/tests pass; the Windows installer is pending.'
target: astromech-nkjz
identity: review-agent (opus, independent)
commit: 3dbca50d1488fd3d8c03b86114b8a7aa14f550b5
created_at: 2026-10-17T06:09:38.521024440Z
event:
  event: approval
---

Approved at 3dbca50. DockedHere::new now keeps state.system when undocked. Its only consumers are the docked_here shell command and the Station page, which branches on here.docked before reading anything else, so no code took 'has a system' to mean 'docked' (grep over Rust, TS and Svelte). The Station page shows the system chip and ExtLinks when not docked; Powerplay links ov.here or the Powerplay system; Materials links inv.here.system on the manufactured tab. The done-when page test exists and is real. Mutation-checked: deleting the Station View's undocked line fails 'links the current system when no station is docked', and reverting DockedHere::new to Self::default() fails dock.rs no_facts_when_not_docked. cargo test -p astromech-station passes, and vitest for station, powerplay and materials passes 23/23. Findings: R1, a doc sentence that misattributes the Materials and Powerplay links to DockedHere (DEFER), and R2, an unrestored spy (NO ACTION). Materials has no test of its own, which the done-when does not ask for. CI on PR #78: Linux and Windows build/tests pass; the Windows installer is pending.
