use serde::{Deserialize, Serialize};
use super::visuals::VisualNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryState {
    pub time: f64,
    pub entropy: f64,          // How erratic the angle changes are (0.0 = perfect circle/line)
    pub is_overlapping: bool,  // True if Pattern B is drawn physically underneath Pattern A
}

pub fn calculate_trajectory(nodes: &[VisualNode]) -> Vec<TrajectoryState> {
    let mut states = Vec::new();
    let window_size = 4; // A standard 4-note visual chunk (A -> B -> C -> D)

    if nodes.len() < window_size {
        return states;
    }

    for window in nodes.windows(window_size) {
        let a = &window[0];
        let b = &window[1];
        let c = &window[2];
        let d = &window[3];

        // 1. Calculate the directional vectors
        let dx1 = b.x - a.x; let dy1 = b.y - a.y;
        let dx2 = c.x - b.x; let dy2 = c.y - b.y;
        let dx3 = d.x - c.x; let dy3 = d.y - c.y;

        // Calculate absolute angles of each movement using atan2 (-PI to PI)
        let angle1 = dy1.atan2(dx1);
        let angle2 = dy2.atan2(dx2);
        let angle3 = dy3.atan2(dx3);

        // Calculate the change in angle (Delta Theta) between movements
        // Normalizing the difference to be between -PI and PI
        let mut d_theta1 = (angle2 - angle1) % (std::f64::consts::PI * 2.0);
        if d_theta1 > std::f64::consts::PI { d_theta1 -= std::f64::consts::PI * 2.0; }
        if d_theta1 < -std::f64::consts::PI { d_theta1 += std::f64::consts::PI * 2.0; }

        let mut d_theta2 = (angle3 - angle2) % (std::f64::consts::PI * 2.0);
        if d_theta2 > std::f64::consts::PI { d_theta2 -= std::f64::consts::PI * 2.0; }
        if d_theta2 < -std::f64::consts::PI { d_theta2 += std::f64::consts::PI * 2.0; }

        // 2. Trajectory Entropy: The difference between the angular changes.
        // If d_theta1 and d_theta2 are identical, it's a perfect curve (Entropy = 0)
        let entropy = (d_theta2.abs() - d_theta1.abs()).abs().to_degrees();

        // 3. Visual Overlap ("Spaghetti" check)
        // If Node D is placed physically extremely close to Node A or B, but is 3 notes later,
        // it means the path crosses over itself. (50.0 pixels is roughly a circle diameter).
        let dist_a_d = ((d.x - a.x).powi(2) + (d.y - a.y).powi(2)).sqrt();
        let dist_b_d = ((d.x - b.x).powi(2) + (d.y - b.y).powi(2)).sqrt();
        
        let is_overlapping = dist_a_d < 50.0 || dist_b_d < 50.0;

        states.push(TrajectoryState {
            time: a.start_time,
            entropy,
            is_overlapping,
        });
    }

    states
}