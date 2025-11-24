use crate::types::{Context, Error};
use once_cell::sync::Lazy;
use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, Colour, CreateEmbed},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::helpers;

#[derive(poise::ChoiceParameter, Debug, Clone, PartialEq)]
pub enum CoinSide {
    #[name = "Heads"]
    Heads,
    #[name = "Tails"]
    Tails,
}

impl std::fmt::Display for CoinSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoinSide::Heads => write!(f, "Heads"),
            CoinSide::Tails => write!(f, "Tails"),
        }
    }
}

struct CoinflipAuthorsData {
    chosen: CoinSide,
    user: Option<serenity::User>,
}

impl CoinflipAuthorsData {
    fn new(chosen: CoinSide, user: serenity::User) -> Self {
        Self {
            chosen: chosen,
            user: Some(user),
        }
    }
    fn get_user(&self) -> Option<&serenity::User> {
        self.user.as_ref()
    }

    fn get_chosen(&self) -> &CoinSide {
        &self.chosen
    }
}

static COINFLIP_MEM: Lazy<Arc<Mutex<HashMap<serenity::ChannelId, CoinflipAuthorsData>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[poise::command(slash_command, prefix_command)]
pub async fn coinflip(
    ctx: Context<'_>,
    #[description = "Pick heads or tails"] side: Option<CoinSide>,
) -> Result<(), Error> {
    let mut mem = COINFLIP_MEM.lock().await;

    if let Some(cfa) = mem.get_mut(&ctx.channel_id()) {
        let _ = ctx.say(format!("Found oppenent {}.", ctx.author())).await;

        let flip = if helpers::generate_thread_safe_random(0u8..=1u8)
            .await
            .unwrap()
            == 0
        {
            CoinSide::Heads
        } else {
            CoinSide::Tails
        };
        // 0 is heads while 1 is tails

        let winner = if *cfa.get_chosen() == flip {
            cfa.get_user().unwrap() // user0 wins
        } else {
            ctx.author() // user1 wins
        };

        let _ = ctx
            .say(format!(
                "{} flips! {} wins! Suck a dick {}!",
                flip,
                winner,
                ctx.author()
            ))
            .await;

        mem.remove(&ctx.channel_id());

        Ok(())
    } else {
        let cfa;

        if let Some(side) = side {
            // fix the option here it does not need to be here
            cfa = CoinflipAuthorsData::new(side, ctx.author().clone());
        } else {
            cfa = CoinflipAuthorsData::new(CoinSide::Heads, ctx.author().clone());
        }

        let mut embed = CreateEmbed::default()
            .title("Coinflip")
            .field("player1", format!("{}", cfa.get_chosen()), false)
            .field(
                "player2",
                "N/A (Be player2 with the /coinflip command!)",
                false,
            );
        embed = embed.colour(Colour::GOLD);

        ctx.send(CreateReply::default().embed(embed)).await?;

        mem.insert(ctx.channel_id(), cfa);

        Ok(())
    }
}
