use reqwest::Client;
use serde_json::json;

const DISCORD_API_BASE: &str = "https://discord.com/api/v10";

/// Register all 7 slash commands with Guild Install + User Install support.
///
/// Serenity 0.12 doesn't support the `integration_types` and `contexts`
/// fields on commands, so we register them via a direct HTTP call instead.
pub async fn register_commands(token: &str, application_id: u64) -> Result<(), String> {
    let client = Client::new();

    let beatmap_option = json!({
        "type": 3,
        "name": "beatmap",
        "description": "Beatmap URL or ID",
        "required": true
    });

    let commands = vec![
        json!({
            "name": "analyze-all",
            "description": "Show all analysis sections for an osu! beatmap",
            "options": [beatmap_option.clone()],
            "integration_types": [0, 1],
            "contexts": [0, 1, 2]
        }),
        json!({
            "name": "analyze-jump",
            "description": "Show overview + jump analysis for an osu! beatmap",
            "options": [beatmap_option.clone()],
            "integration_types": [0, 1],
            "contexts": [0, 1, 2]
        }),
        json!({
            "name": "analyze-stream",
            "description": "Show overview + stream analysis for an osu! beatmap",
            "options": [beatmap_option.clone()],
            "integration_types": [0, 1],
            "contexts": [0, 1, 2]
        }),
        json!({
            "name": "analyze-slider",
            "description": "Show overview + slider analysis for an osu! beatmap",
            "options": [beatmap_option.clone()],
            "integration_types": [0, 1],
            "contexts": [0, 1, 2]
        }),
        json!({
            "name": "analyze-finger_ctrl",
            "description": "Show overview + finger control analysis for an osu! beatmap",
            "options": [beatmap_option.clone()],
            "integration_types": [0, 1],
            "contexts": [0, 1, 2]
        }),
        json!({
            "name": "analyze-aim_ctrl",
            "description": "Show overview + aim control analysis for an osu! beatmap",
            "options": [beatmap_option.clone()],
            "integration_types": [0, 1],
            "contexts": [0, 1, 2]
        }),
        json!({
            "name": "analyze-reading",
            "description": "Show overview + reading analysis for an osu! beatmap",
            "options": [beatmap_option],
            "integration_types": [0, 1],
            "contexts": [0, 1, 2]
        }),
    ];

    let url = format!("{}/applications/{}/commands", DISCORD_API_BASE, application_id);

    let response = client
        .put(&url)
        .header("Authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .json(&commands)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Discord API {}: {}", status, body));
    }

    tracing::info!(
        "Registered 7 commands with integration_types=[0,1] contexts=[0,1,2]"
    );
    Ok(())
}
