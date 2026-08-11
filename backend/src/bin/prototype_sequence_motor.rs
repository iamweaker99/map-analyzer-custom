//! PROTOTYPE — Sequence Motor Descriptors
//!
//! Load a .osu beatmap and display per-pattern MPA / MM / SC metrics.
//!
//! Usage:
//!   cargo run --bin prototype_sequence_motor -- "path/to/file.osu"
//!   cargo run --bin prototype_sequence_motor -- "path/to/file.osu" --json
//!   cargo run --bin prototype_sequence_motor -- "path/to/file.osu" --patterns  (verbose: all patterns with precise timing)
//!   cargo run --bin prototype_sequence_motor -- "path/to/file.osu" --T     (temporal segmentation)
//!   cargo run --bin prototype_sequence_motor -- "path/to/file.osu" --R     (rhythm segmentation, default)
//!   cargo run --bin prototype_sequence_motor -- "path/to/file.osu" --patterns --R --exp   (experimental segmentation)
//!
//! The `--json` flag prints the sequence_motor JSON for further processing.
//! The `--patterns` flag dumps ALL patterns (incl. singletons) with precise
//! millisecond timing — useful for cross-referencing with the in-game editor.
//! Use `--T` (temporal) or `--R` (rhythm) to choose the discontinuity signal
//! for pattern segmentation. When neither is specified, R is the default for
//! backward compatibility.
//!
//! `--exp` switches to the experimental asymmetric type-boundary segmentation
//! (2026-08-09/08-10 decisions, wiki [[rhythm-segmentation]]): slider→circle
//! always splits; circle→slider splits only when the following slider run has
//! ≥2 sliders (a lone slider within 2× the circle diameter engulfs into the
//! preceding circle group); pure slider runs ≥2 become "SliderChain" instead
//! of "Stream"; the R/T discontinuity signal is skipped at slider→circle
//! transition windows (its boundary would land one note inside the circle
//! run — the type rule already bounds the run at the transition). These rules
//! were ported into production rhythm_segmentation.rs on 2026-08-11
//! ([[segmentation-unification]]); this prototype path is kept as the
//! cross-check reference. Adds `new_combo` / `mid_combo_breaks` reference
//! columns to the --patterns dump (raw .osu parse — rosu_pp drops the bit).

use std::ops::Range;
use std::path::Path;
use std::process;

use backend::analysis::finger_control::rhythm_segmentation::Pattern;
use backend::analysis::finger_control::rhythm_segmentation;
use backend::analysis::finger_control::snap_filter;
use backend::analysis::reading::sequence_motor;
use backend::analysis::reading::visuals;
use rosu_pp::model::control_point::TimingPoint;
use rosu_pp::model::hit_object::HitObject;
use rosu_pp::Beatmap;

fn load_beatmap(path: &str) -> rosu_pp::Beatmap {
    let p = Path::new(path);
    if !p.exists() {
        eprintln!("Error: file not found: {}", p.display());
        process::exit(1);
    }
    if p.extension().map_or(true, |e| e != "osu") {
        eprintln!("Error: not a .osu file: {}", p.display());
        process::exit(1);
    }
    match rosu_pp::Beatmap::from_path(p) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error: failed to parse .osu: {}", e);
            process::exit(1);
        }
    }
}

fn format_time(ms: f64) -> String {
    let total_secs = ms / 1000.0;
    let mins = total_secs as u64 / 60;
    let secs = total_secs as u64 % 60;
    let millis = (ms % 1000.0).round() as u64;
    format!("{:02}:{:02}:{:03}", mins, secs, millis)
}

