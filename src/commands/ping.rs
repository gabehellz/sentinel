use crate::{Context, Error};

/// Checks if the bot is running
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong 🏓").await?;
    Ok(())
}
