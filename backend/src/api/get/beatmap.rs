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
    let map = rosu_map::from_path::<rosu_map::Beatmap>(path).unwrap();

    match analyze_type.to_lowercase().as_str() {
        "all" | "jump" | "stream" => {
            let total_obj_count = map.hit_objects.len() as f64;
            
            // --- JUMP LOGIC ---
            let mut j_total_dist = 0.0;
            let mut j_count = 0;
            let mut n_cnt = 0; let mut m_cnt = 0; let mut w_cnt = 0; let mut e_cnt = 0;

            // --- STREAM LOGIC ---
            let mut s_stack = 0; let mut s_over = 0; let mut s_space = 0; let mut s_extr = 0;
            let mut v_stead = 0; let mut v_vari = 0; let mut v_dyna = 0;
            let mut s_total_dist = 0.0; let mut s_total_gaps = 0;
            let mut s_buffer: Vec<f64> = Vec::new();

            for window in map.hit_objects.windows(2) {
                let time_diff = window[1].start_time - window[0].start_time;
                
                let p1 = match &window[0].kind {
                    rosu_map::section::hit_objects::HitObjectKind::Circle(c) => Some(c.pos),
                    rosu_map::section::hit_objects::HitObjectKind::Slider(s) => Some(s.pos),
                    _ => None,
                };
                let p2 = match &window[1].kind {
                    rosu_map::section::hit_objects::HitObjectKind::Circle(c) => Some(c.pos),
                    rosu_map::section::hit_objects::HitObjectKind::Slider(s) => Some(s.pos),
                    _ => None,
                };

                if let (Some(pos1), Some(pos2)) = (p1, p2) {
                    let dx = (pos2.x - pos1.x) as f64;
                    let dy = (pos2.y - pos1.y) as f64;
                    let d = (dx * dx + dy * dy).sqrt();

                    if d > 0.0 {
                        // JUMP PROCESSING
                        j_total_dist += d;
                        j_count += 1;
                        if d < 120.0 { n_cnt += 1; }
                        else if d < 190.0 { m_cnt += 1; }
                        else if d < 280.0 { w_cnt += 1; }
                        else { e_cnt += 1; }

                        // STREAM PROCESSING (Time Threshold)
                        if time_diff <= 125.0 {
                            s_buffer.push(d);
                            s_total_dist += d;
                            s_total_gaps += 1;
                            if d < 10.0 { s_stack += 1; }
                            else if d < 30.0 { s_over += 1; }
                            else if d < 60.0 { s_space += 1; }
                            else { s_extr += 1; }
                        } else {
                            // End Stream Session
                            if s_buffer.len() >= 2 {
                                let count = s_buffer.len() as f64;
                                let mean = s_buffer.iter().sum::<f64>() / count;
                                let var = s_buffer.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count;
                                let cv = if mean > 0.0 { var.sqrt() / mean } else { 0.0 };
                                if cv < 0.15 { v_stead += s_buffer.len(); }
                                else if cv < 0.40 { v_vari += s_buffer.len(); }
                                else { v_dyna += s_buffer.len(); }
                            }
                            s_buffer.clear();
                        }
                    }
                }
            }

            let avg_j = if j_count > 0 { j_total_dist / j_count as f64 } else { 0.0 };
            let avg_s = if s_total_gaps > 0 { s_total_dist / s_total_gaps as f64 } else { 0.0 };

            let jump_analysis = analyze::Jump::new(map.clone()).analyze();
            let stream_analysis = analyze::Stream::new(map).analyze();

            let mut j_val = serde_json::to_value(jump_analysis).unwrap();
            let j_obj = j_val.as_object_mut().unwrap();
            j_obj.insert("avg_spacing".to_string(), serde_json::to_value(avg_j).unwrap());
            j_obj.insert("narrow_count".to_string(), serde_json::to_value(n_cnt).unwrap());
            j_obj.insert("moderate_count".to_string(), serde_json::to_value(m_cnt).unwrap());
            j_obj.insert("wide_count".to_string(), serde_json::to_value(w_cnt).unwrap());
            j_obj.insert("extreme_count".to_string(), serde_json::to_value(e_cnt).unwrap());
            j_obj.insert("narrow_dens".to_string(), serde_json::to_value(n_cnt as f64 / total_obj_count).unwrap());
            j_obj.insert("moderate_dens".to_string(), serde_json::to_value(m_cnt as f64 / total_obj_count).unwrap());
            j_obj.insert("wide_dens".to_string(), serde_json::to_value(w_cnt as f64 / total_obj_count).unwrap());
            j_obj.insert("extreme_dens".to_string(), serde_json::to_value(e_cnt as f64 / total_obj_count).unwrap());

            let mut s_val = serde_json::to_value(stream_analysis).unwrap();
            let s_obj = s_val.as_object_mut().unwrap();
            s_obj.insert("avg_stream_spacing".to_string(), serde_json::to_value(avg_s).unwrap());
            s_obj.insert("s_stacked_count".to_string(), serde_json::to_value(s_stack).unwrap());
            s_obj.insert("s_overlapping_count".to_string(), serde_json::to_value(s_over).unwrap());
            s_obj.insert("s_spaced_count".to_string(), serde_json::to_value(s_space).unwrap());
            s_obj.insert("s_extreme_count".to_string(), serde_json::to_value(s_extr).unwrap());
            s_obj.insert("s_stack_dens".to_string(), serde_json::to_value(s_stack as f64 / total_obj_count).unwrap());
            s_obj.insert("s_over_dens".to_string(), serde_json::to_value(s_over as f64 / total_obj_count).unwrap());
            s_obj.insert("s_space_dens".to_string(), serde_json::to_value(s_space as f64 / total_obj_count).unwrap());
            s_obj.insert("s_extr_dens".to_string(), serde_json::to_value(s_extr as f64 / total_obj_count).unwrap());
            s_obj.insert("v_steady_count".to_string(), serde_json::to_value(v_stead).unwrap());
            s_obj.insert("v_variable_count".to_string(), serde_json::to_value(v_vari).unwrap());
            s_obj.insert("v_dynamic_count".to_string(), serde_json::to_value(v_dyna).unwrap());

            if analyze_type.to_lowercase() == "jump" {
                Ok(reply::with_status(reply::json(&AnalysisResult { analysis_type: String::from("jump"), analysis: j_val }), StatusCode::OK))
            } else if analyze_type.to_lowercase() == "stream" {
                Ok(reply::with_status(reply::json(&AnalysisResult { analysis_type: String::from("stream"), analysis: s_val }), StatusCode::OK))
            } else {
                Ok(reply::with_status(reply::json(&vec![
                    AnalysisResult { analysis_type: String::from("jump"), analysis: j_val },
                    AnalysisResult { analysis_type: String::from("stream"), analysis: s_val },
                ]), StatusCode::OK))
            }
        }
        _ => Ok(reply::with_status(reply::json(&ApiError { error: "Bad request".to_string() }), StatusCode::BAD_REQUEST)),
    }
}