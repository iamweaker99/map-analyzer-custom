use std::collections::HashMap;
use serde::Serialize;
use super::patterns::{Pattern, PatternType};

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransitionMatrix {
    pub top_transitions: Vec<TransitionOccurrence>, // Overall Top 10
    pub delta_groups: HashMap<u32, Vec<TransitionOccurrence>>, // Groups for Δ0, Δ1, Δ2, Δ3
    pub category_counts: CategoryCounts,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransitionOccurrence {
    pub label: String, // e.g., "Jump <-> Slider"
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
    let mut transitions_by_delta: HashMap<u32, HashMap<(String, String), u32>> = HashMap::new();
    let mut categories = CategoryCounts::default();
    let total_transitions = (patterns.len() as f32 - 1.0).max(1.0);

    for window in patterns.windows(2) {
        let p1 = &window[0].p_type;
        let p2 = &window[1].p_type;

        // 1. Determine Delta (Δ)
        let n1 = p1.note_count();
        let n2 = p2.note_count();
        let delta = (n1 as i32 - n2 as i32).abs() as u32;

        // Skip Δ > 3 for specific recording per your notes
        if delta <= 3 {
            // 2. Bidirectional Labeling (Sort names alphabetically to group A->B and B->A)
            let mut labels = vec![p1.as_str(), p2.as_str()];
            labels.sort();
            let key = (labels[0].clone(), labels[1].clone());

            let delta_map = transitions_by_delta.entry(delta).or_default();
            *delta_map.entry(key).or_insert(0) += 1;
        }

        // 3. Numbered Burst Categories
        match (p1, p2) {
            (PatternType::Burst(_), PatternType::Burst(_)) => {
                if p1.is_odd() && p2.is_odd() { categories.odd_to_odd += 1; }
                else if p1.is_even() && p2.is_even() { categories.even_to_even += 1; }
                else { categories.odd_to_even += 1; }
            },
            _ => {}
        }
    }

    // Process maps into sorted TransitionOccurrence lists
    let mut delta_groups = HashMap::new();
    let mut all_transitions: Vec<TransitionOccurrence> = Vec::new();

    for (delta, map) in transitions_by_delta {
        let mut group_list: Vec<TransitionOccurrence> = map.into_iter()
            .map(|((a, b), count)| TransitionOccurrence {
                label: if a == b { a } else { format!("{} <-> {}", a, b) },
                percentage: (count as f32 / total_transitions) * 100.0,
            })
            .collect();
        
        group_list.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());
        
        // Add to global list for overall Top 10
        all_transitions.extend(group_list.clone());
        
        // Cap individual delta sub-tables to top 10 as requested
        delta_groups.insert(delta, group_list.into_iter().take(10).collect());
    }

    all_transitions.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());

    TransitionMatrix {
        top_transitions: all_transitions.into_iter().take(10).collect(),
        delta_groups,
        category_counts: categories,
    }
}