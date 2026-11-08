---
created_at: "2026-11-07T14:00:15.068523371+00:00"
id: "astromech-fqw9"
evidence_type: "validation"
commit: "a2686ace45e2ae1025e85396e16c8d3945b3126a"
result: "blocked"
captured_at: "2026-11-07T14:00:15.068442961+00:00"
uri: "https://github.com/brewer-corp/astromech/actions/runs/42376155384"
agent_identity: "astromech-wgtr implementer (Claude)"
target:
  kind: "issue"
  id: "astromech-wgtr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-wgtr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Windows CI run 42376155384 (branch commit a2686ace), job restart: restart registration could not be shown on CI. Reported, not gated.\nThe app called RegisterApplicationRestart(\"--background --restarted\", RESTART_NO_PATCH | RESTART_NO_REBOOT) (returned 0) and was crashed after 65 s by abort (exit 0xC0000409), access_violation (0xC0000005), main_thread_panic (0x65), TerminateProcess (0xF), and abort after 10 s, in three configurations:\n- runner WER: WerSvc stopped and manual, HKLM DontShowUI=1, launched by the probe; the app saw error mode 0x3 (includes SEM_NOGPFAULTERRORBOX, which by itself keeps WER away) inside the runner's job (limit flags 0x0);\n- WER on: WerSvc running and manual, DontShowUI, Disabled, LoggingDisabled and Consent removed under HKLM and HKCU, no WER policy keys; same launch, so the same error mode 0x3;\n- WER on, clean launch: CREATE_DEFAULT_ERROR_MODE, so the app saw error mode 0x0; CREATE_BREAKAWAY_FROM_JOB was refused (access denied), so it stayed in the runner's job (limit flags 0x0, no DIE_ON_UNHANDLED_EXCEPTION).\nIn all 15 cases no process with --restarted appeared within 90 s, no new WER report folder appeared under ProgramData or LOCALAPPDATA, no WerFault/wermgr process was seen, and the desktop 5 s after each crash showed no WER dialog. The runner has an interactive desktop (the screenshots show it), so a missing session isn't the cause. The first two configurations are explained by the inherited SEM_NOGPFAULTERRORBOX. The third isn't explained: WER didn't engage even with error mode 0, possibly because of the runner's job or the Server 2025 SKU. So this is inconclusive, not a failed premise.\nThe 0x65 exit for a main-thread panic is an ordinary exit (tao catches the panic and re-raises it after the event loop ends), so restart registration would never cover it, whatever WER does.\nArtifact one-process-restart: restart.md, restart.json, spike.log, 15 desktop screenshots.\nLimits: the probe polled for WER processes once a second, starting only after the crashed process had exited, so a short-lived WerFault could have been missed; the report folders and the absence of any restart are the stronger signals. A normal Windows 10/11 desktop was not tested."
updated_at: "2026-11-07T14:00:15.244645578+00:00"
---

Windows CI run 42376155384 (branch commit a2686ace), job restart: restart registration could not be shown on CI. Reported, not gated.
The app called RegisterApplicationRestart("--background --restarted", RESTART_NO_PATCH | RESTART_NO_REBOOT) (returned 0) and was crashed after 65 s by abort (exit 0xC0000409), access_violation (0xC0000005), main_thread_panic (0x65), TerminateProcess (0xF), and abort after 10 s, in three configurations:
- runner WER: WerSvc stopped and manual, HKLM DontShowUI=1, launched by the probe; the app saw error mode 0x3 (includes SEM_NOGPFAULTERRORBOX, which by itself keeps WER away) inside the runner's job (limit flags 0x0);
- WER on: WerSvc running and manual, DontShowUI, Disabled, LoggingDisabled and Consent removed under HKLM and HKCU, no WER policy keys; same launch, so the same error mode 0x3;
- WER on, clean launch: CREATE_DEFAULT_ERROR_MODE, so the app saw error mode 0x0; CREATE_BREAKAWAY_FROM_JOB was refused (access denied), so it stayed in the runner's job (limit flags 0x0, no DIE_ON_UNHANDLED_EXCEPTION).
In all 15 cases no process with --restarted appeared within 90 s, no new WER report folder appeared under ProgramData or LOCALAPPDATA, no WerFault/wermgr process was seen, and the desktop 5 s after each crash showed no WER dialog. The runner has an interactive desktop (the screenshots show it), so a missing session isn't the cause. The first two configurations are explained by the inherited SEM_NOGPFAULTERRORBOX. The third isn't explained: WER didn't engage even with error mode 0, possibly because of the runner's job or the Server 2025 SKU. So this is inconclusive, not a failed premise.
The 0x65 exit for a main-thread panic is an ordinary exit (tao catches the panic and re-raises it after the event loop ends), so restart registration would never cover it, whatever WER does.
Artifact one-process-restart: restart.md, restart.json, spike.log, 15 desktop screenshots.
Limits: the probe polled for WER processes once a second, starting only after the crashed process had exited, so a short-lived WerFault could have been missed; the report folders and the absence of any restart are the stronger signals. A normal Windows 10/11 desktop was not tested.
