---
schema: atelier.review
schema_version: 2
id: astromech-puh1
title: finding on astromech-h95t @f19cd827
target: astromech-h95t
identity: claude-reviewer-h95t
commit: f19cd827b023ba7d64a0e93f3cf1bc7c8ce070f8
created_at: 2026-11-08T16:01:08.229456159Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/releases/v1.0.0-alpha.5.md:15 says the in-app updater and setup.exe 'were tested from one that was running', but the setup.exe route was tested only as a silent 'setup.exe /S' (astromech-9lh3, upgrade job). The interactive setup.exe a player double-clicks, which first runs alpha.4's uninstaller, was never run (astromech-y2m1). Fix: 'The in-app update and a silent setup.exe /S were tested from a running alpha.4; setup.exe with its pages shown has not been.' The y2m1 Start with Windows wording itself is right and should stay hedged with 'may': alpha.4's NSIS_HOOK_PREUNINSTALL deletes the Run value when UpdateMode is not 1, and alpha.4 already sets autostart_default_applied, so it applies to alpha.4 to alpha.5, but it comes from reading the template and was not seen on Windows.
