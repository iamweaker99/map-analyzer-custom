pub mod jumps;
pub mod streams;
pub mod sliders;

use rosu_pp::Beatmap;

#[derive(Clone, Debug)]
pub struct Movement {
    pub distance: f64,
    pub time_gap: f64,
}

pub fn get_diameter(cs: f32) -> f64 {
    108.8 - (8.96 * cs as f64)
}

pub fn create_movements(map: &Beatmap) -> Vec<Movement> {
    let mut movements = Vec::new();
    for window in map.hit_objects.windows(2) {
        let obj1 = &window[0];
        let obj2 = &window[1];
        
        let time_gap = obj2.start_time - obj1.start_time;
        let dx = (obj2.pos.x - obj1.pos.x) as f64;
        let dy = (obj2.pos.y - obj1.pos.y) as f64;
        let distance = (dx * dx + dy * dy).sqrt();

        movements.push(Movement {
            distance,
            time_gap,
        });
    }
    movements
}