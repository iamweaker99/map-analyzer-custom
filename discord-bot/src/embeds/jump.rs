use serenity::all::CreateEmbed;

use super::progress_bar;
use crate::types::JumpAnalysis;

fn spacing_tag(spacing: f64, d: f64) -> &'static str {
    if spacing <= 0.0 {
        return "N/A";
    }
    if spacing < 2.0 * d {
        "Narrow"
    } else if spacing < 3.5 * d {
        "Moderate"
    } else if spacing < 5.0 * d {
        "Wide"
    } else {
        "Cross-Screen (Extreme)"
    }
}

pub fn build(data: &JumpAnalysis) -> CreateEmbed {
    let d = data.circle_diameter.max(1.0);
    let tag = spacing_tag(data.avg_spacing, d);
    let total_distances = data.absolute_short_count
        + data.absolute_medium_count
        + data.absolute_long_count
        + data.absolute_extreme_count
        + data.absolute_cross_screen_count;
    let distance_pct = |count: i32| {
        if total_distances > 0 {
            count as f64 / total_distances as f64 * 100.0
        } else {
            0.0
        }
    };

    let dist = format!(
        "Narrow (<20% / 76.8 px): {} ({:.1}%)\nModerate (<40% / 153.6 px): {} ({:.1}%)\nWide (<60% / 230.4 px): {} ({:.1}%)\nExtreme (<80% / 307.2 px): {} ({:.1}%)\nCross-Screen (≥80% / 307.2 px): {} ({:.1}%)",
        data.absolute_short_count, distance_pct(data.absolute_short_count),
        data.absolute_medium_count, distance_pct(data.absolute_medium_count),
        data.absolute_long_count, distance_pct(data.absolute_long_count),
        data.absolute_extreme_count, distance_pct(data.absolute_extreme_count),
        data.absolute_cross_screen_count, distance_pct(data.absolute_cross_screen_count),
    );

    let chains = format!(
        "Short (<1s): {}\nMedium (<2s): {}\nLong (<4s): {}\nExtreme (≥4s): {}\nMax chain: **{} notes / {:.1}s**",
        data.duration_short_chains,
        data.duration_medium_chains,
        data.duration_long_chains,
        data.duration_extreme_chains,
        data.max_jump_length,
        data.max_jump_duration,
    );

    let bpm_bar = progress_bar(data.bpm_consistency, 10);

    CreateEmbed::new()
        .title("Jump Analysis")
        .color(0xec4899)
        .field(
            "Spacing",
            format!("**{}** ({:.1} px)", tag, data.avg_spacing),
            true,
        )
        .field("Distance Profile", dist, false)
        .field("Jump Chains", chains, true)
        .field(
            "BPM Consistency",
            format!("{} {:.1}%", bpm_bar, data.bpm_consistency * 100.0),
            true,
        )
}
