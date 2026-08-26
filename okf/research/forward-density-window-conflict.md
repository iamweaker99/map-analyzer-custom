---
id: research.forward-density-window-conflict
type: research
title: Forward-density window reconciliation
description: Bootstrap research record for reconciling two raw project proposals with different forward-density windows.
lifecycle: draft
created_at: 2026-08-25T00:00:00Z
updated_at: 2026-08-25T00:00:00Z
generated:
  by: process:okf-bootstrap/v1
  at: 2026-08-25T00:00:00Z
research_question: Which forward-density window, if any, is the accepted current project contract?
claims:
- id: prd-window
  lifecycle: draft
  statement: The raw reading-analysis PRD proposes a 1000ms forward-looking note-density window.
  load_bearing: true
  relations:
  - type: supported_by
    target:
      object: reference.reading-analysis-prd
  semantic_hash: sha256:653c903a51d5d15b8f53d3fbcc7a5831dd793afed112391e0665a06c5054c8db
- id: handoff-window
  lifecycle: draft
  statement: The raw forward-density handoff proposes a 3000ms forward-looking note-density window.
  load_bearing: true
  relations:
  - type: supported_by
    target:
      object: reference.forward-density-handoff
  semantic_hash: sha256:b7ac25284728b109b50ac319312216c628fe0f5a12cf62ca093e8f28e30a48ed
- id: unresolved-window
  lifecycle: draft
  statement: The repository contains conflicting raw proposals for the forward-density window; no current Decision should be inferred until the conflict is resolved.
  load_bearing: true
  relations:
  - type: contradicts
    target:
      object: research.forward-density-window-conflict
      claim: prd-window
  - type: contradicts
    target:
      object: research.forward-density-window-conflict
      claim: handoff-window
  semantic_hash: sha256:6aede74b29148e395b82ca112ff0c8c8888b2bad34c92b81d2d82201a1533d2e
semantic_hash: sha256:f7ce76301ffce2ce545c329d2ceca0aff0782284d129a22cd0fe62da341c85d8
---

This is intentionally a draft. The raw documents are evidence of proposals, not proof of the current product contract.
