use serde::{Deserialize, Serialize};
use super::visuals::VisualNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrapState {
    pub time: f64,
    pub is_deceleration_trap: bool,
    pub distance: f64,
    pub time_gap: f64,
    pub magnitude: f64,
}

pub fn calculate_traps(nodes: &[VisualNode], bpm: f64) -> Vec<TrapState> {
    let mut states = Vec::new();
    if nodes.len() < 3 { return states; }

    // Dynamic Inertia Reset: 1.5 beats
    let beat_duration_ms = 60000.0 / bpm.max(1.0);
    let inertia_reset_threshold = beat_duration_ms * 1.5;

    for window in nodes.windows(3) {
        let prev_node = &window[0];
        let curr_node = &window[1];
        let next_node = &window[2];

        let dt_prev = (curr_node.start_time - prev_node.start_time).max(1.0);
        let dt_curr = next_node.start_time - curr_node.start_time;
        
        // Break detection filter
        if dt_curr > inertia_reset_threshold { continue; }

        let dx = next_node.x - curr_node.x;
        let dy = next_node.y - curr_node.y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Rhythmic shock clamped to prevent mathematical blowouts on edge cases
        let rhythmic_shock = (dt_curr / dt_prev).clamp(0.0, 3.0);
        let magnitude = rhythmic_shock * (distance / 100.0);

        if magnitude > 1.5 && dt_curr > dt_prev {
            states.push(TrapState {
                time: curr_node.start_time,
                is_deceleration_trap: true,
                distance,
                time_gap: dt_curr,
                magnitude,
            });
        }
    }
    states
}