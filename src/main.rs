use std::env;

use poise::serenity_prelude::{self as serenity, GatewayIntents};
use serde::Deserialize;

const KAHOOT_URL: &str = "https://felixhub.dev/kahootswarm";

mod commands;
mod helpers;
mod types;

struct Data {} // User data, which is stored and accessible in all command invocations

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable not set");

    let intents = GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            #[rustfmt::skip]
            commands: vec![
                commands::age(),
                commands::coinflip(),
                commands::gay(),
                // commands::jakbot_raid()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(types::Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