fn build_seq_json(seq_output: &sequence_motor::SequenceMotorOutput) -> serde_json::Value {
    let timeline: Vec<serde_json::Value> = seq_output
        .timeline
        .iter()
        .map(|p| serde_json::json!({
            "time": format_time(p.time_ms),
            "notes": p.note_count,
            "mpa": p.mpa,
            "mm": p.mm,
            "sc": p.sc,
        }))
        .collect();

    serde_json::json!({
        "timeline": timeline,
        "summary": {
            "mpa": {
                "mean": seq_output.summary_mpa.mean,
                "max": seq_output.summary_mpa.max,
                "p95": seq_output.summary_mpa.p95,
            },
            "mm": {
                "mean": seq_output.summary_mm.mean,
                "max": seq_output.summary_mm.max,
                "p95": seq_output.summary_mm.p95,
            },
            "sc": {
                "mean": seq_output.summary_sc.mean,
                "max": seq_output.summary_sc.max,
                "p95": seq_output.summary_sc.p95,
            },
        }
    })
}

// ══ Experimental type-rule segmentation (--exp) ══════════════════════════════
// Prototype-only implementation of the asymmetric type-boundary rule agreed on
// 2026-08-09 (wiki [[rhythm-segmentation]]). Production rhythm_segmentation.rs
// is NOT touched until this has been cross-checked against the in-game editor.
//
// Rules (boundary OR'd with gap + R/T discontinuity, unchanged from production):
//   • slider→circle change             → always a boundary
//   • circle→slider change             → boundary iff the following slider run
//                                         has ≥2 sliders; a lone slider engulfs
//                                         into the preceding circle group iff
//                                         ≤ 2× circle diameter away (CS-scaled;
//                                         2026-08-10: 25px was too strict —
//                                         trailing slider heads measured 38.9–
//                                         116px in vs ≥185px out)
//   • R/T signal skipped at slider→circle transition windows (2026-08-10:
//     its boundary lands one note inside the circle run — issues A/B)
//   • spinner/hold adjacency           → boundary
//   • pure slider runs ≥2              → "SliderChain" (no longer "Stream")
//   • R_THRESHOLD 0.35 (2026-08-11, [[experiment-protocol]] #1): threshold
//     lowered 0.5 → 0.35 to catch 1/4→1/3 (R = 0.415) while staying above
//     jitter noise ~0.29. Pivot rule (speed-up → boundary before the pivot)
//     abandoned 2026-08-11 — boundary placement is always AFTER the pivot (k+1).

/// Engulf proximity = 2× the circle diameter — one "note's worth" of space.
const ENGULF_DIAMETERS: f64 = 2.0;

/// Prototype threshold (2026-08-11: 0.5 → 0.35 — catches 1/4→1/3 = 0.415,
/// stays above jitter noise ~0.29). Production `R_THRESHOLD` now matches
/// (ported 2026-08-11, [[segmentation-unification]]).
const DISCONTINUITY_THRESHOLD: f64 = 0.35;

/// Object kind for the type-boundary rules (anything not circle/slider = Other).
#[derive(Clone, Copy, PartialEq, Debug)]
enum ObjKind {
    Circle,
    Slider,
    Other,
}

fn obj_kind(o: &HitObject) -> ObjKind {
    if o.is_circle() {
        ObjKind::Circle
    } else if o.is_slider() {
        ObjKind::Slider
    } else {
        ObjKind::Other
    }
}

/// Consecutive sliders starting at index `i`.
fn slider_run_from(kinds: &[ObjKind], i: usize) -> usize {
    let mut n = 0;
    while i + n < kinds.len() && kinds[i + n] == ObjKind::Slider {
        n += 1;
    }
    n
}

/// Euclidean distance between objects i and i+1.
fn obj_distance(a: &HitObject, b: &HitObject) -> f64 {
    let dx = (b.pos.x - a.pos.x) as f64;
    let dy = (b.pos.y - a.pos.y) as f64;
    (dx * dx + dy * dy).sqrt()
}

/// Skip the discontinuity signal (R/T) when the window (k-1, k, k+1) straddles
/// a slider→circle type transition — there R's boundary would land one note
/// inside the circle run (974/792 cases), while the type rule already bounds
/// the run at the transition itself. Asymmetric: circle→slider windows keep
/// the signal (that's what isolates the transition slider from a following
/// chain, e.g. 964).
fn skip_discontinuity(kinds: &[ObjKind], k: usize) -> bool {
    (kinds[k - 1] == ObjKind::Slider && kinds[k] == ObjKind::Circle)
        || (kinds[k] == ObjKind::Slider && kinds[k + 1] == ObjKind::Circle)
}

