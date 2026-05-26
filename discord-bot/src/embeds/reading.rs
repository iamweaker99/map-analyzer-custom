use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::types::ReadingResult;
use super::{progress_bar, format_time};

pub fn build(data: &ReadingResult, page: usize, total: usize) -> CreateEmbed {
    let clutter = format!(
        "Isolated: {} {:.1}%\nChunking:  {} {:.1}%\nClutter:   {} {:.1}%\nOverload:  {} {:.1}%",
        progress_bar(data.density.isolated_pct / 100.0, 10), data.density.isolated_pct,
        progress_bar(data.density.chunking_pct / 100.0, 10), data.density.chunking_pct,
        progress_bar(data.density.clutter_pct / 100.0, 10), data.density.clutter_pct,
        progress_bar(data.density.overload_pct / 100.0, 10), data.density.overload_pct,
    );

    let traj = format!(
        "Predictable: {} {:.1}%\nMild Shifts: {} {:.1}%\nSharp Kinks: {} {:.1}%\nSpaghetti:   {} {:.1}%",
        progress_bar(data.trajectory.linear_pct / 100.0, 10), data.trajectory.linear_pct,
        progress_bar(data.trajectory.mild_shifts_pct / 100.0, 10), data.trajectory.mild_shifts_pct,
        progress_bar(data.trajectory.sharp_kinks_pct / 100.0, 10), data.trajectory.sharp_kinks_pct,
        progress_bar(data.trajectory.spaghetti_pct / 100.0, 10), data.trajectory.spaghetti_pct,
    );

    let mut trap_lines = String::new();
    trap_lines.push_str(&format!("Total Traps: **{}**  |  Index: **{:.1}/1k**\n\n", data.traps.count, data.traps.trap_index));

    if !data.traps.notable_traps.is_empty() {
        trap_lines.push_str("**Notable Spikes:**\n");
        for trap in data.traps.notable_traps.iter().take(5) {
            let mag_tag = if trap.magnitude > 2.5 { " ⚠️" } else { "" };
            trap_lines.push_str(&format!("`{}` **{:.2}x**{}\n", format_time(trap.time), trap.magnitude, mag_tag));
        }
    }

    CreateEmbed::new()
        .title("Reading Analysis")
        .color(0x06b6d4)
        .field("Peak Cognitive Strain", format!("**{:.2}**", data.summary.peak_strain), true)
        .field("AR Preempt Time", format!("**{:.0} ms**", data.summary.ar_preempt_ms), true)
        .field("Visual Clutter", clutter, false)
        .field("Trajectory Chaos", traj, false)
        .field("Reading Traps", trap_lines, false)
        .footer(CreateEmbedFooter::new(format!("Page {}/{}  •  Navigate with the buttons below", page, total)))
}
