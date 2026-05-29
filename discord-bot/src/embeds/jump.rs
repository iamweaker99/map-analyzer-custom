use serenity::all::CreateEmbed;

use crate::types::JumpAnalysis;
use super::progress_bar;

fn spacing_tag(spacing: f64, d: f64) -> &'static str {
    if spacing <= 0.0 { return "N/A"; }
    if spacing < 2.0 * d { "Narrow" }
    else if spacing < 3.5 * d { "Moderate" }
    else if spacing < 5.0 * d { "Wide" }
    else { "Cross-Screen (Extreme)" }
}

pub fn build(data: &JumpAnalysis) -> CreateEmbed {
    let d = data.circle_diameter.max(1.0);
    let tag = spacing_tag(data.avg_spacing, d);

    let dist = format!(
        "Narrow (<2D): {} ({:.1}%)\nModerate (2-3.5D): {} ({:.1}%)\nWide (3.5-5D): {} ({:.1}%)\nExtreme (>5D): {} ({:.1}%)",
        data.narrow_count, data.narrow_dens * 100.0,
        data.moderate_count, data.moderate_dens * 100.0,
        data.wide_count, data.wide_dens * 100.0,
        data.extreme_count, data.extreme_dens * 100.0,
    );

    let chains = format!(
        "Short (3-5): {}\nMedium (6-11): {}\nLong (12+): {}\nMax chain: **{} notes**",
        data.short_jumps, data.medium_jumps, data.long_jumps, data.max_jump_length
    );

    let bpm_bar = progress_bar(data.bpm_consistency, 10);

    CreateEmbed::new()
        .title("Jump Analysis")
        .color(0xec4899)
        .field("Spacing", format!("**{}** ({:.1} px)", tag, data.avg_spacing), true)
        .field("Distance Profile", dist, false)
        .field("Jump Chains", chains, true)
        .field("BPM Consistency", format!("{} {:.1}%", bpm_bar, data.bpm_consistency * 100.0), true)
}
