use rosu_pp::Beatmap;
use serde::Serialize;
use std::collections::HashMap;

pub mod snap_filter;
pub mod patterns;
pub mod transitions;
pub mod timeline; // NEW: Register timeline

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FingerControlAnalysis {
    pub beatmap_md5: String,
    pub overall_confidence: f32, 
    pub snap_distribution: Vec<SnapBucket>,
    pub burst_histogram: HashMap<u32, u32>,
    pub off_grid_details: Vec<snap_filter::OffGridNote>,
    pub off_grid_buckets: [u32; 10],
    pub transition_matrix: transitions::TransitionMatrix,
    pub timeline: Vec<timeline::TimelinePoint>, // NEW: Curve data
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnapBucket {
    pub label: String,
    pub percentage: f32,
}

pub fn analyze(map: &Beatmap, md5: String) -> FingerControlAnalysis {
    let (snaps, mut bursts, off_grid, buckets) = snap_filter::analyze_foundation(map);
    let pattern_list = patterns::extract_patterns(map);
    let matrix = transitions::analyze(&pattern_list);

    // REMOVED: The problematic recursive call line was here.

    // FIX: Populate histogram strictly from the Action-Based pattern list
    bursts.clear();
    for p in &pattern_list {
        if let patterns::PatternType::Burst(n) = p.p_type {
            *bursts.entry(n).or_insert(0) += 1;
        }
    }

    let first_obj = map.hit_objects.first().map(|o| o.start_time).unwrap_or(0.0);
    let last_obj = map.hit_objects.last().map(|o| o.start_time).unwrap_or(0.0);
    let map_duration = (last_obj - first_obj).max(1.0);
    let timeline_data = timeline::generate_timeline(&pattern_list, map_duration);

    let total_snaps: u32 = snaps.values().sum();
    let technical_density: f32 = snaps.iter()
        .filter(|(l, _)| *l != "1/1" && *l != "1/2")
        .map(|(_, c)| *c as f32 / total_snaps.max(1) as f32)
        .sum();

    let snap_dist = snaps.into_iter().map(|(label, count)| {
        SnapBucket { label, percentage: count as f32 / total_snaps.max(1) as f32 }
    }).collect();

    FingerControlAnalysis {
        beatmap_md5: md5, // MD5 is now correctly assigned here
        overall_confidence: technical_density,
        snap_distribution: snap_dist,
        burst_histogram: bursts,
        off_grid_details: off_grid,
        off_grid_buckets: buckets,
        transition_matrix: matrix,
        timeline: timeline_data,
    }
}