---
id: system.frontend-analysis-ui
type: system
title: Frontend analysis UI
description: Frontend application surfaces that present beatmap analysis results and category profiles.
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
- id: profile-surfaces
  lifecycle: active
  statement: The frontend contains analysis presentation surfaces for jump, finger-control, aim-control, slider, reading, and stream profiles.
  load_bearing: true
  relations:
  - type: supported_by
    target:
      object: reference.frontend-source
  semantic_hash: sha256:6f11e063198b7b37f2535868cbea7c78cb987099744bffefcd60af7bf525ea30
behavior_claims:
- profile-surfaces
semantic_hash: sha256:4c4e6bdd5861c369959486c078c0fcb4bc465640603d30fb3976546007567c99
verified:
- by: human:iamweaker99
  at: 2026-08-25T12:00:14Z
  subject:
    object: system.frontend-analysis-ui
    claim: profile-surfaces
  revision: sha256:6f11e063198b7b37f2535868cbea7c78cb987099744bffefcd60af7bf525ea30
---

## Purpose

Present beatmap details and analysis results to users.

## Boundary

The system covers frontend analysis presentation and request wiring. It does not implement backend analysis algorithms.

## Current behavior

The frontend contains analysis presentation surfaces for jump, finger-control, aim-control, slider, reading, and stream profiles.

## Interfaces and dependencies

The page calls the backend beatmap details and analysis endpoints and passes results into analysis components.

## Known limitations

The frontend type and rendering contracts should be reconciled against each backend response before further UI claims are activated.
