use std::sync::Arc;

use axum::{response::Json, routing::get, Extension, Router};
use serde_json::json;
use rosu_v2::Osu as OsuClient;

use crate::api::get;

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

pub fn router(osu_client: Arc<OsuClient>) -> Router {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route(
            "/api/beatmaps/{id}/analyze/{type}",
            get(get::analyze_beatmap),
        )
        .route(
            "/api/beatmaps/{id}/details",
            get(get::beatmap_details),
        )
        .layer(Extension(osu_client))
}
