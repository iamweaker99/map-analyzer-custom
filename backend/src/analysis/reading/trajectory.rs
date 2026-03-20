use serde::{Deserialize, Serialize};
use super::visuals::VisualNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryState {
    pub time: f64,
    pub entropy: f64,
    pub is_spaghetti: bool, 
}

pub fn calculate_trajectory(nodes: &[VisualNode], circle_diameter: f64) -> Vec<TrajectoryState> {
    let mut states = Vec::new();
    let window_size = 4;

    if nodes.len() < window_size { return states; }

    for window in nodes.windows(window_size) {
        let a = &window[0]; let b = &window[1];
        let c = &window[2]; let d = &window[3];

        let dx1 = b.x - a.x; let dy1 = b.y - a.y;
        let dx2 = c.x - b.x; let dy2 = c.y - b.y;
        let dx3 = d.x - c.x; let dy3 = d.y - c.y;

        let angle1 = dy1.atan2(dx1);
        let angle2 = dy2.atan2(dx2);
        let angle3 = dy3.atan2(dx3);

        let mut d_theta1 = (angle2 - angle1) % (std::f64::consts::PI * 2.0);
        if d_theta1 > std::f64::consts::PI { d_theta1 -= std::f64::consts::PI * 2.0; }
        if d_theta1 < -std::f64::consts::PI { d_theta1 += std::f64::consts::PI * 2.0; }

        let mut d_theta2 = (angle3 - angle2) % (std::f64::consts::PI * 2.0);
        if d_theta2 > std::f64::consts::PI { d_theta2 -= std::f64::consts::PI * 2.0; }
        if d_theta2 < -std::f64::consts::PI { d_theta2 += std::f64::consts::PI * 2.0; }

        let entropy = (d_theta2.abs() - d_theta1.abs()).abs().to_degrees();

        let dist_a_d = ((d.x - a.x).powi(2) + (d.y - a.y).powi(2)).sqrt();
        let dist_b_d = ((d.x - b.x).powi(2) + (d.y - b.y).powi(2)).sqrt();
        
        let physically_overlaps = dist_a_d < circle_diameter || dist_b_d < circle_diameter;
        
        // NEW: Contextual Awareness. S-curves and streams are protected by low entropy.
        let is_spaghetti = physically_overlaps && entropy > 60.0;

        states.push(TrajectoryState {
            time: a.start_time,
            entropy,
            is_spaghetti,
        });
    }

    states
}