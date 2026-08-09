# Domain Glossary — osu! Beatmap Analyzer

## Beatmap
A playable osu! difficulty (a single "map"). Identified by a beatmap ID. Belongs to a beatmap set (identified by a set ID). Not "bitmap."

## Classification
Quantity-based overview of a beatmap's pattern composition. Answers "how much of this type exists?" — based on overall_confidence per analysis type. Only covers three pattern types: Jump, Stream, Slider. Lossy by design — does not reflect difficulty or quality.

## Analysis Detail Card
A deep-dive into one specific aspect of the beatmap (Jump, Stream, Slider, Finger Control, Aim Control, or Reading). Covers difficulty, technicality, distribution, and sub-metrics. One card per analysis type, rendered in a 3×2 grid.

## Beatmap Stats
Fixed properties of the beatmap itself (AR, OD, HP, CS, BPM, Star Rating, total_objects). Not computed by analysis — pulled from the beatmap metadata.
