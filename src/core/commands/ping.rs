use chrono;

use poise::serenity_prelude::*;
use poise::CreateReply;

use crate::utils;
use crate::Context;
use crate::Error;

/// Pings the bot
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let time_invoked = ctx.created_at().timestamp_millis();
    let time_now = chrono::Local::now().timestamp_millis();

    let latency = (time_now - time_invoked).abs();
    // println!("{} - {} = {}", time_now, time_invoked, latency);

    let config = utils::config::load_config().expect("Failed to load config");

    let color_rgb = utils::colors::hex_to_rgb(config.colors.primary.as_str());
    let embed_color = Color::from_rgb(color_rgb.0, color_rgb.1, color_rgb.2);
    let embed = CreateEmbed::default()
        .color(embed_color)
        .description(format!("{} Pong! `{}ms`", config.emojis.network, latency));

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
