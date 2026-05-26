use serenity::all::{
    ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::client::Context;

use crate::commands::analyze::{build_components, build_embed};

pub async fn handle_component(ctx: &Context, component: ComponentInteraction) {
    let custom_id = &component.data.custom_id;

    let parts: Vec<&str> = custom_id.splitn(3, '_').collect();
    if parts.len() != 3 || parts[0] != "nav" {
        return;
    }

    let cache_key = parts[1].to_string();
    let action = parts[2];

    if action == "indicator" {
        return;
    }

    let cache = ctx
        .data
        .read()
        .await
        .get::<crate::SharedCache>()
        .cloned()
        .expect("SharedCache not registered");

    let mut cache = cache.lock().await;

    let state = match cache.get_mut(&cache_key) {
        Some(s) => s,
        None => {
            let _ = component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .content("This analysis session has expired. Please run `/analyze` again."),
                    ),
                )
                .await;
            return;
        }
    };

    match action {
        "prev" if state.page > 1 => state.page -= 1,
        "next" if state.page < state.total_pages => state.page += 1,
        _ => {}
    }

    let embed = build_embed(&state.details, &state.results, state.page, state.total_pages);
    let components = build_components(&cache_key, state.page, state.total_pages);

    let _ = component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .add_embed(embed)
                    .components(components),
            ),
        )
        .await;
}
