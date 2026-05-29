use serenity::all::{CommandDataOptionValue, CommandInteraction, EditInteractionResponse};
use serenity::client::Context;

use crate::api::BackendApi;
use crate::embeds;
use crate::types::*;

pub struct AnalysisState {
    pub jump: Option<JumpAnalysis>,
    pub stream: Option<StreamAnalysis>,
    pub slider: Option<SliderAnalysis>,
    pub finger_control: Option<FingerControlAnalysis>,
    pub aim_control: Option<AimControlResult>,
    pub reading: Option<ReadingResult>,
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

/// Shared analysis logic: fetch data, parse, build and send embeds for the requested sections.
async fn run_analysis(
    ctx: &Context,
    command: CommandInteraction,
    analysis_type: &str,
    sections: &[&str],
) {
    // Defer the response since analysis can take a while
    if let Err(e) = command.defer(&ctx.http).await {
        tracing::error!("Failed to defer: {}", e);
        return;
    }

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

    let backend_url = ctx
        .data
        .read()
        .await
        .get::<crate::BackendUrl>()
        .cloned()
        .expect("BackendUrl not registered");

    let api = BackendApi::new(backend_url);

    let (details_res, analysis_res) = tokio::join!(
        api.fetch_details(beatmap_id),
        api.fetch_analysis_by_type(beatmap_id, analysis_type),
    );

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

    let mut results = match analysis_res {
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

    // Strip graph-only data from stored JSON (not used in Discord embeds)
    for result in &mut results {
        if let Some(obj) = result.analysis.as_object_mut() {
            match result.analysis_type.as_str() {
                "fingercontrol" => {
                    obj.remove("timeline");
                }
                "reading" => {
                    obj.remove("trajectoryTimeline");
                    if let Some(topo) = obj.get_mut("topography").and_then(|v| v.as_object_mut()) {
                        topo.remove("klines");
                    }
                }
                _ => {}
            }
        }
    }

    // Parse all analysis types that are present in the response
    let jump = parse_analysis::<JumpAnalysis>(&results, "jump");
    let stream = parse_analysis::<StreamAnalysis>(&results, "stream");
    let slider = parse_analysis::<SliderAnalysis>(&results, "slider");
    let finger_control = parse_analysis::<FingerControlAnalysis>(&results, "fingercontrol");
    let aim_control = parse_analysis::<AimControlResult>(&results, "aimcontrol");
    let reading = parse_analysis::<ReadingResult>(&results, "reading");

    let state = AnalysisState {
        jump,
        stream,
        slider,
        finger_control,
        aim_control,
        reading,
    };

    let mut response = EditInteractionResponse::new();
    for section in sections {
        response = response.add_embed(build_embed(&details, &state, section));
    }

    let _ = command.edit_response(&ctx.http, response).await;
}

pub async fn run_all(ctx: &Context, command: CommandInteraction) {
    run_analysis(
        ctx, command, "all",
        &["overview", "jump", "stream", "slider", "fingerctrl", "aimctrl", "reading"],
    )
    .await;
}

pub async fn run_jump(ctx: &Context, command: CommandInteraction) {
    run_analysis(ctx, command, "jump", &["overview", "jump"]).await;
}

pub async fn run_stream(ctx: &Context, command: CommandInteraction) {
    run_analysis(ctx, command, "stream", &["overview", "stream"]).await;
}

pub async fn run_slider(ctx: &Context, command: CommandInteraction) {
    run_analysis(ctx, command, "slider", &["overview", "slider"]).await;
}

pub async fn run_finger_ctrl(ctx: &Context, command: CommandInteraction) {
    run_analysis(ctx, command, "fingercontrol", &["overview", "fingerctrl"]).await;
}

pub async fn run_aim_ctrl(ctx: &Context, command: CommandInteraction) {
    run_analysis(ctx, command, "aimcontrol", &["overview", "aimctrl"]).await;
}

pub async fn run_reading(ctx: &Context, command: CommandInteraction) {
    run_analysis(ctx, command, "reading", &["overview", "reading"]).await;
}

pub fn build_embed(
    details: &DetailsResult,
    state: &AnalysisState,
    section: &str,
) -> serenity::all::CreateEmbed {
    match section {
        "overview" => embeds::overview::build(details, state),
        "jump" => state
            .jump
            .as_ref()
            .map(|data| embeds::jump::build(data))
            .unwrap_or_else(|| {
                serenity::all::CreateEmbed::new()
                    .title("Jump Analysis")
                    .description("Data unavailable.")
                    .color(0xec4899)
            }),
        "stream" => state
            .stream
            .as_ref()
            .map(|data| embeds::stream::build(data))
            .unwrap_or_else(|| {
                serenity::all::CreateEmbed::new()
                    .title("Stream Analysis")
                    .description("Data unavailable.")
                    .color(0x3b82f6)
            }),
        "slider" => state
            .slider
            .as_ref()
            .map(|data| embeds::slider::build(data))
            .unwrap_or_else(|| {
                serenity::all::CreateEmbed::new()
                    .title("Slider Analysis")
                    .description("Data unavailable.")
                    .color(0x22c55e)
            }),
        "fingerctrl" => state
            .finger_control
            .as_ref()
            .map(|data| embeds::finger_control::build(data))
            .unwrap_or_else(|| {
                serenity::all::CreateEmbed::new()
                    .title("Finger Control Analysis")
                    .description("Data unavailable.")
                    .color(0xa855f7)
            }),
        "aimctrl" => state
            .aim_control
            .as_ref()
            .map(|data| embeds::aim_control::build(data))
            .unwrap_or_else(|| {
                serenity::all::CreateEmbed::new()
                    .title("Aim Control Analysis")
                    .description("Data unavailable.")
                    .color(0xf97316)
            }),
        "reading" => state
            .reading
            .as_ref()
            .map(|data| embeds::reading::build(data))
            .unwrap_or_else(|| {
                serenity::all::CreateEmbed::new()
                    .title("Reading Analysis")
                    .description("Data unavailable.")
                    .color(0x06b6d4)
            }),
        _ => unreachable!(),
    }
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

