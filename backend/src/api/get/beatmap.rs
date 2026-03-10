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
    
    let bpm = pp_map.bpm();
    let stream_threshold = (60000.0 / bpm / 4.0) * 1.5;
    let total_obj_count = map.hit_objects.len() as f64;
    let d_circle = 108.8 - (8.96 * pp_map.cs as f64);

 // --- MOVE ALL THESE TO THE TOP (BEFORE THE MATCH BLOCK) ---
    let mut j_dist = 0.0; let mut j_cnt = 0;
    let mut n_cnt = 0; let mut m_cnt = 0; let mut w_cnt = 0; let mut e_cnt = 0;
    
    let mut max_stream_length = 0;
    let mut bursts = 0; let mut short_len = 0; let mut med_len = 0; let mut long_len = 0; let mut death_len = 0;
    let mut s_p_stack = 0; let mut s_p_over = 0; let mut s_p_space = 0; let mut s_p_extr = 0;
    let mut s_n_stack = 0.0; let mut s_n_over = 0.0; let mut s_n_space = 0.0; let mut s_n_extr = 0.0;
    let mut v_stead = 0; let mut v_vari = 0; let mut v_dyna = 0;
    let mut s_total_dist = 0.0; let mut s_total_gaps = 0;
    let mut s_buffer: Vec<f64> = Vec::new();
    
    let mut sl_count = 0; let mut total_sv = 0.0;
    let mut l_short = 0; let mut l_med = 0; let mut l_long = 0; let mut l_ext = 0;
    let mut b_buzz = 0; let mut b_static = 0;
    let mut a_simple = 0; let mut a_curved = 0; let mut a_complex = 0; let mut a_art = 0;
    // -----------------------------------------------------------

    match analyze_type.to_lowercase().as_str() {

        "all" => {
            // --- UNIFIED ANALYSIS LOOP ---
            for window in pp_map.hit_objects.windows(2) {
                let obj1 = &window[0];
                let obj2 = &window[1];
                let time_diff = obj2.start_time - obj1.start_time;
                let d = {
                    let dx = (obj2.pos.x - obj1.pos.x) as f64;
                    let dy = (obj2.pos.y - obj1.pos.y) as f64;
                    (dx * dx + dy * dy).sqrt()
                };

                if d > 0.0 {
                    // 1. Jump Logic (Any gap that isn't a stream)
                    if time_diff > stream_threshold || d > 2.5 * d_circle {
                        j_dist += d; j_cnt += 1;
                        if d < 2.0 * d_circle { n_cnt += 1; }
                        else if d < 3.5 * d_circle { m_cnt += 1; }
                        else if d < 5.0 * d_circle { w_cnt += 1; }
                        else { e_cnt += 1; }
                    }

                    // 2. Stream Logic
                    if time_diff <= stream_threshold && d <= 2.5 * d_circle {
                        s_buffer.push(d);
                    } else {
                        if s_buffer.len() >= 4 { // 5+ notes
                            let note_count = s_buffer.len() + 1;
                            if note_count > max_stream_length { max_stream_length = note_count; }
                            if note_count <= 12 { short_len += 1; }
                            else if note_count <= 24 { med_len += 1; }
                            else if note_count <= 48 { long_len += 1; }
                            else { death_len += 1; }

                            let count = s_buffer.len() as f64;
                            let mean = s_buffer.iter().sum::<f64>() / count;
                            if mean < 0.5 * d_circle { s_p_stack += 1; }
                            else if mean < 1.0 * d_circle { s_p_over += 1; }
                            else if mean < 2.0 * d_circle { s_p_space += 1; }
                            else { s_p_extr += 1; }

                            let var = s_buffer.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count;
                            let cv = if mean > 0.0 { var.sqrt() / mean } else { 0.0 };
                            if cv < 0.15 { v_stead += 1; }
                            else if cv < 0.40 { v_vari += 1; }
                            else { v_dyna += 1; }

                            for &dist in &s_buffer {
                                s_total_dist += dist; s_total_gaps += 1;
                                if dist < 0.5 * d_circle { s_n_stack += 1.0; }
                                else if dist < 1.0 * d_circle { s_n_over += 1.0; }
                                else if dist < 2.0 * d_circle { s_n_space += 1.0; }
                                else { s_n_extr += 1.0; }
                            }
                        } else if s_buffer.len() >= 2 { // 3-4 notes
                            // THIS LINE FIXES THE WARNING:
                            bursts += 1;     
                            // Triplets/Bursts are treated as jumps
                            for &dist in &s_buffer {
                                j_dist += dist; j_cnt += 1;
                                if dist < 2.0 * d_circle { n_cnt += 1; }
                                else if dist < 3.5 * d_circle { m_cnt += 1; }
                                else if dist < 5.0 * d_circle { w_cnt += 1; }
                                else { e_cnt += 1; }
                            }
                        }
                        s_buffer.clear();
                    }
                }
            }

            // --- 3. SLIDER ANALYSIS ---
            for obj in &map.hit_objects {
                if let rosu_map::section::hit_objects::HitObjectKind::Slider(s) = &obj.kind {
                    sl_count += 1;
                    let body_len = s.path.expected_dist().unwrap_or(0.0);
                    if body_len < 1.5 * d_circle { l_short += 1; }
                    else if body_len < 3.0 * d_circle { l_med += 1; }
                    else if body_len < 4.5 * d_circle { l_long += 1; }
                    else { l_ext += 1; }

                    if s.repeat_count > 0 {
                        if body_len < 5.0 { b_static += 1; } else { b_buzz += 1; }
                    }

                    let points = s.path.control_points().len();
                    if points <= 2 { a_simple += 1; }
                    else if points <= 4 { a_curved += 1; }
                    else if points <= 10 { a_complex += 1; }
                    else { a_art += 1; }
                    total_sv += body_len / 100.0; 
                }
            }

            let slider_ratio = sl_count as f64 / total_obj_count;
            let avg_sv = if sl_count > 0 { total_sv / sl_count as f64 } else { 0.0 };
            let sl_f = sl_count as f64;

            let mut j_val = serde_json::to_value(analyze::Jump::new(map.clone()).analyze()).unwrap();
            let mut s_val = serde_json::to_value(analyze::Stream::new(map.clone()).analyze()).unwrap();
            let mut sl_val = serde_json::to_value(analyze::Stream::new(map).analyze()).unwrap();

            if let Some(j_obj) = j_val.as_object_mut() {
                j_obj.insert("overall_confidence".to_string(), serde_json::to_value(j_cnt as f64 / total_obj_count).unwrap());
                j_obj.insert("avg_spacing".to_string(), serde_json::to_value(if j_cnt > 0 { j_dist / j_cnt as f64 } else { 0.0 }).unwrap());
                j_obj.insert("narrow_count".to_string(), serde_json::to_value(n_cnt).unwrap());
                j_obj.insert("moderate_count".to_string(), serde_json::to_value(m_cnt).unwrap());
                j_obj.insert("wide_count".to_string(), serde_json::to_value(w_cnt).unwrap());
                j_obj.insert("extreme_count".to_string(), serde_json::to_value(e_cnt).unwrap());
                j_obj.insert("narrow_dens".to_string(), serde_json::to_value(n_cnt as f64 / total_obj_count).unwrap());
                j_obj.insert("moderate_dens".to_string(), serde_json::to_value(m_cnt as f64 / total_obj_count).unwrap());
                j_obj.insert("wide_dens".to_string(), serde_json::to_value(w_cnt as f64 / total_obj_count).unwrap());
                j_obj.insert("extreme_dens".to_string(), serde_json::to_value(e_cnt as f64 / total_obj_count).unwrap());
            }

            if let Some(s_obj) = s_val.as_object_mut() {
                s_obj.insert("overall_confidence".to_string(), serde_json::to_value(s_total_gaps as f64 / total_obj_count).unwrap());
                s_obj.insert("avg_stream_spacing".to_string(), serde_json::to_value(if s_total_gaps > 0 { s_total_dist / s_total_gaps as f64 } else { 0.0 }).unwrap());
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

            if let Some(sl_obj) = sl_val.as_object_mut() {
                sl_obj.insert("overall_confidence".to_string(), serde_json::to_value(slider_ratio).unwrap());
                sl_obj.insert("slider_ratio".to_string(), serde_json::to_value(slider_ratio).unwrap());
                sl_obj.insert("avg_velocity".to_string(), serde_json::to_value(avg_sv).unwrap());
                sl_obj.insert("l_short_count".to_string(), serde_json::to_value(l_short).unwrap());
                sl_obj.insert("l_short_dens".to_string(), serde_json::to_value(l_short as f64 / total_obj_count).unwrap());
                sl_obj.insert("l_med_count".to_string(), serde_json::to_value(l_med).unwrap());
                sl_obj.insert("l_med_dens".to_string(), serde_json::to_value(l_med as f64 / total_obj_count).unwrap());
                sl_obj.insert("l_long_count".to_string(), serde_json::to_value(l_long).unwrap());
                sl_obj.insert("l_long_dens".to_string(), serde_json::to_value(l_long as f64 / total_obj_count).unwrap());
                sl_obj.insert("l_ext_count".to_string(), serde_json::to_value(l_ext).unwrap());
                sl_obj.insert("l_ext_dens".to_string(), serde_json::to_value(l_ext as f64 / total_obj_count).unwrap());
                sl_obj.insert("b_buzz_count".to_string(), serde_json::to_value(b_buzz).unwrap());
                sl_obj.insert("b_buzz_dens".to_string(), serde_json::to_value(if sl_f > 0.0 { b_buzz as f64 / sl_f } else { 0.0 }).unwrap());
                sl_obj.insert("b_static_count".to_string(), serde_json::to_value(b_static).unwrap());
                sl_obj.insert("b_static_dens".to_string(), serde_json::to_value(if sl_f > 0.0 { b_static as f64 / sl_f } else { 0.0 }).unwrap());
                sl_obj.insert("a_simple_count".to_string(), serde_json::to_value(a_simple).unwrap());
                sl_obj.insert("a_simple_dens".to_string(), serde_json::to_value(if sl_f > 0.0 { a_simple as f64 / sl_f } else { 0.0 }).unwrap());
                sl_obj.insert("a_curved_count".to_string(), serde_json::to_value(a_curved).unwrap());
                sl_obj.insert("a_curved_dens".to_string(), serde_json::to_value(if sl_f > 0.0 { a_curved as f64 / sl_f } else { 0.0 }).unwrap());
                sl_obj.insert("a_complex_count".to_string(), serde_json::to_value(a_complex).unwrap());
                sl_obj.insert("a_complex_dens".to_string(), serde_json::to_value(if sl_f > 0.0 { a_complex as f64 / sl_f } else { 0.0 }).unwrap());
                sl_obj.insert("a_artistic_count".to_string(), serde_json::to_value(a_art).unwrap());
                sl_obj.insert("a_artistic_dens".to_string(), serde_json::to_value(if sl_f > 0.0 { a_art as f64 / sl_f } else { 0.0 }).unwrap());
            }

            Ok(reply::with_status(reply::json(&vec![
                AnalysisResult { analysis_type: String::from("jump"), analysis: j_val },
                AnalysisResult { analysis_type: String::from("stream"), analysis: s_val },
                AnalysisResult { analysis_type: String::from("slider"), analysis: sl_val },
            ]), StatusCode::OK))
        }
        _ => Ok(reply::with_status(reply::json(&ApiError { error: "Bad request".to_string() }), StatusCode::BAD_REQUEST)),
    }
}