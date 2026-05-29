pub mod overview;
pub mod jump;
pub mod stream;
pub mod slider;
pub mod finger_control;
pub mod aim_control;
pub mod reading;

pub fn progress_bar(pct: f64, width: usize) -> String {
    let filled_count = (pct * width as f64).round() as usize;
    let empty_count = width.saturating_sub(filled_count);
    let filled = "\u{2588}".repeat(filled_count);
    let empty = "\u{2591}".repeat(empty_count);
    format!("{}{}", filled, empty)
}

pub fn format_time(ms: f64) -> String {
    if ms <= 0.0 {
        return "0:00".to_string();
    }
    let total_secs = (ms / 1000.0) as u64;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{}:{:02}", mins, secs)
}
