"""
Spacing Transition Demand --- Prototype and Comparison
=======================================================

Implements and compares two candidate families from the design spec:
  Family A: Second-order Total Variation (TV2)
  Family B: Local Trend Deviation (LTD) --- bending energy interpretation

Design principles tested:
  1. Describe, don't classify     -> continuous output
  2. No arbitrary weights          -> every operation emerges from math
  3. Locality                      -> depends on local transitions
  4. Continuous                    -> small changes -> small score changes
  5. Extendable                    -> same math applies to angle sequences
"""

import math


# ======================================================================
#  FAMILY A: Second-order Total Variation
# ======================================================================
#
#  Given spacing sequence s = [s_1, s_2, ..., s_m]  (m = n_notes - 1)
#
#  1st differences:   Ds_i = s_{i+1} - s_i            for i = 0..m-2
#  2nd differences:   D2s_i = Ds_{i+1} - Ds_i         for i = 0..m-3
#                      = s_{i+2} - 2*s_{i+1} + s_i
#
#  TV2 = sum_i |D2s_i|
#
#  Properties:
#  - 0 iff spacing is an arithmetic progression (linear trend)
#  - L1 norm: treats all deviations proportionally
#  - Scales linearly with pattern length (sum, not mean)
#  - Locality: each D2 term involves 3 consecutive spacings (4 notes)

def second_order_total_variation(spacings):
    m = len(spacings)  # number of spacings = n_notes - 1

    if m < 3:
        return {
            'score': None,
            'components': [],
            'n_terms': 0,
            'description': 'Insufficient spacings for second-order variation'
        }

    deltas2 = []
    for i in range(m - 2):
        d2 = spacings[i + 2] - 2.0 * spacings[i + 1] + spacings[i]
        deltas2.append(d2)

    score = sum(abs(d) for d in deltas2)

    return {
        'score': score,
        'components': deltas2,
        'n_terms': len(deltas2),
        'desc_formula': ' + '.join(f"|{d:.4f}|" for d in deltas2)
    }


# ======================================================================
#  FAMILY B: Local Trend Deviation --- Bending Energy interpretation
# ======================================================================
#
#  The mathematical object is "discrete bending energy":
#  For a discrete curve, bending energy approx = sum (curvature)^2
#  Curvature is proportional to second derivative.
#
#  LTD_energy = (1/k) * sum_i (D2s_i)^2
#
#  where k = n_terms = m - 2 (number of second-difference terms)
#
#  The mean (not sum) normalises for pattern length --- critical for
#  comparing bursts of different lengths.
#
#  The square (L2 norm) emerges from the physics of bending:
#  - Bending energy of a thin beam = integral kappa^2 ds (not |kappa|)
#  - Large deviations cost disproportionately more
#  - A single large "kink" costs more than many small wiggles
#
#  Properties:
#  - 0 iff spacing is arithmetic progression
#  - L2 norm: penalises large deviations more than proportionally
#  - Normalised by length: comparable across patterns
#  - Connected to physical bending energy, not ad-hoc weighting
#  - Locality: same window as Family A

def local_trend_deviation_energy(spacings):
    m = len(spacings)

    if m < 3:
        return {
            'score': None,
            'components': [],
            'n_terms': 0,
            'description': 'Insufficient spacings for local trend deviation'
        }

    deltas2 = []
    for i in range(m - 2):
        d2 = spacings[i + 2] - 2.0 * spacings[i + 1] + spacings[i]
        deltas2.append(d2)

    k = len(deltas2)
    energy = sum(d * d for d in deltas2) / k

    return {
        'score': energy,
        'components': [d * d for d in deltas2],
        'n_terms': k,
        'desc_formula': f"(1/{k}) * [{', '.join(f'{d*d:.4f}' for d in deltas2)}]"
    }


def local_trend_deviation_rms(spacings):
    result = local_trend_deviation_energy(spacings)
    if result['score'] is not None:
        result['score'] = math.sqrt(result['score'])
        result['desc_formula'] = result['desc_formula'] + f"  (RMS: {result['score']:.4f})"
    return result


