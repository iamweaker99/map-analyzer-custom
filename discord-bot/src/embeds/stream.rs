use serenity::all::CreateEmbed;

use crate::types::StreamAnalysis;
use super::progress_bar;

fn spacing_tag(spacing: f64, d: f64) -> &'static str {
    if spacing <= 0.0 { return "N/A"; }
    if spacing < 0.5 * d { "Stacked" }
    else if spacing < 1.0 * d { "Overlapping" }
    else if spacing < 2.0 * d { "Spaced" }
    else { "Extreme (Jump-Stream)" }
}

pub fn build(data: &StreamAnalysis) -> CreateEmbed {
    let d = data.circle_diameter.max(1.0);
    let tag = spacing_tag(data.avg_stream_spacing, d);
    let total_patterns = data.total_stream_patterns.max(1);

    let dist = format!(
        "Stacked (<0.5D): {} ({:.1}%)\nOverlapping (0.5-1D): {} ({:.1}%)\nSpaced (1-2D): {} ({:.1}%)\nExtreme (2-2.5D): {} ({:.1}%)",
        data.s_stacked_count, data.s_stack_dens * 100.0,
        data.s_overlapping_count, data.s_over_dens * 100.0,
        data.s_spaced_count, data.s_space_dens * 100.0,
        data.s_extreme_count, data.s_extr_dens * 100.0,
    );

    let var = format!(
        "Steady (CV<15%): {} ({:.1}%)\nVariable (15-40%): {} ({:.1}%)\nDynamic (>40%): {} ({:.1}%)",
        data.v_steady_count, (data.v_steady_count as f64 / total_patterns as f64) * 100.0,
        data.v_variable_count, (data.v_variable_count as f64 / total_patterns as f64) * 100.0,
        data.v_dynamic_count, (data.v_dynamic_count as f64 / total_patterns as f64) * 100.0,
    );

    let len = format!(
        "Burst (0.45-1s): {}\nShort (1-2s): {}\nMedium (2-4s): {}\nLong (4-6s): {}\nDeathstream (6s+): {}",
        data.bursts, data.short_streams, data.medium_streams, data.long_streams, data.death_streams
    );

    let bpm_bar = progress_bar(data.bpm_consistency, 10);

    CreateEmbed::new()
        .title("Streams")
        .color(0x3b82f6)
        .field("Type", format!("**{}** ({:.1} px)", tag, data.avg_stream_spacing), true)
        .field("Spacing Profile (Based on Circle Diameter)", dist, false)
        .field("Variance of Distance Profile", var, false)
        .field("Length Profile", len, true)
        .field(
            "Max stream",
            format!("{:.1}s / {} notes", data.max_stream_duration, data.max_stream_length),
            true,
        )
        .field("BPM Consistency", format!("{} {:.1}%", bpm_bar, data.bpm_consistency * 100.0), true)
}
