use chrono;

use poise::serenity_prelude::*;
use poise::CreateReply;

use crate::prelude::*;
use crate::{Context, Error};

/// Pings the bot
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    // Calculate latency
    let time_invoked = ctx.created_at().timestamp_millis();
    let time_now = chrono::Local::now().timestamp_millis();
    let latency = (time_now - time_invoked).abs();

    // Embed
    let config = Config::load().expect("Failed to load config");
    let embed_color = config.colors.primary.to_color();
    let embed = CreateEmbed::default()
        .color(embed_color)
        .description(format!("{} Pong! `{}ms`", config.emojis.network, latency));

    // Respond
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