/// Apply the asymmetric type-boundary rules on top of gap/discontinuity
/// boundaries. `dists[i]` = distance between objects i and i+1; `engulf_px` =
/// proximity within which a lone slider merges into the preceding circle group.
fn apply_type_rules(kinds: &[ObjKind], dists: &[f64], engulf_px: f64, is_boundary: &mut [bool]) {
    for i in 0..kinds.len().saturating_sub(1) {
        if kinds[i] == ObjKind::Other || kinds[i + 1] == ObjKind::Other {
            is_boundary[i + 1] = true; // never merge across a spinner/hold
            continue;
        }
        match (kinds[i], kinds[i + 1]) {
            (ObjKind::Slider, ObjKind::Circle) => is_boundary[i + 1] = true,
            (ObjKind::Circle, ObjKind::Slider) => {
                if slider_run_from(kinds, i + 1) >= 2 || dists[i] > engulf_px {
                    is_boundary[i + 1] = true;
                }
                // else: lone near slider engulfs into the circle group
            }
            _ => {} // same-kind runs stay continuous
        }
    }
}

/// Classify a pattern by composition (not count only — fixes "slider runs
/// ≥7 → Stream"). The engulfed slider head counts as the nth note of the
/// Burst(n); no mixed label.
fn classify(kinds: &[ObjKind], range: &Range<usize>) -> String {
    let all_sliders = kinds[range.clone()].iter().all(|k| *k == ObjKind::Slider);
    if range.len() >= 2 && all_sliders {
        "SliderChain".to_string()
    } else if range.len() >= 7 {
        "Stream".to_string()
    } else if range.len() >= 2 {
        format!("{}n Burst", range.len())
    } else if kinds[range.start] == ObjKind::Slider {
        "Slider".to_string()
    } else {
        "Jump".to_string()
    }
}

/// Group by boundaries and classify; returns (label, index range) pairs.
fn group_and_classify(kinds: &[ObjKind], is_boundary: &[bool]) -> Vec<(String, Range<usize>)> {
    let mut out = Vec::new();
    let mut start = 0;
    for i in 1..=kinds.len() {
        if i == kinds.len() || is_boundary[i] {
            let range = start..i;
            if !range.is_empty() {
                out.push((classify(kinds, &range), range));
            }
            start = i;
        }
    }
    out
}

/// Snap label for a pattern — mirrors production `group_into_patterns`.
fn exp_snap(times: &[f64], timings: &[TimingPoint], range: &Range<usize>) -> String {
    let (delta, at_time) = if range.len() >= 2 {
        let avg = times[range.clone()].windows(2).map(|w| w[1] - w[0]).sum::<f64>()
            / (range.len() - 1) as f64;
        (avg, times[range.start])
    } else if range.end < times.len() {
        (times[range.end] - times[range.start], times[range.end])
    } else {
        return "End".to_string();
    };
    let beat_len = timings
        .iter()
        .rev()
        .find(|tp| tp.time <= at_time)
        .map_or(500.0, |tp| tp.beat_len);
    snap_filter::identify_snap(delta, beat_len).unwrap_or_else(|| "Unstable".to_string())
}

/// Parse new-combo bits straight from the raw .osu `[HitObjects]` section —
/// rosu_pp's `HitObject` model drops the bit (NC = 0x4 in the type field).
fn parse_new_combos(path: &str) -> Option<Vec<bool>> {
    let text = std::fs::read_to_string(path).ok()?;
    let section = text.split("[HitObjects]").nth(1)?.split('\n');
    let mut bits = Vec::new();
    for line in section {
        let line = line.trim();
        if line.starts_with('[') {
            break; // next section
        }
        if line.is_empty() {
            continue; // blank line right after the section header
        }
        let mut f = line.split(',');
        f.next()?; // x
        f.next()?; // y
        f.next()?; // time
        let type_field: u32 = f.next()?.trim().parse().ok()?;
        bits.push(type_field & 0x4 != 0);
    }
    Some(bits)
}

