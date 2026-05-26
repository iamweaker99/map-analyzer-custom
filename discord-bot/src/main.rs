mod api;
mod commands;
mod components;
mod embeds;
mod types;

use std::collections::HashMap;
use std::sync::Arc;

use serenity::all::*;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;

use crate::commands::analyze::AnalysisCache;

/// TypeMap key for the shared analysis cache
pub struct SharedCache;
impl TypeMapKey for SharedCache {
    type Value = AnalysisCache;
}

/// TypeMap key for the backend URL
pub struct BackendUrl;
impl TypeMapKey for BackendUrl {
    type Value = String;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// Register slash commands on startup
    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);

        let command = CreateCommand::new("analyze")
            .description("Analyze an osu! beatmap")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "beatmap",
                    "Beatmap URL or ID",
                )
                .required(true),
            );

        let commands = Command::create_global_command(&ctx.http, command).await;

        match commands {
            Ok(_) => tracing::info!("Registered /analyze command"),
            Err(e) => tracing::error!("Failed to register command: {}", e),
        }
    }

    /// Handle interactions (commands + component interactions)
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(command) = interaction.clone().as_command() {
            commands::handle_command(&ctx, command.clone()).await;
        } else if let Some(component) = interaction.message_component() {
            components::handle_component(&ctx, component).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load .env
    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in environment");
    let backend_url = std::env::var("BACKEND_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());

    // Set up intents (minimal for a bot that only handles slash commands)
    let intents = GatewayIntents::empty();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create Discord client");

    // Insert shared data
    {
        let mut data = client.data.write().await;
        data.insert::<SharedCache>(Arc::new(Mutex::new(HashMap::new())));
        data.insert::<BackendUrl>(backend_url);
    }

    // Start the bot
    if let Err(why) = client.start().await {
        tracing::error!("Client error: {:?}", why);
    }
}
