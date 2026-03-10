use super::{Movement, get_diameter};
use serde_json::{json, Value};

pub fn analyze(movements: &[Movement], cs: f32, bpm: f64, total_obj: f64) -> Value {
    let d = get_diameter(cs);
    let stream_threshold = (60000.0 / bpm / 4.0) * 1.5;

    let mut s_p_stack = 0; let mut s_p_over = 0; let mut s_p_space = 0; let mut s_p_extr = 0;
    let mut s_n_stack = 0.0; let mut s_n_over = 0.0; let mut s_n_space = 0.0; let mut s_n_extr = 0.0;
    let mut v_stead = 0; let mut v_vari = 0; let mut v_dyna = 0;
    let mut bursts = 0; let mut short_len = 0; let mut med_len = 0; let mut long_len = 0; let mut death_len = 0;
    let mut s_total_dist = 0.0; let mut s_gaps = 0; let mut max_stream = 0;
    
    let mut buffer: Vec<f64> = Vec::new();
    let mut stream_times: Vec<f64> = Vec::new(); // For timing consistency

    for m in movements {
        if m.time_gap <= stream_threshold && m.distance <= 2.5 * d && m.distance > 0.0 {
            buffer.push(m.distance);
            stream_times.push(m.time_gap);
        } else {
            if buffer.len() >= 2 {
                let note_count = buffer.len() + 1;
                if note_count > max_stream { max_stream = note_count; }
                if note_count <= 4 { bursts += 1; }
                else {
                    if note_count <= 12 { short_len += 1; } else if note_count <= 24 { med_len += 1; } else if note_count <= 48 { long_len += 1; } else { death_len += 1; }
                    let mean = buffer.iter().sum::<f64>() / buffer.len() as f64;
                    if mean < 0.5 * d { s_p_stack += 1; } else if mean < 1.0 * d { s_p_over += 1; } else if mean < 2.0 * d { s_p_space += 1; } else { s_p_extr += 1; }
                    let var = buffer.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / buffer.len() as f64;
                    let cv = if mean > 0.0 { var.sqrt() / mean } else { 0.0 };
                    if cv < 0.15 { v_stead += 1; } else if cv < 0.40 { v_vari += 1; } else { v_dyna += 1; }
                    for &dist in &buffer {
                        s_total_dist += dist; s_gaps += 1;
                        if dist < 0.5 * d { s_n_stack += 1.0; } else if dist < 1.0 * d { s_n_over += 1.0; } else if dist < 2.0 * d { s_n_space += 1.0; } else { s_n_extr += 1.0; }
                    }
                }
            }
            buffer.clear();
        }
    }

    // BPM Consistency Math
    let consistency = if stream_times.len() >= 2 {
        let count = stream_times.len() as f64;
        let mean = stream_times.iter().sum::<f64>() / count;
        let var = stream_times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / count;
        (1.0 - (var.sqrt() / mean)).max(0.0)
    } else { 0.0 };

    json!({
        "overall_confidence": s_gaps as f64 / total_obj,
        "avg_stream_spacing": if s_gaps > 0 { s_total_dist / s_gaps as f64 } else { 0.0 },
        "s_stacked_count": s_p_stack, "s_overlapping_count": s_p_over, "s_spaced_count": s_p_space, "s_extreme_count": s_p_extr,
        "s_stack_dens": s_n_stack / total_obj, "s_over_dens": s_n_over / total_obj, "s_space_dens": s_n_space / total_obj, "s_extr_dens": s_n_extr / total_obj,
        "v_steady_count": v_stead, "v_variable_count": v_vari, "v_dynamic_count": v_dyna,
        "total_stream_patterns": (short_len + med_len + long_len + death_len),
        "bursts": bursts, "short_streams": short_len, "medium_streams": med_len, "long_streams": long_len, "death_streams": death_len,
        "max_stream_length": max_stream, "bpm_consistency": consistency, "circle_diameter": d
    })
}