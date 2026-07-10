# Frontend Design Rules (July 2026)

Canonical reference for all beatmap analyzer UI design decisions. **Any future frontend changes must be checked against these rules.**

---

## 1. Layout & Grid

### Container
- `max-w-6xl` centred, `mx-auto` with `px-4`
- Responsive: full width mobile → constrained desktop

### Card Dashboard Grid

| Row | Columns (xl) | Columns (md) | Columns (sm) |
|---|---|---|---|
| Stats + Classification | `md:grid-cols-2` | 2 | 1 |
| Section cards (6 profiles) | `grid-cols-3` | 2 | 1 |

- **Gap**: `gap-4` between cards
- **Between rows**: `mb-6`

### Section Cards
- shadcn/ui `<Card>` with:
  - `border-t-2 border-t-{type}-500/50` accent (top edge)
  - `<CardHeader>` → title only, no badges/progress bars
  - `<ScrollArea className="h-72 pr-3">` wrapping content
- Profiles render directly inside ScrollArea (no accordion, no extra Card wrappers)

---

## 2. Heading Tier System

All headings use CSS-only styling (no runtime cost). Applied per-profile as specified below.

### Tier Definitions

| Tier | Usage | CSS | Font | Weight | Case | Accent | Margin |
|---|---|---|---|---|---|---|---|
| **1** | `CardTitle` | `text-base font-semibold` | base | semibold | Sentence | — | — |
| **2** | Section heading | `text-sm font-semibold text-{type}-400` | sm (14px) | semibold | Sentence | Type color | `mb-4` |
| **3** | Sub-section / chart title | `text-[11px] font-semibold text-gray-400 border-l-2 border-l-{type}-500 pl-2` | 11px | semibold | Sentence | Type-colored left border | `mb-3` |
| **4** | *(unused)* | — | — | — | — | — | — |
| **5** | Data labels, key-value pairs | As-is per profile | — | — | — | — | — |

### Tier 2 Spacing
- Between Tier-2 sections within a profile: `space-y-6` on the outer container
- Heading itself: `mb-4` below

### Tier 3 Spacing
- `mb-3` below heading
- Content below uses standard StatBar spacing (`mb-2` per bar)

### Tier 5 Rule
Leave as-is. No unified styling applied. These include:
- Spacing/Type/Style tag lines
- Max chain/length values
- BPM consistency values
- Stat summary cards
- Chart k-line charts

---

## 3. StatBar Component

Shared component at `analysis_engine/StatBar.tsx`.

### API
```tsx
interface StatBarProps {
    label: string;
    value?: number;     // shows "value (pct%)" when provided
    total?: number;     // pct = value/total * 100
    percentage?: number; // direct percentage (0-100)
    colorClass: string;  // Tailwind bg color, e.g. "bg-green-500"
}
```

### Style
- `h-1` thin bar, `bg-gray-800` track, `rounded-full` overflow-hidden
- Label: `text-xs text-gray-300`
- Value: `font-mono text-gray-400` with `text-[10px]` percentage suffix
- Container: `mb-2`

### Layout
- StatBars are wrapped in `grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1` per sub-section
- Applies to all sub-sections with StatBars (Jump, Stream, Slider, AimControl, Reading)
- **FingerControl is excluded** from StatBar and 2-column grid changes

---

## 4. Severity Color Gradient (StatBars)

Each profile's StatBars follow a difficulty gradient: **easiest → hardest**.

| Severity | Color | Meaning |
|---|---|---|
| Easiest | `green-500` | Low difficulty / simple |
| Moderate | `blue-500` | Medium difficulty |
| Hard | `orange-500` | High difficulty |
| Hardest | `red-500` | Extreme / demanding |

### Per-Profile Mappings

**Jump**
- Distance: Narrow (green) / Moderate (blue) / Wide (orange) / Extreme (red)
- Chains: Short (green) / Medium (blue) / Long (red)

**Stream**
- Distance: Stacked (green) / Overlap (blue) / Spaced (orange) / Extreme (red)
- Variance: Steady (green) / Variable (blue) / Dynamic (red)
- Length: Bursts (green) / Short (blue) / Medium (orange) / Long (red) / Deathstream (red)

**Slider**
- Length: Short (green) / Medium (blue) / Long (orange) / Extended (red)
- Buzz: Static (green) / Buzz (blue)
- Artistic: Simple (green) / Curved (blue) / Complex (orange) / Artistic/Tech (red)

**Reading** (keep existing colors)
- Visual Clutter: emerald / blue / yellow / red
- Trajectory Chaos: emerald / blue / orange / red

**AimControl** (keep existing colors)
- Spacing: gray / blue / emerald / orange / red
- Angles: emerald / blue / orange / red
- Alignment: emerald / orange / red

---

## 5. Type Color System

Each analysis type has an assigned accent color used for:
- Card top border (`border-t-2 border-t-{type}-500/50`)
- Tier 2 heading text color (`text-{type}-400`)
- Tier 3 left border (`border-l-{type}-500`)
- Any type-specific icons or accents

| Type | Color Class | hex |
|---|---|---|
| **Jump** | `pink-400` / `pink-500` | #f472b6 / #ec4899 |
| **Stream** | `blue-400` / `blue-500` | #60a5fa / #3b82f6 |
| **Slider** | `green-400` / `green-500` | #4ade80 / #22c55e |
| **Finger Control** | `purple-400` / `purple-500` | #c084fc / #a855f7 |
| **Aim Control** | `cyan-400` / `cyan-500` | #22d3ee / #06b6d4 |
| **Reading** | `amber-400` / `amber-500` | #fbbf24 / #f59e0b |

---

## 6. Classification Card

- Shows only **Jump**, **Stream**, **Slider** types
- Sorted by `overall_confidence` descending
- Format: progress bar (type-colored) + "Map Presence: XX.X%"
- No accordion, no details — summary only

---

## 7. Profile-Specific Structural Rules

### Aim Control
- No "Deflection & Vectors" heading exists
- Angles and Alignment are Tier 2 headings directly

### Reading
- No Roman numeral numbering on headings (no "I.", "II.", "III.")
- Cognitive Strain Topography is Tier 5 (leave as-is)
- Relational Deception heading is a flex row: `<h3>Tier 2</h3>` + inline Trap Index

### Finger Control
- **Excluded** from StatBar color changes and 2-column grid layout
- **Included** in heading tier system (all Tier 2 and Tier 3 headings apply)

---

## 8. Spacing Reference

| Context | Class |
|---|---|
| Card-to-card gap | `gap-4` |
| Between major rows | `mb-6` |
| Between Tier-2 sections (within profile) | `space-y-6` |
| Below Tier 2 heading | `mb-4` |
| Below Tier 3 heading | `mb-3` |
| Between StatBars | `mb-2` (built into StatBar) |
| Gap between StatBar grid items | `gap-x-8 gap-y-1` |
| Section scroll area height | `h-72 pr-3` |

---

## 9. Profile Components Are Flat

Sub-sections within profile components use **plain `<div>` wrappers**, not `<Card>` containers. The only Cards are the outer section cards in the grid layout (defined in `analysis.tsx`). This applies to all 6 profiles uniformly.

### Applies to
- AimControl: Spacing Profile, Angles, Alignment, ACCV, Sustained Aim Strain
- Reading: Cognitive Strain Topography, Visual Clutter, Trajectory Chaos, Relational Deception
- Jump, Stream, Slider, FingerControl: already flat (no change needed)
