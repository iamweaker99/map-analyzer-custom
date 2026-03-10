use super::{Movement, get_diameter};
use serde_json::{json, Value};

pub fn analyze(movements: &[Movement], cs: f32, bpm: f64, total_obj: f64) -> Value {
    let d = get_diameter(cs);
    let stream_threshold = (60000.0 / bpm / 4.0) * 1.5;
    let jump_rhythm_threshold = 60000.0 / bpm; 

    let mut n_cnt = 0; let mut m_cnt = 0; let mut w_cnt = 0; let mut e_cnt = 0;
    let mut total_dist = 0.0;
    let mut j_cnt = 0;

    let mut max_chain = 0;
    let mut current_chain = 0;
    let mut s_chain = 0; let mut m_chain = 0; let mut l_chain = 0;
    let mut jump_times: Vec<f64> = Vec::new();

    let process_chain = |chain: &mut i32, max: &mut i32, sc: &mut i32, mc: &mut i32, lc: &mut i32| {
        let note_count = *chain + 1;
        if note_count >= 3 {
            if note_count > *max { *max = note_count; }
            if note_count <= 5 { *sc += 1; }
            else if note_count <= 11 { *mc += 1; }
            else { *lc += 1; }
        }
        *chain = 0;
    };

    for m in movements {
        if m.time_gap <= jump_rhythm_threshold && (m.time_gap > stream_threshold || m.distance > 2.5 * d) {
            if m.distance > 0.0 {
                j_cnt += 1;
                total_dist += m.distance;
                current_chain += 1;
                jump_times.push(m.time_gap);

                if m.distance < 2.0 * d { n_cnt += 1; }
                else if m.distance < 3.5 * d { m_cnt += 1; }
                else if m.distance < 5.0 * d { w_cnt += 1; }
                else { e_cnt += 1; }
            }
        } else {
            process_chain(&mut current_chain, &mut max_chain, &mut s_chain, &mut m_chain, &mut l_chain);
        }
    }
    process_chain(&mut current_chain, &mut max_chain, &mut s_chain, &mut m_chain, &mut l_chain);

    // Calculate BPM Consistency (1.0 - CV)
    let consistency = if jump_times.len() >= 2 {
        let count = jump_times.len() as f64;
        let mean = jump_times.iter().sum::<f64>() / count;
        let var = jump_times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / count;
        (1.0 - (var.sqrt() / mean)).max(0.0)
    } else { 0.0 };

    json!({
        "overall_confidence": j_cnt as f64 / total_obj,
        "avg_spacing": if j_cnt > 0 { total_dist / j_cnt as f64 } else { 0.0 },
        "narrow_count": n_cnt, "moderate_count": m_cnt, "wide_count": w_cnt, "extreme_count": e_cnt,
        "narrow_dens": n_cnt as f64 / total_obj, "moderate_dens": m_cnt as f64 / total_obj,
        "wide_dens": w_cnt as f64 / total_obj, "extreme_dens": e_cnt as f64 / total_obj,
        "max_jump_length": max_chain,
        "short_jumps": s_chain, "medium_jumps": m_chain, "long_jumps": l_chain,
        "bpm_consistency": consistency,
        "circle_diameter": d,
        "jump_density": j_cnt as f64 / total_obj
    })
}