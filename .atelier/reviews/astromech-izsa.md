---
schema: atelier.review
schema_version: 1
id: astromech-izsa
title: 'R1 low, contract wording: 6bjl''s Done-when says the search ''minimises jumps, then stops'', but every real route measured goes to the guided phase (Route::fewest false) and is near-fewest: per the author''s unbounded A* runs 114 vs 110 (Sol-Colonia R60 k4) and 53 vs 51 (R80 k6). The Unknowns pre-authorised a looser heuristic if plain A* missed 2 s, the measurement is in the module docs (crates/astromech-galaxy/src/neutron_plot.rs:22-33, 49-51), Route::fewest says honestly which kind a route is, and the parent''s goal (no more jumps than Spansh) is met with margin (139/383/161 vs 160/438/181 in the model). So the behaviour is acceptable, but the Done-when as written is not met and the issue would close against a false statement. Fix: amend 6bjl''s Done-when (exact when plain A* finishes within its work limit; otherwise guided and near-fewest, reported by Route::fewest, never more jumps than Spansh''s recorded routes), or get Sumeet''s call if near-fewest is not acceptable. Planning edit only; no code change.'
target: astromech-1842
identity: review-agent-6bjl-1
commit: 730a39ec6ed356aba4bd173975f5c46745944c48
created_at: 2026-10-22T23:18:34.325764303Z
event:
  event: finding
  recommendation: fix_now
---

R1 low, contract wording: 6bjl's Done-when says the search 'minimises jumps, then stops', but every real route measured goes to the guided phase (Route::fewest false) and is near-fewest: per the author's unbounded A* runs 114 vs 110 (Sol-Colonia R60 k4) and 53 vs 51 (R80 k6). The Unknowns pre-authorised a looser heuristic if plain A* missed 2 s, the measurement is in the module docs (crates/astromech-galaxy/src/neutron_plot.rs:22-33, 49-51), Route::fewest says honestly which kind a route is, and the parent's goal (no more jumps than Spansh) is met with margin (139/383/161 vs 160/438/181 in the model). So the behaviour is acceptable, but the Done-when as written is not met and the issue would close against a false statement. Fix: amend 6bjl's Done-when (exact when plain A* finishes within its work limit; otherwise guided and near-fewest, reported by Route::fewest, never more jumps than Spansh's recorded routes), or get Sumeet's call if near-fewest is not acceptable. Planning edit only; no code change.
