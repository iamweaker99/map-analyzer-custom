use axum::{extract::Path, http::StatusCode, Json, Extension, response::IntoResponse};
use md5;
use osu_map_analyzer::rosu_map;
use rosu_pp::{Beatmap, Difficulty};
use rosu_v2::{prelude::RankStatus, Osu as OsuClient};
use serde::Serialize;
use serde_json::Value;
use std::{
    fs::File,
    io::{ErrorKind, Read},
    path::Path as FilePath,
    str::FromStr,
    sync::Arc,
};

use crate::analysis;
use crate::utils::download_beatmap;

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
    Path(beatmap_id): Path<u32>,
    Extension(osu_client): Extension<Arc<OsuClient>>,
) -> impl IntoResponse {
    let beatmap = match osu_client.beatmap().map_id(beatmap_id).await {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("Error while fetching beatmap: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Error while fetching beatmap: {}", err) })),
            );
        }
    };

    let beatmapset = match beatmap.mapset {
        Some(s) => s,
        None => {
            eprintln!("Couldn't get beatmapset from beatmap (wtf?)");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Couldn't get beatmapset from beatmap (wtf?)" })),
            );
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
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": format!("Error while converting bytes to string: {}", err) })),
                    );
                }
            },
            Err(err) => {
                eprintln!("Error while downloading beatmap: {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": format!("Error while downloading beatmap: {}", err) })),
                );
            }
        }
    } else {
        match File::open(format!("maps/{}.osu", beatmap_id)) {
            Ok(mut file) => {
                let mut data_buf = String::new();

                if let Err(why) = file.read_to_string(&mut data_buf) {
                    eprintln!("Error while reading file: {}", why);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": format!("Error while reading file: {}", why) })),
                    );
                }

                data_buf
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => match download_beatmap(beatmap_id).await {
                    Ok(bytes) => match String::from_utf8(bytes.to_vec()) {
                        Ok(string) => string,
                        Err(err) => {
                            eprintln!("Error while converting bytes to string: {}", err);
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(serde_json::json!({ "error": format!("Error while converting bytes to string: {}", err) })),
                            );
                        }
                    },
                    Err(err) => {
                        eprintln!("Error while downloading beatmap: {}", err);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({ "error": format!("Error while downloading beatmap: {}", err) })),
                        );
                    }
                },

                _ => {
                    eprintln!("Internal server error: {}", err);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": format!("Internal server error: {}", err) })),
                    );
                }
            },
        }
    };

    let map_calculate = match Beatmap::from_str(&map_file) {
        Ok(map) => map,
        Err(err) => {
            eprintln!("Error parsing beatmap: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Error parsing beatmap: {}", err) })),
            );
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

    (
        StatusCode::OK,
        Json(serde_json::to_value(DetailsResult {
            title: beatmapset.title,
            artist: beatmapset.artist,
            creator: beatmapset.creator_name.to_string(),
            creator_id: beatmapset.creator_id,
            version: beatmap.version,
            set_id: beatmapset.mapset_id,
            statistics,
        })
        .unwrap()),
    )
}

#[derive(Serialize)]
struct AnalysisResult {
    analysis_type: String,
    analysis: Value,
}

pub async fn analyze_beatmap(
    Path((beatmap_id, analyze_type)): Path<(u32, String)>,
) -> impl IntoResponse {
    let path = FilePath::new("maps").join(format!("{}.osu", beatmap_id));

    // Download the map if it doesn't exist on disk yet
    if !path.exists() {
        if let Err(e) = download_beatmap(beatmap_id).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Failed to download beatmap: {}", e) })),
            );
        }
    }

    // 1. Calculate MD5 for the Chart Reset Key
    let md5_string = match std::fs::read(&path) {
        Ok(bytes) => format!("{:x}", md5::compute(bytes)),
        Err(_) => "unknown-md5".to_string(),
    };

    let map = match rosu_map::from_path::<rosu_map::Beatmap>(&path) {
        Ok(m) => m,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Failed to parse map: {}", e) })),
            );
        }
    };
    let pp_map = match rosu_pp::Beatmap::from_path(&path) {
        Ok(m) => m,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to parse PP map".to_string() })),
            );
        }
    };

    let movements = analysis::create_movements(&pp_map);
    let total_obj = pp_map.hit_objects.len() as f64;
    let bpm = pp_map.bpm();
    let cs = pp_map.cs;

    let results = match analyze_type.to_lowercase().as_str() {
        "all" => {
            let j_val = analysis::jumps::analyze(&movements, cs, bpm, total_obj);
            let s_val = analysis::streams::analyze(&movements, cs, bpm, total_obj);
            let sl_val = analysis::sliders::analyze(&map, cs, total_obj);

            let fc_raw = analysis::finger_control::analyze(&pp_map, md5_string.clone());
            let fc_val = serde_json::to_value(fc_raw).unwrap_or(serde_json::Value::Null);

            let ac_val = analysis::aim_control::analyze(&pp_map, cs);

            let reading_val = analysis::reading::analyze(&pp_map);

            vec![
                AnalysisResult { analysis_type: String::from("jump"), analysis: j_val },
                AnalysisResult { analysis_type: String::from("stream"), analysis: s_val },
                AnalysisResult { analysis_type: String::from("slider"), analysis: sl_val },
                AnalysisResult { analysis_type: String::from("fingercontrol"), analysis: fc_val },
                AnalysisResult { analysis_type: String::from("aimcontrol"), analysis: ac_val },
                AnalysisResult { analysis_type: String::from("reading"), analysis: reading_val },
            ]
        }
        "aimcontrol" => {
            let ac_val = analysis::aim_control::analyze(&pp_map, cs);
            vec![AnalysisResult {
                analysis_type: String::from("aimcontrol"),
                analysis: ac_val,
            }]
        }
        "fingercontrol" => {
            let fc_raw = analysis::finger_control::analyze(&pp_map, md5_string);
            let fc_val = serde_json::to_value(fc_raw).unwrap_or(serde_json::Value::Null);
            vec![AnalysisResult {
                analysis_type: String::from("fingercontrol"),
                analysis: fc_val,
            }]
        }
        "reading" => {
            let reading_val = analysis::reading::analyze(&pp_map);
            vec![AnalysisResult {
                analysis_type: String::from("reading"),
                analysis: reading_val,
            }]
        }
        "jump" => {
            let j_val = analysis::jumps::analyze(&movements, cs, bpm, total_obj);
            vec![AnalysisResult { analysis_type: String::from("jump"), analysis: j_val }]
        }
        "stream" => {
            let s_val = analysis::streams::analyze(&movements, cs, bpm, total_obj);
            vec![AnalysisResult { analysis_type: String::from("stream"), analysis: s_val }]
        }
        "slider" => {
            let sl_val = analysis::sliders::analyze(&map, cs, total_obj);
            vec![AnalysisResult { analysis_type: String::from("slider"), analysis: sl_val }]
        }
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Bad request: analyze_type must be all, jump, stream, slider, fingercontrol, aimcontrol, or reading" })),
            );
        }
    };

    (StatusCode::OK, Json(serde_json::to_value(results).unwrap()))
}
