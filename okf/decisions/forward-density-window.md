---
id: decision.forward-density-window
type: decision
title: Forward-density window contract
description: Accepted project decision for the forward-density window specified by the closed primary issue.
lifecycle: active
created_at: 2026-08-25T00:00:00Z
updated_at: "2026-08-26T04:11:20Z"
generated:
  by: process:okf-bootstrap/v1
  at: 2026-08-25T00:00:00Z
freshness:
  state: fresh
  checked_at: "2026-08-26T04:11:20Z"
decision_key: reading.forward-density-window
decision_claim: window-size
decided_at: 2026-08-25T12:00:14Z
claims:
- id: window-size
  lifecycle: active
  statement: The primary closed GitHub issue specifies a 1000ms forward-looking note-density window, with raw counts and no weighting or decay.
  load_bearing: true
  relations:
  - type: supported_by
    target:
      object: reference.github-issue-4
  - type: contradicts
    target:
      object: research.forward-density-window-conflict
      claim: handoff-window
  semantic_hash: sha256:1a915dc0f0b608b3dbbda5681c22e8f6c3021803f51ea10169a4a1509b43ec18
semantic_hash: sha256:d189a8cd31892cabd97aa437f8d5edf995fd457fa9cca10c271cb1631c5d46fd
verified:
- by: human:iamweaker99
  at: 2026-08-25T12:00:14Z
  subject:
    object: decision.forward-density-window
    claim: window-size
  revision: sha256:1a915dc0f0b608b3dbbda5681c22e8f6c3021803f51ea10169a4a1509b43ec18
---

## Context

Raw project documents contain both 1000ms and 3000ms proposals.

## Decision

The accepted current contract is a 1000ms forward-looking note-density window using raw counts with no weighting or decay.

## Rationale

The closed issue is a primary project decision surface; raw handoffs remain evidence of earlier or competing proposals.

## Alternatives considered

The 3000ms proposal remains captured in draft research and has not been silently deleted.

## Consequences

Implementation may treat the 1000ms raw-count contract as verified project guidance.
