use rosu_pp::Beatmap;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OffGridNote {
    pub time: f64,
    pub delta: f64,
}

pub fn analyze_foundation(map: &Beatmap) -> (HashMap<String, u32>, HashMap<u32, u32>, Vec<OffGridNote>, [u32; 10]) {
    let mut snap_counts = HashMap::new();
    let mut burst_histogram = HashMap::new();
    let mut off_grid_notes = Vec::new();
    let mut buckets = [0u32; 10];

    if map.hit_objects.is_empty() {
        return (snap_counts, burst_histogram, off_grid_notes, buckets);
    }

    let base_bpm = map.bpm();
    let ms_per_beat = 60000.0 / base_bpm;
    
    let start_time = map.hit_objects.first().unwrap().start_time;
    let end_time = map.hit_objects.last().unwrap().start_time;
    let total_duration = (end_time - start_time).max(1.0);

    // 1. Snap & Off-grid Logic
    for window in map.hit_objects.windows(2) {
        let delta = window[1].start_time - window[0].start_time;
        if let Some(label) = identify_snap(delta, ms_per_beat) {
            *snap_counts.entry(label).or_insert(0) += 1;
        } else {
            let note_time = window[1].start_time;
            off_grid_notes.push(OffGridNote { time: note_time, delta });
            
            // "Snap" to one of the 10 sections
            let relative_pos = (note_time - start_time) / total_duration;
            let bucket_idx = (relative_pos * 10.0).floor() as usize;
            if bucket_idx < 10 {
                buckets[bucket_idx] += 1;
            } else {
                buckets[9] += 1;
            }
        }
    }

    // 2. Burst Logic (Absolute 150ms threshold)
    let mut current_burst_len = 1;
    for window in map.hit_objects.windows(2) {
        let delta = window[1].start_time - window[0].start_time;
        if delta <= 150.0 { 
            current_burst_len += 1;
        } else {
            if (2..=6).contains(&current_burst_len) {
                *burst_histogram.entry(current_burst_len).or_insert(0) += 1;
            }
            current_burst_len = 1;
        }
    }
    if (2..=6).contains(&current_burst_len) {
        *burst_histogram.entry(current_burst_len).or_insert(0) += 1;
    }

    (snap_counts, burst_histogram, off_grid_notes, buckets)
}

fn identify_snap(delta: f64, beat_len: f64) -> Option<String> {
    let snaps = [(1.0, "1/1"), (0.5, "1/2"), (0.3333, "1/3"), (0.25, "1/4"), (0.1666, "1/6"), (0.125, "1/8")];
    for (fraction, label) in snaps {
        if (delta - (beat_len * fraction)).abs() <= 12.0 {
            return Some(label.to_string());
        }
    }
    None
}