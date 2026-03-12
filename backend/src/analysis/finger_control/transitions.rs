use std::collections::HashMap;
use serde::Serialize;
use super::patterns::{Pattern, PatternType};

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransitionMatrix {
    pub bpm_transitions: Vec<TransitionOccurrence>, // NEW: Snap <-> Snap
    pub top_transitions: Vec<TransitionOccurrence>, 
    pub delta_groups: HashMap<u32, Vec<TransitionOccurrence>>, 
    pub category_counts: CategoryCounts,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransitionOccurrence {
    pub label: String, 
    pub percentage: f32,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCounts {
    pub odd_to_odd: u32,
    pub even_to_even: u32,
    pub odd_to_even: u32,
}

pub fn analyze(patterns: &[Pattern]) -> TransitionMatrix {
    let mut snap_trans_map: HashMap<(String, String), u32> = HashMap::new();
    let mut global_trans_map: HashMap<(String, String), u32> = HashMap::new();
    let mut transitions_by_delta: HashMap<u32, HashMap<(String, String), u32>> = HashMap::new();
    let mut categories = CategoryCounts::default();
    
    let total_transitions = (patterns.len() as f32 - 1.0).max(1.0);

    for window in patterns.windows(2) {
        let p1 = &window[0];
        let p2 = &window[1];

        // 1. Snap-to-Snap Matrix (BPM Switching)
        if p1.snap != "End" && p2.snap != "End" && p1.snap != "Unstable" && p2.snap != "Unstable" {
            let mut snaps = vec![p1.snap.clone(), p2.snap.clone()];
            snaps.sort();
            let snap_key = (snaps[0].clone(), snaps[1].clone());
            *snap_trans_map.entry(snap_key).or_insert(0) += 1;
        }

        // 2. Format Labels with Snaps (e.g., "3n Burst (1/4)")
        let l1 = format!("{} ({})", p1.p_type.as_str(), p1.snap);
        let l2 = format!("{} ({})", p2.p_type.as_str(), p2.snap);
        let mut labels = vec![l1, l2];
        labels.sort();
        let key = (labels[0].clone(), labels[1].clone());

        // 3. Global Top 10 Tracking
        *global_trans_map.entry(key.clone()).or_insert(0) += 1;

        // 4. Delta Math
        let n1 = p1.p_type.note_count();
        let n2 = p2.p_type.note_count();
        let delta = (n1 as i32 - n2 as i32).abs() as u32;

        if delta <= 3 {
            let delta_map = transitions_by_delta.entry(delta).or_default();
            *delta_map.entry(key).or_insert(0) += 1;
        }

        // 5. Numbered Burst Categories
        match (&p1.p_type, &p2.p_type) {
            (PatternType::Burst(_), PatternType::Burst(_)) => {
                if p1.p_type.is_odd() && p2.p_type.is_odd() { categories.odd_to_odd += 1; }
                else if p1.p_type.is_even() && p2.p_type.is_even() { categories.even_to_even += 1; }
                else { categories.odd_to_even += 1; }
            },
            _ => {}
        }
    }

    // Helper closure to process sorted lists
    let process_map = |map: HashMap<(String, String), u32>| -> Vec<TransitionOccurrence> {
        let mut list: Vec<_> = map.into_iter().map(|((a, b), count)| {
            TransitionOccurrence {
                label: if a == b { a } else { format!("{} <-> {}", a, b) },
                percentage: (count as f32 / total_transitions) * 100.0,
            }
        }).collect();
        list.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());
        list.into_iter().take(10).collect()
    };

    let mut delta_groups = HashMap::new();
    for (delta, map) in transitions_by_delta {
        delta_groups.insert(delta, process_map(map));
    }

    TransitionMatrix {
        bpm_transitions: process_map(snap_trans_map),
        top_transitions: process_map(global_trans_map),
        delta_groups,
        category_counts: categories,
    }
}