---
schema: atelier.review
schema_version: 2
id: astromech-zt4n
title: finding on astromech-y6d6 @e9dea942
target: astromech-y6d6
identity: claude-reviewer-wgtr
commit: e9dea94204159f14c934bcc2c0de5b6d7240d3d4
created_at: 2026-11-07T14:05:57.438583138Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/astromech-y6d6.md:53. The parent's Done when still requires 'A Windows CI test crashes the process ... and sees it come back tray-only'. After this branch, evidence astromech-fqw9 and the rewritten astromech-dl53 say CI can't show a restart, so dl53 now asserts registration plus a faked --restarted start and moves the real restart to an after-merge check at Sumeet's PC. The parent line can no longer be met as written, and its after-merge line (:58) and stand-in leave out dl53's restart check. Fix: reword :53 to match dl53's CI proof (registration read back, faked --restarted start and restored state), and add dl53's restart check at Sumeet's PC to :58 with its stand-in. This is a proof-method change within the delegated plan authority, not a success criterion.
