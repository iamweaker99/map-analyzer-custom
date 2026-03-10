use rosu_pp::Beatmap;
use crate::analysis::finger_control::SnapBucket;
use std::collections::HashMap;

pub fn quantize_intervals(map: &Beatmap) -> Vec<SnapBucket> {
    if map.hit_objects.len() < 2 {
        return vec![];
    }

    let mut snap_counts: HashMap<String, u32> = HashMap::new();
    let mut total_snapped_notes = 0;

    // Use the base BPM of the map
    let base_bpm = map.bpm();
    let ms_per_beat = 60000.0 / base_bpm;

    for window in map.hit_objects.windows(2) {
        let obj_a = &window[0];
        let obj_b = &window[1];
        
        let delta = obj_b.start_time - obj_a.start_time;
        
        if let Some(snap_label) = identify_snap(delta, ms_per_beat) {
            *snap_counts.entry(snap_label).or_insert(0) += 1;
            total_snapped_notes += 1;
        }
    }

    let mut distribution: Vec<SnapBucket> = snap_counts
        .into_iter()
        .map(|(label, count)| SnapBucket {
            label,
            percentage: (count as f32) / (total_snapped_notes as f32),
        })
        .collect();

    distribution.sort_by(|a, b| a.label.cmp(&b.label));
    distribution
}

fn identify_snap(delta: f64, beat_len: f64) -> Option<String> {
    let snaps = [
        (1.0, "1/1"),
        (0.5, "1/2"),
        (0.3333, "1/3"),
        (0.25, "1/4"),
        (0.1666, "1/6"),
        (0.125, "1/8"),
        (0.0833, "1/12"),
    ];

    let threshold = 10.0;

    for (fraction, label) in snaps {
        let theoretical_ms = beat_len * fraction;
        if (delta - theoretical_ms).abs() <= threshold {
            return Some(label.to_string());
        }
    }

    None
}