---
name: wiki-ingest
description: Ingest a new source (handoff, doc, issue thread, prototype result) into the LLM wiki — triage, merge into pages, update index, append log. Use when the user says "ingest X", a handoff/PRD/doc needs to enter the wiki, or new knowledge from code/prototypes should be recorded. Follows wiki/SCHEMA.md (the contract).
---

# Wiki Ingest

Contract: `wiki/SCHEMA.md` — read it if anything here is ambiguous. **raw/ is read-only for the LLM — never write there.**

## 1. Read the source

- Read the full source file(s). Also check related existing pages and the latest `wiki/wiki/log.md` entries — merge, don't duplicate.
- Classify each fact: **new** (no page), **update** (page exists, supersedes content), **disputed** (contradicts a page — record the contradiction, don't silently overwrite), **no material** (nothing wiki-worthy).

## 2. Triage → merge into pages

- Create/update pages by type per SCHEMA: `entity/` (domain things), `concept/` (recurring principles), `module/` (codebase subsystems, with `status: stable|stale`), `research/` (compiled findings, `sources: [...]`), `decision/` (ADR-lite, `status: accepted|superseded`), `issue/` (GitHub issue pointers — GitHub stays authoritative, `github: #n`), `hub/` (routing tables, only when wiki passes ~100 pages), `session/` (conversation state).
- **Page names are unique across the tree** — resolution is by name, so new pages must not collide with existing ones.
- **In-place updates only** — merge by title, never create "v2" pages. Version history lives in git + log.
- One source can touch 5–15 pages. Load-bearing facts must trace to `raw/` or code — un-citable knowledge becomes a `[gap]` in log.md, not a claim.

## 3. Update index + log

- Update `wiki/wiki/index.md` — one line per page, catalog style. Add new pages, refresh changed lines.
- Append to `wiki/wiki/log.md` under `## [YYYY-MM-DD]` — what was ingested, what was created/updated, contradictions resolved, `[gap]` entries logged.

## 4. Save ritual (on "save this" / at session end)

1. Draft the session page `wiki/wiki/session/YYYY-MM-DD-handoff.md` (type: session; where the work is, decisions, open threads, next step) + page updates.
2. **Human reviews** the draft. Interpretation is never filed as fact without approval.
3. Commit after approval. Include the log entry.

## 5. Verify

- Run `/wiki-lint` — must come back green before the ingest is done.
- Confirm the new/updated page is reachable from `index.md` in ≤ 2 hops.
