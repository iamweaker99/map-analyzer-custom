use rosu_pp::Beatmap;

pub fn calculate_complexity(map: &Beatmap) -> (f32, f32) {
    if map.hit_objects.len() < 2 {
        return (0.0, 0.0);
    }

    let mut groups: Vec<usize> = Vec::new();
    let mut current_group_size = 1;
    
    // 1. Grouping Logic: Define a "Burst"
    // If the gap between notes is small (e.g., < 200ms), they belong to the same rhythmic group.
    for window in map.hit_objects.windows(2) {
        let delta = window[1].start_time - window[0].start_time;
        
        if delta < 200.0 { 
            current_group_size += 1;
        } else {
            if current_group_size > 1 {
                groups.push(current_group_size);
            }
            current_group_size = 1;
        }
    }
    if current_group_size > 1 { groups.push(current_group_size); }

    if groups.is_empty() { return (0.0, 0.0); }

    // 2. Even-Odd Analysis
    let even_bursts = groups.iter().filter(|&&size| size % 2 == 0).count();
    let even_ratio = even_bursts as f32 / groups.len() as f32;

    // 3. Size Variance (Cognitive Load)
    // Measuring how often the brain has to adjust to a new burst size.
    let mut variance_score = 0.0;
    for window in groups.windows(2) {
        if window[0] != window[1] {
            variance_score += 1.0;
        }
    }
    
    let complexity_score = (variance_score / groups.len() as f32) * 10.0;

    (complexity_score, even_ratio)
}