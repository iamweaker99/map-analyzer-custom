pub mod analyze;

use serenity::{client::Context, model::application::CommandInteraction};

pub async fn handle_command(ctx: &Context, command: CommandInteraction) {
    match command.data.name.as_str() {
        "analyze" => analyze::run(ctx, command).await,
        other => {
            tracing::warn!("Unknown command: {}", other);
        }
    }
}
