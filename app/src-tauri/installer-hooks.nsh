; astromech NSIS hooks (docs/V1-DESIGN.md §7.2, §7.5). These ask the app (astromech-core.exe)
; to quit cleanly before files are replaced or removed, so the session ends and held keys are
; released, and undo "Start with Windows" on uninstall.
;
; Order: Tauri's template runs NSIS_HOOK_PREINSTALL (and NSIS_HOOK_PREUNINSTALL) first, then its
; CheckIfAppIsRunning on the main binary, which force-closes it through the Restart Manager
; (RmShutdown with RmForceShutdown). So the clean --quit below always comes before any forced
; stop; by the template's check the app has gone.

!macro ASTROMECH_STOP_CORE
  ; 1. The clean quit. --quit returns once the app has removed core.json, which it does after
  ; releasing held keys and ending the session (up to 5 s).
  IfFileExists "$INSTDIR\astromech-core.exe" 0 +3
    nsExec::Exec '"$INSTDIR\astromech-core.exe" --quit'
    Pop $0
  ; 2. An old release's window (astromech-shell.exe, up to v1.0.0-alpha.4; now the forwarder,
  ; which never stays running). Ended before the forced fallback: that leaves core.json behind,
  ; and the old window's watchdog would start the core again.
  nsExec::Exec 'taskkill /F /T /IM astromech-shell.exe'
  Pop $0
  ; 3. Fallback if the app didn't answer (hung).
  nsExec::Exec 'taskkill /F /T /IM astromech-core.exe'
  Pop $0
  nsExec::Exec 'taskkill /F /T /IM astromech-voice-worker.exe'
  Pop $0
!macroend

!macro NSIS_HOOK_PREINSTALL
  !insertmacro ASTROMECH_STOP_CORE
  ; A two-process release (up to v1.0.0-alpha.4) registered astromech-shell.exe as its main
  ; binary. After copying the files, Tauri's template deletes a registered main binary whose
  ; name differs from this one's, which would delete the forwarder just installed and break
  ; taskbar pins to it (decision astromech-nade). Register the new name first. The Start-menu
  ; and desktop shortcuts still end up on astromech-core.exe: an install without /UPDATE writes
  ; them again.
  ReadRegStr $0 SHCTX "${UNINSTKEY}" "MainBinaryName"
  ${If} $0 == "astromech-shell.exe"
    WriteRegStr SHCTX "${UNINSTKEY}" "MainBinaryName" "${MAINBINARYNAME}.exe"
  ${EndIf}
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  !insertmacro ASTROMECH_STOP_CORE
  ; Upgrades run the old uninstaller with /UPDATE: keep "Start with Windows" as the user set it.
  ${If} $UpdateMode <> 1
    DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "astromech"
  ${EndIf}
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  ; "Delete the application data" box: config, logs, models and data live here.
  ${If} $DeleteAppDataCheckboxState = 1
    RMDir /r "$INSTDIR\data"
    RMDir "$INSTDIR"
  ${EndIf}
!macroend
