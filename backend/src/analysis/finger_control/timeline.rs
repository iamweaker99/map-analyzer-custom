use super::patterns::Pattern;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimelinePoint {
    pub time: f64,
    pub pattern_sma: f32,
    pub bpm_sma: f32,
    
    pub bpm_ordinary_sma: f32,
    pub bpm_minor_sma: f32,
    pub bpm_major_sma: f32,

    pub note_delta_0_cons_sma: f32,
    pub note_delta_0_reset_sma: f32,
    pub note_delta_1_sma: f32,
    pub note_delta_2_sma: f32,
    pub note_delta_3_sma: f32,
}

pub fn generate_timeline(patterns: &[Pattern], map_duration: f64) -> Vec<TimelinePoint> {
    if patterns.is_empty() { return vec![]; }

    let total_objects = patterns.len() as f64;
    let w_objects = 20.0_f64.max(total_objects / 40.0);
    
    let avg_ms_per_object = if total_objects > 0.0 { map_duration / total_objects } else { 500.0 };
    let window_ms = w_objects * avg_ms_per_object;
    let half_window = window_ms / 2.0;

    let first_time = patterns.first().unwrap().time;
    let last_time = patterns.last().unwrap().time;

    let mut timeline = Vec::new();
    let mut current_time = first_time;
    let step_ms = 1000.0; 

    while current_time <= last_time {
        let window_start = current_time - half_window;
        let window_end = current_time + half_window;
        let mut pt = TimelinePoint { time: current_time, ..Default::default() };
        let mut objects_in_center = 0;

        for window in patterns.windows(2) {
            let p1 = &window[0];
            let p2 = &window[1];

            if p1.time >= current_time - 1500.0 && p1.time <= current_time + 1500.0 {
                objects_in_center += 1;
            }

            if p2.time >= window_start && p2.time <= window_end {
                // 1. Overall Pattern
                if p1.p_type.as_str() != p2.p_type.as_str() { pt.pattern_sma += 1.0; }
                
                // 2. Note Delta Categories
                let delta = (p1.p_type.note_count() as i32 - p2.p_type.note_count() as i32).abs() as u32;
                if delta == 0 {
                    if p1.snap == p2.snap { pt.note_delta_0_cons_sma += 1.0; }
                    else { pt.note_delta_0_reset_sma += 1.0; }
                } else if delta == 1 { pt.note_delta_1_sma += 1.0; }
                  else if delta == 2 { pt.note_delta_2_sma += 1.0; }
                  else if delta == 3 { pt.note_delta_3_sma += 1.0; }

                // 3. BPM/Snap Categories
                if p1.snap != p2.snap && p1.snap != "End" && p2.snap != "End" && p1.snap != "Unstable" && p2.snap != "Unstable" {
                    pt.bpm_sma += 1.0;
                    match super::transitions::get_bpm_category(&p1.snap, &p2.snap) {
                        "Ordinary" => pt.bpm_ordinary_sma += 1.0,
                        "Minor" => pt.bpm_minor_sma += 1.0,
                        "Major" => pt.bpm_major_sma += 1.0,
                        _ => {}
                    }
                }
            }
        }

        if objects_in_center == 0 {
            timeline.push(TimelinePoint { time: current_time, ..Default::default() });
        } else {
            timeline.push(pt);
        }

        current_time += step_ms;
    }
    timeline
}