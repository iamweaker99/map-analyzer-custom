---
type: concept
updated: 2026-08-06
---
# Data Philosophy — Raw Data, No Interpretation

All analyses report **flat, non-nested, unweighted values**. No difficulty weightage, no arbitrary "hard/easy" thresholds, no decay/nerf factors. The consumer interprets; the analysis only reveals what exists in the beatmap.

## Consequences (how to apply)
- New metrics report raw values only — e.g. [[forward-density]] counts notes, no exponential falloff
- Angle and spacing are reported side-by-side — correlated, not causative
- Classification is lossy *by design* ([[Analysis-Type]]): quantity, not quality
- Frontend decides what to emphasize

## Related
- [[forward-density]] — an application of this rule
- [[keep-12-snap]] — metric scope chosen by data, not by filter

_Source: raw/prd-reading-analysis.md (Data Philosophy §), raw/CONTEXT.md_