/// One experimental pattern: label, timing, snap. NC reference columns are
/// added at dump time from the raw-file bit array (see `pattern_row`).
struct ExpPattern {
    p_type: String,
    time: f64,
    range: Range<usize>,
    snap: String,
}

/// Experimental segmentation: gap + discontinuity signal (R/T) + asymmetric
/// type-boundary rules.
fn exp_segment(map: &Beatmap, use_temporal: bool) -> Vec<ExpPattern> {
    let objects = &map.hit_objects;
    let timings = &map.timing_points;
    let n = objects.len();

    if n == 0 {
        return Vec::new();
    }

    let times: Vec<f64> = objects.iter().map(|o| o.start_time).collect();
    let kinds: Vec<ObjKind> = objects.iter().map(obj_kind).collect();

    let mut is_boundary = vec![false; n];
    is_boundary[0] = true;

    // Gap threshold: ½ beat of the active timing point at the second note
    for i in 0..n.saturating_sub(1) {
        let gap = times[i + 1] - times[i];
        let beat_len = snap_filter::timing_point_at(timings, times[i + 1])
            .map_or(500.0, |tp| tp.beat_len);
        if gap > beat_len / 2.0 + 10.0 {
            is_boundary[i + 1] = true;
        }
    }

    // Discontinuity signal (same as production: T with --T, else R) — skipped
    // at slider→circle transition windows (2026-08-10: R boundary lands one
    // note inside the circle run there)
    if use_temporal {
        for k in 1..n.saturating_sub(1) {
            if skip_discontinuity(&kinds, k) {
                continue;
            }
            let dt1 = times[k] - times[k - 1];
            let dt2 = times[k + 1] - times[k];
            let t = if dt1 > 0.0 { (dt2 / dt1).log2().abs() } else { 0.0 };
            if t.is_finite() && t > DISCONTINUITY_THRESHOLD {
                let boundary_idx = k + 1; // always after the pivot (pivot rule abandoned 2026-08-11)
                if boundary_idx < n {
                    is_boundary[boundary_idx] = true;
                }
            }
        }
    } else {
        let snaps = snap_filter::note_pair_snaps(&times, timings);
        for k in 1..snaps.len() {
            if skip_discontinuity(&kinds, k) {
                continue;
            }
            let r = if snaps[k - 1].1 > 0.0 {
                (snaps[k].1 / snaps[k - 1].1).log2().abs()
            } else {
                0.0
            };
            if r.is_finite() && r > DISCONTINUITY_THRESHOLD {
                let boundary_idx = k + 1; // always after the pivot (pivot rule abandoned 2026-08-11)
                if boundary_idx < n {
                    is_boundary[boundary_idx] = true;
                }
            }
        }
    }

    // Asymmetric type-boundary rules
    let dists: Vec<f64> = (0..n.saturating_sub(1))
        .map(|i| obj_distance(&objects[i], &objects[i + 1]))
        .collect();
    let engulf_px = (54.4 - 4.48 * map.cs as f64) * 2.0 * ENGULF_DIAMETERS;
    apply_type_rules(&kinds, &dists, engulf_px, &mut is_boundary);

    group_and_classify(&kinds, &is_boundary)
        .into_iter()
        .map(|(p_type, range)| ExpPattern {
            p_type,
            time: objects[range.start].start_time,
            range: range.clone(),
            snap: exp_snap(&times, timings, &range),
        })
        .collect()
}

