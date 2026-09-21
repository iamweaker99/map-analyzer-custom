---
id: decision.reading-analysis-methodology
type: decision
title: Reading-analysis methodology pivot
description: Current guidance for grounding reading-analysis signals in established osu! difficulty-evaluation work.
lifecycle: active
created_at: 2026-09-17T00:00:00Z
updated_at: 2026-09-17T00:00:00Z
generated:
  by: process:okf-methodology-pivot/v1
  at: 2026-09-17T00:00:00Z
decision_key: reading.analysis-methodology
decision_claim: methodology
decided_at: 2026-09-17T00:00:00Z
claims:
- id: methodology
  lifecycle: active
  statement: Reading analysis must expose signals grounded in osu! developers' established difficulty-evaluation work; forward-density, intra-pattern angle and spacing, and sequence-motor descriptors are retired until they have upstream grounding and validation.
  load_bearing: true
  semantic_hash: sha256:604275431a614998cc196b2a518fdc331f11e772dabb3647f21719f54ac94a3a
semantic_hash: sha256:bcd155511bacbe13e167dbe64d9da9b2ea2b69d7e95ba98c434ef8119d01565e
---

## Context

Reading analysis accumulated standalone experimental metrics without validation against osu! developers' established difficulty-evaluation work.

## Decision

Reading analysis now exposes only signals grounded in that established difficulty-evaluation work. The forward-density, intra-pattern angle and spacing, and sequence-motor experiments are retired and are not current product guidance.

## Rationale

The retired descriptors were not validated as upstream-grounded difficulty-evaluation signals.

## Alternatives considered

Keeping the experimental descriptors active would preserve unvalidated metrics as if they were product signals.

## Consequences

The sequence-motor production implementation and experiment utilities are archived, the API no longer emits `sequence_motor`, and the existing finger-control rhythm segmentation infrastructure remains active.
