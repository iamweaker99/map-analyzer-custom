mod api;
mod commands;
mod embeds;
mod register_commands;
mod types;

use serenity::all::*;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::prelude::TypeMapKey;

/// TypeMap key for the backend URL
pub struct BackendUrl;
impl TypeMapKey for BackendUrl {
    type Value = String;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// Register slash commands via raw HTTP (supports integration_types for User Install)
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);

        let token = std::env::var("DISCORD_TOKEN")
            .expect("DISCORD_TOKEN must be set");
        let application_id = ready.application.id.get();

        match register_commands::register_commands(&token, application_id).await {
            Ok(()) => tracing::info!("Commands registered with User Install support"),
            Err(e) => tracing::error!("Failed to register commands: {}", e),
        }
    }

    /// Handle interactions (slash commands only)
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(command) = interaction.as_command() {
            commands::handle_command(&ctx, command.clone()).await;
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
        data.insert::<BackendUrl>(backend_url);
    }

    // Start the bot
    if let Err(why) = client.start().await {
        tracing::error!("Client error: {:?}", why);
    }
}
