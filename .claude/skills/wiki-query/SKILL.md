---
name: wiki-query
description: Answer a question from the LLM wiki — start at the index, follow wikilinks, synthesize a cited answer; fall back to code/raw only if pages lack the answer, then record the gap. Use when the user asks a project question and context should come from the wiki (wiki-first rule), or when the user says "query the wiki" / "what does the wiki say about X".
---

# Wiki Query

Contract: `wiki/SCHEMA.md`. **Answers cite [[wikilinks]] to wiki pages, not raw files.**

## Route (cheapest first)

1. **`wiki/wiki/index.md`** — scan the ~one-line catalog. Pick candidate pages. If the wiki has ~100+ pages, start at the area `hubs/` page instead.
2. **Open the page** — read frontmatter first: `status: stale` → treat with suspicion and reroute; `superseded` → follow to its replacement.
3. **Follow related links** — the page's [[wikilinks]] carry cross-cutting constraints (decisions, dependents, issues).
4. **Fallback** — code / `raw/` ONLY if pages lack the answer. Whatever the fallback revealed gets fed back as an ingest candidate (tell the user it should be ingested, or `/wiki-ingest` it if they agree).
5. **Gaps** — if the wiki is missing the knowledge entirely, say so explicitly and propose the `[gap]` entry for log.md. The wiki must record what it doesn't know.

## Answer shape

- Synthesize from the loaded pages; cite the page(s) you used (`[[page]]`).
- If sources contradict (e.g. a handoff vs a PRD), surface the contradiction and the recorded resolution — don't silently pick one.
- Keep it short: answer first, citation trail second, extra context only if asked.
