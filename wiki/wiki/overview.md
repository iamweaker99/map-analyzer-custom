---
type: overview
updated: 2026-08-06
---
# Overview — map-analyzer-custom

An osu! beatmap analyzer that reveals raw beatmap data contributing to skills — without assigning difficulty weightage (see [[Data-Philosophy]]).

## Stack
- **Rust backend** (`backend/`) — analysis modules: finger_control, aim_control, reading (Jump/Stream/Slider alongside)
- **Next.js frontend** (`frontend/`) — six analysis cards in a 3×2 grid
- **Discord bot** (`discord-bot/`) — DM + server commands

## Analysis pipeline (per beatmap)
1. Parse map (rosu_pp); compute beatmap stats (AR/OD/HP/CS/BPM/star rating — metadata, not analysis)
2. finger_control: snap filter → patterns → rhythm segmentation → transitions → timeline
3. reading: visuals → density → trajectory → traps → strain → sequence motor descriptors
4. Aggregate into classification (Jump/Stream/Slider only, lossy by design) + per-analysis detail cards

## Entities & concepts
- [[Beatmap]], [[Analysis-Type]], [[Data-Philosophy]]

## Status map
| Area | State |
|---|---|
| Reading analysis iteration | In progress — branch `Reading_Analysis_Iteration` |
| Forward density | Issue #4 CLOSED + archived 2026-08-08; design notes → [[forward-density]] |
| Spacing demand | Backend prototyped; frontend undecided → [[spacing-demand]] |
| Frontend overhaul | Unified six-card grid; Jump/Stream/Slider keep minor leftover markup, no charts |

_Sources: repo structure, raw/CONTEXT.md, memory files (2026-07-09 → 07-16)._
