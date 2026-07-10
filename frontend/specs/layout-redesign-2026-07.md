# Layout Redesign Spec — July 2026

## Goal
Restructure the beatmap analysis page from a single-column scroll layout to a card-grid dashboard. Remove the radar chart. Improve readability via consistent progress bars, StatBar components, and 3-column grid for the six analysis sections.

## Layout Overview

```
┌──────────────────────────────────────────────────────────────┐
│                     Header (existing)                        │
│         "osu! Beatmap Analyzer" + subtitle + URL input       │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│                  Beatmap Banner (existing)                    │
│           Cover image | Title | Artist | Creator | Version   │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│   ┌─────────────────────────────┬──────────────────────────┐ │
│   │     Beatmap Stats Card      │    Classification Card    │ │
│   │   AR  OD  HP  CS  BPM  SR   │  Jumps/Streams/Sliders   │ │
│   │                             │  sorted by confidence    │ │
│   └─────────────────────────────┴──────────────────────────┘ │
│                                                              │
│   ┌──────────────────┬──────────────────┬──────────────────┐ │
│   │      JUMPS       │     STREAMS      │     SLIDERS      │ │
│   │  (scrollable)    │   (scrollable)   │   (scrollable)   │ │
│   │  Full profile    │   Full profile   │   Full profile   │ │
│   │  StatBars + text │   StatBars + text│   StatBars + text│ │
│   └──────────────────┴──────────────────┴──────────────────┘ │
│                                                              │
│   ┌──────────────────┬──────────────────┬──────────────────┐ │
│   │  Finger Control  │   Aim Control    │    Reading       │ │
│   │  (scrollable)    │   (scrollable)   │   (scrollable)   │ │
│   │  Full profile    │   Full profile   │   Full profile   │ │
│   └──────────────────┴──────────────────┴──────────────────┘ │
│                                                              │
│                  Alert (existing, keep)                        │
└──────────────────────────────────────────────────────────────┘
```

## Container
- Widen from `max-w-3xl` to `max-w-6xl` on the analysis results wrapper
- The URL input form stays at the top (above the banner), keep existing styling
- Alert stays at the bottom, keep existing

## 1. Header Row (Keep As-Is)
- Existing site nav (header.tsx)
- URL input + Analyze button
- No changes

## 2. Beatmap Banner (Keep As-Is)
- Full-bleed cover image with overlay, title, artist, creator, version
- No changes

## 3. Beatmap Stats Card
- Same card structure, but update value formatting:
  - AR: 2 decimal places
  - OD, HP, CS, BPM: 1 decimal place
  - Star Rating: 2 decimal places
- Keep `grid grid-cols-2` for the 6 stats
- Keep same layout order: AR, OD, HP, CS, BPM, Star Rating

## 4. Classification Card (New Layout)
- **Title**: "Classification" with Music icon (keep existing)
- **Content**: Show only **Jump**, **Stream**, **Slider** analysis types
- **Sorted by**: `overall_confidence` descending (most dominant first)
- **Display format per type**: Existing `AnalysisCardClass` style — progress bar + "Map Presence: XX.X%" label
- The classification card is an independent card beside Beatmap Stats in a 2-column row

### Data Source
- Filter `analysisResult` to `analysis_type === "jump" | "stream" | "slider"`
- Sort by `overall_confidence` descending
- No accordion/details here — keep it summary-only

## 5. Section Cards (6 cards, 2 rows × 3 columns)

### Row 1: Jump | Stream | Slider
### Row 2: Finger Control | Aim Control | Reading

### Card Structure (per section card)
- **shadcn/ui `<Card>`**
- **CardHeader**: Just the title — "Jumps", "Streams", "Sliders", "Finger Control", "Aim Control", "Reading"
  - **No** confidence badge, **no** progress bar in header
- **CardContent**: The full profile component rendered directly (no wrapping accordion)
- **Scroll behavior**: `<ScrollArea>` with fixed height (e.g. `h-72` or `h-80`) wrapping the content
- **Color accent**: Subtle top-border or left-accent per type (see color system below)

### Profile Component Content (unchanged data, restyled formatting)

| Card | Component | Visual update needed |
|---|---|---|
| **Jumps** | `JumpProfile` | Convert distance profile to StatBar style |
| **Streams** | `StreamProfile` | Convert distance, variance, length profiles to StatBar |
| **Sliders** | `SliderProfile` | Convert length, buzz, artistic profiles to StatBar |
| **Finger Control** | `FingerControlProfile` | Keep as-is (already has visual elements) |
| **Aim Control** | `AimControlProfile` | Keep as-is (already uses StatBar) |
| **Reading** | `ReadingProfile` | Keep as-is (already uses ProgressBar) |

## 6. StatBar Component (Shared)

Prefer the **AimControl `StatBar`** pattern (thin `h-1` bar, compact, value+% on right):

```tsx
const StatBar = ({ label, value, total, colorClass }) => (
    <div className="mb-2">
        <div className="flex justify-between text-xs mb-0.5">
            <span className="text-gray-300">{label}</span>
            <span className="font-mono text-gray-400">
                {value} <span className="text-[10px]">({percentage.toFixed(1)}%)</span>
            </span>
        </div>
        <div className="h-1 w-full bg-gray-800 rounded-full overflow-hidden">
            <div className={`h-full ${colorClass}`} style={{ width: `${percentage}%` }} />
        </div>
    </div>
);
```

