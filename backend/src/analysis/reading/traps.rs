use serde::{Deserialize, Serialize};
use super::visuals::VisualNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrapState {
    pub time: f64,
    pub is_deceleration_trap: bool,
    pub distance: f64,
    pub time_gap: f64,
}

pub fn calculate_traps(nodes: &[VisualNode]) -> Vec<TrapState> {
    let mut states = Vec::new();
    let window_size = 3; // We need previous movement vs current movement

    if nodes.len() < window_size {
        return states;
    }

    for window in nodes.windows(window_size) {
        let prev_node = &window[0];
        let curr_node = &window[1];
        let next_node = &window[2];

        // Previous Movement (A -> B)
        let dt_prev = curr_node.start_time - prev_node.start_time;

        // Current Movement (B -> C)
        let dx = next_node.x - curr_node.x;
        let dy = next_node.y - curr_node.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let dt_curr = next_node.start_time - curr_node.start_time;

        // Deceleration Trap Logic:
        // 1. The physical jump is significant (> 100 pixels)
        // 2. The rhythm suddenly slows down significantly (current gap is 1.5x larger than previous gap)
        let is_deceleration_trap = distance > 100.0 && dt_curr >= (dt_prev * 1.5);

        states.push(TrapState {
            time: curr_node.start_time,
            is_deceleration_trap,
            distance,
            time_gap: dt_curr,
        });
    }

    states
}