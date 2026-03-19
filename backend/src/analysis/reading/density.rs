use serde::{Deserialize, Serialize};
use super::visuals::VisualNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DensityState {
    pub time: f64,
    pub concurrent_objects: usize,
}

pub fn calculate_density(nodes: &[VisualNode]) -> Vec<DensityState> {
    let mut states = Vec::with_capacity(nodes.len());

    for i in 0..nodes.len() {
        let current_time = nodes[i].start_time;
        let mut concurrent_objects = 0;

        // Count how many objects are "alive" on the screen at this exact millisecond
        // Since nodes are sorted by start_time, we can search backwards and forwards
        for j in 0..nodes.len() {
            let other = &nodes[j];
            
            // If the other object has faded in BEFORE OR AT current_time, 
            // and it has not been hit yet (start_time >= current_time)
            if other.fade_in_time <= current_time && other.start_time >= current_time {
                concurrent_objects += 1;
            }
            
            // Optimization: If the other object's fade-in time is strictly greater 
            // than our current time, we can break early because the array is sorted by start_time.
            // (Actually, fade_in_time is also roughly sorted, but we will iterate fully to be safe 
            // against weird overlapping inherited timing points).
        }

        states.push(DensityState {
            time: current_time,
            concurrent_objects,
        });
    }

    states
}