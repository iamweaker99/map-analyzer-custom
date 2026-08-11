//! CANARY — production segmentation dump for the user-verified maps
//!
//! Dumps `rhythm_segmentation::extract_pattern_indices` (R mode) output in
//! the same JSON shape as the prototype's `--patterns --R --exp` dump, for
//! row-by-row comparison against the user-verified reference JSONs
//! (`Prototyping/rerun_{yoasobi,feral}_thr035.json`) via
//! `Temp/compare_patterns.py`.
//!
//! Usage:
//!   cargo run --bin canary_segmentation -- "path/to/map.osu" [--out out.json]
//!   cargo run --bin canary_segmentation -- "path/to/map.osu" --out Temp/canary_yoasobi.json
//!
//! Kept (2026-08-11): this is the canary gate harness for the production port
//! ([[segmentation-unification]]) — re-run it whenever the segmentation
//! semantics change.

use std::ops::Range;
use std::path::Path;
use std::process;

use backend::analysis::finger_control::rhythm_segmentation;

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

/// Parse new-combo bits straight from the raw .osu `[HitObjects]` section —
/// rosu_pp's `HitObject` model drops the bit (NC = 0x4 in the type field).
/// Mirrors `prototype_sequence_motor::parse_new_combos`.
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

/// One row of the --patterns dump — same shape as the prototype's
/// `pattern_row` (mode-independent; `new_combo`/`mid_combo_breaks` are
/// reference columns from the raw .osu, never boundary signals).
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

    let file_path = match args.iter().skip(1).find(|s| !s.starts_with("--")) {
        Some(s) => s.clone(),
        None => {
            eprintln!("Usage: cargo run --bin canary_segmentation -- <path/to/file.osu> [--out out.json]");
            process::exit(1);
        }
    };
    let out_path = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let map = load_beatmap(&file_path);

    // New-combo reference bits (raw .osu parse). Reference only — never a
    // boundary signal (decision 2026-08-10).
    let nc: Option<Vec<bool>> = match parse_new_combos(&file_path) {
        Some(bits) if bits.len() == map.hit_objects.len() => Some(bits),
        _ => None,
    };
    let nc_ref = nc.as_deref();

    let patterns = rhythm_segmentation::extract_pattern_indices(&map);
    let p_list: Vec<serde_json::Value> = patterns
        .iter()
        .map(|(pat, range)| pattern_row(&pat.p_type.as_str(), pat.time, range, &pat.snap, nc_ref))
        .collect();

    let out = serde_json::json!({
        "mode": "Rhythm (R)",
        "pattern_count": p_list.len(),
        "patterns": p_list,
    });
    let text = serde_json::to_string_pretty(&out).unwrap();

    match out_path {
        Some(p) => std::fs::write(&p, text).unwrap(),
        None => println!("{}", text),
    }
}
