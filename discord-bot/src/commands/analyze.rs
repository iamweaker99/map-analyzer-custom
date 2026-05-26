use std::collections::HashMap;
use std::sync::Arc;

use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CreateActionRow, CreateButton,
    EditInteractionResponse,
};
use serenity::client::Context;
use tokio::sync::Mutex;

use crate::api::BackendApi;
use crate::embeds;
use crate::types::*;

pub type AnalysisCache = Arc<Mutex<HashMap<String, AnalysisState>>>;

pub struct AnalysisState {
    pub page: usize,
    pub total_pages: usize,
    pub details: DetailsResult,
    pub results: Vec<AnalysisResult>,
}

fn parse_beatmap_id(input: &str) -> Option<u32> {
    if let Ok(id) = input.parse::<u32>() {
        return Some(id);
    }
    if let Some(pos) = input.find("/b/") {
        let rest = &input[pos + 3..];
        let id_str = rest.split(&['/', '?', ' '][..]).next().unwrap_or("");
        return id_str.parse::<u32>().ok();
    }
    if let Some(pos) = input.find("#osu/") {
        let rest = &input[pos + 5..];
        let id_str = rest.split(&['/', '?', ' '][..]).next().unwrap_or("");
        return id_str.parse::<u32>().ok();
    }
    None
}

pub async fn run(ctx: &Context, command: CommandInteraction) {
    // Defer the response since analysis can take a while
    if let Err(e) = command.defer(&ctx.http).await {
        tracing::error!("Failed to defer: {}", e);
        return;
    }

    // Get the beatmap argument
    let beatmap_input = command
        .data
        .options
        .first()
        .and_then(|opt| match &opt.value {
            CommandDataOptionValue::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_default();

    let beatmap_id = match parse_beatmap_id(&beatmap_input) {
        Some(id) => id,
        None => {
            let _ = command
                .edit_response(
                    &ctx.http,
                    EditInteractionResponse::new()
                        .content("Invalid beatmap. Please provide a beatmap ID or URL (e.g. `https://osu.ppy.sh/b/123456`)."),
                )
                .await;
            return;
        }
    };

    let cache = ctx
        .data
        .read()
        .await
        .get::<crate::SharedCache>()
        .cloned()
        .expect("SharedCache not registered");

    let backend_url = ctx
        .data
        .read()
        .await
        .get::<crate::BackendUrl>()
        .cloned()
        .expect("BackendUrl not registered");

    let api = BackendApi::new(backend_url);

    let (details_res, analysis_res) =
        tokio::join!(api.fetch_details(beatmap_id), api.fetch_analysis(beatmap_id));

    let details = match details_res {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("Failed to fetch details: {}", e);
            let _ = command
                .edit_response(
                    &ctx.http,
                    EditInteractionResponse::new()
                        .content(format!("Error fetching beatmap details: {}", e)),
                )
                .await;
            return;
        }
    };

    let results = match analysis_res {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to fetch analysis: {}", e);
            let _ = command
                .edit_response(
                    &ctx.http,
                    EditInteractionResponse::new()
                        .content(format!("Error analyzing beatmap: {}", e)),
                )
                .await;
            return;
        }
    };

    let state = AnalysisState {
        page: 1,
        total_pages: 7,
        details: details.clone(),
        results: results.clone(),
    };

    let cache_key = format!("{:x}", rand_id());
    {
        let mut cache = cache.lock().await;
        cache.insert(cache_key.clone(), state);
    }

    let embed = build_embed(&details, &results, 1, 7);
    let components = build_components(&cache_key, 1, 7);

    let _ = command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new()
                .add_embed(embed)
                .components(components),
        )
        .await;
}

pub fn build_embed(
    details: &DetailsResult,
    results: &[AnalysisResult],
    page: usize,
    total_pages: usize,
) -> serenity::all::CreateEmbed {
    match page {
        1 => embeds::overview::build(details, results),
        2 => {
            parse_analysis::<JumpAnalysis>(results, "jump")
                .map(|data| embeds::jump::build(&data, page, total_pages))
                .unwrap_or_else(|| {
                    serenity::all::CreateEmbed::new()
                        .title("Jump Analysis")
                        .description("Data unavailable.")
                        .color(0xec4899)
                })
        }
        3 => {
            parse_analysis::<StreamAnalysis>(results, "stream")
                .map(|data| embeds::stream::build(&data, page, total_pages))
                .unwrap_or_else(|| {
                    serenity::all::CreateEmbed::new()
                        .title("Stream Analysis")
                        .description("Data unavailable.")
                        .color(0x3b82f6)
                })
        }
        4 => {
            parse_analysis::<SliderAnalysis>(results, "slider")
                .map(|data| embeds::slider::build(&data, page, total_pages))
                .unwrap_or_else(|| {
                    serenity::all::CreateEmbed::new()
                        .title("Slider Analysis")
                        .description("Data unavailable.")
                        .color(0x22c55e)
                })
        }
        5 => {
            parse_analysis::<FingerControlAnalysis>(results, "fingercontrol")
                .map(|data| embeds::finger_control::build(&data, page, total_pages))
                .unwrap_or_else(|| {
                    serenity::all::CreateEmbed::new()
                        .title("Finger Control Analysis")
                        .description("Data unavailable.")
                        .color(0xa855f7)
                })
        }
        6 => {
            parse_analysis::<AimControlResult>(results, "aimcontrol")
                .map(|data| embeds::aim_control::build(&data, page, total_pages))
                .unwrap_or_else(|| {
                    serenity::all::CreateEmbed::new()
                        .title("Aim Control Analysis")
                        .description("Data unavailable.")
                        .color(0xf97316)
                })
        }
        7 => {
            parse_analysis::<ReadingResult>(results, "reading")
                .map(|data| embeds::reading::build(&data, page, total_pages))
                .unwrap_or_else(|| {
                    serenity::all::CreateEmbed::new()
                        .title("Reading Analysis")
                        .description("Data unavailable.")
                        .color(0x06b6d4)
                })
        }
        _ => unreachable!(),
    }
}

pub fn build_components(cache_key: &str, page: usize, total_pages: usize) -> Vec<CreateActionRow> {
    let prev_btn = CreateButton::new(format!("nav_{}_prev", cache_key))
        .label("◀ Previous")
        .disabled(page <= 1);

    let indicator_btn = CreateButton::new(format!("nav_{}_indicator", cache_key))
        .label(format!("{}/{}", page, total_pages))
        .disabled(true);

    let next_btn = CreateButton::new(format!("nav_{}_next", cache_key))
        .label("Next ▶")
        .disabled(page >= total_pages);

    vec![CreateActionRow::Buttons(vec![prev_btn, indicator_btn, next_btn])]
}

fn parse_analysis<T: serde::de::DeserializeOwned>(
    results: &[AnalysisResult],
    atype: &str,
) -> Option<T> {
    results
        .iter()
        .find(|r| r.analysis_type == atype)
        .and_then(|r| serde_json::from_value(r.analysis.clone()).ok())
}

fn rand_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64;
    nanos ^ (std::process::id() as u64).wrapping_mul(0x9e3779b97f4a7c15)
}
