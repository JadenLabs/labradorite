use std::collections::HashMap;

use crate::prelude::*;
use crate::{Context, Error};
use poise::serenity_prelude::*;
use poise::CreateReply;

/// Displays information about the server
#[poise::command(slash_command, subcommands("user", "server"))]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    // Embed
    let config = Config::load().expect("Failed to load config");
    let embed_color = config.colors.primary.to_color();
    let description = format!(
        r#"Labradorite is a multi-purpose bot made in rust using `Serenity` and `Poise`.

I (`roc.py`) am creating this bot with the primary purpose of learning the Rust language.

There is no set goal for the bot; I plan to just add stuff as I experiment with aspects of Rust.

Github: {}
    "#,
        config.github
    );

    let embed = CreateEmbed::default()
        .color(embed_color)
        .title("About Labradorite")
        .description(description);

    // Respond
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Displays information about the server
#[poise::command(slash_command)]
pub async fn server(ctx: Context<'_>) -> Result<(), Error> {
    // Load config
    let config = Config::load().expect("Failed to load config");
    let embed_color = config.colors.primary.to_color();

    // Guild info
    let guild: Guild = ctx.guild().unwrap().clone();
    let guild_icon = guild.icon_url().unwrap_or("none".to_string());

    let mut counts = HashMap::new();

    for (_channel_id, channel) in &guild.channels {
        let counter = counts.entry(channel.kind).or_insert(0);
        *counter += 1;
    }

    let category_count = *counts.get(&ChannelType::Category).unwrap_or(&0);
    let forum_count = *counts.get(&ChannelType::Forum).unwrap_or(&0);
    let news_count = *counts.get(&ChannelType::News).unwrap_or(&0);
    let stage_count = *counts.get(&ChannelType::Stage).unwrap_or(&0);
    let text_count = *counts.get(&ChannelType::Text).unwrap_or(&0);
    let voice_count = *counts.get(&ChannelType::Voice).unwrap_or(&0);

    // General information
    let general_field = format!(
        r#"
Name: `{name}`
{arr_w} ID: `{id}`
{arr_w} Description:
> `{description}`
Owner: {owner}
Members: `{members}`
Channels: `{channels}`
Roles: `{roles}`
Emojis: `{emojis}`
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
        emojis = guild.emojis.len(),
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
    "#,
        arr_w = config.emojis.arr_r,
        category_count = category_count,
        forum_count = forum_count,
        news_count = news_count,
        stage_count = stage_count,
        text_count = text_count,
        voice_count = voice_count,
    );

    // Features
    let fmt_features_vec: Vec<String> = guild
        .features
        .iter()
        .map(|feature| format!("{} {}", config.emojis.arr_r, feature.to_lowercase()))
        .collect();

    let fmt_features = fmt_features_vec.join("\n");

    // Create embed
    let embed = CreateEmbed::default()
        .color(embed_color)
        .thumbnail(guild_icon)
        .title(format!("{} - Server information", guild.name))
        .fields(vec![
            ("General", general_field, true),
            ("Channels", channel_field, true),
            ("Features", fmt_features, false),
        ]);

    // Send message
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Displays information about a user
#[poise::command(slash_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Target user"] user: Option<User>,
) -> Result<(), Error> {
    // Load config
    let config = Config::load().expect("Failed to load config");
    let embed_color = config.colors.primary.to_color();

    // User information
    let user = user.unwrap_or(ctx.author().clone());
    let avatar = user.face();

    // Description
    let description = format!(
        r#"
Username: `{name}`
{arr_w} ID: `{id}`
{arr_w} Global: `{global}`
Created: <t:{created_at}:D>
Bot: `{is_bot}`
    "#,
        arr_w = config.emojis.arr_r,
        name = user.name,
        id = user.id,
        global = user.global_name.clone().unwrap_or("None".to_string()),
        created_at = user.created_at().unix_timestamp(),
        is_bot = user.bot.to_string()
    );

    // Create embed
    let embed = CreateEmbed::default()
        .color(embed_color)
        .thumbnail(avatar)
        .description(description);

    // Send message
    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
