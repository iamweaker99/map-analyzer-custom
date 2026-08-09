# SCHEMA — LLM Wiki Operating Contract (v0.3)

The contract every agent follows when working on this wiki. Edit only with human approval.

## Layers

| Layer | Path | Who writes |
|---|---|---|
| Sources | `raw/` | Human/git sync. LLM **reads only, never writes**. |
| Wiki | `wiki/` | LLM. The compiled knowledge base. |
| Contract | this file | Human + LLM co-evolved. |

## Page types

| Type | Path | Purpose | Frontmatter |
|---|---|---|---|
| entity | `wiki/entity/<name>.md` | Domain things (Beatmap, Analysis Type) | `type: entity`, `updated` |
| concept | `wiki/concept/<name>.md` | Recurring principles | `type: concept` |
| module | `wiki/module/<name>.md` | Codebase subsystem: purpose, files, pipeline | `type: module`, `status: stable\|stale` |
| research | `wiki/research/<name>.md` | Compiled findings from handoffs/prototypes | `type: research`, `sources: [...]`, `status: idea\|designed\|prototyped\|implemented` |
| decision | `wiki/decision/<name>.md` | Why a choice was made (ADR-lite) | `type: decision`, `status: accepted\|superseded` |
| issue | `wiki/issue/<issue-n>.md` | Task-status mirror of a GitHub issue | `type: issue`, `github: #n`, `status: open\|in_progress\|closed` |
| hub | `wiki/hubs/<area>-hub.md` | **Routing table per domain** — cheap lines → pages. Used from ~100 pages up | `type: hub`, `area` |
| session | `wiki/session/<date>-handoff.md` | Conversation state: where the work is, open threads, next step | `type: session`, `status: open\|resolved` |

Plus root-level pages in `wiki/`: `index.md` (L1 catalog, updated every ingest), `log.md` (append-only history), `overview.md` (project map).

### Research status vocabulary (v0.3)

`status` on research pages has two axes, written as one value:

- **Stage** — how far along the work is: `idea` (text description only, mechanism not settled) → `designed` (mechanism/spec/PRD settled on paper) → `prototyped` (working prototype exists) → `implemented` (shipped in the production pipeline).
- **Disposition** — what happens to it; optional, appended in parentheses: `(deferred)` (parked at its current stage) · `(abandoned)` (declared dead by the user).

Examples: `idea (deferred)`, `designed`, `prototyped (abandoned)`, `implemented`. If a finding was superseded by a successor, keep the stage status and point at the successor in the body (ground rule 6) — there is no `superseded` status value for research pages.

## Ground rules

1. **Grounding invariant** — every load-bearing fact must trace to a source in `raw/` (or to code). No invented claims. Un-citable → `[gap]` in log, not a claim.
2. **index.md updated on every ingest**; log.md entries are append-only with `## [YYYY-MM-DD]` prefixes.
3. **Code changed → mark module pages `status: stale`**, never silently rewrite.
4. **Answers cite [[wikilinks]] to wiki pages**, not raw files. Page names are **unique across the tree** — resolution is by name, so new pages must not collide.
5. **In-place updates only** — merge by title, no "v2" pages. Version history lives in git + log.
6. **Forget ≠ delete** — supersede (`status: superseded`) or archive, never silently drop.
7. **Page caps** — soft ~400 lines; split if a page grows past it.
8. **Closed issues → `archive/`** immediately; the active issue surface stays small (GitHub is authoritative; wiki pages are context + pointers).
9. **`[gap]` entries** in log.md = known missing knowledge.
10. **Save ritual** — on "save this": draft session page + page updates → **human reviews** → commit. Interpretation is never filed as fact without approval.

## Workflow

- **ingest** — read source → triage (new / update / disputed / no material) → merge into pages (one source can touch 5–15 pages) → update index → append log.
- **query** — start at index (or area hub ≥100 pages) → follow links → synthesize a cited answer.
- **lint** — broken links, orphan pages, index drift, contradictions, stale claims, page caps exceeded. Auto-fix mechanical; report judgment calls.
- **resume** — session start: read index → read latest session page → run lint → reroute `stale` pages.

## Scaling (from community research, 2026-08-06)

| Wiki size | Structure |
|---|---|
| < ~100 pages | Flat index.md (current) |
| ~100–300 | Hubs per area (two-stage query); active-set discipline; `archive/` |
| ~300+ | Add local hybrid search (BM25 + RRF); index becomes hub-only |
| Enterprise (10k+) | Out of scope — RAG/graph territory |
