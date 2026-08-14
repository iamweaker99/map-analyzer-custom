---
name: wiki-lint
description: Check wiki health — broken wikilinks, orphan pages, index drift, invalid frontmatter, stale flags, page caps. Auto-fix mechanical issues, report judgment calls. Use when the user says "lint the wiki", after every ingest, and as the session-end save-ritual step (not at resume; amended 2026-08-12).
---

# Wiki Lint

Contract: `wiki/SCHEMA.md`. Run against everything under `wiki/wiki/`.

## Checks

1. **Broken wikilinks** — every `[[name]]` must resolve to a file whose basename (without `.md`) matches `name`, anywhere under `wiki/wiki/`. Page names are unique across the tree, so resolution is by name. Links to pages recorded as `[gap]` in log.md count as **pending** (planned ingest), not broken.
2. **Orphan pages** — pages not reachable from `index.md` (or a hub) via wikilinks. `archive/` is exempt from orphan/frontmatter-folder checks by design (archived pages carry whatever type they had; broken-link resolution still applies to them).
3. **Index drift** — every page listed in `index.md` exists; every existing page is listed (session pages may be listed; log.md is exempt).
4. **Frontmatter validity** — `type` matches the folder it lives in (entity/concept/module/research/decision/issue/hub/session); required fields present (`updated` for entities/modules; `status` where the SCHEMA requires it; `sources` for research; `github` + `status` for issues); research `status` ∈ {idea, designed, prototyped, implemented} with optional `(deferred)`/`(abandoned)` disposition (SCHEMA v0.3).
5. **Stale claims** — `status: stale` pages flagged for reroute; superseded pages point at their replacement.
6. **Page caps** — soft ~400 lines; flag pages past it for splitting.
7. **Contradictions** — pages asserting different facts about the same thing (flag, don't auto-resolve — that's a judgment call).

## Fix discipline

- **Auto-fix mechanical only**: broken link typos, missing index lines, frontmatter casing, obvious link renames.
- **Report judgment calls**: contradictions, stale-vs-fresh conflicts, split candidates — never silently rewrite content.
- Record lint runs in `log.md` (append-only, `## [YYYY-MM-DD]`).
- End with a verdict: **lint green** or a numbered list of what needs a human.