- Extract into `analysis_engine/StatBar.tsx`
- Replace the ReadingProfile's local `ProgressBar` with `StatBar` for consistency
- Use in JumpProfile, StreamProfile, SliderProfile for categorized metrics

## 7. Color System

| Analysis Type | Color Class | Accent Usage |
|---|---|---|
| **Jump** | `pink-500` | Progress bars, chart lines |
| **Stream** | `blue-500` | Progress bars, chart lines |
| **Slider** | `green-500` | Progress bars, chart lines |
| **Finger Control** | `purple-500` | Progress bars, chart lines |
| **Aim Control** | `cyan-500` | Progress bars, chart lines |
| **Reading** | `amber-500` | Progress bars, chart lines |

Card accent: thin top border (`border-t-2 border-{type}-500/50`) or left border (`border-l-4 border-{type}-500`).

## 8. Type System

1. Import `BeatmapAnalysisResult` from `./analysis_engine/types` (the 6-type union)
2. Remove the local `BeatmapAnalysisResult` interface definition in `analysis.tsx`
3. The `getBeatmapAnalysis("all")` return type should also use the types.ts version

## 9. Removed

- **RadarChartComponent**: Delete `RadarChartComponent.tsx`, remove import in `analysis.tsx`
- Remove commented-out radar chart code

## 10. Responsive Grid

```
Desktop (xl, >1280px):  max-w-6xl container
  - Row 1: md:grid-cols-2  (Stats | Classification)
  - Row 2: grid-cols-3     (Jump | Stream | Slider)
  - Row 3: grid-cols-3     (Finger Ctrl | Aim Ctrl | Reading)

Tablet (md, ~768px):
  - Row 1: grid-cols-2     (Stats | Classification)
  - Row 2: grid-cols-2     (cards wrap to 2 columns)
  - Row 3: grid-cols-2     (cards wrap to 2 columns)

Mobile (<640px):
  - All rows: grid-cols-1  (stacked)
```

Gap: `gap-4` between grid items (the user delegated this choice; `gap-4` balances density and breathing room at max-w-6xl with 3 columns).

## 11. File Changes Summary

| File | Change |
|---|---|
| `frontend/src/components/analysis.tsx` | Restructure layout, remove radar chart, import types from types.ts, remove local BeatmapAnalysisResult definition, update formatting |
| `frontend/src/components/analysis_engine/index.tsx` | Remove `AnalysisCardDetails` (no longer needed if profiles render directly), or keep as utility |
| `frontend/src/components/analysis_engine/RadarChartComponent.tsx` | DELETE |
| `frontend/src/components/analysis_engine/StatBar.tsx` | NEW — shared StatBar component |
| `frontend/src/components/analysis_engine/JumpProfile.tsx` | Convert to StatBar for distance profile categories |
| `frontend/src/components/analysis_engine/StreamProfile.tsx` | Convert to StatBar for distance/variance/length categories |
| `frontend/src/components/analysis_engine/SliderProfile.tsx` | Convert to StatBar for length/buzz/artistic categories |
| `frontend/src/components/analysis_engine/ReadingProfile.tsx` | Replace local ProgressBar with shared StatBar |
| `frontend/src/app/page.tsx` | Widen container (change max-w-3xl → allow wider) |

## 12. Future: SMA Curves
See memory note [[sma-curves-ui-formatting]] for the plan to add collapsible chart sections when SMA data becomes available for all metrics.

## 13. Phase 2: Profile Readability Overhaul (July 2026)

### 13.1 Heading Tier System
See [[heading-tier-specification]] for the full per-heading assignment.

| Tier | Style | CSS |
|---|---|---|
| **1** | shadcn CardTitle | text-base font-semibold sentence case |
| **2** | Section heading | text-sm font-semibold, type-colored (e.g. text-pink-400), sentence case, mb-4 |
| **3** | Sub-section heading | text-[11px] font-semibold, text-gray-400, sentence case, border-l-2 type-colored, pl-2, mb-3 |
| **4** | *(unused)* | — |
| **5** | Data labels | Left as-is per profile |

### 13.2 StatBar Color Gradient
Per-category severity colors (green → blue → orange → red) replacing single-type-color bars in Jump, Stream, Slider. See [[profile-unified-formatting]].

### 13.3 2-Column Layout
`grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1` for all sub-sections with StatBars (Jump, Stream, Slider, Reading follow AimControl's pattern).

### 13.4 Structural Changes
- "Deflection & Vectors" heading removed from AimControl
- Roman numeral numbering dropped from Reading headings
- Chart titles → Tier 3

## Implementation Order

1. Prep: Delete RadarChartComponent, update imports
2. Type system: Consolidate to use types.ts, remove local duplicate
3. StatBar: Create shared component
4. Update profiles: JumpProfile → StatBar, StreamProfile → StatBar, SliderProfile → StatBar, ReadingProfile → shared StatBar
5. Layout: Restructure analysis.tsx with new grid layout, separate sections
6. Classification: Filter to 3 types, keep summary-only
7. Responsive: Verify grid behavior at all breakpoints
8. Polish: Card accents, spacing, stat formatting

### Phase 2 Order (next session)
9. Update heading tiers (promote/demote as specified)
10. Apply 2-column grid layout to sub-sections
11. Update StatBar colors to severity gradient
12. Apply heading CSS tokens (Tier 2 + Tier 3)
13. Remove "Deflection & Vectors" heading from AimControl