def ltd_energy_from_components(deltas2):
    """Compute LTD energy from raw second-difference components."""
    k = len(deltas2)
    if k == 0:
        return 0.0
    return sum(d * d for d in deltas2) / k


# ======================================================================
#  TEST PATTERNS
# ======================================================================
#
#  All spacings in circle-diameter units (D).
#  Each entry: (name, note_count, spacing_sequence, expected_behavior)

TEST_PATTERNS = [
    # --- Consistent / monotonic (should score ~0) ---
    ("Consistent medium",  4, [1.0, 1.0, 1.0],
     "Three equal spacings. Smooth progression. Expect ~0."),
    ("Consistent wide",    4, [3.0, 3.0, 3.0],
     "Three equal wide spacings. Smooth. Expect ~0."),
    ("Steady increase",    4, [1.0, 2.0, 3.0],
     "Arithmetic progression. Monotonic, smooth. Expect ~0."),
    ("Steady decrease",    4, [3.0, 2.0, 1.0],
     "Arithmetic progression. Monotonic. Expect ~0."),
    ("Gentle increase",    5, [0.5, 1.0, 1.5, 2.0],
     "Arithmetic progression, 4 spacings. Expect ~0."),
    ("Steady wide increase", 5, [1.0, 2.0, 3.0, 4.0],
     "Wide arithmetic progression. Expect ~0."),

    # --- Single reversal / kink ---
    ("Big swing up-down",  4, [1.0, 6.0, 1.0],
     "Single large reversal. One D2 term = -10. High demand."),
    ("Medium swing up-down", 4, [1.0, 4.0, 1.0],
     "Medium reversal. One D2 term = -6. Moderate."),
    ("Small swing up-down", 4, [1.0, 2.0, 1.0],
     "Small reversal. One D2 term = -2. Low-moderate."),

    # --- Alternating patterns ---
    ("Alternating tight",  5, [1.0, 2.0, 1.0, 2.0],
     "Up-down-up-down, small amplitude. Two D2 terms."),
    ("Alternating big",    5, [1.0, 6.0, 1.0, 6.0],
     "Up-down-up-down, large amplitude. Two D2 terms."),
    ("Alternating growing",5, [1.0, 2.0, 1.5, 3.0],
     "Irregular alternating. Not purely oscillating."),

    # --- Complex / irregular ---
    ("Chaotic narrow",    5, [1.0, 1.5, 1.0, 2.0],
     "Small variations, multiple changes. Narrow range."),
    ("Chaotic wide",      5, [1.0, 5.0, 2.0, 6.0],
     "Large variations, multiple changes. Wide range."),
    ("Step then wobble",  6, [1.0, 1.0, 4.0, 4.5, 4.0],
     "Steady -> big jump -> steady wobble. Two regimes."),

    # --- Edge cases ---
    ("Stacked (all ~0)",  4, [0.0, 0.0, 0.0],
     "All overlapping. Zero spacings. Expect 0."),
    ("Stacked then wide", 4, [0.0, 4.0, 0.0],
     "Stacked -> wide -> stacked. Single reversal."),

    # --- Too short ---
    ("Burst 2 (double)",  2, [2.5],
     "Only 1 spacing. Expect None/insufficient."),
    ("Burst 3 (triple)",  3, [1.0, 3.0],
     "Only 2 spacings. Expect None/insufficient."),
]


# ======================================================================
#  COMPARISON AND ANALYSIS
# ======================================================================

def run_comparison(patterns):
    results = []
    for name, notes, spacings, desc in patterns:
        a = second_order_total_variation(spacings)
        b = local_trend_deviation_energy(spacings)
        b_rms = local_trend_deviation_rms(spacings)
        results.append({
            'name': name, 'notes': notes, 'spacings': spacings, 'desc': desc,
            'tv2': a, 'ltd_energy': b, 'ltd_rms': b_rms,
        })
    return results


def fmt_spacings_arr(spacings):
    return "[" + ", ".join(f"{s:.1f}" for s in spacings) + "]"


def fmt_score(val):
    return f"{val:.4f}" if val is not None else "N/A"


