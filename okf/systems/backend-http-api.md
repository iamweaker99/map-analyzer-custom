---
id: system.backend-http-api
type: system
title: Backend HTTP API
description: Axum HTTP surface that serves health, beatmap details, and beatmap analysis responses.
lifecycle: active
created_at: 2026-08-25T00:00:00Z
updated_at: "2026-08-26T04:11:20Z"
generated:
  by: process:okf-bootstrap/v1
  at: 2026-08-25T00:00:00Z
freshness:
  state: fresh
  checked_at: "2026-08-26T04:11:20Z"
claims:
- id: analysis-routes
  lifecycle: active
  statement: The backend exposes beatmap details and analysis routes, including analysis types for jump, stream, slider, finger control, aim control, reading, and all.
  load_bearing: true
  relations:
  - type: supported_by
    target:
      object: reference.backend-api-source
  semantic_hash: sha256:d9b99b33ba8fc6813268a61fcef06690ad7e6cae3463bb73beaccabe56992fa9
behavior_claims:
- analysis-routes
semantic_hash: sha256:f86b5ea9655e6715d7d37be79cdbe3823002a9a534c2d6c456f2eaac0dcfcb9e
verified:
- by: human:iamweaker99
  at: 2026-08-25T12:00:14Z
  subject:
    object: system.backend-http-api
    claim: analysis-routes
  revision: sha256:d9b99b33ba8fc6813268a61fcef06690ad7e6cae3463bb73beaccabe56992fa9
---

## Purpose

Serve beatmap details and analysis results to the frontend through the backend HTTP interface.

## Boundary

The system covers Axum route registration and the beatmap details and analysis handlers. It does not define the analysis algorithms themselves.

## Current behavior

The API exposes health, beatmap details, and typed beatmap analysis endpoints. Supported analysis types include jump, stream, slider, finger control, aim control, reading, and all.

## Interfaces and dependencies

The handlers depend on Axum, osu! API access, local or downloaded beatmap files, and the backend analysis modules.

## Known limitations

The endpoint and failure behavior should be reconciled against integration tests before further claims are activated.
