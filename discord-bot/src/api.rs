use reqwest::Client;
use std::time::Duration;

use crate::types::{AnalysisResult, DetailsResult};

const TIMEOUT_SECS: u64 = 120;

pub struct BackendApi {
    client: Client,
    base_url: String,
}

impl BackendApi {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .build()
            .expect("Failed to create HTTP client");
        Self { client, base_url }
    }

    pub async fn fetch_details(&self, beatmap_id: u32) -> Result<DetailsResult, String> {
        let url = format!("{}/api/beatmaps/{}/details", self.base_url, beatmap_id);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Backend returned {}: {}", status, body));
        }

        resp.json()
            .await
            .map_err(|e| format!("Failed to parse details: {}", e))
    }

    pub async fn fetch_analysis(&self, beatmap_id: u32) -> Result<Vec<AnalysisResult>, String> {
        let url = format!("{}/api/beatmaps/{}/analyze/all", self.base_url, beatmap_id);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Backend returned {}: {}", status, body));
        }

        resp.json()
            .await
            .map_err(|e| format!("Failed to parse analysis: {}", e))
    }
}
