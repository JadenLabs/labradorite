use chrono;

use poise::serenity_prelude::*;

use crate::Context;
use crate::Error;

/// Ping me!
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let time_invoked = ctx.created_at().timestamp_millis();
    let time_now = chrono::Local::now().timestamp_millis();

    let latency = (time_now - time_invoked).abs();
    // println!("{} - {} = {}", time_now, time_invoked, latency);

    ctx.say(format!("Pong! `{}ms`", latency))
        .await
        .expect("Failed to respond to ping command");

    Ok(())
}
