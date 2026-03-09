use osu_map_analyzer::{analyze, rosu_map};
use rosu_pp::{Beatmap, Difficulty};
use serde_json::Value;

use rosu_v2::{prelude::RankStatus, Osu as OsuClient};
use serde::Serialize;
use std::{
    fs::File,
    io::{ErrorKind, Read},
    path::Path,
    str::FromStr,
    sync::Arc,
};
use warp::{http::StatusCode, reply, Rejection, Reply};

use crate::utils::download_beatmap;

#[derive(Serialize)]
struct ApiError {
    error: String,
}

#[derive(Serialize)]
struct Statistics {
    star_rating: f64,
    bpm: f64,
    ar: f32,
    od: f32,
    hp: f32,
    cs: f32,
    total_objects: usize,
}

#[derive(Serialize)]
struct DetailsResult {
    title: String,
    artist: String,
    creator: String,
    creator_id: u32,
    version: String,
    set_id: u32,
    statistics: Statistics,
}

pub async fn beatmap_details(
    beatmap_id: u32,
    osu_client: Arc<OsuClient>,
) -> Result<impl Reply, Rejection> {
    let beatmap = match osu_client.beatmap().map_id(beatmap_id).await {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("Error while fetching beatmap: {}", err);
            return Ok(reply::with_status(
                reply::json(&ApiError {
                    error: format!("Error while fetching beatmap: {}", err),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let beatmapset = match beatmap.mapset {
        Some(s) => s,
        None => {
            eprintln!("Couldn't get beatmapset from beatmap (wtf?)");
            return Ok(reply::with_status(
                reply::json(&ApiError {
                    error: format!("Couldn't get beatmapset from beatmap (wtf?)"),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let should_download = matches!(
        beatmap.status,
        RankStatus::Graveyard | RankStatus::WIP | RankStatus::Pending
    );

    let map_file = if should_download {
        match download_beatmap(beatmap_id).await {
            Ok(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(string) => string,
                Err(err) => {
                    eprintln!("Error while converting bytes to string: {}", err);
                    return Ok(reply::with_status(
                        reply::json(&ApiError {
                            error: format!("Error while converting bytes to string: {}", err),
                        }),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ));
                }
            },
            Err(err) => {
                eprintln!("Error while downloading beatmap: {}", err);
                return Ok(reply::with_status(
                    reply::json(&ApiError {
                        error: format!("Error while downloading beatmap: {}", err),
                    }),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        }
    } else {
        match File::open(format!("maps/{}.osu", beatmap_id)) {
            Ok(mut file) => {
                let mut data_buf = String::new();

                if let Err(why) = file.read_to_string(&mut data_buf) {
                    eprintln!("Error while reading file: {}", why);
                    return Ok(reply::with_status(
                        reply::json(&ApiError {
                            error: format!("Error while reading file: {}", why),
                        }),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ));
                }

                data_buf
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => match download_beatmap(beatmap_id).await {
                    Ok(bytes) => match String::from_utf8(bytes.to_vec()) {
                        Ok(string) => string,
                        Err(err) => {
                            eprintln!("Error while converting bytes to string: {}", err);
                            return Ok(reply::with_status(
                                reply::json(&ApiError {
                                    error: format!(
                                        "Error while converting bytes to string: {}",
                                        err
                                    ),
                                }),
                                StatusCode::INTERNAL_SERVER_ERROR,
                            ));
                        }
                    },
                    Err(err) => {
                        eprintln!("Error while downloading beatmap: {}", err);
                        return Ok(reply::with_status(
                            reply::json(&ApiError {
                                error: format!("Error while downloading beatmap: {}", err),
                            }),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        ));
                    }
                },

                _ => {
                    eprintln!("Internal server error: {}", err);
                    return Ok(reply::with_status(
                        reply::json(&ApiError {
                            error: format!("Internal server error: {}", err),
                        }),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ));
                }
            },
        }
    };

    let map_calculate = match Beatmap::from_str(&map_file) {
        Ok(map) => map,
        Err(err) => {
            eprintln!("Error parsing beatmap: {}", err);
            return Ok(reply::with_status(
                reply::json(&ApiError {
                    error: format!("Error parsing beatmap: {}", err),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let diff_attrs = Difficulty::new().calculate(&map_calculate);

    let perf_attrs = rosu_pp::Performance::new(diff_attrs).calculate();

    let statistics = Statistics {
        ar: map_calculate.ar,
        od: map_calculate.od,
        cs: map_calculate.cs,
        hp: map_calculate.hp,
        bpm: map_calculate.bpm(),
        star_rating: perf_attrs.stars(),
        total_objects: map_calculate.hit_objects.len(), 
    };

    Ok(reply::with_status(
        reply::json(&DetailsResult {
            title: beatmapset.title,
            artist: beatmapset.artist,
            creator: beatmapset.creator_name.to_string(),
            creator_id: beatmapset.creator_id,
            version: beatmap.version,
            set_id: beatmapset.mapset_id,
            statistics,
        }),
        StatusCode::OK,
    ))
}

#[derive(Serialize)]
struct AnalysisResult {
    analysis_type: String,
    analysis: Value,
}

pub async fn analyze_beatmap(
    beatmap_id: u32,
    analyze_type: String,
) -> Result<impl Reply, Rejection> {
    let path = Path::new("maps").join(format!("{}.osu", beatmap_id));
    let map = rosu_map::from_path::<rosu_map::Beatmap>(&path).unwrap();
    let pp_map = rosu_pp::Beatmap::from_path(&path).unwrap();
    
    // --- MAP BASE STATS ---
    let bpm = pp_map.bpm();
    let stream_threshold = (60000.0 / bpm / 4.0) * 1.5;
    let total_obj_count = map.hit_objects.len() as f64;
    
    // Circle Size to Diameter Math (CS4 = 72.96px)
    let cs = pp_map.cs as f64;
    let d_circle = 108.8 - (8.96 * cs);

    // JUMP COUNTERS
    let mut j_dist = 0.0; let mut j_cnt = 0;
    let mut n_cnt = 0; let mut m_cnt = 0; let mut w_cnt = 0; let mut e_cnt = 0;
    
    // STREAM COUNTERS
    let mut max_stream_length = 0;
    let mut bursts = 0; let mut short_len = 0; let mut med_len = 0; let mut long_len = 0; let mut death_len = 0;
    let mut s_p_stack = 0; let mut s_p_over = 0; let mut s_p_space = 0; let mut s_p_extr = 0;
    let mut s_n_stack = 0.0; let mut s_n_over = 0.0; let mut s_n_space = 0.0; let mut s_n_extr = 0.0;
    let mut v_stead = 0; let mut v_vari = 0; let mut v_dyna = 0;
    let mut s_total_dist = 0.0; let mut s_total_gaps = 0;
    let mut s_buffer: Vec<f64> = Vec::new();

    // Helper closure for Jump categorization
    let mut process_jump = |dist: f64| {
        if dist > 0.0 {
            j_dist += dist; j_cnt += 1;
            if dist < 2.0 * d_circle { n_cnt += 1; }
            else if dist < 3.5 * d_circle { m_cnt += 1; }
            else if dist < 5.0 * d_circle { w_cnt += 1; }
            else { e_cnt += 1; }
        }
    };

    for window in pp_map.hit_objects.windows(2) {
        let time_diff = window[1].start_time - window[0].start_time;
        let d = {
            let dx = (window[1].pos.x - window[0].pos.x) as f64;
            let dy = (window[1].pos.y - window[0].pos.y) as f64;
            (dx * dx + dy * dy).sqrt()
        };

        // EXCLUSIVE FILTER: A stream gap must be fast AND <= 2.5x Diameter
        if time_diff <= stream_threshold && d <= 2.5 * d_circle && d > 0.0 {
            s_buffer.push(d);
        } else {
            // Buffer broken. Process what we have.
            let note_count = s_buffer.len() + 1;
            if s_buffer.len() >= 2 { // 3+ notes
                if note_count > max_stream_length { max_stream_length = note_count; }

                if note_count <= 4 { bursts += 1; } 
                else {
                    if note_count <= 12 { short_len += 1; }
                    else if note_count <= 24 { med_len += 1; }
                    else if note_count <= 48 { long_len += 1; }
                    else { death_len += 1; }

                    let count = s_buffer.len() as f64;
                    let mean = s_buffer.iter().sum::<f64>() / count;
                    
                    // Categorize Spacing by Multiples of D
                    if mean < 0.5 * d_circle { s_p_stack += 1; } 
                    else if mean < 1.0 * d_circle { s_p_over += 1; } 
                    else if mean < 2.0 * d_circle { s_p_space += 1; } 
                    else { s_p_extr += 1; }

                    // Categorize Variance
                    let var = s_buffer.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count;
                    let cv = if mean > 0.0 { var.sqrt() / mean } else { 0.0 };
                    if cv < 0.15 { v_stead += 1; } else if cv < 0.40 { v_vari += 1; } else { v_dyna += 1; }
                    
                    for &dist in &s_buffer {
                        s_total_dist += dist; s_total_gaps += 1;
                        if dist < 0.5 * d_circle { s_n_stack += 1.0; } 
                        else if dist < 1.0 * d_circle { s_n_over += 1.0; } 
                        else if dist < 2.0 * d_circle { s_n_space += 1.0; } 
                        else { s_n_extr += 1.0; }
                    }
                }
            } else {
                // If it was 2 notes (1 gap), it's just a fast jump
                for &dist in &s_buffer { process_jump(dist); }
            }
            s_buffer.clear();
            
            // The gap that broke the stream is always a jump
            process_jump(d);
        }
    }

    // Catch the final buffer
    let note_count = s_buffer.len() + 1;
    if s_buffer.len() >= 2 {
        if note_count > max_stream_length { max_stream_length = note_count; }
        if note_count <= 4 { bursts += 1; } 
        else {
            if note_count <= 12 { short_len += 1; } else if note_count <= 24 { med_len += 1; } else if note_count <= 48 { long_len += 1; } else { death_len += 1; }
            let count = s_buffer.len() as f64;
            let mean = s_buffer.iter().sum::<f64>() / count;
            if mean < 0.5 * d_circle { s_p_stack += 1; } else if mean < 1.0 * d_circle { s_p_over += 1; } else if mean < 2.0 * d_circle { s_p_space += 1; } else { s_p_extr += 1; }
            let var = s_buffer.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count;
            let cv = if mean > 0.0 { var.sqrt() / mean } else { 0.0 };
            if cv < 0.15 { v_stead += 1; } else if cv < 0.40 { v_vari += 1; } else { v_dyna += 1; }
            for &dist in &s_buffer {
                s_total_dist += dist; s_total_gaps += 1;
                if dist < 0.5 * d_circle { s_n_stack += 1.0; } else if dist < 1.0 * d_circle { s_n_over += 1.0; } else if dist < 2.0 * d_circle { s_n_space += 1.0; } else { s_n_extr += 1.0; }
            }
        }
    } else {
        for &dist in &s_buffer { process_jump(dist); }
    }

    // --- PACKAGING ---
    let avg_j = if j_cnt > 0 { j_dist / j_cnt as f64 } else { 0.0 };
    let avg_s = if s_total_gaps > 0 { s_total_dist / s_total_gaps as f64 } else { 0.0 };

    let mut j_val = serde_json::to_value(analyze::Jump::new(map.clone()).analyze()).unwrap();
    {
        let j_obj = j_val.as_object_mut().unwrap();
        j_obj.insert("circle_diameter".to_string(), serde_json::to_value(d_circle).unwrap());
        j_obj.insert("avg_spacing".to_string(), serde_json::to_value(avg_j).unwrap());
        j_obj.insert("narrow_count".to_string(), serde_json::to_value(n_cnt).unwrap());
        j_obj.insert("moderate_count".to_string(), serde_json::to_value(m_cnt).unwrap());
        j_obj.insert("wide_count".to_string(), serde_json::to_value(w_cnt).unwrap());
        j_obj.insert("extreme_count".to_string(), serde_json::to_value(e_cnt).unwrap());
        j_obj.insert("narrow_dens".to_string(), serde_json::to_value(n_cnt as f64 / total_obj_count).unwrap());
        j_obj.insert("moderate_dens".to_string(), serde_json::to_value(m_cnt as f64 / total_obj_count).unwrap());
        j_obj.insert("wide_dens".to_string(), serde_json::to_value(w_cnt as f64 / total_obj_count).unwrap());
        j_obj.insert("extreme_dens".to_string(), serde_json::to_value(e_cnt as f64 / total_obj_count).unwrap());
    }

    let mut s_val = serde_json::to_value(analyze::Stream::new(map).analyze()).unwrap();
    {
        let s_obj = s_val.as_object_mut().unwrap();
        s_obj.insert("circle_diameter".to_string(), serde_json::to_value(d_circle).unwrap());
        s_obj.insert("avg_stream_spacing".to_string(), serde_json::to_value(avg_s).unwrap());
        s_obj.insert("s_stacked_count".to_string(), serde_json::to_value(s_p_stack).unwrap());
        s_obj.insert("s_overlapping_count".to_string(), serde_json::to_value(s_p_over).unwrap());
        s_obj.insert("s_spaced_count".to_string(), serde_json::to_value(s_p_space).unwrap());
        s_obj.insert("s_extreme_count".to_string(), serde_json::to_value(s_p_extr).unwrap());
        s_obj.insert("s_stack_dens".to_string(), serde_json::to_value(s_n_stack / total_obj_count).unwrap());
        s_obj.insert("s_over_dens".to_string(), serde_json::to_value(s_n_over / total_obj_count).unwrap());
        s_obj.insert("s_space_dens".to_string(), serde_json::to_value(s_n_space / total_obj_count).unwrap());
        s_obj.insert("s_extr_dens".to_string(), serde_json::to_value(s_n_extr / total_obj_count).unwrap());
        s_obj.insert("v_steady_count".to_string(), serde_json::to_value(v_stead).unwrap());
        s_obj.insert("v_variable_count".to_string(), serde_json::to_value(v_vari).unwrap());
        s_obj.insert("v_dynamic_count".to_string(), serde_json::to_value(v_dyna).unwrap());
        s_obj.insert("total_stream_patterns".to_string(), serde_json::to_value(short_len + med_len + long_len + death_len).unwrap());
        s_obj.insert("bursts".to_string(), serde_json::to_value(bursts).unwrap());
        s_obj.insert("short_streams".to_string(), serde_json::to_value(short_len).unwrap());
        s_obj.insert("medium_streams".to_string(), serde_json::to_value(med_len).unwrap());
        s_obj.insert("long_streams".to_string(), serde_json::to_value(long_len).unwrap());
        s_obj.insert("death_streams".to_string(), serde_json::to_value(death_len).unwrap());
        s_obj.insert("max_stream_length".to_string(), serde_json::to_value(max_stream_length).unwrap());
    }

    match analyze_type.to_lowercase().as_str() {
        "all" => Ok(reply::with_status(reply::json(&vec![
            AnalysisResult { analysis_type: String::from("jump"), analysis: j_val },
            AnalysisResult { analysis_type: String::from("stream"), analysis: s_val },
        ]), StatusCode::OK)),
        "jump" => Ok(reply::with_status(reply::json(&AnalysisResult { analysis_type: String::from("jump"), analysis: j_val }), StatusCode::OK)),
        "stream" => Ok(reply::with_status(reply::json(&AnalysisResult { analysis_type: String::from("stream"), analysis: s_val }), StatusCode::OK)),
        _ => Ok(reply::with_status(reply::json(&ApiError { error: "Bad request".to_string() }), StatusCode::BAD_REQUEST)),
    }
}