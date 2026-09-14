# OKF Retrieval Baseline

## Summary

- Positive cases: 16
- Expected-object hits: 5/16
- Recall@1/@3/@5: 0.312 (5/16) / 0.312 (5/16) / 0.312 (5/16)
- Negative-control pass rate: 0.750 (3/4)
- False-positive negative cases: noise-generic-analysis
- Average/max output: 96.2 bytes / 468.0 bytes
- Git branch: `Stream_Analysis_duration-based`
- Git HEAD: `83fd9e6608da3070deb0280b5b05d87da16468c0`

## Results by test class

| Class | Cases | Hits | Recall@1 | Recall@3 | Recall@5 | Negative pass rate |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `body_only` | 2 | 0 | 0.000 (0/2) | 0.000 (0/2) | 0.000 (0/2) | n/a |
| `claim_only` | 2 | 0 | 0.000 (0/2) | 0.000 (0/2) | 0.000 (0/2) | n/a |
| `code_navigation` | 1 | 0 | n/a | n/a | n/a | 1.000 (1/1) |
| `conflicting_research` | 1 | 0 | 0.000 (0/1) | 0.000 (0/1) | 0.000 (0/1) | n/a |
| `decision_key` | 1 | 1 | 1.000 (1/1) | 1.000 (1/1) | 1.000 (1/1) | n/a |
| `exact_id` | 1 | 1 | 1.000 (1/1) | 1.000 (1/1) | 1.000 (1/1) | n/a |
| `exact_title` | 2 | 2 | 1.000 (2/2) | 1.000 (2/2) | 1.000 (2/2) | n/a |
| `mixed` | 1 | 0 | 0.000 (0/1) | 0.000 (0/1) | 0.000 (0/1) | n/a |
| `natural_language` | 3 | 0 | 0.000 (0/3) | 0.000 (0/3) | 0.000 (0/3) | n/a |
| `no_match` | 2 | 0 | n/a | n/a | n/a | 1.000 (2/2) |
| `noise_control` | 1 | 0 | n/a | n/a | n/a | 0.000 (0/1) |
| `research_draft` | 1 | 1 | 1.000 (1/1) | 1.000 (1/1) | 1.000 (1/1) | n/a |
| `source_only` | 2 | 0 | 0.000 (0/2) | 0.000 (0/2) | 0.000 (0/2) | n/a |

## Case results

### `exact-decision-id` — PASS

- Class: `exact_id`
- Query: decision.forward-density-window
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1)
- Best expected rank: 1
- Output: 100 bytes
- Notes: Exact active decision object ID.

### `exact-decision-key` — PASS

- Class: `decision_key`
- Query: reading.forward-density-window
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1)
- Best expected rank: 1
- Output: 100 bytes
- Notes: Exact active decision key.

### `exact-title-decision` — PASS

