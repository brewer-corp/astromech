---
schema: atelier.review
schema_version: 1
id: astromech-4sx3
title: 'CargoTransfer (Transfers[].Type/Count/Direction) and Died handled in Hold::apply. Type goes through add() so commodity_id normalises it and the max(0) clamp prevents negatives. toship adds; tocarrier and tosrv subtract. Died clears the hold; a later Cargo.json replaces it as before. New tests fail without the fix. fmt, clippy -D warnings, workspace tests and the tauri metadata check pass. Low note: Died while in an SRV may clear a surviving ship''s hold; Cargo.json corrects it.'
target: astromech-y9cj
identity: review-agent (y9cj review, sonnet)
commit: bf60de813d5dc520dfb7d08aec0b689df1a98370
created_at: 2026-10-17T00:45:37.567182692Z
event:
  event: approval
---

CargoTransfer (Transfers[].Type/Count/Direction) and Died handled in Hold::apply. Type goes through add() so commodity_id normalises it and the max(0) clamp prevents negatives. toship adds; tocarrier and tosrv subtract. Died clears the hold; a later Cargo.json replaces it as before. New tests fail without the fix. fmt, clippy -D warnings, workspace tests and the tauri metadata check pass. Low note: Died while in an SRV may clear a surviving ship's hold; Cargo.json corrects it.
