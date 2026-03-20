use serde::{Deserialize, Serialize};
use super::visuals::VisualNode;
use super::density::DensityState;
use super::trajectory::TrajectoryState;
use super::traps::TrapState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrainPoint {
    pub time: f64,
    pub strain: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KLine {
    pub window_start: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: usize,
}

pub fn calculate_strain_and_klines(
    nodes: &[VisualNode],
    density: &[DensityState],
    trajectory: &[TrajectoryState],
    traps: &[TrapState],
) -> (Vec<StrainPoint>, Vec<KLine>) {
    let mut strain_points = Vec::with_capacity(nodes.len());
    let mut current_strain = 0.0;
    let mut last_time = nodes.first().map(|n| n.start_time).unwrap_or(0.0);
    
    let half_life_ms = 500.0;

    for i in 0..nodes.len() {
        let t = nodes[i].start_time;
        let dt = (t - last_time).max(0.0);
        
        let decay_factor = 0.5_f64.powf(dt / half_life_ms);
        current_strain *= decay_factor;

        // Fetch upgraded metrics
        let effective_density = density.get(i).map(|d| d.effective_objects).unwrap_or(0.0);
        
        let local_traj = trajectory.iter().find(|tr| (tr.time - t).abs() < 1.0);
        let entropy = local_traj.map(|tr| tr.entropy).unwrap_or(0.0);
        let is_spaghetti = local_traj.map(|tr| tr.is_spaghetti).unwrap_or(false);

        let local_trap = traps.iter().find(|tr| (tr.time - t).abs() < 1.0);
        let is_decel_trap = local_trap.map(|tr| tr.is_deceleration_trap).unwrap_or(false);

        // Compute instantaneous Cognitive Cost
        let mut base_cost = 1.0; 
        base_cost += effective_density * 0.2; // Chunked streams add minimal penalty
        base_cost += (entropy / 90.0) * 0.5; 
        if is_spaghetti { base_cost += 2.0; } // Only punishes true erratic overlaps
        if is_decel_trap { base_cost += 3.0; } 

        current_strain += base_cost;
        
        strain_points.push(StrainPoint { time: t, strain: current_strain });
        last_time = t;
    }

    // 2. Calculate 5-Second K-Lines (Candlesticks)
    let mut klines = Vec::new();
    let window_duration = 5000.0; // 5 seconds
    
    if !strain_points.is_empty() {
        let mut current_window_start = (strain_points[0].time / window_duration).floor() * window_duration;
        let mut window_strains = Vec::new();

        for sp in &strain_points {
            if sp.time >= current_window_start + window_duration {
                if !window_strains.is_empty() {
                    klines.push(create_candle(current_window_start, &window_strains));
                }
                current_window_start = (sp.time / window_duration).floor() * window_duration;
                window_strains.clear();
            }
            window_strains.push(sp.strain);
        }
        if !window_strains.is_empty() {
            klines.push(create_candle(current_window_start, &window_strains));
        }
    }

    (strain_points, klines)
}

// Ensure create_candle remains below this function
fn create_candle(start_time: f64, strains: &[f64]) -> KLine {
    let open = strains.first().copied().unwrap_or(0.0);
    let close = strains.last().copied().unwrap_or(0.0);
    let mut high = open;
    let mut low = open;

    for &s in strains {
        if s > high { high = s; }
        if s < low { low = s; }
    }

    KLine {
        window_start: start_time,
        open,
        high,
        low,
        close,
        volume: strains.len(),
    }
}