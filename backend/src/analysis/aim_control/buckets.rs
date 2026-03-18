use std::f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeVelocityBucket {
    SignificantlySlower,
    Slower,
    Mean,
    Faster,
    SignificantlyFaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleBucket {
    Sharp,  // < 45 deg
    Wide,   // 45 - 105 deg
    Linear, // 105 - 150 deg
    Flow,   // > 150 deg
    None,   // For the first/last notes where angle can't be calculated
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentBucket {
    Linear,       // Continuous flow
    Orthogonal,   // Right-angle / grid-like movements
    AntiSymmetry, // Sharp switchbacks / reverse direction
    None,         // When angle is unavailable
}

/// Categorizes velocity based on the local window's Mean and Standard Deviation.
pub fn get_relative_velocity_bucket(velocity: f64, mean: f64, std_dev: f64) -> RelativeVelocityBucket {
    // If standard deviation is extremely close to 0, all velocities in this window are essentially identical.
    if std_dev < 1e-6 {
        return RelativeVelocityBucket::Mean;
    }

    let lower_bound_sig = mean - 1.0 * std_dev;
    let lower_bound_mean = mean - 0.25 * std_dev;
    let upper_bound_mean = mean + 0.25 * std_dev;
    let upper_bound_sig = mean + 1.0 * std_dev;

    if velocity < lower_bound_sig {
        RelativeVelocityBucket::SignificantlySlower
    } else if velocity >= lower_bound_sig && velocity < lower_bound_mean {
        RelativeVelocityBucket::Slower
    } else if velocity >= lower_bound_mean && velocity <= upper_bound_mean {
        RelativeVelocityBucket::Mean
    } else if velocity > upper_bound_mean && velocity <= upper_bound_sig {
        RelativeVelocityBucket::Faster
    } else {
        RelativeVelocityBucket::SignificantlyFaster
    }
}

/// Categorizes the angle into 4 texture buckets.
/// Assumes `angle` is the absolute deflection angle in degrees (0.0 to 180.0).
pub fn get_angle_bucket(angle: Option<f64>) -> AngleBucket {
    match angle {
        Some(a) => {
            let abs_a = a.abs();
            if abs_a < 45.0 {
                AngleBucket::Sharp
            } else if abs_a >= 45.0 && abs_a <= 105.0 {
                AngleBucket::Wide
            } else if abs_a > 105.0 && abs_a <= 150.0 {
                AngleBucket::Linear
            } else {
                AngleBucket::Flow
            }
        }
        None => AngleBucket::None,
    }
}

/// Categorizes the angle into 3 alignment buckets.
/// Assumes `angle` is the absolute deflection angle in degrees.
/// Note: Depending on how your `deflection_angle` is calculated:
/// - If 0 is a straight continuation: Linear is ~0, Orthogonal is ~90, AntiSymmetry is ~180.
/// - Adjust these bounds if your math uses 180 as a straight continuation.
pub fn get_alignment_bucket(angle: Option<f64>) -> AlignmentBucket {
    match angle {
        Some(a) => {
            let abs_a = a.abs();
            if abs_a <= 45.0 {
                AlignmentBucket::Linear
            } else if abs_a > 45.0 && abs_a <= 135.0 {
                AlignmentBucket::Orthogonal
            } else {
                AlignmentBucket::AntiSymmetry
            }
        }
        None => AlignmentBucket::None,
    }
}