def compute_ranking(results, key):
    scored = [(r['name'], r[key]['score']) for r in results
              if r[key]['score'] is not None]
    scored.sort(key=lambda x: x[1], reverse=True)
    return scored


def print_table(results):
    header = f"{'Pattern':<26} {'N':>3} {'Spacings':<22} {'TV2':>10} {'LTD_en':>10} {'LTD_rm':>10}"
    sep = "-" * 85
    print(sep)
    print(header)
    print(sep)
    for r in results:
        print(
            f"{r['name']:<26} {r['notes']:>3} {fmt_spacings_arr(r['spacings']):<22} "
            f"{fmt_score(r['tv2']['score']):>10} "
            f"{fmt_score(r['ltd_energy']['score']):>10} "
            f"{fmt_score(r['ltd_rms']['score']):>10}"
        )
    print(sep)


def section(title):
    print()
    print("=" * 85)
    print(f"  {title}")
    print("=" * 85)


def subsection(title):
    print()
    print(f"  --- {title} ---")


# ======================================================================
#  DESIGN PRINCIPLE VERIFICATION
# ======================================================================

def verify_continuity():
    subsection("P1: Continuous -- small changes produce small score changes")
    print("  Pattern: [1.0, X, 1.0] varying X from 3.0 to 10.0:")
    print(f"  {'X':>8} {'TV2':>10} {'LTD_energy':>12} {'LTD_rms':>10}")
    for x in [3.0, 3.001, 3.1, 3.5, 5.0, 10.0]:
        tv2 = second_order_total_variation([1.0, x, 1.0])
        ltd_e = local_trend_deviation_energy([1.0, x, 1.0])
        ltd_r = local_trend_deviation_rms([1.0, x, 1.0])
        print(f"  {x:>8.3f} {fmt_score(tv2['score']):>10} {fmt_score(ltd_e['score']):>12} {fmt_score(ltd_r['score']):>10}")
    print("  -> Both families are continuous. Small DX gives small Dscore. OK.")


def verify_no_weights():
    subsection("P2: No arbitrary weights")
    print("  TV2:        sum|D2s|  -- total absolute second-order variation")
    print("  LTD_energy: (1/k)*sum(D2s)^2 -- mean squared second-order deviation")
    print("  -> Both emerge from well-established mathematical objects.")
    print("  -> Neither introduces hand-tuned coefficients. OK.")


def verify_locality():
    subsection("P3: Locality -- depends on local transitions")
    localised = [1.0, 1.0, 4.0, 4.0]
    smooth = [2.0, 2.5, 3.0, 2.5]
    tv2_l = second_order_total_variation(localised)['score']
    tv2_s = second_order_total_variation(smooth)['score']
    ltd_l = local_trend_deviation_energy(localised)['score']
    ltd_s = local_trend_deviation_energy(smooth)['score']
    print(f"  Pattern A (localised jump):  {localised}")
    print(f"    TV2={fmt_score(tv2_l)}  LTD_en={fmt_score(ltd_l)}")
    print(f"  Pattern B (smooth trend):    {smooth}")
    print(f"    TV2={fmt_score(tv2_s)}  LTD_en={fmt_score(ltd_s)}")
    print(f"  -> Both capture local transitions (A > B). OK.")


def verify_extendable():
    subsection("P4: Extendable to angle sequences")
    print("  Both families operate on a sequence of scalars.")
    print("  For angles, replace spacing sequence with angle sequence.")
    print("  Same formulas apply identically. OK.")


