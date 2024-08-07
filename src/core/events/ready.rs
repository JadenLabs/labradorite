use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ActivityData;

use crate::utils::{config::Config, logger};

pub fn ready(
    ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    data_about_bot: &serenity::Ready,
) {
    let config = Config::load().expect("Failed to load config");
    logger::info(format!("Logged in as {}", data_about_bot.user.name).as_str());

    let activity = Some(ActivityData::custom(config.activity));
    ctx.shard.set_activity(activity);
}
