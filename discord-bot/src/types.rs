use serde::Deserialize;

// ── Beatmap Details ──

#[derive(Debug, Clone, Deserialize)]
pub struct DetailsResult {
    pub title: String,
    pub artist: String,
    pub creator: String,
    #[allow(dead_code)]
    pub creator_id: u32,
    pub version: String,
    pub set_id: u32,
    pub statistics: Statistics,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Statistics {
    pub star_rating: f64,
    pub bpm: f64,
    pub ar: f32,
    pub od: f32,
    pub hp: f32,
    pub cs: f32,
    pub total_objects: usize,
}

// ── API Wrapper ──

#[derive(Debug, Clone, Deserialize)]
pub struct AnalysisResult {
    pub analysis_type: String,
    pub analysis: serde_json::Value,
}

// ── Jump Analysis ──

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct JumpAnalysis {
    pub overall_confidence: f64,
    pub avg_spacing: f64,
    pub narrow_count: i32,
    pub moderate_count: i32,
    pub wide_count: i32,
    pub extreme_count: i32,
    pub narrow_dens: f64,
    pub moderate_dens: f64,
    pub wide_dens: f64,
    pub extreme_dens: f64,
    pub max_jump_length: i32,
    pub short_jumps: i32,
    pub medium_jumps: i32,
    pub long_jumps: i32,
    pub bpm_consistency: f64,
    pub circle_diameter: f64,
    pub jump_density: f64,
}

// ── Stream Analysis ──

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct StreamAnalysis {
    pub overall_confidence: f64,
    pub avg_stream_spacing: f64,
    pub s_stacked_count: i32,
    pub s_overlapping_count: i32,
    pub s_spaced_count: i32,
    pub s_extreme_count: i32,
    pub s_stack_dens: f64,
    pub s_over_dens: f64,
    pub s_space_dens: f64,
    pub s_extr_dens: f64,
    pub v_steady_count: i32,
    pub v_variable_count: i32,
    pub v_dynamic_count: i32,
    pub total_stream_patterns: i32,
    pub bursts: i32,
    pub short_streams: i32,
    pub medium_streams: i32,
    pub long_streams: i32,
    pub death_streams: i32,
    pub max_stream_length: i32,
    pub bpm_consistency: f64,
    pub circle_diameter: f64,
}

// ── Slider Analysis ──

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SliderAnalysis {
    pub overall_confidence: f64,
    pub slider_ratio: f64,
    pub avg_velocity: f64,
    pub l_short_count: i32,
    pub l_short_dens: f64,
    pub l_med_count: i32,
    pub l_med_dens: f64,
    pub l_long_count: i32,
    pub l_long_dens: f64,
    pub l_ext_count: i32,
    pub l_ext_dens: f64,
    pub b_buzz_count: i32,
    pub b_buzz_dens: f64,
    pub b_static_count: i32,
    pub b_static_dens: f64,
    pub a_simple_count: i32,
    pub a_simple_dens: f64,
    pub a_curved_count: i32,
    pub a_curved_dens: f64,
    pub a_complex_count: i32,
    pub a_complex_dens: f64,
    pub a_artistic_count: i32,
    pub a_artistic_dens: f64,
}

// ── Finger Control Analysis (camelCase from Serde rename) ──

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct FingerControlAnalysis {
    pub beatmap_md5: String,
    pub overall_confidence: f32,
    pub snap_distribution: Vec<SnapBucket>,
    pub burst_histogram: std::collections::HashMap<u32, u32>,
    pub off_grid_buckets: Vec<u32>,
    pub transition_matrix: TransitionMatrix,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SnapBucket {
    pub label: String,
    pub percentage: f32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TransitionMatrix {
    pub bpm_transitions: Vec<TransitionOccurrence>,
    pub bpm_ordinary: Vec<TransitionOccurrence>,
    pub bpm_minor: Vec<TransitionOccurrence>,
    pub bpm_major: Vec<TransitionOccurrence>,
    pub top_transitions: Vec<TransitionOccurrence>,
    pub rhythmic_resets: Vec<TransitionOccurrence>,
    pub delta_groups: std::collections::HashMap<String, Vec<TransitionOccurrence>>,
    pub category_counts: CategoryCounts,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TransitionOccurrence {
    pub label: String,
    pub percentage: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CategoryCounts {
    pub odd_to_odd: i32,
    pub even_to_even: i32,
    pub odd_to_even: i32,
    pub rhythmic_resets: i32,
}

// ── Aim Control Analysis ──

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct AimControlResult {
    pub spatial: SpatialData,
    pub kinematics: KinematicsData,
    pub vectors: VectorsData,
    pub endurance: EnduranceData,
    pub accv: Option<AccvMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SpatialData {
    pub total_movements: i32,
    pub avg_spacing_d: f64,
    pub avg_angle: f64,
    pub spacing_distribution: SpacingDistribution,
    pub angle_distribution: AngleDistribution,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SpacingDistribution {
    pub stacked: i32,
    pub micro: i32,
    pub flow: i32,
    pub standard: i32,
    pub large: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct AngleDistribution {
    pub snap_backs: i32,
    pub acute: i32,
    pub wide: i32,
    pub linear: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct KinematicsData {
    pub avg_velocity: f64,
    pub velocity_std_dev: f64,
    pub velocity_distribution: VelocityDistribution,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VelocityDistribution {
    pub significantly_slower: i32,
    pub slower: i32,
    pub mean: i32,
    pub faster: i32,
    pub significantly_faster: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VectorsData {
    pub directional_flips: i32,
    pub directional_chirps: i32,
    pub alignment: AlignmentData,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct AlignmentData {
    pub parallel: i32,
    pub orthogonal: i32,
    pub anti_symmetric: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EnduranceData {
    pub peak_strain: f64,
    pub time_under_tension_ms: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct AccvMetrics {
    pub peak_complexity: f64,
    pub sustained_complexity: f64,
    pub peak_spatial_cv: f64,
    pub peak_temporal_cv: f64,
    pub peak_kinetic_var: f64,
}

// ── Reading Analysis ──

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ReadingResult {
    pub summary: ReadingSummary,
    pub density: DensityData,
    pub trajectory: TrajectoryData,
    pub traps: TrapData,
    pub topography: TopographyData,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ReadingSummary {
    pub peak_strain: f64,
    pub ar_preempt_ms: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DensityData {
    pub isolated_pct: f64,
    pub chunking_pct: f64,
    pub clutter_pct: f64,
    pub overload_pct: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TrajectoryData {
    pub linear_pct: f64,
    pub mild_shifts_pct: f64,
    pub sharp_kinks_pct: f64,
    pub spaghetti_pct: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TrapData {
    pub count: i32,
    pub trap_index: f64,
    pub peak_magnitude: f64,
    pub notable_traps: Vec<NotableTrap>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct NotableTrap {
    pub time: f64,
    pub magnitude: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TopographyData {
    pub klines: Vec<KLine>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct KLine {
    pub window_start: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i32,
}
