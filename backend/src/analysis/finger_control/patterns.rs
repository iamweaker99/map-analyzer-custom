use rosu_pp::Beatmap;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PatternType {
    Jump,
    Slider,
    Burst(u32), // 2 to 6
    Stream,     // 7+
}

impl PatternType {
    pub fn as_str(&self) -> String {
        match self {
            Self::Jump => "Jump".to_string(),
            Self::Slider => "Slider".to_string(),
            Self::Burst(n) => format!("{}n Burst", n),
            Self::Stream => "Stream".to_string(),
        }
    }

    pub fn note_count(&self) -> u32 {
        match self {
            Self::Jump | Self::Slider => 1,
            Self::Burst(n) => *n,
            Self::Stream => 7, 
        }
    }

    pub fn is_odd(&self) -> bool {
        match self {
            Self::Burst(n) => n % 2 != 0,
            _ => false,
        }
    }

    pub fn is_even(&self) -> bool {
        match self {
            Self::Burst(n) => n % 2 == 0,
            _ => false,
        }
    }
}
// ... [Keep your extract_patterns logic from before] ...

pub struct Pattern {
    pub p_type: PatternType,
    pub time: f64,
}

pub fn extract_patterns(map: &Beatmap) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let objects = &map.hit_objects;
    let mut i = 0;

    while i < objects.len() {
        let current = &objects[i];
        
        // Check if this note starts a burst/stream (gap <= 150ms)
        let mut is_complex = false;
        if i + 1 < objects.len() {
            if objects[i+1].start_time - current.start_time <= 150.0 {
                is_complex = true;
            }
        }

        if is_complex {
            let start_time = current.start_time;
            let mut count = 1;
            // Group all contiguous notes within the 150ms threshold
            while i + 1 < objects.len() && objects[i+1].start_time - objects[i].start_time <= 150.0 {
                count += 1;
                i += 1;
            }

            let p_type = if count >= 7 { PatternType::Stream } else { PatternType::Burst(count) };
            patterns.push(Pattern { p_type, time: start_time });
        } else {
            // Single note: Use rosu_pp's detection to distinguish Slider vs Jump (Circle)
            let p_type = if current.is_slider() { PatternType::Slider } else { PatternType::Jump };
            patterns.push(Pattern { p_type, time: current.start_time });
        }
        i += 1;
    }
    patterns
}