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
    pub snap: String, // NEW: We now store the rhythm of the pattern
}

pub fn extract_patterns(map: &Beatmap) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let objects = &map.hit_objects;
    let mut i = 0;
    
    // Reuse BPM math from your snap_filter
    let ms_per_beat = 60000.0 / map.bpm();

    while i < objects.len() {
        let current = &objects[i];
        
        let mut is_complex = false;
        let mut first_delta = 0.0;
        
        if i + 1 < objects.len() {
            first_delta = objects[i+1].start_time - current.start_time;
            if first_delta <= 150.0 {
                is_complex = true;
            }
        }

        if is_complex {
            let start_time = current.start_time;
            let mut count = 1;
            let mut sum_delta = 0.0;
            
            while i + 1 < objects.len() && objects[i+1].start_time - objects[i].start_time <= 150.0 {
                sum_delta += objects[i+1].start_time - objects[i].start_time;
                count += 1;
                i += 1;
            }

            // Average the gap inside the burst to find its true snap
            let avg_delta = sum_delta / (count - 1) as f64;
            let snap = super::snap_filter::identify_snap(avg_delta, ms_per_beat)
                .unwrap_or_else(|| "Unstable".to_string());

            let p_type = if count >= 7 { PatternType::Stream } else { PatternType::Burst(count) };
            patterns.push(Pattern { p_type, time: start_time, snap });
        } else {
            // Find snap to the next object for Jumps/Sliders
            let snap = if i + 1 < objects.len() {
                super::snap_filter::identify_snap(first_delta, ms_per_beat)
                    .unwrap_or_else(|| "Unstable".to_string())
            } else {
                "End".to_string()
            };

            let p_type = if current.is_slider() { PatternType::Slider } else { PatternType::Jump };
            patterns.push(Pattern { p_type, time: current.start_time, snap });
        }
        i += 1;
    }
    patterns
}