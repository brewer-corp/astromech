---
schema: atelier.review
schema_version: 1
id: astromech-h7cb
title: 'R7 low: the transport reason of a failed download is dropped from both the page and the log. DownloadError::Offline(String) (crates/astromech-data/src/download.rs:16, filled at :41, :50, :85) displays only ''Cannot reach the download. Check your internet connection.'', has no #[source], and app/src-tauri/src/main.rs neutron_download logs {e:#}, so the detail (TLS or proxy error, DNS failure, ''got N of M bytes'' for a cut-off transfer) is never written anywhere. Scenario: a user behind a corporate proxy or with a TLS-intercepting antivirus reports ''Cannot reach the download'' while their internet works, and the shell log has nothing more. Fix: log the detail (e.g. Debug or a detail() accessor in the shell''s log line), keeping the sentence for the page.'
target: astromech-1842
identity: review-agent-1842-2
commit: 047e839664bce33880db5033c28fedd4ea4b4355
created_at: 2026-10-22T21:21:29.043808030Z
event:
  event: finding
  recommendation: defer
---

R7 low: the transport reason of a failed download is dropped from both the page and the log. DownloadError::Offline(String) (crates/astromech-data/src/download.rs:16, filled at :41, :50, :85) displays only 'Cannot reach the download. Check your internet connection.', has no #[source], and app/src-tauri/src/main.rs neutron_download logs {e:#}, so the detail (TLS or proxy error, DNS failure, 'got N of M bytes' for a cut-off transfer) is never written anywhere. Scenario: a user behind a corporate proxy or with a TLS-intercepting antivirus reports 'Cannot reach the download' while their internet works, and the shell log has nothing more. Fix: log the detail (e.g. Debug or a detail() accessor in the shell's log line), keeping the sentence for the page.
