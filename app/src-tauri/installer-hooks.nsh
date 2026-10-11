; astromech NSIS hooks (docs/V1-DESIGN.md §7.2, §7.5). Tauri's template already closes the
; app window; these stop the always-running core cleanly before files are replaced or removed,
; and undo "Start with Windows" on uninstall.

!macro ASTROMECH_STOP_CORE
  IfFileExists "$INSTDIR\astromech-core.exe" 0 +3
    nsExec::Exec '"$INSTDIR\astromech-core.exe" --quit'
    Pop $0
  ; Fallback if it didn't answer (old version, hung).
  nsExec::Exec 'taskkill /F /T /IM astromech-core.exe'
  Pop $0
  nsExec::Exec 'taskkill /F /T /IM astromech-voice-worker.exe'
  Pop $0
!macroend

!macro NSIS_HOOK_PREINSTALL
  !insertmacro ASTROMECH_STOP_CORE
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
