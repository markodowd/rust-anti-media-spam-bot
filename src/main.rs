use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::env;

struct Handler {
    log_channel_id: ChannelId,
    blacklisted_hashes: HashSet<String>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.attachments.len() == 4 {
            let attachment = &msg.attachments[0];

            let bytes = match reqwest::get(&attachment.url).await {
                Ok(resp) => match resp.bytes().await {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("Failed to read attachment bytes: {:?}", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to download attachment: {:?}", e);
                    return;
                }
            };

            let hash = hex::encode(Sha256::digest(&bytes));

            if !self.blacklisted_hashes.contains(&hash) {
                return;
            }

            match msg.delete(&ctx.http).await {
                Ok(_) => {
                    let log_entry = format!(
                        "🗑️ **Spam Filter Triggered**\n\
                         **User:** {} (`{}`)\n\
                         **Action:** Deleted message with 4 attachments (blacklisted hash: `{}`)\n\
                         **Channel:** <#{}>",
                        msg.author.tag(),
                        msg.author.id,
                        hash,
                        msg.channel_id
                    );

                    if let Err(e) = self.log_channel_id.say(&ctx.http, log_entry).await {
                        eprintln!("Failed to send log to channel: {:?}", e);
                    }
                }
                Err(why) => {
                    let error_log = format!(
                        "⚠️ **Deletion Failed**\n**User:** {}\n**Error:** {:?}",
                        msg.author.tag(),
                        why
                    );
                    let _ = self.log_channel_id.say(&ctx.http, error_log).await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment variable 'DISCORD_TOKEN'");

    let log_channel_str =
        env::var("LOG_CHANNEL_ID").expect("Expected LOG_CHANNEL_ID in environment");

    let log_channel_u64 = log_channel_str
        .parse::<u64>()
        .expect("LOG_CHANNEL_ID must be a valid integer");

    let hash_file = std::fs::read_to_string("BLACKLISTED_HASHES.txt")
        .expect("Failed to read BLACKLISTED_HASHES.txt");

    let blacklisted_hashes: HashSet<String> = hash_file
        .lines()
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty())
        .collect();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let hash_count = blacklisted_hashes.len();

    let handler = Handler {
        log_channel_id: ChannelId::new(log_channel_u64),
        blacklisted_hashes,
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    println!(
        "Bot is starting... Monitoring for 4-attachment spam ({} blacklisted hashes loaded).",
        hash_count
    );

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
