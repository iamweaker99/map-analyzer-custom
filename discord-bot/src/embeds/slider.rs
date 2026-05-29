use serenity::all::CreateEmbed;

use crate::types::SliderAnalysis;

fn slider_tag(ratio: f64) -> &'static str {
    if ratio < 0.30 { "Mechanical Tech" }
    else if ratio < 0.60 { "Technical" }
    else { "Slider Tech" }
}

pub fn build(data: &SliderAnalysis) -> CreateEmbed {
    let len = format!(
        "Short (<1.5D): {} ({:.1}%)\nMedium (1.5-3D): {} ({:.1}%)\nLong (3-4.5D): {} ({:.1}%)\nExtended (>4.5D): {} ({:.1}%)",
        data.l_short_count, data.l_short_dens * 100.0,
        data.l_med_count, data.l_med_dens * 100.0,
        data.l_long_count, data.l_long_dens * 100.0,
        data.l_ext_count, data.l_ext_dens * 100.0,
    );

    let buzz = format!(
        "Buzz Sliders: {} ({:.1}%)\nStatic Buzz: {} ({:.1}%)",
        data.b_buzz_count, data.b_buzz_dens * 100.0,
        data.b_static_count, data.b_static_dens * 100.0,
    );

    let art = format!(
        "Simple (Linear): {} ({:.1}%)\nCurved: {} ({:.1}%)\nComplex: {} ({:.1}%)\nArtistic/Tech: {} ({:.1}%)",
        data.a_simple_count, data.a_simple_dens * 100.0,
        data.a_curved_count, data.a_curved_dens * 100.0,
        data.a_complex_count, data.a_complex_dens * 100.0,
        data.a_artistic_count, data.a_artistic_dens * 100.0,
    );

    CreateEmbed::new()
        .title("Slider Analysis")
        .color(0x22c55e)
        .field("Style", format!("**{}** (Avg SV: {:.2})", slider_tag(data.slider_ratio), data.avg_velocity), false)
        .field("Slider Length", len, false)
        .field("Buzz Profile", buzz, true)
        .field("Artistic Profile", art, false)
}
