---
type: decision
status: accepted
updated: 2026-08-12
---
# Decision: Orchestration cache experiment — reader agent, verbatim shell, agent reuse

**Date:** 2026-08-12 · **Status:** accepted (trial — user judges keep/abandon after multi-session observation)

## Context

LLM API cache hit rate dropped 95% → ~90% after sub-agent orchestration became the standing directive (CLAUDE.md §7). Discussion (2026-08-12, main thread) isolated three structural causes:

1. **Cold spawns** — every new agent has a fresh context with no transcript history to hit on.
2. **Per-call task prompts** — the task text sits *after* the cached prefix (system prompt + history), so it is a miss by construction; the longer it is, the lower the call's hit rate.
3. **Short-lived agents** — agents that live for one task never accumulate a cached transcript.

Levers considered and dropped: cache TTL/idle discipline (#4) — user's observation: sessions return to 95% even after hours idle, so the system-prompt side of the cache persists; model alignment (#5) — user runs a single model, already satisfied.

## Decision

Adopt, as a trial:

1. **Relocate** — the reusable [delegate] data-gathering conventions move from per-prompt text into `.claude/agents/reader.md`, the cached system-prompt side. Main-thread context is unaffected (agent definitions never load into it).
2. **Reuse** — keep one `reader` agent alive per session; route all [delegate] data-gathering through SendMessage continuations; retire + respawn only when its context genuinely fills.
3. **Trim** — fan-out prompts follow a verbatim shell with a short variable tail, only the tail varies:

   `Reader task | task: <specific> | sources: <paths> | deliver: <format>`

## Rationale

Cache hits are prefix-based: the cached region is the system prompt plus the accumulated transcript. Relocation moves reusable text into the cached side (paid once per agent type, hit thereafter); reuse makes every continuation turn hit on the growing transcript; trimming shrinks the uncached tail of every cold-start call. The [delegate]/[main] tagging rules are unchanged — they decide routing, not prompt structure or which instance answers.

## Observation protocol (user-defined)

Both axes are monitored across multiple sessions before the keep/abandon judgment:

- **Research-framework axis** — fan-out findings quality must not regress (bar: the 2026-08-10 verdict, orchestration surfaced 5+ facts a single pass missed).
- **Cache-rate axis** — LLM API cache hit rate vs the ~90% baseline (95% pre-orchestration).

## Consequences

- First spawn of `reader` pays a one-time system-prompt write; later spawns and continuations hit.
- Reader context grows with reuse — growth is exactly what caching rewards; respawn only at window pressure.
- Reader never drafts wiki content (unchanged, CLAUDE.md §7); wiki updates stay on the main thread.

## Related

- CLAUDE.md §7 (classification test) · memory: `subagent-orchestration-preference`, `orchestration-cache-experiment`
- [[2026-08-12-handoff]]

_Sources: cache-rate discussion 2026-08-12 (main thread), CLAUDE.md §7, memory subagent-orchestration-preference_
