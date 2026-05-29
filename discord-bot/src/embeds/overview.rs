use serenity::all::CreateEmbed;

use crate::commands::analyze::AnalysisState;
use crate::types::DetailsResult;
use super::progress_bar;

pub fn build(details: &DetailsResult, state: &AnalysisState) -> CreateEmbed {
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
        let (conf, label) = match *atype {
            "jump" => (state.jump.as_ref().map(|j| j.overall_confidence), "Jump"),
            "stream" => (state.stream.as_ref().map(|s| s.overall_confidence), "Stream"),
            "slider" => (state.slider.as_ref().map(|s| s.overall_confidence), "Slider"),
            _ => (None, *atype),
        };
        if let Some(conf) = conf {
            let pct = conf * 100.0;
            let bar = progress_bar(conf, 12);
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
}
