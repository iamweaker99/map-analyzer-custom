---
name: resume
description: Resume a previous session from its handoff page. Triggered when the user's prompt starts with a resume token (e.g. "resume:2026-08-07-wiki-bootstrap") — read the handoff, run lint, continue from the next step. Unrelated sessions load zero wiki context.
---

# Resume

The user pasted a `resume:` token. It must resolve **deterministically** — parse it exactly:

## 1. Resolve the token

- Format: `resume:YYYY-MM-DD[-freeform-slug]` — only the date is load-bearing; anything after the date is a thread label, ignore it for resolution.
- Target file: `wiki/wiki/session/YYYY-MM-DD-handoff.md`.
- Not found? List `wiki/wiki/session/*-handoff.md` and ask which one — never guess a date.

## 2. Read + verify

1. Read the handoff page: where the work is, decisions, open threads, next step.
2. Run `/wiki-lint` — the wiki must be green before continuing (reroute `stale` pages per the lint result).
3. State to the user, in a few lines: what the handoff says, the wiki's health, and what you're about to do (the handoff's next step). Confirm before doing anything destructive.

## 3. Continue

- Follow the handoff's "Next step" exactly. If it references phases or plans, execute the current phase, not the whole plan.
- On completion, update the session page (status, progress) and append to log.md per the save ritual.
- If the handoff is marked `resolved`, skip straight to its successor page if one exists, else ask.
