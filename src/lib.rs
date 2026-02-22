use std::{sync::Arc, time::Duration};

use poise::serenity_prelude::{ClientBuilder, GatewayIntents, GuildId};
use tracing::info;

mod commands;

pub struct Data {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    dbg!(error.to_string());
}

async fn on_ready(
    ctx: &poise::serenity_prelude::Context,
    ready: &poise::serenity_prelude::Ready,
    framework: &poise::Framework<Data, Error>,
) -> Result<Data, Error> {
    dbg!(ready);

    let guild_id = std::env::var("GUILD_ID")
        .expect("Expected GUILD_ID environment variable")
        .parse()
        .expect("GUILD_ID must be numbers only");

    let guild_id = GuildId::new(guild_id);

    poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
    info!(guild_id = %guild_id, "Bot {} is ready!", ready.user.tag());
    Ok(Data {})
}

async fn event_handler(
    _framework: poise::FrameworkContext<'_, Data, Error>,
    event: &poise::serenity_prelude::FullEvent,
) -> Result<(), Error> {
    dbg!(event.to_owned());
    Ok(())
}

fn prefix_options() -> poise::PrefixFrameworkOptions<Data, Error> {
    poise::PrefixFrameworkOptions {
        prefix: Some("ns.".into()),
        additional_prefixes: vec![
            poise::Prefix::Literal("?"),
            poise::Prefix::Literal("!"),
            poise::Prefix::Literal("-"),
            poise::Prefix::Literal("."),
        ],
        case_insensitive_commands: true,
        edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
            Duration::from_secs(3600),
        ))),
        ..Default::default()
    }
}

fn build_framework() -> poise::Framework<Data, Error> {
    let commands = vec![commands::ping::ping()];
    let prefix_options = prefix_options();

    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options,
            initialize_owners: true,
            on_error: |error| Box::pin(on_error(error)),
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            ..Default::default()
        })
        .setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)))
        .build()
}

pub async fn start_client(token: &str) -> Result<(), Error> {
    let framework = build_framework();

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client");

    client.start().await.expect("Failed to initialize client");
    Ok(())
}
