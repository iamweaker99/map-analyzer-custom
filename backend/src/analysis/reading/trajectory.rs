use serde::{Deserialize, Serialize};
use super::visuals::VisualNode;
use super::density::DensityState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryState {
    pub time: f64,
    pub entropy: f64,
    pub is_spaghetti: bool, 
}

pub fn calculate_trajectory(nodes: &[VisualNode], density: &[DensityState], circle_diameter: f64) -> Vec<TrajectoryState> {
    let mut states = Vec::with_capacity(nodes.len());
    let safe_diameter = circle_diameter.max(1.0);

    for i in 0..nodes.len() {
        // 1. DYNAMIC WINDOW SIZING
        let local_density = density.get(i).map(|d| d.raw_objects).unwrap_or(4);
        let w = local_density.clamp(4, 16); // Min 4 notes for angles, Max 16 to prevent blowout
        let end_idx = (i + w).min(nodes.len());
        let window = &nodes[i..end_idx];

        if window.len() < 4 {
            states.push(TrajectoryState { time: nodes[i].start_time, entropy: 0.0, is_spaghetti: false });
            continue;
        }

        // 2. SPATIAL SPREAD (The "Cheese" Filter)
        let mut max_dist_sq = 0.0;
        let mut min_dist_sq = f64::MAX;
        
        for a in 0..window.len() {
            for b in (a+1)..window.len() {
                let dist_sq = (window[a].x - window[b].x).powi(2) + (window[a].y - window[b].y).powi(2);
                if dist_sq > max_dist_sq { max_dist_sq = dist_sq; }
                if dist_sq < min_dist_sq { min_dist_sq = dist_sq; }
            }
        }
        
        let max_d = max_dist_sq.sqrt();
        let min_d = min_dist_sq.sqrt();
        let spread_factor = if max_d >= safe_diameter { 1.0 } else { (max_d / safe_diameter).sqrt().clamp(0.0, 1.0) };

        // 3. ADAPTIVE MEAN ENTROPY
        let mut angle_changes = Vec::new();
        for j in 0..(window.len() - 2) {
            let p1 = &window[j]; let p2 = &window[j+1]; let p3 = &window[j+2];
            let a1 = (p2.y - p1.y).atan2(p2.x - p1.x);
            let a2 = (p3.y - p2.y).atan2(p3.x - p2.x);
            
            let mut d_theta = (a2 - a1) % (std::f64::consts::PI * 2.0);
            if d_theta > std::f64::consts::PI { d_theta -= std::f64::consts::PI * 2.0; }
            if d_theta < -std::f64::consts::PI { d_theta += std::f64::consts::PI * 2.0; }
            angle_changes.push(d_theta);
        }

        let mut total_entropy = 0.0;
        for j in 0..(angle_changes.len() - 1) {
            total_entropy += (angle_changes[j+1].abs() - angle_changes[j].abs()).abs().to_degrees();
        }

        let mean_entropy = if angle_changes.len() > 1 { total_entropy / (angle_changes.len() - 1) as f64 } else { 0.0 };
        
        // 4. FINAL CALCULATION
        let final_entropy = mean_entropy * spread_factor;
        let is_spaghetti = min_d < safe_diameter && final_entropy > 60.0; // Overlaps AND is chaotic

        states.push(TrajectoryState {
            time: window[0].start_time,
            entropy: final_entropy,
            is_spaghetti,
        });
    }
    states
}