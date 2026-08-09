---
type: entity
updated: 2026-08-06
---
# Analysis Type

Six analysis dimensions, each rendered as a **detail card** in a 3×2 grid: **Jump · Stream · Slider · Finger Control · Aim Control · Reading**.

## The three layers of output
| Layer | What it is | Notes |
|---|---|---|
| Classification | Quantity-based overview per type (overall_confidence) | Covers Jump/Stream/Slider only; lossy by design |
| Analysis detail card | Deep-dive per type: difficulty, technicality, distribution, sub-metrics | One card per analysis |
| Beatmap stats | AR/OD/HP/CS/BPM/star rating, total_objects | From metadata, **not** computed by analysis |

## Related
- [[Beatmap]] · [[Data-Philosophy]] · [[reading-analysis]] · [[finger-control]]

_Source: raw/CONTEXT.md_
