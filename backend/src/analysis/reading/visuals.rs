use rosu_pp::Beatmap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNode {
    pub start_time: f64,       // The exact ms the object must be hit
    pub end_time: f64,         // Kept for future slider-clutter updates
    pub fade_in_time: f64,     // The exact ms the object becomes visible (start_time - preempt)
    pub x: f64,
    pub y: f64,
    pub is_slider: bool,
}

/// Converts osu! Approach Rate (AR) to Preempt Time in milliseconds.
/// Preempt is the amount of time an object is visible on screen before it must be hit.
pub fn ar_to_preempt(ar: f32) -> f64 {
    let ar_f64 = ar as f64;
    if ar_f64 < 5.0 {
        1200.0 + 120.0 * (5.0 - ar_f64)
    } else if ar_f64 > 5.0 {
        1200.0 - 150.0 * (ar_f64 - 5.0)
    } else {
        1200.0
    }
}

pub fn extract_visual_nodes(map: &Beatmap) -> Vec<VisualNode> {
    let mut nodes = Vec::with_capacity(map.hit_objects.len());
    
    // Default to AR 9.0 if the parser fails to expose it, though standard Beatmap structs include it.
    let ar = map.ar; 
    let preempt = ar_to_preempt(ar);

    for obj in &map.hit_objects {
        let mut is_slider = false;
        let mut end_time = obj.start_time;

        if obj.is_slider() {
            is_slider = true;
            // Phase 1: Defaulting to start_time for end_time. 
            // This isolates our testing to standard note density first.
            end_time = obj.start_time; 
        } else if obj.is_spinner() {
            end_time = obj.start_time;
        }

        nodes.push(VisualNode {
            start_time: obj.start_time,
            end_time,
            fade_in_time: obj.start_time - preempt,
            x: obj.pos.x as f64,
            y: obj.pos.y as f64,
            is_slider,
        });
    }

    nodes
}