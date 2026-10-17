---
created_at: "2026-10-17T05:48:08.154148644+00:00"
id: "astromech-s9kx"
evidence_type: "test"
commit: "f2cabbc3cedc3be1fca7a71dcb102a119e6b17a9"
result: "pass"
captured_at: "2026-10-17T05:48:08.154054128+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-znhx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-znhx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging main (astromech-p4ls) and reconciling at f2cabbc: one data/catalog/exobiology.json (genus-keyed, EDR species symbols, wiki values, uncertain notes) read by astromech_exobiology::catalog, the predictor and astromech_exploration::exobiology::species_value; biology view built on astromech_exploration::system (no second body tracker); Biology page in the Exploration group. Rerun: astromech-exobiology --test surface 4, --test predict 4 (48 Spansh bodies, 70 species), --test sampling 2; astromech-exploration --test biology 3, --test unsold 3, lib exobiology 2; astromech-announce --test sampling 4, --test biology 5, --test replay 8 (17 rules, catch-up silent); astromech-core --test copy 1, lib biology 3; all pass. scripts/ui headless Chrome on ?core=ready#/exobiology shows the page under Exploration next to System. Full gates /tmp/landq/gates.sh exit 0 (108 cargo test suites, check-deps 28 crates, shell cargo metadata --locked, pnpm lint/check/build/test 182 tests)."
updated_at: "2026-10-17T05:48:08.154629170+00:00"
---

After merging main (astromech-p4ls) and reconciling at f2cabbc: one data/catalog/exobiology.json (genus-keyed, EDR species symbols, wiki values, uncertain notes) read by astromech_exobiology::catalog, the predictor and astromech_exploration::exobiology::species_value; biology view built on astromech_exploration::system (no second body tracker); Biology page in the Exploration group. Rerun: astromech-exobiology --test surface 4, --test predict 4 (48 Spansh bodies, 70 species), --test sampling 2; astromech-exploration --test biology 3, --test unsold 3, lib exobiology 2; astromech-announce --test sampling 4, --test biology 5, --test replay 8 (17 rules, catch-up silent); astromech-core --test copy 1, lib biology 3; all pass. scripts/ui headless Chrome on ?core=ready#/exobiology shows the page under Exploration next to System. Full gates /tmp/landq/gates.sh exit 0 (108 cargo test suites, check-deps 28 crates, shell cargo metadata --locked, pnpm lint/check/build/test 182 tests).
