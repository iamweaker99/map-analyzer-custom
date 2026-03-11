use rosu_pp::Beatmap;
use serde::Serialize;
use std::collections::HashMap;

pub mod snap_filter;
pub mod morphology;
pub mod complexity;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FingerControlAnalysis {
    pub overall_confidence: f32, 
    pub snap_distribution: Vec<SnapBucket>,
    pub burst_histogram: HashMap<u32, u32>,
    pub off_grid_details: Vec<snap_filter::OffGridNote>,
    pub off_grid_buckets: [u32; 10], // New array of 10
    pub complexity_score: f32,
    pub morphology_index: f32,
    pub even_burst_ratio: f32,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnapBucket {
    pub label: String,
    pub percentage: f32,
}

pub fn analyze(map: &Beatmap) -> FingerControlAnalysis {
    // Stage 1: Rhythmic Foundation
    // We must capture all 4 values: snaps, bursts, off_grid, AND buckets
    let (snaps, bursts, off_grid, buckets) = snap_filter::analyze_foundation(map);
    
    let total_snaps: u32 = snaps.values().sum();
    let technical_density: f32 = snaps.iter()
        .filter(|(label, _)| *label != "1/1" && *label != "1/2")
        .map(|(_, count)| *count as f32 / total_snaps.max(1) as f32)
        .sum();

    let snap_dist = snaps.into_iter().map(|(label, count)| {
        SnapBucket { 
            label, 
            percentage: count as f32 / total_snaps.max(1) as f32 
        }
    }).collect();

    FingerControlAnalysis {
        overall_confidence: technical_density,
        snap_distribution: snap_dist,
        burst_histogram: bursts,
        off_grid_details: off_grid,
        off_grid_buckets: buckets, // This was likely causing the "not found in this scope" error
        complexity_score: 0.0,
        morphology_index: 0.0,
        even_burst_ratio: 0.0,
    }
}