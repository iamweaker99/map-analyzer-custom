//! PROTOTYPE — Spacing Transition Demand (TV2 vs LTD) on real beatmaps
//!
//! Loads two .osu beatmaps, runs both candidate families on every Burst(2/3/4)
//! pattern, prints top-10 highest-scoring patterns per method per map.
//! Excludes patterns at full-beat (1/1) or half-beat (1/2) snap.
//!
//! Usage:
//!   cargo run --bin prototype_spacing_demand

use std::ops::Range;
use std::path::Path;

use backend::analysis::finger_control::rhythm_segmentation::PatternType;
use backend::analysis::finger_control::rhythm_segmentation;
use backend::analysis::get_diameter;

// ── Beatmap loading ─────────────────────────────────────────────────────────

fn load_beatmap(path: &str) -> Result<rosu_pp::Beatmap, String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }
    rosu_pp::Beatmap::from_path(path).map_err(|e| format!("Failed to parse .osu: {}", e))
}

// ── Filtering ───────────────────────────────────────────────────────────────

fn is_target_pattern(p_type: &PatternType) -> bool {
    matches!(p_type, PatternType::Burst(2..=4))
}

/// Exclude full-beat and half-beat snap patterns.
fn is_excluded_snap(snap: &str) -> bool {
    snap == "1/1" || snap == "1/2"
}

// ── Spacing extraction ──────────────────────────────────────────────────────

fn extract_spacings(map: &rosu_pp::Beatmap, range: &Range<usize>) -> Option<Vec<f64>> {
    let d = get_diameter(map.cs);
    let objs = &map.hit_objects;
    let slice = &objs[range.start..range.end];

    if slice.len() < 2 {
        return None;
    }

    let spacings: Vec<f64> = slice
        .windows(2)
        .map(|w| {
            let dx = (w[1].pos.x - w[0].pos.x) as f64;
            let dy = (w[1].pos.y - w[0].pos.y) as f64;
            (dx * dx + dy * dy).sqrt() / d
        })
        .collect();

    Some(spacings)
}

// ── Metric computations ─────────────────────────────────────────────────────

struct Metrics {
    tv2: Option<f64>,
    ltd: Option<f64>,
}

fn compute_metrics(spacings: &[f64]) -> Metrics {
    let m = spacings.len();
    if m < 3 {
        return Metrics { tv2: None, ltd: None };
    }

    let mut sum_abs = 0.0_f64;
    let mut sum_sq = 0.0_f64;
    let k = (m - 2) as f64;

    for i in 0..m - 2 {
        let d2 = spacings[i + 2] - 2.0 * spacings[i + 1] + spacings[i];
        sum_abs += d2.abs();
        sum_sq += d2 * d2;
    }

    Metrics { tv2: Some(sum_abs), ltd: Some(sum_sq / k) }
}

// ── Format helpers ──────────────────────────────────────────────────────────

fn fmt_timestamp(ms: f64) -> String {
    let total_secs = (ms / 1000.0).round() as u64;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("[{:02}:{:02}]", mins, secs)
}

fn fmt_pattern_type(pt: &PatternType) -> String {
    match pt {
        PatternType::Burst(n) => format!("{}n Burst", n),
        _ => pt.as_str(),
    }
}

fn fmt_score(val: Option<f64>) -> String {
    match val {
        Some(v) => format!("{:.3}", v),
        None => "  N/A  ".to_string(),
    }
}

// ── Analyse one map ─────────────────────────────────────────────────────────

struct PatternResult {
    time_ms: f64,
    ptype: PatternType,
    snap: String,
    spacings: Vec<f64>,
    tv2: Option<f64>,
    ltd: Option<f64>,
}

fn analyse_map(map: &rosu_pp::Beatmap, exclude_snaps: bool) -> Vec<PatternResult> {
    let patterns = rhythm_segmentation::extract_pattern_indices(map);
    let mut results = Vec::new();

    for (pat, range) in &patterns {
        if !is_target_pattern(&pat.p_type) {
            continue;
        }
        if exclude_snaps && is_excluded_snap(&pat.snap) {
            continue;
        }

        if let Some(spacings) = extract_spacings(map, range) {
            let m = compute_metrics(&spacings);
            if m.tv2.is_none() && m.ltd.is_none() {
                continue;
            }
            results.push(PatternResult {
                time_ms: pat.time,
                ptype: pat.p_type.clone(),
                snap: pat.snap.clone(),
                spacings,
                tv2: m.tv2,
                ltd: m.ltd,
            });
        }
    }

    results
}

