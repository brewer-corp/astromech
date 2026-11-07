---
schema: atelier.review
schema_version: 2
id: astromech-13kk
title: finding on astromech-7rcn @387084e6
target: astromech-7rcn
identity: claude-reviewer-xil2
commit: 387084e6a623284f73e619d64e1f13c38016305c
created_at: 2026-11-07T05:27:45.627961850Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: d51df0540b4771e455f537f8219729bde9d5bdee
    recorded_at: 2026-11-07T05:30:13.746781020Z
    resolution_commit: d51df0540b4771e455f537f8219729bde9d5bdee
    reason: 'Cheap to close now; fixed: lint covers dynamic imports, bare preview, @tauri-apps/api root and property access'
---

low: app/eslint.config.js:22-33 does not catch every way back to the preview or Tauri that Done when asks lint to fail on: a dynamic import('../preview/transport'), a bare '../preview' import, an '@tauri-apps/api' root import and window.__TAURI_INTERNALS__ (member access, not a string literal) all pass lint (tried with eslint --stdin on src/lib/zz.ts). The static forms are caught. scripts/check-dist.py would still catch a dynamic preview import in a release build via its markers. Suggested fix: add '**/preview' and '@tauri-apps/api' to the patterns and no-restricted-syntax selectors for ImportExpression sources under preview/fixtures/@tauri-apps and for MemberExpression[property.name='__TAURI_INTERNALS__'].
