use crate::types::{Context, Error};
use reqwest::{self};
use serde_json::json;

#[poise::command(slash_command, prefix_command)]
pub async fn jakbot_raid(
    ctx: Context<'_>,
    #[description = "The game pin for the bots to join"] gamepin: String,
    #[description = "Amount of bots you want to send"] amount: u8,
    #[description = "Name of the bots (A hash will be attached to each one)"] nickname: String,
    #[description = "TTL (time to live)"] ttl: u16,
    #[description = "Crash the game"] crash: bool,
) -> Result<(), Error> {
    if !gamepin.chars().all(|c| c.is_ascii_digit()) && !gamepin.to_lowercase().contains("kahoot.it")
    {
        return Err(
            "Invalid gamepin. Either type in the gamepin or past the url that you copied from clicking on the kahoot symbol.".into()
        );
    }

    // the bot will detect this but its good to have some extra safety checkes here to :P
    if amount == 0 || amount > 100 {
        return Err(format!(
            "The amount cannot be {}. Try a number between 0-100.",
            amount
        )
        .into());
    }

    if nickname.len() < 1 || nickname.len() > 30 {
        return Err(format!(
            "Nickname {}, is either to short or to long. Try to keep it in a inclusive range of 1-30.",
            nickname
        ).into());
    }

    if ttl < 10 || ttl > 500 {
        return Err(format!(
            "TTL {}, is either to short or to long. Try to keep it in a inclusive range of 10-500.",
            ttl
        )
        .into());
    }
    let body_json = json!({
        "amount": amount,
        "gamepin": gamepin,
        "nickname": nickname,
        "crash": crash,
        "ttl": ttl,
    })
    .to_string();

    let client = reqwest::Client::new();

    let result = client
        .post("http://felixhub-felixhub-internal.default.svc.cluster.local:8080/kahootswarm")
        .header("User-Agent", "autismal/0.1.0")
        .header("Content-Type", "application/json")
        .body(body_json)
        .send()
        .await;

    let r = result
        .map(|response| {
            format!(
                "{} Your request has been queued for a raid.",
                response.status()
            )
        })
        .map_err(|e| {
            // i gotta replace this with a actual custom logger
            println!(
                "Error in function jakbot_raid: kahoot raid failed to start
            with status code from the felixhub server of {}",
                e.status()
                    .map_or("NO_STATUS_CODE".to_string(), |s| s.to_string())
            );

            format!(
                "Got bad response when sending your raid request to the JAKBOT.\n\
                Got status {}. Make an issue on {} if this issue persists.",
                e.status()
                    .map_or("NO_STATUS_CODE".to_string(), |s| s.to_string()),
                "https://github.com/Feelfeel20088/autismal/issues"
            )
        });

    match r {
        Ok(ok) => ctx.say(ok).await?,
        Err(err) => return Err(err.into()),
    };

    Ok(())
}
