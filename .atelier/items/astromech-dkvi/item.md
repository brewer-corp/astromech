---
schema: atelier.item
schema_version: 3
id: astromech-dkvi
title: trade_route fails on Ardent's current Sol market answer (rows with null names, market IDs and prices)
status: ready
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-07T19:13:29.939959767Z
trivial: null
caused_by: null
created_at: 2026-11-07T19:13:29.939959767Z
---

## Goal

`trade_route` handles Ardent Insight's current market answers, which include rows with null commodity names, market IDs and prices, instead of failing with "Ardent Insight sent an answer astromech does not understand".

## Done when

- [ ] A recorded raw Ardent answer that contains such rows is a test fixture (wiki-visible market data only), and a test over it shows `trade_route` answering, with the null rows skipped.
- [ ] `cargo test -p astromech-data` and the trade tests pass.

## Context

Found by astromech-gllh's implementer (mission astromech-kjfs) during a live record-mode capture: Ardent's real Sol market answer now has null fields; the repo fixture predates the change, so real users would hit this. Not part of mission astromech-kjfs.
