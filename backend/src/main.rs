mod api;
mod models;
mod routes;
mod utils;
mod analysis;

use dotenvy::from_filename;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    // Load environment variables (safe to call even if file doesn't exist)
    from_filename(".env.local").ok();

    let osu_client_id: u64 = env::var("OSU_CLIENT_ID")
        .expect("Expected OSU_CLIENT_ID to be defined in environment.")
        .parse()
        .expect("OSU_CLIENT_ID is not a number!");

    let osu_client_secret = env::var("OSU_CLIENT_SECRET")
        .expect("Expected OSU_CLIENT_SECRET to be defined in environment.");

    let osu_client = Arc::new(
        rosu_v2::Osu::new(osu_client_id, osu_client_secret)
            .await
            .unwrap(),
    );

    let router = routes::router(osu_client);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT is not a number!");

    let addr = format!("0.0.0.0:{}", port);
    println!("Server started at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");
    axum::serve(listener, router)
        .await
        .expect("Server error");
}
