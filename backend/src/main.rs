mod api;
mod models;
mod routes;
mod utils;
mod analysis;

use dotenvy::from_filename;
use std::{
    env,
    path::Path,
    sync::Arc,
    time::{Duration, SystemTime},
};

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

    // Clean up any leftover beatmap files from previous runs
    let maps_dir = Path::new("maps");
    if maps_dir.exists() {
        let count = std::fs::read_dir(maps_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().map_or(false, |ext| ext == "osu"))
                    .filter_map(|e| std::fs::remove_file(e.path()).ok())
                    .count()
            })
            .unwrap_or(0);
        if count > 0 {
            println!("Cleaned up {} stale beatmap files.", count);
        }
    }

    // Periodically clean up old .osu files (safety net)
    tokio::spawn(async {
        let mut interval = tokio::time::interval(Duration::from_secs(1800)); // 30 min
        loop {
            interval.tick().await;
            let one_hour_ago = SystemTime::now() - Duration::from_secs(3600);
            if let Ok(entries) = std::fs::read_dir("maps") {
                for entry in entries.flatten() {
                    if entry.path().extension().map_or(false, |ext| ext == "osu") {
                        if let Ok(metadata) = entry.metadata() {
                            if let Ok(modified) = metadata.modified() {
                                if modified < one_hour_ago {
                                    let _ = std::fs::remove_file(entry.path());
                                }
                            }
                        }
                    }
                }
            }
        }
    });

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
