---
schema: atelier.review
schema_version: 2
id: astromech-2xr8
title: finding on astromech-7rcn @9be1d69a
target: astromech-7rcn
identity: claude-reviewer-8zdt
commit: 9be1d69a3387eee0fadd751cdaa4dd8d04ed74cd
created_at: 2026-11-07T06:32:17.839814733Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 850ae885d9ee67faa304f05a48a8a238f2ff6287
    recorded_at: 2026-11-07T06:48:28.246791773Z
    resolution_commit: d113cdcf496a0d5c65863586404267a96312ac67
    reason: 'Fixed: each page waits for its own read command'
---

medium: app/src/lib/ui/previewstates.browser.test.ts:81 the rewritten 'says it is waiting while a call never answers' waits for sent to have been called with any command, replacing the old 300 ms wait that let Logbook's 200 ms-debounced read go out. Logbook (and any page that sends core_status first) sends core_status at mount, so the assertions now run before the page's own read is sent and the case no longer tests that the page keeps waiting while that read is pending. Reproduced in a scratch copy: making Logbook's refresh() set failed='Could not read the journal history: broken' as soon as it sends the read, the test as committed passes; with the wait changed to vi.waitFor(() => expect(sent).toHaveBeenCalledWith('logbook', expect.anything())) it fails ('expected 0 to be greater than 0'). Fix: give each PAGES entry its read command (logbook, ...) and wait for sent to have been called with that command.
