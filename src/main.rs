use dotenv::dotenv;

use poise::serenity_prelude as serenity;
use poise::serenity_prelude::*;

mod core;
mod utils;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // Load config
    let config = utils::config::load_config().expect("Failed to load config");
    utils::logger::info(format!("Running {}", config.name).as_str());

    // Login Details
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    utils::logger::info("Loaded config, token, and intents");

    // Framework loading
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            // * Commands go here
            commands: vec![
                core::commands::age::age(),
                core::commands::ping::ping(),
                core::commands::info::info(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    utils::logger::info(
        format!("{} Commands loaded", &framework.options().commands.len()).as_str(),
    );

    // Create client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            core::events::ready::ready(ctx, event, data_about_bot)
        }
        serenity::FullEvent::InteractionCreate { interaction, .. } => {
            core::events::interaction::interaction_create(ctx, event, interaction)
        }
        _ => {}
    }
    Ok(())
}
