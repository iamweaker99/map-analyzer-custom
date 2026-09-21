# OKF Routing Stage 1 Comparison

## Summary

| Metric | Stage 0 retrieve | Stage 1 context |
| --- | ---: | ---: |
| Recall@1 | 0.312 (5/16) | 0.750 (12/16) |
| Recall@3 | 0.312 (5/16) | 0.938 (15/16) |
| Recall@5 | 0.312 (5/16) | 1.000 (16/16) |
| Negative-control pass rate | 0.750 (3/4) | 1.000 (4/4) |
| Average output | 96.2 bytes | 1531.5 bytes |

Git branch: `Stream_Analysis_duration-based`; HEAD: `83fd9e6608da3070deb0280b5b05d87da16468c0`.

## Results by test class

| Class | Stage 0 hits | Stage 1 hits | Stage 0 recall@1 | Stage 1 recall@1 | Stage 1 negative pass |
| --- | ---: | ---: | ---: | ---: | ---: |
| `body_only` | 0 | 2 | 0.000 (0/2) | 1.000 (2/2) | n/a |
| `claim_only` | 0 | 2 | 0.000 (0/2) | 0.500 (1/2) | n/a |
| `code_navigation` | 0 | 0 | n/a | n/a | 1.000 (1/1) |
| `conflicting_research` | 0 | 1 | 0.000 (0/1) | 0.000 (0/1) | n/a |
| `decision_key` | 1 | 1 | 1.000 (1/1) | 1.000 (1/1) | n/a |
| `exact_id` | 1 | 1 | 1.000 (1/1) | 1.000 (1/1) | n/a |
| `exact_title` | 2 | 2 | 1.000 (2/2) | 1.000 (2/2) | n/a |
| `mixed` | 0 | 1 | 0.000 (0/1) | 1.000 (1/1) | n/a |
| `natural_language` | 0 | 3 | 0.000 (0/3) | 0.333 (1/3) | n/a |
| `no_match` | 0 | 0 | n/a | n/a | 1.000 (2/2) |
| `noise_control` | 0 | 0 | n/a | n/a | 1.000 (1/1) |
| `research_draft` | 1 | 1 | 1.000 (1/1) | 1.000 (1/1) | n/a |
| `source_only` | 0 | 2 | 0.000 (0/2) | 1.000 (2/2) | n/a |

## Stage 1 case results

### `exact-decision-id` — PASS

