use std::f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeVelocityBucket {
    SignificantlySlower = 0,
    Slower = 1,
    Mean = 2,
    Faster = 3,
    SignificantlyFaster = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleBucket {
    Sharp,  // Linear (<= 45)
    Wide,   // Wide Flow (<= 90)
    Linear, // Acute Tech (<= 135)
    Flow,   // Snapbacks (> 135)
    None,   
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentBucket {
    Linear,       
    Orthogonal,   
    AntiSymmetry, 
    None,         
}

pub fn get_relative_velocity_bucket(velocity: f64, mean: f64, std_dev: f64) -> RelativeVelocityBucket {
    if std_dev < 1e-6 { return RelativeVelocityBucket::Mean; }
    let m = mean;
    let s = std_dev;

    if velocity <= m - (2.0 * s) { RelativeVelocityBucket::SignificantlySlower }
    else if velocity < m - (0.5 * s) { RelativeVelocityBucket::Slower }
    else if velocity <= m + (0.5 * s) { RelativeVelocityBucket::Mean }
    else if velocity < m + (2.0 * s) { RelativeVelocityBucket::Faster }
    else { RelativeVelocityBucket::SignificantlyFaster }
}

pub fn get_angle_bucket(angle: Option<f64>) -> AngleBucket {
    match angle {
        Some(a) => {
            let abs_a = a.abs();
            if abs_a <= 45.0 { AngleBucket::Sharp }
            else if abs_a <= 90.0 { AngleBucket::Wide }
            else if abs_a <= 135.0 { AngleBucket::Linear }
            else { AngleBucket::Flow }
        }
        None => AngleBucket::None,
    }
}

pub fn get_alignment_bucket(angle: Option<f64>) -> AlignmentBucket {
    match angle {
        Some(a) => {
            let abs_a = a.abs();
            if abs_a <= 45.0 { AlignmentBucket::Linear }
            else if abs_a > 45.0 && abs_a <= 135.0 { AlignmentBucket::Orthogonal }
            else { AlignmentBucket::AntiSymmetry }
        }
        None => AlignmentBucket::None,
    }
}