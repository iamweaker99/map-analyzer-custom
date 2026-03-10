export interface BeatmapDetailsResult {
    title: string; artist: string; creator: string; creator_id: number;
    version: string; set_id: number;
    statistics: { ar: number; od: number; hp: number; cs: number; bpm: number; star_rating: number; total_objects: number; };
}

export interface BeatmapAnalysisResult {
    // Ensure "fingercontrol" is exactly as it appears in the backend JSON
    analysis_type: "jump" | "stream" | "slider" | "fingercontrol"; 
    analysis: JumpAnalysis | StreamAnalysis | SliderAnalysis | FingerControlAnalysis;
}

export interface JumpAnalysis {
    overall_confidence: number; circle_diameter: number;
    max_jump_length: number; short_jumps: number; medium_jumps: number; long_jumps: number;
    jump_density: number; bpm_consistency: number; avg_spacing: number;
    narrow_count: number; moderate_count: number; wide_count: number; extreme_count: number;
    narrow_dens: number; moderate_dens: number; wide_dens: number; extreme_dens: number;
}

export interface StreamAnalysis {
    overall_confidence: number; total_stream_patterns: number; circle_diameter: number;
    s_stacked_count: number; s_overlapping_count: number; s_spaced_count: number; s_extreme_count: number;
    s_stack_dens: number; s_over_dens: number; s_space_dens: number; s_extr_dens: number;
    avg_stream_spacing: number; v_steady_count: number; v_variable_count: number; v_dynamic_count: number;
    bursts: number; short_streams: number; medium_streams: number; long_streams: number; death_streams: number;
    max_stream_length: number; stream_density: number; bpm_consistency: number;
}

export interface SliderAnalysis {
    overall_confidence: number; avg_velocity: number; slider_ratio: number;
    l_short_count: number; l_short_dens: number; l_med_count: number; l_med_dens: number;
    l_long_count: number; l_long_dens: number; l_ext_count: number; l_ext_dens: number;
    b_buzz_count: number; b_buzz_dens: number; b_static_count: number; b_static_dens: number;
    a_simple_count: number; a_simple_dens: number; a_curved_count: number; a_curved_dens: number;
    a_complex_count: number; a_complex_dens: number; a_artistic_count: number; a_artistic_dens: number;
}

// Add these interfaces to the file
export interface SnapBucket {
  label: string;
  percentage: number;
}

export interface FingerControlAnalysis {
  complexityScore: number;
  morphologyIndex: number;
  snapDistribution: SnapBucket[];
  evenBurstRatio: number;
}