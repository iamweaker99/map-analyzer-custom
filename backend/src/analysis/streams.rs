use super::{get_diameter, Movement};
use serde_json::{json, Value};

struct StreamMetrics {
    s_p_stack: usize, s_p_over: usize, s_p_space: usize, s_p_extr: usize,
    s_n_stack: f64, s_n_over: f64, s_n_space: f64, s_n_extr: f64,
    v_stead: usize, v_vari: usize, v_dyna: usize,
    bursts: usize, short_len: usize, med_len: usize, long_len: usize, death_len: usize,
    s_total_dist: f64, s_gaps: usize, max_stream: usize, max_duration: f64,
}

impl StreamMetrics {
    fn new() -> Self {
        Self {
            s_p_stack: 0, s_p_over: 0, s_p_space: 0, s_p_extr: 0,
            s_n_stack: 0.0, s_n_over: 0.0, s_n_space: 0.0, s_n_extr: 0.0,
            v_stead: 0, v_vari: 0, v_dyna: 0,
            bursts: 0, short_len: 0, med_len: 0, long_len: 0, death_len: 0,
            s_total_dist: 0.0, s_gaps: 0, max_stream: 0, max_duration: 0.0,
        }
    }

    fn record(&mut self, distances: &[f64], times: &[f64], diameter: f64) -> bool {
        let note_count = distances.len() + 1;
        let duration = times.iter().sum::<f64>();
        if note_count < 5 || duration < 450.0 { return false; }

        self.max_stream = self.max_stream.max(note_count);
        self.max_duration = self.max_duration.max(duration / 1000.0);
        match duration {
            d if d < 1_000.0 => self.bursts += 1,
            d if d < 2_000.0 => self.short_len += 1,
            d if d < 4_000.0 => self.med_len += 1,
            d if d < 6_000.0 => self.long_len += 1,
            _ => self.death_len += 1,
        }

        let mean = distances.iter().sum::<f64>() / distances.len() as f64;
        if mean < 0.5 * diameter { self.s_p_stack += 1; }
        else if mean < diameter { self.s_p_over += 1; }
        else if mean < 2.0 * diameter { self.s_p_space += 1; }
        else { self.s_p_extr += 1; }

        let variance = distances.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / distances.len() as f64;
        let cv = if mean > 0.0 { variance.sqrt() / mean } else { 0.0 };
        if cv < 0.15 { self.v_stead += 1; }
        else if cv < 0.40 { self.v_vari += 1; }
        else { self.v_dyna += 1; }

        for &distance in distances {
            self.s_total_dist += distance;
            self.s_gaps += 1;
            if distance < 0.5 * diameter { self.s_n_stack += 1.0; }
            else if distance < diameter { self.s_n_over += 1.0; }
            else if distance < 2.0 * diameter { self.s_n_space += 1.0; }
            else { self.s_n_extr += 1.0; }
        }
        true
    }
}

