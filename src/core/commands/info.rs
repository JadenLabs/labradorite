// use serde_json;

use poise::serenity_prelude::*;
use poise::CreateReply;

use crate::utils;
use crate::Context;
use crate::Error;

/// Displays information about the server
#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    // Load config
    let config = utils::config::load_config().expect("Failed to load config");

    // Color
    let color_rgb = utils::colors::hex_to_rgb(config.colors.primary.as_str());
    let embed_color = Color::from_rgb(color_rgb.0, color_rgb.1, color_rgb.2);

    // Guild info
    let guild: Guild = ctx.guild().unwrap().clone();
    let guild_icon = guild.icon_url().unwrap_or("none".to_string());

    // Channel info
    let mut category_count = 0;
    let mut forum_count = 0;
    let mut news_count = 0;
    let mut stage_count = 0;
    let mut text_count = 0;
    let mut voice_count = 0;
    let mut other_count = 0;
    for (_channel_id, channel) in &guild.channels {
        match channel.kind {
            ChannelType::Category => category_count += 1,
            ChannelType::Forum => forum_count += 1,
            ChannelType::News => news_count += 1,
            ChannelType::Stage => stage_count += 1,
            ChannelType::Text => text_count += 1,
            ChannelType::Voice => voice_count += 1,
            _ => other_count += 1,
        }
    }

    // General information
    let general_field = format!(
        r#"
**Name**: `{name}`
{arr_w} ID: `{id}`
{arr_w} Description:
> `{description}`
**Owner**: {owner}
**Members**: `{members}`
**Channels**: `{channels}`
**Roles**: `{roles}`
    "#,
        arr_w = config.emojis.arr_r,
        name = guild.name,
        id = guild.id,
        description = guild.description.unwrap_or("No description".to_string()),
        owner = ctx
            .http()
            .get_user(guild.owner_id)
            .await
            .expect("Unable to get owner")
            .mention(),
        members = guild.member_count,
        channels = guild.channels.len(),
        roles = guild.roles.len(),
    );

    // Channel information
    let channel_field = format!(
        r#"
{arr_w} Categories: `{category_count}`
{arr_w} Forums: `{forum_count}`
{arr_w} News: `{news_count}`
{arr_w} Stages: `{stage_count}`
{arr_w} Text: `{text_count}`
{arr_w} Voice: `{voice_count}`
{arr_w} Other: `{other_count}`
    "#,
        arr_w = config.emojis.arr_r,
        category_count = category_count,
        forum_count = forum_count,
        news_count = news_count,
        stage_count = stage_count,
        text_count = text_count,
        voice_count = voice_count,
        other_count = other_count
    );

    // Create embed
    let embed = CreateEmbed::default()
        .color(embed_color)
        .thumbnail(guild_icon)
        .title(format!("{} - Server information", guild.name))
        .fields(vec![
            ("General", general_field, true),
            ("Channels", channel_field, true),
        ]);

    // Send message
    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
