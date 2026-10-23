---
schema: atelier.review
schema_version: 1
id: astromech-fi33
title: 'docs/releases/v1.0.0-alpha.3.md:118-121 (Known limits) and :15: a neutron route plotted by Spansh still shows its leg count as its jumps (astromech-6h7t open; plot.rs:231 jumps = plan.total_jumps, which Spansh counts as legs; live Sol-Maia showed 1 for 8 real, Sol-Colonia 128 for 166). A player with no neutron file sees understated Spansh routes, and one comparing the two sees Spansh''s route look shorter, contradicting line 15''s ''never had more jumps than Spansh''s''. Fix: add a Known limit, e.g. ''A neutron route plotted by Spansh shows its number of legs as its jumps, so it looks shorter than it is (Sol to Colonia says 128, not 166).'''
target: astromech-rcui
identity: review-agent-rcui-1
commit: ecf43e6eede049aa060234ca4b46625c66f1651c
created_at: 2026-10-23T00:54:04.042049912Z
event:
  event: finding
  recommendation: fix_now
---

docs/releases/v1.0.0-alpha.3.md:118-121 (Known limits) and :15: a neutron route plotted by Spansh still shows its leg count as its jumps (astromech-6h7t open; plot.rs:231 jumps = plan.total_jumps, which Spansh counts as legs; live Sol-Maia showed 1 for 8 real, Sol-Colonia 128 for 166). A player with no neutron file sees understated Spansh routes, and one comparing the two sees Spansh's route look shorter, contradicting line 15's 'never had more jumps than Spansh's'. Fix: add a Known limit, e.g. 'A neutron route plotted by Spansh shows its number of legs as its jumps, so it looks shorter than it is (Sol to Colonia says 128, not 166).'
