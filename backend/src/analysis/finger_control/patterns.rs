use rosu_pp::Beatmap;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PatternType {
    Jump,
    Slider,
    Burst(u32), // 2 to 6 notes
    Stream,     // 7+ notes
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

pub struct Pattern {
    pub p_type: PatternType,
    pub time: f64,
    pub snap: String,
}

pub fn extract_patterns(map: &Beatmap) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let objects = &map.hit_objects;
    let mut i = 0;
    
    let ms_per_beat = 60000.0 / map.bpm();
    let gap_threshold = ms_per_beat / 2.0 + 10.0; 
    let proximity_threshold = 25.0; // Distance in pixels to be considered a "rhythmic chunk"

    while i < objects.len() {
        let current = &objects[i];
        
        if current.is_circle() && i + 1 < objects.len() {
            let mut count = 1;
            let mut j = i;
            let mut sum_delta = 0.0;

            // 1. Group consecutive circles
            while j + 1 < objects.len() && objects[j+1].is_circle() {
                let delta = objects[j+1].start_time - objects[j].start_time;
                if delta <= gap_threshold {
                    sum_delta += delta;
                    count += 1;
                    j += 1;
                } else {
                    break;
                }
            }

            // 2. Refined Method B: Check if an ending slider belongs to the rhythmic action
            if j + 1 < objects.len() && objects[j+1].is_slider() {
                let delta = objects[j+1].start_time - objects[j].start_time;
                let dx = objects[j+1].pos.x - objects[j].pos.x;
                let dy = objects[j+1].pos.y - objects[j].pos.y;
                let distance = (dx*dx + dy*dy).sqrt();

                // Logic: Rhythmic consistency + Spacing constraint
                if delta <= gap_threshold && distance <= proximity_threshold {
                    sum_delta += delta;
                    count += 1;
                    j = j + 1; // "Swallow" the slider head into the burst
                }
            }

            if count >= 2 {
                let avg_delta = sum_delta / (count - 1) as f64;
                let snap = super::snap_filter::identify_snap(avg_delta, ms_per_beat)
                    .unwrap_or_else(|| "Unstable".to_string());

                let p_type = if count >= 7 { PatternType::Stream } else { PatternType::Burst(count) };
                patterns.push(Pattern { p_type, time: current.start_time, snap });
                
                i = j + 1; // Move pointer past the last object of the burst
                continue;
            }
        }
        
        // 3. Isolated Object Logic (Jumps or Standalone Sliders)
        let first_delta = if i + 1 < objects.len() {
            objects[i+1].start_time - current.start_time
        } else {
            0.0
        };

        let snap = if i + 1 < objects.len() {
            super::snap_filter::identify_snap(first_delta, ms_per_beat)
                .unwrap_or_else(|| "Unstable".to_string())
        } else {
            "End".to_string()
        };

        let p_type = if current.is_slider() { PatternType::Slider } else { PatternType::Jump };
        patterns.push(Pattern { p_type, time: current.start_time, snap });
        
        i += 1;
    }
    patterns
}