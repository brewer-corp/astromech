---
created_at: "2026-10-22T23:33:13.704660357+00:00"
id: "astromech-k24e"
evidence_type: "validation"
commit: "ed861d2534e76accb1888784bc7d47d6273fedb7"
result: "pass"
captured_at: "2026-10-22T23:33:13.704585066+00:00"
agent_identity: "claude-implementer-3x0x"
target:
  kind: "issue"
  id: "astromech-3x0x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3x0x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "UI check (CONTRIBUTING.md): Vite preview at /?core=ready#/route in headless Chrome via scripts/ui at ed861d25. No file: Plot a route → Neutron shows Efficiency, 'Until it is downloaded, Spansh plots neutron routes.', and plotting Colonia gives 'Following Sol to Colonia: 128 jumps · 22,000 ly · 1 neutron boost, at 39.87 ly a jump. Plotted by Spansh.' After Download: no Efficiency field, 'Neutron routes are plotted on this PC with it, in seconds.', and the result 'Following Sol to Colonia: 180 jumps · 22,000 ly · 149 neutron boosts, at 39.87 ly a jump. Plotted on this PC, near the fewest jumps.' with the 151-stop route followed (the preview's local route is the real plotter's output on the 22 Oct 2026 file, 0.38 s). Screenshots looked right; layout unchanged. The fallback-reason line is covered by Plot.test.ts only (the preview has no broken-file state). The Tauri shell is not compiled here (no glib): Windows CI on the PR."
updated_at: "2026-10-22T23:33:13.704958512+00:00"
---

UI check (CONTRIBUTING.md): Vite preview at /?core=ready#/route in headless Chrome via scripts/ui at ed861d25. No file: Plot a route → Neutron shows Efficiency, 'Until it is downloaded, Spansh plots neutron routes.', and plotting Colonia gives 'Following Sol to Colonia: 128 jumps · 22,000 ly · 1 neutron boost, at 39.87 ly a jump. Plotted by Spansh.' After Download: no Efficiency field, 'Neutron routes are plotted on this PC with it, in seconds.', and the result 'Following Sol to Colonia: 180 jumps · 22,000 ly · 149 neutron boosts, at 39.87 ly a jump. Plotted on this PC, near the fewest jumps.' with the 151-stop route followed (the preview's local route is the real plotter's output on the 22 Oct 2026 file, 0.38 s). Screenshots looked right; layout unchanged. The fallback-reason line is covered by Plot.test.ts only (the preview has no broken-file state). The Tauri shell is not compiled here (no glib): Windows CI on the PR.
