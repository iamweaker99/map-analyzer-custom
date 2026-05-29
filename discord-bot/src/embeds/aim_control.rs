use serenity::all::CreateEmbed;

use crate::types::AimControlResult;
use super::format_time;

fn stat_bar(label: &str, value: i32, total: i32) -> String {
    let pct = if total > 0 { (value as f64 / total as f64) * 100.0 } else { 0.0 };
    format!("{}: {} ({:.1}%)", label, value, pct)
}

pub fn build(data: &AimControlResult) -> CreateEmbed {
    let overview = format!(
        "Avg Spacing: **{:.2}D**\nAvg Angle: **{:.2}°**\nAvg Velocity: **{:.2} px/ms**\nDir Flips: **{}**  Chirps: **{}**\nPeak Strain: **{:.0}**",
        data.spatial.avg_spacing_d,
        data.spatial.avg_angle,
        data.kinematics.avg_velocity,
        data.vectors.directional_flips,
        data.vectors.directional_chirps,
        data.endurance.peak_strain,
    );

    let sd = &data.spatial.spacing_distribution;
    let total_sp = sd.stacked + sd.micro + sd.flow + sd.standard + sd.large;
    let spacing = [
        stat_bar("Stacked/Overlap", sd.stacked, total_sp),
        stat_bar("Micro (Wiggles)", sd.micro, total_sp),
        stat_bar("Flow Aim", sd.flow, total_sp),
        stat_bar("Standard Jumps", sd.standard, total_sp),
        stat_bar("Fullscreen", sd.large, total_sp),
    ].join("\n");

    let ad = &data.spatial.angle_distribution;
    let total_ang = ad.snap_backs + ad.acute + ad.wide + ad.linear;
    let angles = [
        stat_bar("Linear", ad.linear, total_ang),
        stat_bar("Wide (Flow)", ad.wide, total_ang),
        stat_bar("Acute (Tech)", ad.acute, total_ang),
        stat_bar("Snap-Backs", ad.snap_backs, total_ang),
    ].join("\n");

    let al = &data.vectors.alignment;
    let total_al = al.parallel + al.orthogonal + al.anti_symmetric;
    let align = [
        stat_bar("Parallel", al.parallel, total_al),
        stat_bar("Orthogonal", al.orthogonal, total_al),
        stat_bar("Anti-Sym", al.anti_symmetric, total_al),
    ].join("\n");

    let mut embed = CreateEmbed::new()
        .title("Aim Control Analysis")
        .color(0xf97316)
        .field("Overview", overview, false)
        .field("Spacing Profile", spacing, true)
        .field("Angle Profile", angles, true)
        .field("Alignment", align, true);

    if let Some(accv) = &data.accv {
        let accv_text = format!(
            "Peak (95%): **{:.2}**\nSustained (50%): **{:.2}**\nSpatial CV: **{:.2}**\nTemporal CV: **{:.2}**\nKinetic Var: **{:.2}**",
            accv.peak_complexity, accv.sustained_complexity,
            accv.peak_spatial_cv, accv.peak_temporal_cv, accv.peak_kinetic_var,
        );
        embed = embed.field("ACCV Complexity", accv_text, true);
    }

    embed
        .field("Time Under Tension", format!("**{}**", format_time(data.endurance.time_under_tension_ms)), false)
}
