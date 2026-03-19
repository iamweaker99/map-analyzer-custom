pub mod visuals;
pub mod density;
pub mod trajectory;
pub mod traps;
pub mod strain;

use rosu_pp::Beatmap;
use serde_json::{json, Value};

pub fn analyze(map: &Beatmap) -> Value {
    // 1. EXTRACTION
    let visual_nodes = visuals::extract_visual_nodes(map);
    if visual_nodes.is_empty() {
        return json!({ "error": "Not enough objects for reading analysis" });
    }

    // 2. CORE ENGINES
    let density_states = density::calculate_density(&visual_nodes);
    let trajectory_states = trajectory::calculate_trajectory(&visual_nodes);
    let trap_states = traps::calculate_traps(&visual_nodes);

    // 3. SYSTEM SIMULATION (Strain & K-Lines)
    let (_strain_points, klines) = strain::calculate_strain_and_klines(
        &visual_nodes, 
        &density_states, 
        &trajectory_states, 
        &trap_states
    );

    // 4. AGGREGATION (Distributions)
    let total_nodes = visual_nodes.len() as f64;
    let total_traj = trajectory_states.len().max(1) as f64;

    // Density Distribution
    let mut d_isolated = 0; let mut d_chunking = 0; let mut d_clutter = 0; let mut d_overload = 0;
    for d in &density_states {
        match d.concurrent_objects {
            0..=2 => d_isolated += 1,
            3..=5 => d_chunking += 1,
            6..=8 => d_clutter += 1,
            _ => d_overload += 1,
        }
    }

    // Trajectory Distribution
    let mut t_linear = 0; let mut t_mild = 0; let mut t_kinks = 0; let mut t_spaghetti = 0;
    for t in &trajectory_states {
        if t.is_overlapping {
            t_spaghetti += 1;
        } else if t.entropy < 30.0 {
            t_linear += 1;
        } else if t.entropy < 90.0 {
            t_mild += 1;
        } else {
            t_kinks += 1;
        }
    }

    // Traps
    let total_decel_traps = trap_states.iter().filter(|t| t.is_deceleration_trap).count();
    let mut sorted_traps = trap_states.clone();
    sorted_traps.sort_by(|a, b| b.magnitude.partial_cmp(&a.magnitude).unwrap());
    
    let top_traps = sorted_traps.iter().take(5).map(|t| json!({
        "time": t.time,
        "magnitude": t.magnitude
    })).collect::<Vec<_>>();

    // Normalize: Traps per 1000 objects
    let trap_index = (trap_states.len() as f64 / total_nodes) * 1000.0;

    // 5. SERIALIZATION
    json!({
        "summary": {
            "peak_strain": klines.iter().map(|k| k.high).fold(0.0, f64::max),
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
        "traps": {
            "total_deceleration_traps": total_decel_traps,
            "trap_index": trap_index, // "Story" metric: Traps per 1k notes
            "peak_magnitude": sorted_traps.first().map(|t| t.magnitude).unwrap_or(0.0),
            "notable_traps": top_traps // Localized data
        },
        "topography": {
            "klines": klines
        }
    })
}