- Class: `exact_title`
- Query: Forward-density window contract
- Expected: `decision.forward-density-window`
- Returned: `decision.forward-density-window` (#1)
- Best expected rank: 1
- Output: 100 bytes
- Notes: Exact title-like lookup.

### `exact-title-system` — PASS

- Class: `exact_title`
- Query: Frontend analysis UI
- Expected: `system.frontend-analysis-ui`
- Returned: `system.frontend-analysis-ui` (#1)
- Best expected rank: 1
- Output: 85 bytes
- Notes: Exact title-like lookup for a current system.

### `natural-forward-density-window` — FAIL

- Class: `natural_language`
- Query: Why do we use a one second forward density window?
- Expected: `decision.forward-density-window`
- Returned: (none)
- Best expected rank: null
- Output: 68 bytes
- Notes: Natural-language recall of a known active decision.

### `natural-spacing-transition` — FAIL

- Class: `natural_language`
- Query: What did we learn about spacing transition demand?
- Expected: `research.spacing-transition-demand`
- Returned: (none)
- Best expected rank: null
- Output: 68 bytes
- Notes: Natural-language recall of draft research.

### `natural-sequence-motor` — FAIL

- Class: `natural_language`
- Query: What are the limitations of the sequence motor prototype?
- Expected: `research.sequence-motor-prototype-limitations`
- Returned: (none)
- Best expected rank: null
- Output: 75 bytes
- Notes: Natural-language recall of draft research.

### `claim-only-density-contract` — FAIL

- Class: `claim_only`
- Query: raw counts and no weighting or decay
- Expected: `decision.forward-density-window`
- Returned: (none)
- Best expected rank: null
- Output: 54 bytes
- Notes: Useful wording exists in the claim statement, not searchable description/title.

### `claim-only-geometry-limit` — FAIL

- Class: `claim_only`
- Query: geometry and abrupt-transition descriptions are not represented
- Expected: `research.sequence-motor-prototype-limitations`
- Returned: (none)
- Best expected rank: null
- Output: 81 bytes
- Notes: Useful wording exists in a research claim statement.

### `body-only-current-contract` — FAIL

- Class: `body_only`
- Query: Implementation may treat the 1000ms raw-count contract as verified project guidance.
- Expected: `decision.forward-density-window`
- Returned: (none)
- Best expected rank: null
- Output: 102 bytes
- Notes: Useful wording exists in Markdown body content.

### `body-only-frontend-wiring` — FAIL

- Class: `body_only`
- Query: passes results into analysis components
- Expected: `system.frontend-analysis-ui`
- Returned: (none)
- Best expected rank: null
- Output: 57 bytes
- Notes: Useful wording exists in Markdown body content.

### `draft-research-id` — PASS

- Class: `research_draft`
- Query: research.spacing-transition-demand
- Expected: `research.spacing-transition-demand`
- Returned: `research.spacing-transition-demand` (#1)
- Best expected rank: 1
- Output: 108 bytes
- Notes: Exact lookup of draft research must remain visible as draft knowledge.

### `competing-density-research` — FAIL

- Class: `conflicting_research`
- Query: What are the competing 1000ms and 3000ms forward density proposals?
- Expected: `research.forward-density-window-conflict`
- Returned: (none)
- Best expected rank: null
- Output: 85 bytes
- Notes: Known research object records competing proposals.

### `known-decision-versus-implementation` — FAIL

- Class: `mixed`
- Query: Compare the documented density-window decision against the implementation.
- Expected: `decision.forward-density-window`
- Returned: (none)
- Best expected rank: null
- Output: 92 bytes
- Notes: Combines documented project knowledge with current-code inspection.

### `source-only-stream-buffer` — FAIL

- Class: `source_only`
- Query: Does a stream at the end of the map get counted?
- Expected: `reference.github-issue-6`
- Returned: (none)
- Best expected rank: null
- Output: 66 bytes
- Notes: The issue reference exists, but the detail is not distilled into a semantic object.

### `source-only-duration-rules` — FAIL

- Class: `source_only`
- Query: What are our duration-based stream rules?
- Expected: `reference.github-issue-6`
- Returned: (none)
- Best expected rank: null
- Output: 59 bytes
- Notes: The issue reference describes the workstream, without semantic claim coverage.

### `ordinary-code-navigation` — PASS

- Class: `code_navigation`
- Query: Where is StreamProfile rendered?
- Expected: (none)
- Returned: (none)
- Best expected rank: null
- Output: 50 bytes
- Notes: Ordinary source navigation should not require OKF.

### `no-match-database-policy` — PASS

- Class: `no_match`
- Query: What is the database migration policy?
- Expected: (none)
- Returned: (none)
- Best expected rank: null
- Output: 56 bytes
- Notes: Unrelated project question.

### `no-match-deployment-owner` — PASS

- Class: `no_match`
- Query: Who owns production deployment?
- Expected: (none)
- Returned: (none)
- Best expected rank: null
- Output: 49 bytes
- Notes: Unrelated project question.

### `noise-generic-analysis` — FAIL

- Class: `noise_control`
- Query: analysis
- Expected: (none)
- Returned: `reference.backend-analysis-source` (#1), `reference.frontend-source` (#2), `system.backend-analysis` (#3), `system.backend-http-api` (#4), `system.frontend-analysis-ui` (#5)
- Best expected rank: null
- Output: 468 bytes
- Notes: Generic term should expose whether description substring matching creates noise.

The baseline uses the original narrow `retrieve()` operation; negative controls are excluded from recall denominators.
