use poise::serenity_prelude as serenity;
use poise::serenity_prelude::*;

use crate::Context;
use crate::Error;

/// Displays a user's account creation date
#[poise::command(slash_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Target user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!(
        "{}'s account was created at <t:{}:F>",
        u.name,
        u.created_at().unix_timestamp()
    );
    ctx.say(response).await?;
    Ok(())
}
