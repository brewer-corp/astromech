---
schema: atelier.review
schema_version: 2
id: astromech-aw36
title: approval on astromech-y6d6 @98daaece
target: astromech-y6d6
identity: claude-reviewer-9lh3
commit: 98daaeced31c0fbe9e266fb567e6370696bbb7fc
created_at: 2026-11-07T20:46:44.636169544Z
event:
  event: approval
---

Reviewed 98daaece (9lh3: upgrade smoke reads the window's page after its first page load; every route runs). Run 42410580322's log matches the diagnosis: ui_state was read 0.3 s after the one-core check and gave {page: None, window: open}; the checks before it (helper exit 0, the old core's clean quit, exactly one 0.0.0-dev core, no astromech-shell) passed. Fix is sound: the page-load baseline is taken before the helper starts, and alpha.4's core never writes that line (its window logs to astromech-shell.log), so only the relaunched app can raise the count; then a wait for page, with a fallback read so the check reports what it saw. page_of(url) behaves as before for real URLs and returns None for about:blank (test passes locally: cargo test -p astromech-core --lib the_window_page). Running every route and failing if any failed keeps the job's gate. UiState doc and the TS binding agree. a7edc42e and 7441499d were approved separately (astromech-usny). The updater route's later checks (Run value, old shortcut) and the whole NSIS route still need a passing CI run on the current head.
