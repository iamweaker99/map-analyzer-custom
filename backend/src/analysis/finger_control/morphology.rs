use rosu_pp::Beatmap;
use rosu_pp::model::hit_object::HitObjectKind;

pub fn calculate_morphology_index(map: &Beatmap) -> f32 {
    if map.hit_objects.len() < 2 {
        return 0.0;
    }

    let mut switches = 0;
    let total_objects = map.hit_objects.len() as f32;

    for window in map.hit_objects.windows(2) {
        let obj_a = &window[0];
        let obj_b = &window[1];

        // Check the kind using matches! on the correct Enum path
        let type_a_is_circle = matches!(obj_a.kind, HitObjectKind::Circle);
        let type_b_is_circle = matches!(obj_b.kind, HitObjectKind::Circle);

        if type_a_is_circle != type_b_is_circle {
            switches += 1;
        }
    }

    (switches as f32 / total_objects) * 10.0
}