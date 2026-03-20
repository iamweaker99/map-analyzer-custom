use serde::{Deserialize, Serialize};
use super::visuals::VisualNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DensityState {
    pub time: f64,
    pub raw_objects: usize,
    pub effective_objects: f64,
}

pub fn calculate_density(nodes: &[VisualNode], circle_diameter: f64) -> Vec<DensityState> {
    let mut states = Vec::with_capacity(nodes.len());
    let safe_diameter = circle_diameter.max(1.0); // Prevent division by zero

    for i in 0..nodes.len() {
        let current_time = nodes[i].start_time;
        let mut raw_count = 0;
        
        let mut min_x = f64::MAX; let mut max_x = f64::MIN;
        let mut min_y = f64::MAX; let mut max_y = f64::MIN;

        for j in 0..nodes.len() {
            let other = &nodes[j];
            
            if other.fade_in_time <= current_time && other.start_time >= current_time {
                raw_count += 1;
                if other.x < min_x { min_x = other.x; }
                if other.x > max_x { max_x = other.x; }
                if other.y < min_y { min_y = other.y; }
                if other.y > max_y { max_y = other.y; }
            }
        }

        // Apply Spatial Chunking Logic (Quadratic Smoothing)
        let effective_objects = if raw_count > 1 {
            let diagonal = ((max_x - min_x).powi(2) + (max_y - min_y).powi(2)).sqrt();
            
            let spread_factor = if diagonal >= safe_diameter {
                1.0 // Fully spread out, no chunking possible
            } else {
                // Square Root smoothing for overlapping objects
                (diagonal / safe_diameter).sqrt().clamp(0.0, 1.0)
            };
            
            // 1 base object + (remaining objects * spread multiplier)
            1.0 + ((raw_count as f64 - 1.0) * spread_factor)
        } else {
            raw_count as f64
        };

        states.push(DensityState {
            time: current_time,
            raw_objects: raw_count,
            effective_objects,
        });
    }

    states
}