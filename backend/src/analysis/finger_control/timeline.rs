use super::patterns::Pattern;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimelinePoint {
    pub time: f64,
    pub pattern_sma: f32,
    pub bpm_sma: f32,
}

pub fn generate_timeline(patterns: &[Pattern], map_duration: f64) -> Vec<TimelinePoint> {
    if patterns.is_empty() { return vec![]; }

    let total_objects = patterns.len() as f64;
    // Dynamic Window: max(20 objects, Total Objects / 40)
    let w_objects = 20.0_f64.max(total_objects / 40.0);
    
    let avg_ms_per_object = if total_objects > 0.0 { map_duration / total_objects } else { 500.0 };
    let window_ms = w_objects * avg_ms_per_object;
    let half_window = window_ms / 2.0;

    // FIXED: Changed `start_time` to `.time` to match your struct
    let first_time = patterns.first().unwrap().time;
    let last_time = patterns.last().unwrap().time;

    let mut timeline = Vec::new();
    let mut current_time = first_time;
    let step_ms = 1000.0; // 1 data point per second

    while current_time <= last_time {
        let window_start = current_time - half_window;
        let window_end = current_time + half_window;

        let mut pattern_switches = 0;
        let mut bpm_switches = 0;
        let mut objects_in_center = 0;

        for window in patterns.windows(2) {
            let p1 = &window[0];
            let p2 = &window[1];

            // FIXED: Changed `.time` to `.time`
            if p1.time >= current_time - 1500.0 && p1.time <= current_time + 1500.0 {
                objects_in_center += 1;
            }

            // FIXED: Changed `.time` to `.time`
            if p2.time >= window_start && p2.time <= window_end {
                // Pattern switch logic
                if p1.p_type.as_str() != p2.p_type.as_str() {
                    pattern_switches += 1;
                }
                // BPM/Snap switch logic
                if p1.snap != p2.snap && p1.snap != "End" && p2.snap != "End" && p1.snap != "Unstable" && p2.snap != "Unstable" {
                    bpm_switches += 1;
                }
            }
        }

        if objects_in_center == 0 {
            timeline.push(TimelinePoint { time: current_time, pattern_sma: 0.0, bpm_sma: 0.0 });
        } else {
            timeline.push(TimelinePoint {
                time: current_time,
                pattern_sma: pattern_switches as f32,
                bpm_sma: bpm_switches as f32,
            });
        }

        current_time += step_ms;
    }

    timeline
}