// ── Print a combined table (both scores per pattern), sorted by primary key ──

fn print_combined_table(
    title: &str,
    results: &[PatternResult],
    count: usize,
) {
    // Build scored list, sorting by TV2 descending
    let mut scored: Vec<_> = results
        .iter()
        .filter_map(|r| r.tv2.map(|s| (s, r)))
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    println!("{}", title);
    println!();

    if scored.is_empty() {
        println!("  (no patterns match the filter)");
        println!();
        return;
    }

    let limit = scored.len().min(count);

    // Header
    println!(
        "  {:>3}  {:>8}  {:>8}  {:>12}  {:>6}  {:>20}",
        "#", "TV2", "LTD(be)", "Time", "Snap", "Spacings (\u{00d7} D)"
    );
    println!("  {}", "-".repeat(85));

    for (rank, (_tv2_score, r)) in scored.iter().enumerate().take(limit) {
        let spacings_str = r
            .spacings
            .iter()
            .map(|s| format!("{:.2}", s))
            .collect::<Vec<_>>()
            .join(", ");

        println!(
            "  {:>3}  {:>8.3}  {:>8}  {:>8}  {:>6}  [{:20}]",
            rank + 1,
            r.tv2.unwrap_or(0.0),
            fmt_score(r.ltd),
            fmt_timestamp(r.time_ms),
            r.snap,
            spacings_str,
        );
    }

    if scored.len() > limit {
        println!("  ... and {} more patterns (total found: {})", scored.len() - limit, scored.len());
    }
    println!();

    // Also print top-10 by LTD separately
    let mut scored_ltd: Vec<_> = results
        .iter()
        .filter_map(|r| r.ltd.map(|s| (s, r)))
        .collect();
    scored_ltd.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    println!("  Top 10 by LTD (bending energy):");
    println!(
        "  {:>3}  {:>8}  {:>8}  {:>12}  {:>6}",
        "#", "LTD(be)", "TV2", "Time", "Snap"
    );
    println!("  {}", "-".repeat(55));
    for (rank, (score, r)) in scored_ltd.iter().enumerate().take(10.min(scored_ltd.len())) {
        println!(
            "  {:>3}  {:>8.3}  {:>8.3}  {:>8}  {:>6}",
            rank + 1,
            score,
            r.tv2.unwrap_or(0.0),
            fmt_timestamp(r.time_ms),
            r.snap,
        );
    }
    if scored_ltd.len() > 10 {
        println!("  ... and {} more patterns", scored_ltd.len() - 10);
    }
    println!();
}

// ── Main ────────────────────────────────────────────────────────────────────

fn main() {
    let maps = [
        (
            r"D:\osu files\AngelMaker - A Dark Omen (Kyu96) [Demonic Colossus].osu",
            "AngelMaker — A Dark Omen [Demonic Colossus]",
        ),
        (
            r"D:\osu files\YOASOBI - Yoru ni Kakeru (CoLouRed GlaZeE) [Collab Extra].osu",
            "YOASOBI — Yoru ni Kakeru [Collab Extra]",
        ),
    ];

    let filter_note = "  (excluded 1/1 and 1/2 snap patterns)";

    for (path, label) in &maps {
        let map = match load_beatmap(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("  ERROR: {}", e);
                continue;
            }
        };

        let results = analyse_map(&map, true); // exclude 1/1 and 1/2

        let d = get_diameter(map.cs);
        println!("══════════════════════════════════════════════════════════════");
        println!("  Map: {}", label);
        println!("  CS={:.1}, D={:.2}px, BPM={:.1}  {}", map.cs, d, map.bpm(), filter_note);
        println!(
            "  Burst(2/3/4) after filter: {}",
            results.len()
        );
        println!();

        if results.is_empty() {
            println!("  (no patterns after excluding 1/1 and 1/2 snap)");
            println!();
            continue;
        }

        // Combined table — sorted by TV2, top 10
        print_combined_table(
            &format!("Combined table — sorted by TV2 (top {})", 10.min(results.len())),
            &results,
            10,
        );
    }
}
