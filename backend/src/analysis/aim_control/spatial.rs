use rosu_pp::Beatmap;
use super::vectors::AimVector;

pub fn calculate_spatial_vectors(map: &Beatmap) -> Vec<AimVector> {
    let mut spatial_vectors: Vec<AimVector> = Vec::new();
    
    if map.hit_objects.is_empty() {
        return spatial_vectors;
    }

    let mut prev_tail_pos = map.hit_objects[0].pos; 
    let mut prev_start_time = map.hit_objects[0].start_time;
    let mut prev_end_time = map.hit_objects[0].start_time; 

    for i in 1..map.hit_objects.len() {
        let curr = &map.hit_objects[i];
        let curr_head_pos = curr.pos;
        
        // Spatial calculation using f64 precision
        let dx = (curr_head_pos.x - prev_tail_pos.x) as f64;
        let dy = (curr_head_pos.y - prev_tail_pos.y) as f64;
        let norm_distance = (dx * dx + dy * dy).sqrt();

        let dt = curr.start_time - prev_start_time; 
        let dt_break = curr.start_time - prev_end_time; 
        
        let safe_dt = if dt_break > 0.0 { dt_break } else { 1.0 }; 
        let velocity = norm_distance / safe_dt;

        let mut is_slider = false;
        let mut current_end_time = curr.start_time;
        let mut current_tail_pos = curr_head_pos;

        // Correct rosu-pp checks
        if curr.is_slider() { 
            is_slider = true;
            // Note: True slider tail tracking requires slider velocity parsing.
            // For Phase 1, we use head-pos. This will be refined in Phase 2.
            current_end_time = curr.start_time; 
            current_tail_pos = curr_head_pos;   
        } else if curr.is_spinner() {
            current_end_time = curr.start_time;
            current_tail_pos.x = 256.0; 
            current_tail_pos.y = 192.0;
        }

        let mut deflection_angle: Option<f64> = None;
        if let Some(last_vec) = spatial_vectors.last() {
            if last_vec.norm_distance > 0.0 && norm_distance > 0.0 {
                let dot_product = (last_vec.dx * dx) + (last_vec.dy * dy);
                let cos_theta = dot_product / (last_vec.norm_distance * norm_distance);
                let clamped_cos = cos_theta.clamp(-1.0, 1.0);
                deflection_angle = Some(clamped_cos.acos().to_degrees());
            }
        }

        spatial_vectors.push(AimVector {
            start_time: curr.start_time,
            end_time: current_end_time,
            norm_distance,
            dx,
            dy,
            dt,
            dt_break,
            velocity,
            deflection_angle,
            is_slider,
        });

        prev_tail_pos = current_tail_pos;
        prev_start_time = curr.start_time;
        prev_end_time = current_end_time;
    }
    
    spatial_vectors
}