- Class: `exact_id`
- Query: decision.forward-density-window
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1), `research.forward-density-window-conflict` (#2), `reference.forward-density-handoff` (#3), `reference.github-issue-4` (#4), `reference.reading-analysis-prd` (#5)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 2225 bytes
- Notes: Exact active decision object ID.

### `exact-decision-key` — PASS

- Class: `decision_key`
- Query: reading.forward-density-window
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1), `research.forward-density-window-conflict` (#2), `reference.forward-density-handoff` (#3), `reference.github-issue-4` (#4), `reference.reading-analysis-prd` (#5)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 2100 bytes
- Notes: Exact active decision key.

### `exact-title-decision` — PASS

- Class: `exact_title`
- Query: Forward-density window contract
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1), `research.forward-density-window-conflict` (#2), `reference.github-issue-4` (#3), `reference.forward-density-handoff` (#4), `reference.reading-analysis-prd` (#5)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 2178 bytes
- Notes: Exact title-like lookup.

### `exact-title-system` — PASS

- Class: `exact_title`
- Query: Frontend analysis UI
- Expected: `system.frontend-analysis-ui`
- Returned: `system.frontend-analysis-ui` (#1), `reference.frontend-source` (#2)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 1040 bytes
- Notes: Exact title-like lookup for a current system.

### `natural-forward-density-window` — PASS

- Class: `natural_language`
- Query: Why do we use a one second forward density window?
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1), `research.forward-density-window-conflict` (#2), `reference.forward-density-handoff` (#3), `reference.github-issue-4` (#4), `reference.reading-analysis-prd` (#5)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 2100 bytes
- Notes: Natural-language recall of a known active decision.

### `natural-spacing-transition` — PASS

- Class: `natural_language`
- Query: What did we learn about spacing transition demand?
- Expected: `research.spacing-transition-demand`
- Returned: `reference.spacing-transition-handoff` (#1), `reference.github-issue-3` (#2), `reference.sequence-motor-prototype` (#3), `research.spacing-transition-demand` (#4), `research.sequence-motor-prototype-limitations` (#5)
- Best expected rank: 4
- Coverage: expected `semantic`, actual `semantic`
- Output: 2205 bytes
- Notes: Natural-language recall of draft research.

### `natural-sequence-motor` — PASS

- Class: `natural_language`
- Query: What are the limitations of the sequence motor prototype?
- Expected: `research.sequence-motor-prototype-limitations`
- Returned: `reference.sequence-motor-prototype` (#1), `reference.sequence-motor-sample` (#2), `research.sequence-motor-prototype-limitations` (#3), `reference.forward-density-handoff` (#4), `research.spacing-transition-demand` (#5)
- Best expected rank: 3
- Coverage: expected `semantic`, actual `semantic`
- Output: 2458 bytes
- Notes: Natural-language recall of draft research.

### `claim-only-density-contract` — PASS

- Class: `claim_only`
- Query: raw counts and no weighting or decay
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1), `reference.reading-analysis-prd` (#2), `research.forward-density-window-conflict` (#3)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 1758 bytes
- Notes: Useful wording exists in the claim statement, not searchable description/title.

### `claim-only-geometry-limit` — PASS

- Class: `claim_only`
- Query: geometry and abrupt-transition descriptions are not represented
- Expected: `research.sequence-motor-prototype-limitations`
- Returned: `reference.spacing-transition-handoff` (#1), `research.sequence-motor-prototype-limitations` (#2), `research.spacing-transition-demand` (#3)
- Best expected rank: 2
- Coverage: expected `semantic`, actual `semantic`
- Output: 1659 bytes
- Notes: Useful wording exists in a research claim statement.

### `body-only-current-contract` — PASS

- Class: `body_only`
- Query: Implementation may treat the 1000ms raw-count contract as verified project guidance.
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1), `reference.reading-analysis-prd` (#2), `reference.forward-density-handoff` (#3), `reference.github-issue-4` (#4), `reference.spacing-transition-handoff` (#5)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 2011 bytes
- Notes: Useful wording exists in Markdown body content.

### `body-only-frontend-wiring` — PASS

- Class: `body_only`
- Query: passes results into analysis components
- Expected: `system.frontend-analysis-ui`
- Returned: `system.frontend-analysis-ui` (#1), `system.backend-analysis` (#2)
- Best expected rank: 1
- Coverage: expected `source_only`, actual `source_only`
- Output: 831 bytes
- Notes: Useful wording exists in Markdown body content.

### `draft-research-id` — PASS

- Class: `research_draft`
- Query: research.spacing-transition-demand
- Expected: `research.spacing-transition-demand`
- Returned: `research.spacing-transition-demand` (#1), `reference.spacing-transition-handoff` (#2), `reference.github-issue-3` (#3), `reference.sequence-motor-prototype` (#4), `research.sequence-motor-prototype-limitations` (#5)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 2216 bytes
- Notes: Exact lookup of draft research must remain visible as draft knowledge.

### `competing-density-research` — PASS

- Class: `conflicting_research`
- Query: What are the competing 1000ms and 3000ms forward density proposals?
- Expected: `research.forward-density-window-conflict`
- Returned: `decision.forward-density-window` (#1), `research.forward-density-window-conflict` (#2), `reference.github-issue-4` (#3), `reference.forward-density-handoff` (#4), `reference.reading-analysis-prd` (#5)
- Best expected rank: 2
- Coverage: expected `semantic`, actual `semantic`
- Output: 2482 bytes
- Notes: Known research object records competing proposals.

### `known-decision-versus-implementation` — PASS

- Class: `mixed`
- Query: Compare the documented density-window decision against the implementation.
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1), `research.forward-density-window-conflict` (#2), `reference.forward-density-handoff` (#3), `reference.github-issue-4` (#4), `reference.reading-analysis-prd` (#5)
- Best expected rank: 1
- Coverage: expected `semantic`, actual `semantic`
- Output: 2359 bytes
- Notes: Combines documented project knowledge with current-code inspection.

### `source-only-stream-buffer` — PASS

- Class: `source_only`
- Query: Does a stream at the end of the map get counted?
- Expected: `reference.github-issue-6`
- Returned: `reference.github-issue-6` (#1), `system.backend-http-api` (#2), `system.frontend-analysis-ui` (#3)
- Best expected rank: 1
- Coverage: expected `source_only`, actual `source_only`
- Output: 1287 bytes
- Notes: The issue reference exists, but the detail is not distilled into a semantic object.

### `source-only-duration-rules` — PASS

- Class: `source_only`
- Query: What are our duration-based stream rules?
- Expected: `reference.github-issue-6`
- Returned: `reference.github-issue-6` (#1), `system.backend-http-api` (#2), `system.frontend-analysis-ui` (#3)
- Best expected rank: 1
- Coverage: expected `source_only`, actual `source_only`
- Output: 1281 bytes
- Notes: The issue reference describes the workstream, without semantic claim coverage.

### `ordinary-code-navigation` — PASS

- Class: `code_navigation`
- Query: Where is StreamProfile rendered?
- Expected: (none)
- Returned: (none)
- Best expected rank: null
- Coverage: expected `none`, actual `none`
- Output: 115 bytes
- Notes: Ordinary source navigation should not require OKF.

### `no-match-database-policy` — PASS

- Class: `no_match`
- Query: What is the database migration policy?
- Expected: (none)
- Returned: (none)
- Best expected rank: null
- Coverage: expected `none`, actual `none`
- Output: 121 bytes
- Notes: Unrelated project question.

### `no-match-deployment-owner` — PASS

- Class: `no_match`
- Query: Who owns production deployment?
- Expected: (none)
- Returned: (none)
- Best expected rank: null
- Coverage: expected `none`, actual `none`
- Output: 114 bytes
- Notes: Unrelated project question.

### `noise-generic-analysis` — PASS

- Class: `noise_control`
- Query: analysis
- Expected: (none)
- Returned: (none)
- Best expected rank: null
- Coverage: expected `none`, actual `none`
- Output: 91 bytes
- Notes: Generic term should expose whether description substring matching creates noise.

## Interpretation

Context retrieval uses deterministic lexical matching over stored IDs, keys, titles, descriptions, claims, headings, and body text.
Source-only misses remain coverage observations; no semantic objects were added to make them pass.
