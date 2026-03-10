use super::get_diameter;
use osu_map_analyzer::rosu_map::section::hit_objects::HitObjectKind;
use serde_json::{json, Value};

pub fn analyze(map: &osu_map_analyzer::rosu_map::Beatmap, cs: f32, total_obj: f64) -> Value {
    let d = get_diameter(cs);
    
    let mut sl_count = 0; 
    let mut total_sv = 0.0;
    let mut l_short = 0; let mut l_med = 0; let mut l_long = 0; let mut l_ext = 0;
    let mut b_buzz = 0; let mut b_static = 0;
    let mut a_simple = 0; let mut a_curved = 0; let mut a_complex = 0; let mut a_art = 0;

    for obj in &map.hit_objects {
        if let HitObjectKind::Slider(s) = &obj.kind {
            sl_count += 1;
            let body_len = s.path.expected_dist().unwrap_or(0.0);
            
            // 1. Length Profile (Relative to Map Density)
            if body_len < 1.5 * d { l_short += 1; }
            else if body_len < 3.0 * d { l_med += 1; }
            else if body_len < 4.5 * d { l_long += 1; }
            else { l_ext += 1; }

            // 2. Buzz Profile (Relative to Sliders)
            if s.repeat_count > 0 {
                if body_len < 5.0 { b_static += 1; }
                else { b_buzz += 1; }
            }

            // 3. Artistic Profile (Control Points - Relative to Sliders)
            let points = s.path.control_points().len();
            if points <= 2 { a_simple += 1; }
            else if points <= 4 { a_curved += 1; }
            else if points <= 10 { a_complex += 1; }
            else { a_art += 1; }

            total_sv += body_len / 100.0; // Normalized SV representation
        }
    }

    let sl_f = sl_count as f64;

    json!({
        "overall_confidence": sl_f / total_obj,
        "slider_ratio": sl_f / total_obj,
        "avg_velocity": if sl_f > 0.0 { total_sv / sl_f } else { 0.0 },
        "l_short_count": l_short, "l_short_dens": l_short as f64 / total_obj,
        "l_med_count": l_med,     "l_med_dens": l_med as f64 / total_obj,
        "l_long_count": l_long,   "l_long_dens": l_long as f64 / total_obj,
        "l_ext_count": l_ext,     "l_ext_dens": l_ext as f64 / total_obj,
        "b_buzz_count": b_buzz,   "b_buzz_dens": if sl_f > 0.0 { b_buzz as f64 / sl_f } else { 0.0 },
        "b_static_count": b_static, "b_static_dens": if sl_f > 0.0 { b_static as f64 / sl_f } else { 0.0 },
        "a_simple_count": a_simple, "a_simple_dens": if sl_f > 0.0 { a_simple as f64 / sl_f } else { 0.0 },
        "a_curved_count": a_curved, "a_curved_dens": if sl_f > 0.0 { a_curved as f64 / sl_f } else { 0.0 },
        "a_complex_count": a_complex, "a_complex_dens": if sl_f > 0.0 { a_complex as f64 / sl_f } else { 0.0 },
        "a_artistic_count": a_art, "a_artistic_dens": if sl_f > 0.0 { a_art as f64 / sl_f } else { 0.0 }
    })
}