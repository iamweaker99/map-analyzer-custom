use rosu_pp::Beatmap;
use serde::Serialize;
use std::collections::HashMap;

pub mod snap_filter;
pub mod patterns;
pub mod transitions;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FingerControlAnalysis {
    pub overall_confidence: f32, 
    pub snap_distribution: Vec<SnapBucket>,
    pub burst_histogram: HashMap<u32, u32>,
    pub off_grid_details: Vec<snap_filter::OffGridNote>,
    pub off_grid_buckets: [u32; 10],
    // Stage 2
    pub transition_matrix: transitions::TransitionMatrix,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnapBucket {
    pub label: String,
    pub percentage: f32,
}

pub fn analyze(map: &Beatmap) -> FingerControlAnalysis {
    let (snaps, bursts, off_grid, buckets) = snap_filter::analyze_foundation(map);
    
    // Stage 2 Pipeline
    let pattern_list = patterns::extract_patterns(map);
    let matrix = transitions::analyze(&pattern_list);

    let total_snaps: u32 = snaps.values().sum();
    let technical_density = snaps.iter()
        .filter(|(l, _)| *l != "1/1" && *l != "1/2")
        .map(|(_, c)| *c as f32 / total_snaps.max(1) as f32)
        .sum();

    let snap_dist = snaps.into_iter().map(|(label, count)| {
        SnapBucket { label, percentage: count as f32 / total_snaps.max(1) as f32 }
    }).collect();

    FingerControlAnalysis {
        overall_confidence: technical_density,
        snap_distribution: snap_dist,
        burst_histogram: bursts,
        off_grid_details: off_grid,
        off_grid_buckets: buckets,
        transition_matrix: matrix,
    }
}