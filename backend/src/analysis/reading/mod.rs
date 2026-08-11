pub mod visuals;
pub mod density;
pub mod trajectory;
pub mod traps;
pub mod strain;
pub mod sequence_motor;

use rosu_pp::Beatmap;
use serde_json::{json, Value};

pub fn analyze(map: &Beatmap) -> Value {
    let cs = map.cs as f64;
    let circle_radius = 54.4 - 4.48 * cs;
    let circle_diameter = circle_radius * 2.0;
    let bpm = map.bpm();

    let visual_nodes = visuals::extract_visual_nodes(map);
    if visual_nodes.is_empty() {
        return json!({ "error": "Not enough objects for reading analysis" });
    }

    let density_states = density::calculate_density(&visual_nodes, circle_diameter);
    // Trajectory now requires density_states to know local cluster size
    let trajectory_states = trajectory::calculate_trajectory(&visual_nodes, &density_states, circle_diameter);
    let trap_states = traps::calculate_traps(&visual_nodes, bpm);

    let (_strain_points, klines) = strain::calculate_strain_and_klines(
        &visual_nodes,
        &density_states,
        &trajectory_states,
        &trap_states
    );

    // ── Sequence Motor Descriptors (per-pattern) ──
    let pattern_data: Vec<(std::ops::Range<usize>, String)> = super::finger_control::rhythm_segmentation::extract_pattern_indices(map)
        .into_iter()
        .map(|(pattern, range)| (range, pattern.p_type.as_str()))
        .collect();
    // Convert to (&str) borrows for the API; collect owned strings, then borrow
    let pattern_refs: Vec<(std::ops::Range<usize>, &str)> = pattern_data
        .iter()
        .map(|(r, s)| (r.clone(), s.as_str()))
        .collect();
    let seq_output = sequence_motor::analyze_patterns(&visual_nodes, &pattern_refs, circle_diameter);

    // Format timeline with MM:SS time strings
    fn format_time(ms: f64) -> String {
        let total_secs = (ms / 1000.0).round() as u64;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}:{:02}", mins, secs)
    }

    let seq_timeline: Vec<serde_json::Value> = seq_output
        .timeline
        .iter()
        .map(|p| json!({
            "time": format_time(p.time_ms),
            "notes": p.note_count,
            "mpa": p.mpa,
            "mm": p.mm,
            "sc": p.sc,
        }))
        .collect();

    let total_nodes = visual_nodes.len() as f64;
    let total_traj = trajectory_states.len().max(1) as f64;

    // Density Aggregation (Effective Objects)
    let mut d_isolated = 0; let mut d_chunking = 0; let mut d_clutter = 0; let mut d_overload = 0;
    for d in &density_states {
        match d.effective_objects.round() as usize {
            0..=2 => d_isolated += 1,
            3..=5 => d_chunking += 1,
            6..=8 => d_clutter += 1,
            _ => d_overload += 1,
        }
    }

    // Trajectory Aggregation (Spaghetti & Adaptive Entropy)
    let mut t_linear = 0; let mut t_mild = 0; let mut t_kinks = 0; let mut t_spaghetti = 0;
    for t in &trajectory_states {
        if t.is_spaghetti {
            t_spaghetti += 1;
        } else if t.entropy < 30.0 {
            t_linear += 1;
        } else if t.entropy < 90.0 {
            t_mild += 1;
        } else {
            t_kinks += 1;
        }
    }

    // Trajectory Timeline: bucket trajectory states into 5-second windows
    let window_duration = 5000.0;
    let mut trajectory_timeline: Vec<Value> = Vec::new();
    if !trajectory_states.is_empty() {
        let first_time = trajectory_states[0].time;
        let last_time = trajectory_states.last().unwrap().time;
        let start_win = (first_time / window_duration).floor() * window_duration;
        let end_win = (last_time / window_duration).ceil() * window_duration;
        let mut win_start = start_win;
        while win_start < end_win {
            let win_end = win_start + window_duration;
            let mut w_linear = 0; let mut w_mild = 0; let mut w_kinks = 0; let mut w_spaghetti = 0;
            let mut w_count = 0;
            for t in &trajectory_states {
                if t.time >= win_start && t.time < win_end {
                    w_count += 1;
                    if t.is_spaghetti {
                        w_spaghetti += 1;
                    } else if t.entropy < 30.0 {
                        w_linear += 1;
                    } else if t.entropy < 90.0 {
                        w_mild += 1;
                    } else {
                        w_kinks += 1;
                    }
                }
            }
            if w_count > 0 {
                trajectory_timeline.push(json!({"time": win_start, "linear_count": w_linear, "mild_shifts_count": w_mild, "sharp_kinks_count": w_kinks, "spaghetti_count": w_spaghetti}));
            }
            win_start = win_end;
        }
    }

    // Density Timeline: bucket density states into 5-second windows
    let mut density_timeline: Vec<Value> = Vec::new();
    if !density_states.is_empty() {
        let first_time = density_states[0].time;
        let last_time = density_states.last().unwrap().time;
        let start_win = (first_time / window_duration).floor() * window_duration;
        let end_win = (last_time / window_duration).ceil() * window_duration;
        let mut win_start = start_win;
        while win_start < end_win {
            let win_end = win_start + window_duration;
            let mut w_isolated = 0; let mut w_chunking = 0; let mut w_clutter = 0; let mut w_overload = 0;
            let mut w_count = 0;
            for d in &density_states {
                if d.time >= win_start && d.time < win_end {
                    w_count += 1;
                    match d.effective_objects.round() as usize {
                        0..=2 => w_isolated += 1,
                        3..=5 => w_chunking += 1,
                        6..=8 => w_clutter += 1,
                        _ => w_overload += 1,
                    }
                }
            }
            if w_count > 0 {
                density_timeline.push(json!({"time": win_start, "isolated_count": w_isolated, "chunking_count": w_chunking, "clutter_count": w_clutter, "overload_count": w_overload}));
            }
            win_start = win_end;
        }
    }

    let mut sorted_traps = trap_states.clone();
    sorted_traps.sort_by(|a, b| b.magnitude.partial_cmp(&a.magnitude).unwrap());
    
    let top_traps = sorted_traps.iter().take(5).map(|t| json!({
        "time": t.time,
        "magnitude": t.magnitude
    })).collect::<Vec<_>>();

    let trap_index = (trap_states.len() as f64 / total_nodes) * 1000.0;

    let mut sorted_klines = klines.clone();
    sorted_klines.sort_by(|a, b| a.high.partial_cmp(&b.high).unwrap_or(std::cmp::Ordering::Equal));
    let peak_idx = (sorted_klines.len() as f64 * 0.95).floor() as usize;
    let peak_strain = sorted_klines.get(peak_idx).map(|k| k.high).unwrap_or(0.0);

    json!({
        "summary": {
            "peak_strain": peak_strain,
            "ar_preempt_ms": visuals::ar_to_preempt(map.ar)
        },
        "density": {
            "isolated_pct": (d_isolated as f64 / total_nodes) * 100.0,
            "chunking_pct": (d_chunking as f64 / total_nodes) * 100.0,
            "clutter_pct": (d_clutter as f64 / total_nodes) * 100.0,
            "overload_pct": (d_overload as f64 / total_nodes) * 100.0,
        },
        "trajectory": {
            "linear_pct": (t_linear as f64 / total_traj) * 100.0,
            "mild_shifts_pct": (t_mild as f64 / total_traj) * 100.0,
            "sharp_kinks_pct": (t_kinks as f64 / total_traj) * 100.0,
            "spaghetti_pct": (t_spaghetti as f64 / total_traj) * 100.0,
        },
        "trajectory_timeline": trajectory_timeline,
        "density_timeline": density_timeline,
        "traps": {
            "count": trap_states.len(),
            "trap_index": trap_index,
            "peak_magnitude": sorted_traps.first().map(|t| t.magnitude).unwrap_or(0.0),
            "notable_traps": top_traps
        },
        "topography": {
            "klines": klines
        },
        "sequence_motor": {
            "timeline": seq_timeline,
            "summary": {
                "mpa": { "mean": seq_output.summary_mpa.mean, "max": seq_output.summary_mpa.max, "p95": seq_output.summary_mpa.p95 },
                "mm":  { "mean": seq_output.summary_mm.mean,  "max": seq_output.summary_mm.max,  "p95": seq_output.summary_mm.p95 },
                "sc":  { "mean": seq_output.summary_sc.mean,  "max": seq_output.summary_sc.max,  "p95": seq_output.summary_sc.p95 }
            }
        }
    })
}