use std::sync::Arc;

use crate::api::get;
use axum::{routing::get, Extension, Router};
use rosu_v2::Osu as OsuClient;

pub fn router(osu_client: Arc<OsuClient>) -> Router {
    Router::new()
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
