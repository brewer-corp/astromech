---
created_at: "2026-10-22T23:05:31.303091411+00:00"
id: "astromech-0zwk"
evidence_type: "validation"
commit: "eb5b71b2fdbffdd1dadc903364da4756c4b78d2d"
result: "pass"
captured_at: "2026-10-22T23:05:31.303015339+00:00"
agent_identity: "claude-implementer-6bjl"
target:
  kind: "issue"
  id: "astromech-6bjl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6bjl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Measured on this host (12 cores, load ~9, release, real file of 5,045,154 neutron stars, one route per process): Sol-Colonia R60 k4 114 jumps/101 stops 0.63 s peak 93 MB; Sol-Colonia R80 k6 53/51 1.00 s 100 MB; Sol-Beagle Point R60 k4 383/279 0.60 s 91 MB; R80 k6 161/146 1.34 s 103 MB. Peak is the process during the search; 79 MB is the loaded file (load 0.1-0.2 s, 153 MB peak while opening). Names read only for the stops (<1 ms). Unbounded plain A* gives 110 (2.9 s) and 51 (2.6 s) for the Colonia routes, so guided routes are 2-4 jumps over the fewest there. Spansh (eff 60), its count / in the model: Colonia R50 k4 166/160 jumps 128 stops vs local 139 jumps 119 stops; Beagle R60 k4 467/438, 298 vs 383, 279; Beagle R80 k6 190/181, 150 vs 161, 146. Spansh's total_jumps field is the number of legs, not jumps."
updated_at: "2026-10-22T23:05:31.303595019+00:00"
---

Measured on this host (12 cores, load ~9, release, real file of 5,045,154 neutron stars, one route per process): Sol-Colonia R60 k4 114 jumps/101 stops 0.63 s peak 93 MB; Sol-Colonia R80 k6 53/51 1.00 s 100 MB; Sol-Beagle Point R60 k4 383/279 0.60 s 91 MB; R80 k6 161/146 1.34 s 103 MB. Peak is the process during the search; 79 MB is the loaded file (load 0.1-0.2 s, 153 MB peak while opening). Names read only for the stops (<1 ms). Unbounded plain A* gives 110 (2.9 s) and 51 (2.6 s) for the Colonia routes, so guided routes are 2-4 jumps over the fewest there. Spansh (eff 60), its count / in the model: Colonia R50 k4 166/160 jumps 128 stops vs local 139 jumps 119 stops; Beagle R60 k4 467/438, 298 vs 383, 279; Beagle R80 k6 190/181, 150 vs 161, 146. Spansh's total_jumps field is the number of legs, not jumps.
