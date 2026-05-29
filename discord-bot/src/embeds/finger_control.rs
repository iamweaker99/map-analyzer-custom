use serenity::all::CreateEmbed;

use crate::types::FingerControlAnalysis;

pub fn build(data: &FingerControlAnalysis) -> CreateEmbed {
    let mut snaps: Vec<&crate::types::SnapBucket> = data.snap_distribution.iter().collect();
    snaps.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap_or(std::cmp::Ordering::Equal));
    let snap_text = snaps.iter().take(6).map(|s| {
        format!("{}: **{:.1}%**", s.label, s.percentage * 100.0)
    }).collect::<Vec<_>>().join("\n");

    let mut bursts: Vec<_> = data.burst_histogram.iter().collect();
    bursts.sort_by_key(|(k, _)| *k);
    let burst_text = bursts.iter().map(|(k, v)| {
        format!("**{}n**: {}", k, v)
    }).collect::<Vec<_>>().join("  ");

    let cats = &data.transition_matrix.category_counts;
    let trans = format!(
        "Odd→Odd: **{}**  Even→Even: **{}**\nOdd→Even: **{}**  Rhythmic Resets: **{}**",
        cats.odd_to_odd, cats.even_to_even, cats.odd_to_even, cats.rhythmic_resets,
    );

    let mut embed = CreateEmbed::new()
        .title("Finger Control Analysis")
        .color(0xa855f7)
        .field("Snap Distribution", snap_text, true)
        .field("Burst Profile", burst_text, false)
        .field("Pattern Transitions", trans, false);

    let top_bpm: Vec<_> = data.transition_matrix.bpm_transitions.iter().take(5).collect();
    if !top_bpm.is_empty() {
        let bpm_text = top_bpm.iter().map(|t| {
            format!("{}: **{:.1}%**", t.label, t.percentage)
        }).collect::<Vec<_>>().join("\n");
        embed = embed.field("Top Snap Transitions", bpm_text, true);
    }

    let off_grid_count: u32 = data.off_grid_buckets.iter().sum();
    embed = embed.field("Off-Grid Notes", format!("**{}** total", off_grid_count), true);

    embed
}
