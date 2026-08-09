---
type: hub
area: reading
updated: 2026-08-06
---
# Hub: Reading

*Routing hub for the reading domain. Two-stage query: read this hub (cheap lines) → open only the 2–3 pages you need.*

## Index (routing lines)
- [[reading-analysis]] -- module: pipeline visuals→density→trajectory→traps→strain→sequence_motor #module
- [[forward-density]] -- research: 1000ms window design, agreed API, not implemented #research #not-implemented
- [[spacing-demand]] -- research: TV2/LTD, keep 1/2 snap; frontend undecided #research #blocked-frontend
- [[keep-12-snap]] -- decision: include 1/2 snap (22→2 lesson) #decision
- [[issue-3-intra-pattern-spacing]] -- open: per-pattern spacing, prefactor exists in WIP #issue
- [[issue-5-angle-distribution]] -- open: angle distribution, blocked by #3 #issue
- [[issue-3-intra-pattern-spacing]] -- sibling issue, same Slice 2 infra [link: gh#3]
- [[issue-5-angle-distribution]] -- blocked by #3 [link: gh#5]

## What lives here
Modules, research, decisions, and issues for: reading analysis, pattern-aware metrics, spacing demand, angle distribution.

## Rules for this area
- New metrics must be siblings in reading JSON ([[Data-Philosophy]])
- Code change → mark [[reading-analysis]] `status: stale`, never silently rewrite

## Archived
- [[issue-4-forward-density]] — gh#4, archived 2026-08-08
- (closed issues and superseded research move to `archive/` — see SCHEMA: compaction)
