use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::types::{AnalysisResult, DetailsResult};
use super::progress_bar;

pub fn build(details: &DetailsResult, results: &[AnalysisResult]) -> CreateEmbed {
    let stats = &details.statistics;

    let info = format!(
        "by **{}**\nmapped by **{}**  |  [{}]",
        details.artist, details.creator, details.version
    );

    let stats_line = format!(
        "AR: **{}**  OD: **{}**  HP: **{}**  CS: **{}**\nBPM: **{}**  Star Rating: **{:.2}**  Objects: **{}**",
        stats.ar, stats.od, stats.hp, stats.cs,
        stats.bpm.round() as u64, stats.star_rating, stats.total_objects
    );

    let mut class_lines = String::from("**Classification:**\n");
    for atype in &["jump", "stream", "slider"] {
        if let Some(r) = results.iter().find(|r| r.analysis_type == *atype) {
            let conf = r.analysis.get("overall_confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let pct = conf * 100.0;
            let bar = progress_bar(conf, 12);
            let label = match *atype {
                "jump" => "Jump",
                "stream" => "Stream",
                "slider" => "Slider",
                _ => atype,
            };
            class_lines.push_str(&format!("`{}` {} **{:.1}%**\n", bar, label, pct));
        }
    }

    let cover_url = format!("https://assets.ppy.sh/beatmaps/{}/covers/cover.jpg", details.set_id);

    CreateEmbed::new()
        .title(format!("{} - {} [{}]", details.title, details.artist, details.version))
        .color(0x8b5cf6)
        .thumbnail(cover_url)
        .field("Info", info, false)
        .field("Stats", stats_line, false)
        .field("\u{200b}", class_lines, false)
        .footer(CreateEmbedFooter::new("Page 1/7  •  Use the buttons below to explore details"))
}