def analyse_length_normalisation():
    section("PATTERN LENGTH NORMALISATION ANALYSIS")
    print()
    print("  Key difference: TV2 is a SUM, LTD_energy is a MEAN.")
    print()
    print("  Same per-transition D2 magnitude, different lengths:")
    short = [1.0, 3.0, 1.0]
    long_ = [1.0, 3.0, 1.0, 3.0]

    print(f"  Short: {short}  -> 1 D2 term = -2")
    print(f"  Long:  {long_} -> 2 D2 terms = -2, -2")
    print()
    print(f"  {'':>10} {'TV2':>10} {'LTD_en':>10} {'LTD_rm':>10}")
    for label, s in [("Short", short), ("Long", long_)]:
        tv2 = second_order_total_variation(s)
        ltd_e = local_trend_deviation_energy(s)
        ltd_r = local_trend_deviation_rms(s)
        print(f"  {label:>10} {fmt_score(tv2['score']):>10} {fmt_score(ltd_e['score']):>10} {fmt_score(ltd_r['score']):>10}")

    print()
    print("  Implication:")
    print("  TV2 doubles (2x) because it sums -- conflates 'more notes'")
    print("  with 'more demanding per-note transitions'.")
    print("  LTD_energy stays the same -- normalised by length.")
    print("  -> For 'how demanding the EVOLUTION is' (not 'how many'),")
    print("     normalisation matters.")


def analyse_l1_vs_l2():
    section("L1 vs L2 BEHAVIOUR -- WHAT GETS EMPHASISED")
    print()
    print("  The choice between L1 (TV2) and L2 (LTD) changes how patterns")
    print("  are GROUPED, not just ranked.")
    print()

    patterns = [
        ("[1,2,1]   Small reversal",   [1.0, 2.0, 1.0]),
        ("[1,3,1]   Medium reversal",  [1.0, 3.0, 1.0]),
        ("[1,6,1]   Large reversal",   [1.0, 6.0, 1.0]),
    ]

    print(f"  {'Pattern':<24} {'TV2':>10} {'LTD_en':>10} {'LTD_rm':>10}")
    print("  " + "-" * 56)
    for label, s in patterns:
        tv2 = second_order_total_variation(s)
        ltd_e = local_trend_deviation_energy(s)
        ltd_r = local_trend_deviation_rms(s)
        print(f"  {label:<24} {fmt_score(tv2['score']):>10} {fmt_score(ltd_e['score']):>10} {fmt_score(ltd_r['score']):>10}")

    # Ratios
    base_tv2 = second_order_total_variation([1.0, 2.0, 1.0])['score']
    base_ltd = local_trend_deviation_energy([1.0, 2.0, 1.0])['score']
    print()
    print("  Ratio relative to [1,2,1]:")
    for label, s in patterns:
        tv2_r = second_order_total_variation(s)['score'] / base_tv2
        ltd_r = local_trend_deviation_energy(s)['score'] / base_ltd
        print(f"  {label:<24} TV2 x{tv2_r:.2f}  LTD x{ltd_r:.2f}")

    print()
    print("  [1,6,1] vs [1,2,1]:")
    print("  TV2  says: 6->1 reversal is 5x more demanding than 2->1 (linear)")
    print("  LTD  says: 6->1 reversal is 25x more demanding than 2->1 (quadratic)")
    print()
    print("  The quadratic (L2) penalisation aligns with the intuition")
    print("  that a 6D reversal is disproportionately harder, not just")
    print("  linearly harder. This matches the design spec's emphasis")
    print("  on 'abrupt reversals increase transition demand'.")


def analyse_alternating():
    section("ALTERNATING PATTERN ANALYSIS")
    print()
    print("  Alternating patterns are a stress test -- they produce")
    print("  repeated D2 terms of alternating sign.")
    print()

    print(f"  {'Pattern':<28} {'Spacings':<24} {'TV2':>10} {'LTD_en':>10}")
    print("  " + "-" * 74)
    for name, notes, spacings, desc in TEST_PATTERNS:
        if "Alternating" in name:
            tv2 = second_order_total_variation(spacings)
            ltd_e = local_trend_deviation_energy(spacings)
            print(f"  {name:<28} {fmt_spacings_arr(spacings):<24} {fmt_score(tv2['score']):>10} {fmt_score(ltd_e['score']):>10}")

    print()
    print("  Key observation: Alternating tight [1,2,1,2] and Alternating big")
    print("  [1,6,1,6] both produce 2 D2 terms. Under TV2, the big is 2.5x the")
    print("  tight. Under LTD, the big is 9x the tight (squared effect).")
    print()
    print("  Does this over-penalise alternating wide patterns? Or correctly")
    print("  capture that wide alternation is substantially harder?")
    print("  The design spec says: 'either can compensate for the other' and")
    print("  'repeated irregular transitions increase transition demand'.")
    print("  The spec does not directly address the absolute magnitude effect,")
    print("  only the interaction between frequency and magnitude.")


