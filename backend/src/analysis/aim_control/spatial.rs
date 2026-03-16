use rosu_pp::Beatmap;
use super::super::get_diameter;

#[derive(Clone, Debug)]
pub struct AimVector {
    pub start_time: f64,
    pub end_time: f64,
    pub dx: f64,
    pub dy: f64,
    pub dt: f64,
    pub distance: f64,
    pub norm_distance: f64, // Spacing in Circle Diameters (D)
    pub deflection_angle: Option<f64>, // In degrees
}

pub fn calculate_spatial_vectors(map: &Beatmap) -> Vec<AimVector> {
    let mut vectors = Vec::new();
    let cs = map.cs;
    let diameter = get_diameter(cs);

    if map.hit_objects.len() < 2 {
        return vectors;
    }

    for i in 1..map.hit_objects.len() {
        let prev = &map.hit_objects[i - 1];
        let curr = &map.hit_objects[i];

        let dx = (curr.pos.x - prev.pos.x) as f64;
        let dy = (curr.pos.y - prev.pos.y) as f64;
        let dt = curr.start_time - prev.start_time;
        
        // Skip stacked/0ms anomalies
        if dt <= 0.0 { continue; }

        let distance = (dx * dx + dy * dy).sqrt();
        let norm_distance = distance / diameter;

        let mut deflection_angle = None;

        // Calculate Deflection Angle if we have a previous vector
        if let Some(last_vec) = vectors.last() {
            if last_vec.distance > 0.0 && distance > 0.0 {
                let dot_product = (last_vec.dx * dx) + (last_vec.dy * dy);
                let cos_theta = dot_product / (last_vec.distance * distance);
                // Clamp to prevent NaN due to floating point inaccuracies
                let cos_theta_clamped = cos_theta.clamp(-1.0, 1.0);
                deflection_angle = Some(cos_theta_clamped.acos().to_degrees());
            }
        }

        vectors.push(AimVector {
            start_time: prev.start_time,
            end_time: curr.start_time,
            dx,
            dy,
            dt,
            distance,
            norm_distance,
            deflection_angle,
        });
    }

    vectors
}