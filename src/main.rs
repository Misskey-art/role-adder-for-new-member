use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{GuildId, Member};
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, guild_ids: Vec<GuildId>) {
        println!("Servers ({})", guild_ids.len());
        for guild_id in guild_ids {
            if let Some(guild_info) = ctx.cache.guild(guild_id) {
                if let Ok(owner_info) = ctx
                    .http
                    .get_member(guild_id.into(), guild_info.owner_id.into())
                    .await
                {
                    println!(
                        "  - {} ({:?}) Owners: {}",
                        guild_info.name,
                        guild_info.member_count,
                        owner_info.user.tag()
                    );
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in as {}!", ready.user.tag());
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        old_member: Option<Member>,
        mut new_member: Member,
    ) {
        if let Some(old_member) = old_member {
            if old_member.pending && !new_member.pending {
                println!("{} approved rule screen.", old_member.user.tag());
                let role_id = env::var("ROLE_ID")
                    .expect("env variable `ROLE_ID` is not set.")
                    .parse::<u64>()
                    .expect("Failed to parse role id!");
                if let Err(err) = new_member.add_role(ctx.http, role_id).await {
                    eprintln!("Faile to add role: {:#?}", err)
                } else {
                    println!("Role added succesfuly!");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("env variable `DISCORD_TOKEN` is not set.");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why)
    }
}
