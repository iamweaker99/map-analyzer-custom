use rosu_pp::Beatmap;
use serde::Serialize;

pub mod snap_filter;
pub mod morphology;
pub mod complexity;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FingerControlAnalysis {
    /// Maps to "Map Presence" in the frontend progress bar
    pub overall_confidence: f32, 
    pub complexity_score: f32,
    pub morphology_index: f32,
    pub snap_distribution: Vec<SnapBucket>,
    pub even_burst_ratio: f32,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnapBucket {
    pub label: String,
    pub percentage: f32,
}

pub fn analyze(map: &Beatmap) -> FingerControlAnalysis {
    let quantized_intervals = snap_filter::quantize_intervals(map);
    let morph_index = morphology::calculate_morphology_index(map);
    let (complexity, even_ratio) = complexity::calculate_complexity(map);
    
    // "Density" calculation: 
    // We sum the percentages of all snaps except 1/1 and 1/2.
    // This represents the portion of the map requiring finger control.
    let technical_density: f32 = quantized_intervals.iter()
        .filter(|s| s.label != "1/1" && s.label != "1/2")
        .map(|s| s.percentage)
        .sum();

    FingerControlAnalysis {
        overall_confidence: technical_density,
        complexity_score: complexity,
        morphology_index: morph_index,
        snap_distribution: quantized_intervals,
        even_burst_ratio: even_ratio,
    }
}
