use crate::{Context, Error};
use poise::serenity_prelude::User;

/// Shows the avatar of the specified user or the caller
#[poise::command(
    slash_command,
    prefix_command,
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS"
)]
pub async fn avatar(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    let avatar_url = match user {
        Some(user) => user.avatar_url().unwrap_or(user.default_avatar_url()),
        None => {
            let user = ctx.author();
            user.avatar_url().unwrap_or(user.default_avatar_url())
        }
    };

    ctx.reply(avatar_url).await?;
    Ok(())
}
