use crate::{
    helpers,
    types::{Context, Error},
};
use poise::serenity_prelude::{self as serenity};

/// Calculates how gay you are based on your chat history
#[poise::command(slash_command, prefix_command)]
pub async fn gay(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let min: u8 = 0;
    let max: u8 = 100;
    let gayness = helpers::generate_thread_safe_random(min..=max)
        .await
        .unwrap();
    let response = format!(
        "{} is {}% gay! {}",
        u.name,
        gayness,
        if gayness > 90 {
            "They are pretty much a faggot"
        } else {
            ""
        }
    );
    ctx.say(response).await?;
    Ok(())
}