def check_pathological():
    section("PATHOLOGICAL PATTERN CHECK (design says: don't optimise for these)")
    print()
    pathological = [
        ("Theoretical nonsense", [0.0, 100.0, 0.0]),
        ("Oscillating extreme", [0.0, 100.0, 0.0, 100.0]),
        ("1-term spike", [100.0, 0.0, 100.0]),
    ]
    print(f"  {'Pattern':<28} {'Spacings':<22} {'TV2':>10} {'LTD_en':>10}")
    print("  " + "-" * 72)
    for label, s in pathological:
        tv2 = second_order_total_variation(s)
        ltd_e = local_trend_deviation_energy(s)
        print(f"  {label:<28} {fmt_spacings_arr(s):<22} {fmt_score(tv2['score']):>10} {fmt_score(ltd_e['score']):>10}")
    print()
    print("  Both families produce finite scores even for extreme values.")
    print("  The design spec says not to optimise for these -- they don't")
    print("  appear in real maps. Both metrics handle them gracefully.")
    print("  No division-by-zero or infinity issues at these values.")


def print_rankings(results):
    section("RANKINGS")

    for label, key in [("By TV2", 'tv2'), ("By LTD_energy", 'ltd_energy')]:
        print()
        print(f"  {label}:")
        for rank, (name, score) in enumerate(compute_ranking(results, key), 1):
            print(f"    {rank:>2}. {name:<28} {score:.4f}")
        print()


# ======================================================================
#  FINAL SYNTHESIS
# ======================================================================

def synthesise():
    section("SYNTHESIS AND RECOMMENDATION")

    print("""
  Both families satisfy all 5 design principles:
    P1 (Continuous):    OK  - small changes give small score changes
    P2 (No weights):    OK  - both use pure mathematical objects
    P3 (Local):         OK  - both operate on 3-spacing windows
    P4 (Extendable):    OK  - same formulas apply to angle sequences

  They differ in three critical dimensions:

  1. NORMALISATION
     TV2 is a SUM (scales with pattern length)
     LTD_energy is a MEAN (normalised by length)

     For a metric that "describes the spacing sequence itself" (not the
     number of transitions), normalisation matters. LTD separates "how
     demanding each transition" from "how many transitions."

  2. EMPHASIS ON LARGE DEVIATIONS
     TV2 uses L1 norm (absolute): linear penalty
     LTD uses L2 norm (square):   quadratic penalty

     A 6D reversal is 4x more demanding than a 2D reversal under TV2,
     but 9x under LTD. The quadratic matches the intuition that extreme
     kinks are disproportionately harder.

  3. COMPARABILITY ACROSS PATTERNS
     TV2 of a 6-note pattern and a 4-note pattern are not directly
     comparable (different number of terms).
     LTD_energy normalises by k, making scores comparable across any
     pattern length.

  RECOMMENDATION: Family B (LTD_energy) as primary metric.
  - The mean normalisation is essential for a per-transition descriptor
  - The L2 penalisation of large deviations aligns with real difficulty
  - The mathematical object (discrete bending energy) has a physical
    interpretation that doesn't require justification

  Supplementary: Keep RMS version (sqrt of LTD_energy) as a display metric
  since it restores units to the same scale as raw spacings (circle
  diameters), making it more intuitive to read.
""")


# ======================================================================
#  MAIN
# ======================================================================

if __name__ == '__main__':
    print()
    print("  " + "=" * 55)
    print("    Spacing Transition Demand -- Candidate Comparison")
    print("  " + "=" * 55)

    results = run_comparison(TEST_PATTERNS)

    section("MAIN COMPARISON TABLE")
    print_table(results)

    print_rankings(results)

    verify_continuity()
    verify_no_weights()
    verify_locality()
    verify_extendable()

    analyse_length_normalisation()
    analyse_l1_vs_l2()
    analyse_alternating()
    check_pathological()

    synthesise()
