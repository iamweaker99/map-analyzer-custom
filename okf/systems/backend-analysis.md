---
id: system.backend-analysis
type: system
title: Backend analysis system
description: Rust backend analysis modules that derive beatmap characteristics and expose analysis data to the application.
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
- id: analysis-modules
  lifecycle: active
  statement: The backend groups analysis behavior into jumps, streams, sliders, finger control, aim control, and reading modules.
  load_bearing: true
  relations:
  - type: supported_by
    target:
      object: reference.backend-analysis-source
  semantic_hash: sha256:de6c9a7816b66ec2cedd674fc4ed775cd6c1b2b5fafd2e5d9a62da293ac06765
behavior_claims:
- analysis-modules
semantic_hash: sha256:f39f1f9ac625d48e6362a48aa27ab4e9cbab59e6e357cd929e6fe523b1f13300
verified:
- by: human:iamweaker99
  at: 2026-08-25T12:00:14Z
  subject:
    object: system.backend-analysis
    claim: analysis-modules
  revision: sha256:de6c9a7816b66ec2cedd674fc4ed775cd6c1b2b5fafd2e5d9a62da293ac06765
---

## Purpose

Derive beatmap analysis data from backend analysis modules.

## Boundary

The system covers the analysis module family and shared movement primitives. It does not cover HTTP routing or frontend presentation.

## Current behavior

The backend groups analysis behavior into jumps, streams, sliders, finger control, aim control, and reading modules.

## Interfaces and dependencies

The modules consume parsed beatmaps and expose analysis values to backend handlers.

## Known limitations

Individual metric semantics and cross-module aggregation require deeper claim-level documentation.
