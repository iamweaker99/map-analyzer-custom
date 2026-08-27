use super::{get_diameter, Movement};
use serde_json::{json, Value};

pub fn analyze(movements: &[Movement], cs: f32, bpm: f64, total_obj: f64) -> Value {
    let d = get_diameter(cs);
    let stream_threshold = (60000.0 / bpm / 4.0) * 1.5;
    let jump_rhythm_threshold = 60000.0 / bpm;

    let mut abs_short = 0;
    let mut abs_medium = 0;
    let mut abs_long = 0;
    let mut abs_extreme = 0;
    let mut abs_cross_screen = 0;
    let mut total_dist = 0.0;
    let mut j_cnt = 0;

    let mut max_chain = 0;
    let mut max_chain_duration = 0.0;
    let mut current_chain = 0;
    let mut current_chain_duration = 0.0;
    let mut duration_short = 0;
    let mut duration_medium = 0;
    let mut duration_long = 0;
    let mut duration_extreme = 0;
    let mut jump_times: Vec<f64> = Vec::new();

    let process_chain = |chain: &mut i32,
                         duration: &mut f64,
                         max: &mut i32,
                         max_duration: &mut f64,
                         dsc: &mut i32,
                         dmc: &mut i32,
                         dlc: &mut i32,
                         dec: &mut i32| {
        let note_count = *chain + 1;
        if note_count >= 3 {
            if note_count > *max {
                *max = note_count;
                *max_duration = *duration;
            } else if note_count == *max && *duration > *max_duration {
                *max_duration = *duration;
            }

            if *duration < 1_000.0 {
                *dsc += 1;
            } else if *duration < 2_000.0 {
                *dmc += 1;
            } else if *duration < 4_000.0 {
                *dlc += 1;
            } else {
                *dec += 1;
            }
        }
        *chain = 0;
        *duration = 0.0;
    };

    for m in movements {
        if m.time_gap <= jump_rhythm_threshold
            && (m.time_gap > stream_threshold || m.distance > 2.5 * d)
        {
            if m.distance > 0.0 {
                j_cnt += 1;
                total_dist += m.distance;
                current_chain += 1;
                current_chain_duration += m.time_gap;
                jump_times.push(m.time_gap);

                if m.distance < 76.8 {
                    abs_short += 1;
                } else if m.distance < 153.6 {
                    abs_medium += 1;
                } else if m.distance < 230.4 {
                    abs_long += 1;
                } else if m.distance < 307.2 {
                    abs_extreme += 1;
                } else {
                    abs_cross_screen += 1;
                }
            }
        } else {
            process_chain(
                &mut current_chain,
                &mut current_chain_duration,
                &mut max_chain,
                &mut max_chain_duration,
                &mut duration_short,
                &mut duration_medium,
                &mut duration_long,
                &mut duration_extreme,
            );
        }
    }
    process_chain(
        &mut current_chain,
        &mut current_chain_duration,
        &mut max_chain,
        &mut max_chain_duration,
        &mut duration_short,
        &mut duration_medium,
        &mut duration_long,
        &mut duration_extreme,
    );

    // Calculate BPM Consistency (1.0 - CV)
    let consistency = if jump_times.len() >= 2 {
        let count = jump_times.len() as f64;
        let mean = jump_times.iter().sum::<f64>() / count;
        let var = jump_times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / count;
        (1.0 - (var.sqrt() / mean)).max(0.0)
    } else {
        0.0
    };

    json!({
        "overall_confidence": j_cnt as f64 / total_obj,
        "avg_spacing": if j_cnt > 0 { total_dist / j_cnt as f64 } else { 0.0 },
        "absolute_short_count": abs_short, "absolute_medium_count": abs_medium,
        "absolute_long_count": abs_long, "absolute_extreme_count": abs_extreme,
        "absolute_cross_screen_count": abs_cross_screen,
        "max_jump_length": max_chain,
        "max_jump_duration": max_chain_duration / 1000.0,
        "duration_short_chains": duration_short, "duration_medium_chains": duration_medium,
        "duration_long_chains": duration_long, "duration_extreme_chains": duration_extreme,
        "bpm_consistency": consistency,
        "circle_diameter": d,
        "jump_density": j_cnt as f64 / total_obj
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn movement(time_gap: f64, distance: f64) -> Movement {
        Movement { distance, time_gap }
    }
    fn count(result: &Value, key: &str) -> i64 {
        result[key].as_i64().unwrap()
    }
    fn value(result: &Value, key: &str) -> f64 {
        result[key].as_f64().unwrap()
    }

    #[test]
    fn classifies_jump_distances_against_playfield_height() {
        let movements = [76.7, 76.8, 153.6, 230.4, 307.2].map(|distance| movement(500.0, distance));
        let result = analyze(&movements, 4.0, 60.0, 6.0);

        assert_eq!(count(&result, "absolute_short_count"), 1);
        assert_eq!(count(&result, "absolute_medium_count"), 1);
        assert_eq!(count(&result, "absolute_long_count"), 1);
        assert_eq!(count(&result, "absolute_extreme_count"), 1);
        assert_eq!(count(&result, "absolute_cross_screen_count"), 1);
    }

    #[test]
    fn classifies_jump_chains_by_elapsed_duration() {
        let mut movements = Vec::new();
        for duration in [999.8, 1_000.0, 2_000.0, 4_000.0] {
            movements.extend(vec![movement(duration / 2.0, 200.0); 2]);
            movements.push(movement(5_000.0, 0.0));
        }
        let result = analyze(&movements, 4.0, 15.0, 12.0);

        assert_eq!(count(&result, "duration_short_chains"), 1);
        assert_eq!(count(&result, "duration_medium_chains"), 1);
        assert_eq!(count(&result, "duration_long_chains"), 1);
        assert_eq!(count(&result, "duration_extreme_chains"), 1);
    }

    #[test]
    fn reports_longest_duration_when_maximum_note_count_is_tied() {
        let mut movements = vec![movement(400.0, 200.0); 3];
        movements.push(movement(5_000.0, 0.0));
        movements.extend(vec![movement(600.0, 200.0); 3]);
        movements.push(movement(5_000.0, 0.0));
        movements.extend(vec![movement(500.0, 200.0); 2]);
        let result = analyze(&movements, 4.0, 15.0, 11.0);

        assert_eq!(count(&result, "max_jump_length"), 4);
        assert_eq!(value(&result, "max_jump_duration"), 1.8);
    }
}