pub fn analyze(movements: &[Movement], cs: f32, bpm: f64, total_obj: f64) -> Value {
    let d = get_diameter(cs);
    let stream_threshold = (60000.0 / bpm / 4.0) * 1.5;
    let mut metrics = StreamMetrics::new();
    let mut buffer = Vec::new();
    let mut stream_times = Vec::new();
    let mut eligible_times = Vec::new();

    for movement in movements {
        if movement.time_gap <= stream_threshold && movement.distance <= 2.5 * d && movement.distance > 0.0 {
            buffer.push(movement.distance);
            stream_times.push(movement.time_gap);
        } else {
            if metrics.record(&buffer, &stream_times, d) { eligible_times.extend(stream_times.iter().copied()); }
            buffer.clear();
            stream_times.clear();
        }
    }
    if metrics.record(&buffer, &stream_times, d) { eligible_times.extend(stream_times.iter().copied()); }

    let consistency = if eligible_times.len() >= 2 {
        let count = eligible_times.len() as f64;
        let mean = eligible_times.iter().sum::<f64>() / count;
        let variance = eligible_times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / count;
        (1.0 - (variance.sqrt() / mean)).max(0.0)
    } else { 0.0 };
    let total_patterns = metrics.bursts + metrics.short_len + metrics.med_len + metrics.long_len + metrics.death_len;

    json!({
        "overall_confidence": metrics.s_gaps as f64 / total_obj,
        "avg_stream_spacing": if metrics.s_gaps > 0 { metrics.s_total_dist / metrics.s_gaps as f64 } else { 0.0 },
        "s_stacked_count": metrics.s_p_stack, "s_overlapping_count": metrics.s_p_over, "s_spaced_count": metrics.s_p_space, "s_extreme_count": metrics.s_p_extr,
        "s_stack_dens": metrics.s_n_stack / total_obj, "s_over_dens": metrics.s_n_over / total_obj, "s_space_dens": metrics.s_n_space / total_obj, "s_extr_dens": metrics.s_n_extr / total_obj,
        "v_steady_count": metrics.v_stead, "v_variable_count": metrics.v_vari, "v_dynamic_count": metrics.v_dyna,
        "total_stream_patterns": total_patterns,
        "bursts": metrics.bursts, "short_streams": metrics.short_len, "medium_streams": metrics.med_len, "long_streams": metrics.long_len, "death_streams": metrics.death_len,
        "max_stream_length": metrics.max_stream, "max_stream_duration": metrics.max_duration, "bpm_consistency": consistency, "circle_diameter": d
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn movement(time_gap: f64, distance: f64) -> Movement { Movement { distance, time_gap } }
    fn pattern(duration: f64) -> Vec<Movement> { vec![movement(duration / 4.0, 10.0); 4] }
    fn value(result: &Value, key: &str) -> f64 { result[key].as_f64().unwrap() }

    #[test]
    fn classifies_eligible_patterns_by_elapsed_duration() {
        let mut movements = Vec::new();
        for (index, duration) in [450.0, 1_000.0, 2_000.0, 4_000.0, 6_000.0].iter().enumerate() {
            movements.extend(pattern(*duration));
            if index < 4 { movements.push(movement(100.0, 200.0)); }
        }
        let result = analyze(&movements, 4.0, 10.0, 30.0);
        assert_eq!(value(&result, "bursts"), 1.0);
        assert_eq!(value(&result, "short_streams"), 1.0);
        assert_eq!(value(&result, "medium_streams"), 1.0);
        assert_eq!(value(&result, "long_streams"), 1.0);
        assert_eq!(value(&result, "death_streams"), 1.0);
        assert_eq!(value(&result, "total_stream_patterns"), 5.0);
    }

    #[test]
    fn excludes_short_duration_and_small_patterns_from_all_metrics() {
        let mut movements = pattern(400.0);
        movements.push(movement(100.0, 200.0));
        movements.extend(vec![movement(100.0, 10.0); 3]);
        movements.push(movement(100.0, 200.0));
        movements.extend(pattern(500.0));
        let result = analyze(&movements, 4.0, 10.0, 20.0);
        assert_eq!(value(&result, "total_stream_patterns"), 1.0);
        assert_eq!(value(&result, "max_stream_length"), 5.0);
        assert_eq!(value(&result, "max_stream_duration"), 0.5);
        assert_eq!(value(&result, "s_stacked_count"), 1.0);
        assert_eq!(value(&result, "v_steady_count"), 1.0);
    }

    #[test]
    fn includes_a_qualifying_pattern_ending_at_the_last_hit_object() {
        let result = analyze(&pattern(1_000.0), 4.0, 10.0, 5.0);
        assert_eq!(value(&result, "total_stream_patterns"), 1.0);
        assert_eq!(value(&result, "short_streams"), 1.0);
        assert_eq!(value(&result, "max_stream_duration"), 1.0);
    }

    #[test]
    fn applies_exclusive_upper_duration_boundaries_and_note_minimum() {
        let mut movements = pattern(449.9);
        movements.push(movement(100.0, 200.0));
        for duration in [999.9, 1_999.9, 3_999.9, 5_999.9] {
            movements.extend(pattern(duration));
            movements.push(movement(100.0, 200.0));
        }
        movements.extend(vec![movement(100.0, 10.0); 3]);
        let result = analyze(&movements, 4.0, 10.0, 30.0);
        assert_eq!(value(&result, "bursts"), 1.0);
        assert_eq!(value(&result, "short_streams"), 1.0);
        assert_eq!(value(&result, "medium_streams"), 1.0);
        assert_eq!(value(&result, "long_streams"), 1.0);
        assert_eq!(value(&result, "total_stream_patterns"), 4.0);
    }
}