/// One row of the --patterns dump (used by both segmentation modes).
fn pattern_row(
    p_type: &str,
    time: f64,
    range: &Range<usize>,
    snap: &str,
    nc: Option<&[bool]>,
) -> serde_json::Value {
    let (new_combo, mid_combo_breaks) = match nc {
        Some(bits) => (
            serde_json::json!(bits[range.start]),
            serde_json::json!(bits[range.start + 1..range.end].iter().filter(|b| **b).count()),
        ),
        None => (serde_json::Value::Null, serde_json::Value::Null),
    };
    serde_json::json!({
        "time_ms": time,
        "type": p_type,
        "notes": range.len(),
        "range": format!("{}..{}", range.start, range.end),
        "snap": snap,
        "new_combo": new_combo,
        "mid_combo_breaks": mid_combo_breaks,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse flags
    let use_temporal = args.iter().any(|a| a == "--T");
    let use_rhythm = args.iter().any(|a| a == "--R");
    let dump_json = args.iter().any(|a| a == "--json");
    let dump_patterns = args.iter().any(|a| a == "--patterns");
    let use_exp = args.iter().any(|a| a == "--exp");

    if use_temporal && use_rhythm {
        eprintln!("Error: specify only one of --T (temporal) or --R (rhythm)");
        process::exit(1);
    }

    let segmentation_mode = if use_temporal {
        "Temporal (T)"
    } else {
        "Rhythm (R)"
    };

    // Get file path (first positional argument after binary name)
    let file_path = match args.iter().skip(1).find(|s| !s.starts_with("--")) {
        Some(s) => s.clone(),
        None => {
            eprintln!("Usage: cargo run --bin prototype_sequence_motor -- <path/to/file.osu> [--json | --patterns] [--T | --R]");
            process::exit(1);
        }
    };

    let map = load_beatmap(&file_path);
    let cs = map.cs as f64;
    let circle_diameter = (54.4 - 4.48 * cs) * 2.0;

    let visual_nodes = visuals::extract_visual_nodes(&map);

    // ── New-combo reference bits ──────────────────────────────────────────
    // rosu_pp drops the bit, so parse the raw .osu. Reference only — never a
    // boundary signal (decision 2026-08-10).
    let nc: Option<Vec<bool>> = match parse_new_combos(&file_path) {
        Some(bits) if bits.len() == map.hit_objects.len() => Some(bits),
        Some(bits) => {
            eprintln!(
                "Warning: new-combo bits ({}) != parsed objects ({}) — NC columns omitted",
                bits.len(),
                map.hit_objects.len()
            );
            None
        }
        None => {
            eprintln!("Warning: could not parse new-combo bits from .osu — NC columns omitted");
            None
        }
    };
    let nc_ref = nc.as_deref();

    // ── Segmentation ───────────────────────────────────────────────────────
    let exp_patterns: Vec<ExpPattern>;
    let raw_patterns: Vec<(Pattern, Range<usize>)>;
    if use_exp {
        exp_patterns = exp_segment(&map, use_temporal);
        raw_patterns = Vec::new();
    } else {
        exp_patterns = Vec::new();
        raw_patterns = if use_temporal {
            rhythm_segmentation::extract_pattern_indices_temporal(&map)
        } else {
            rhythm_segmentation::extract_pattern_indices(&map) // default: R
        };
    }

    let pattern_count = if use_exp { exp_patterns.len() } else { raw_patterns.len() };
    let total_notes = visual_nodes.len();

    // Handle --patterns flag: dump all patterns with precise MS timing
    if dump_patterns {
        let mut p_list = Vec::new();
        if use_exp {
            for p in &exp_patterns {
                p_list.push(pattern_row(&p.p_type, p.time, &p.range, &p.snap, nc_ref));
            }
        } else {
            for (pat, range) in &raw_patterns {
                p_list.push(pattern_row(&pat.p_type.as_str(), pat.time, range, &pat.snap, nc_ref));
            }
        }
        let out = serde_json::json!({
            "mode": segmentation_mode,
            "pattern_count": pattern_count,
            "patterns": p_list,
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
        return;
    }

    // Convert to pattern_refs for sequence_motor
    let mut multi_note_count = 0;
    let pattern_data: Vec<(std::ops::Range<usize>, String)> = if use_exp {
        exp_patterns
            .iter()
            .map(|p| {
                if p.range.len() >= 2 {
                    multi_note_count += 1;
                }
                (p.range.clone(), p.p_type.clone())
            })
            .collect()
    } else {
        raw_patterns
            .into_iter()
            .map(|(pattern, range)| {
                if range.len() >= 2 {
                    multi_note_count += 1;
                }
                (range, pattern.p_type.as_str())
            })
            .collect()
    };
    let pattern_refs: Vec<(std::ops::Range<usize>, &str)> = pattern_data
        .iter()
        .map(|(r, s)| (r.clone(), s.as_str()))
        .collect();

    let seq_output = sequence_motor::analyze_patterns(&visual_nodes, &pattern_refs, circle_diameter);

    if dump_json {
        println!("{}", serde_json::to_string_pretty(&build_seq_json(&seq_output)).unwrap());
        return;
    }

    // ── TUI Output ─────────────────────────────────────────────────────────

    let file_stem = Path::new(&file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("???");

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  PROTOTYPE: Sequence Motor Descriptors                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("  Map:   {}", file_stem);
    println!("  CS:    {:.1}", cs);
    println!("  Mode:  {} discontinuity", segmentation_mode);
    if use_exp {
        println!("  Type:  --exp experimental asymmetric type-boundary rule");
    }
    println!();

    // Pattern boundary summary
    println!("  ── Pattern Summary ──────────────────────────────────────────");
    println!("  Total patterns:   {}  (from {} notes)", pattern_count, total_notes);
    println!("  Multi-note only:  {}  (fed to sequence motor)", multi_note_count);
    println!();

    // Timeline table
    let timeline = &seq_output.timeline;
    if timeline.is_empty() {
        println!("  (no multi-note patterns found — timeline empty)");
    } else {
        println!("  {:<8} {:>6} {:>10} {:>10} {:>10}", "Time", "Notes", "MPA", "MM", "SC");
        println!("  {:-<8} {:->6} {:->10} {:->10} {:->10}", "", "", "", "", "");
        for entry in timeline {
            println!(
                "  {:<8} {:>6} {:>10.4} {:>10.4} {:>10.4}",
                format_time(entry.time_ms),
                entry.note_count,
                entry.mpa,
                entry.mm,
                entry.sc,
            );
        }
    }

    // Summary
    println!();
    println!("  ── Summary ──────────────────────────────────────────────────");
    println!("  {:<12} {:>12} {:>12} {:>12}", "Metric", "Mean", "Max", "P95");
    println!("  {:-<12} {:->12} {:->12} {:->12}", "", "", "", "");

    for (label, s) in [
        ("MPA", &seq_output.summary_mpa),
        ("MM",  &seq_output.summary_mm),
        ("SC",  &seq_output.summary_sc),
    ] {
        println!(
            "  {:<12} {:>12.4} {:>12.4} {:>12.4}",
            label, s.mean, s.max, s.p95,
        );
    }
    println!();
    println!("  N patterns (≥2 notes): {}", timeline.len());
    println!();
}

// ── Tests ────────────────────────────────────────────────────────────────────
// Run with: cargo test --bin prototype_sequence_motor
#[cfg(test)]
mod tests {
    use super::*;

    /// 'c' → circle, 's' → slider; any other char → Other.
    fn kinds_from(desc: &[char]) -> Vec<ObjKind> {
        desc.iter()
            .map(|c| match c {
                'c' => ObjKind::Circle,
                's' => ObjKind::Slider,
                _ => ObjKind::Other,
            })
            .collect()
    }

    /// Segment with only the type rules active (no gap/discontinuity signal),
    /// then group + classify. `dists[i]` = distance between objects i and i+1.
    fn segment(desc: &[char], dists: &[f64], engulf_px: f64) -> Vec<(String, Range<usize>)> {
        let kinds = kinds_from(desc);
        let mut is_boundary = vec![false; kinds.len()];
        is_boundary[0] = true;
        apply_type_rules(&kinds, dists, engulf_px, &mut is_boundary);
        group_and_classify(&kinds, &is_boundary)
    }

    #[test]
    fn pure_slider_chain_is_slider_chain() {
        // 8 consecutive sliders — must NOT be "Stream" (the original bug)
        let pats = segment(&['s'; 8], &[10.0; 7], 146.0);
        assert_eq!(pats, vec![("SliderChain".to_string(), 0..8)]);
    }

    #[test]
    fn lone_slider_engulfs_into_circle_group() {
        // c,c,c,s,c,c with the slider within proximity → [c,c,c,s] + [c,c]
        let pats = segment(&['c', 'c', 'c', 's', 'c', 'c'], &[10.0; 5], 146.0);
        assert_eq!(
            pats,
            vec![("4n Burst".to_string(), 0..4), ("2n Burst".to_string(), 4..6)]
        );
    }

    #[test]
    fn trailing_slider_engulfs_within_two_diameters() {
        // 2026-08-10: stream trailing sliders measure 38.9–116px — a 100px head
        // engulfs at 2× diameter (146px at CS4) but would not at the old 25px
        let pats = segment(&['c', 'c', 'c', 's', 'c', 'c'], &[10.0, 10.0, 100.0, 10.0, 10.0], 146.0);
        assert_eq!(
            pats,
            vec![("4n Burst".to_string(), 0..4), ("2n Burst".to_string(), 4..6)]
        );
    }

    #[test]
    fn far_slider_does_not_engulf() {
        // same shape, slider beyond proximity → its own "Slider" pattern
        let pats = segment(&['c', 'c', 'c', 's', 'c', 'c'], &[10.0, 10.0, 40.0, 10.0, 10.0], 25.0);
        assert_eq!(
            pats,
            vec![
                ("3n Burst".to_string(), 0..3),
                ("Slider".to_string(), 3..4),
                ("2n Burst".to_string(), 4..6),
            ]
        );
    }

    #[test]
    fn alternation_yields_doubles() {
        // c,s,c,s,c,s → three [c,s] doubles (no annihilation)
        let pats = segment(&['c', 's', 'c', 's', 'c', 's'], &[10.0; 5], 146.0);
        assert_eq!(
            pats,
            vec![
                ("2n Burst".to_string(), 0..2),
                ("2n Burst".to_string(), 2..4),
                ("2n Burst".to_string(), 4..6),
            ]
        );
    }

    #[test]
    fn slider_to_circle_always_splits() {
        // the 9s+1c case: 10 sliders + 8 circles split at the type change
        let desc: Vec<char> = vec!['s'; 10].into_iter().chain(vec!['c'; 8]).collect();
        let pats = segment(&desc, &[10.0; 17], 146.0);
        assert_eq!(
            pats,
            vec![
                ("SliderChain".to_string(), 0..10),
                ("Stream".to_string(), 10..18),
            ]
        );
    }

    #[test]
    fn slider_run_of_two_splits() {
        // c,c,s,s,c → [2n Burst][SliderChain][Jump]
        let pats = segment(&['c', 'c', 's', 's', 'c'], &[10.0; 4], 146.0);
        assert_eq!(
            pats,
            vec![
                ("2n Burst".to_string(), 0..2),
                ("SliderChain".to_string(), 2..4),
                ("Jump".to_string(), 4..5),
            ]
        );
    }

    // ── skip_discontinuity (R/T suppressed at slider→circle windows) ────────

    #[test]
    fn discontinuity_skipped_at_slider_to_circle_first_pair() {
        // s→c at the window's first pair — R would land a boundary one note
        // inside the circle run (the 974 / 792 cases)
        let kinds = kinds_from(&['s', 'c', 'c']);
        assert!(skip_discontinuity(&kinds, 1));
    }

    #[test]
    fn discontinuity_skipped_at_slider_to_circle_second_pair() {
        // s→c at the window's second pair — same one-note-in fragmentation
        let kinds = kinds_from(&['s', 's', 'c']);
        assert!(skip_discontinuity(&kinds, 1));
    }

    #[test]
    fn discontinuity_kept_at_circle_to_slider_window() {
        // c→s keeps the signal — isolates the transition slider (issue D: 964)
        let kinds = kinds_from(&['c', 'c', 's']);
        assert!(!skip_discontinuity(&kinds, 1));
    }

    #[test]
    fn discontinuity_kept_for_pure_circles() {
        // no type change in the window — R still fires (02:08:539 stays split)
        let kinds = kinds_from(&['c', 'c', 'c']);
        assert!(!skip_discontinuity(&kinds, 1));
    }

}
