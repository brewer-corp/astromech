---
schema: atelier.review
schema_version: 2
id: astromech-8i82
title: approval on astromech-bp23 @b6733015
target: astromech-bp23
identity: claude-reviewer-96dv
commit: b6733015f44f2429a6f8457a150e8230268a6fc9
created_at: 2026-11-07T12:15:23.943456573Z
event:
  event: approval
---

Review of b6733015 (the palette.test.ts flake on PR #216 at 164ea727). Cause: lib/transport statically imported preview/transport, which imports lib modules back; the new lib/transport has no runtime import at all (only 'import type' of api/commands; Tauri's API by dynamic import), so no static cycle can pass through it. Regression test: lib/transport.test.ts copied into a scratch tree at 164ea727 fails 3 of 3 runs with "Cannot access 'current' before initialization" at microtasks 100-180; at HEAD it passes 5 of 5 alone. It steps microtasks only, no sleeps or retries. palette.test.ts passes 15 of 15 alone. pnpm check 0 errors, lint clean, pnpm test 75 files 534 tests twice and 3 more runs with --sequence.shuffle, all pass. Release: pnpm build then check-dist: 76 files, no browser preview (the 2 new chunks are @tauri-apps/api core and event). VITE_PREVIEW build keeps the preview in its own chunk; served, the page body is empty at load and Materials shows its sample once the preview loads, because main.ts mounts after it, so there is no error flash. Tauri: transport() checks __TAURI_INTERNALS__ per call and uses the real transport whenever no preview registered or in Tauri; proxy/staleTauri/tauriFailure tests pass with vi.mock after setup. Lint exemptions (main.ts from the preview/Tauri import rule, transport.test.ts from the test rule) are narrow and commented. No findings.
