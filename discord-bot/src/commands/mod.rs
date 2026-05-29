pub mod analyze;

use serenity::{client::Context, model::application::CommandInteraction};

pub async fn handle_command(ctx: &Context, command: CommandInteraction) {
    match command.data.name.as_str() {
        "analyze-all" => analyze::run_all(ctx, command).await,
        "analyze-jump" => analyze::run_jump(ctx, command).await,
        "analyze-stream" => analyze::run_stream(ctx, command).await,
        "analyze-slider" => analyze::run_slider(ctx, command).await,
        "analyze-finger_ctrl" => analyze::run_finger_ctrl(ctx, command).await,
        "analyze-aim_ctrl" => analyze::run_aim_ctrl(ctx, command).await,
        "analyze-reading" => analyze::run_reading(ctx, command).await,
        other => {
            tracing::warn!("Unknown command: {}", other);
        }
    }
}
