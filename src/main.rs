use dotenvy::dotenv;
use img_hash::{HasherConfig, HashAlg, ImageHash};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::env;
use std::fs;

const HAMMING_THRESHOLD: u32 = 5;

struct Handler {
    log_channel_id: ChannelId,
    blacklisted_hashes: Vec<ImageHash>,
}

fn phash_from_bytes(bytes: &[u8]) -> Option<ImageHash> {
    let img = image::load_from_memory(bytes).ok()?;
    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::Gradient)
        .to_hasher();
    Some(hasher.hash_image(&img))
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.attachments.len() != 4 {
            return;
        }

        let bytes = match reqwest::get(&msg.attachments[0].url).await {
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

        let hash = match phash_from_bytes(&bytes) {
            Some(h) => h,
            None => {
                eprintln!("Failed to compute pHash for attachment");
                return;
            }
        };

        let min_dist = match self.blacklisted_hashes.iter().map(|bh| hash.dist(bh)).min() {
            Some(d) => d,
            None => return,
        };

        if min_dist > HAMMING_THRESHOLD {
            return;
        }

        match msg.delete(&ctx.http).await {
            Ok(_) => {
                let log_entry = format!(
                    "🗑️ **Spam Filter Triggered**\n\
                     **User:** {} (`{}`)\n\
                     **Action:** Deleted message with 4 attachments (pHash distance: `{}`)\n\
                     **Channel:** <#{}>",
                    msg.author.tag(),
                    msg.author.id,
                    min_dist,
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

fn prepare_blacklist_hashes() -> Vec<ImageHash> {
    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::Gradient)
        .to_hasher();

    let mut blacklisted_hashes: Vec<ImageHash> = Vec::new();
    let mut hash_hex_lines: Vec<String> = Vec::new();

    match fs::read_dir("data/bad_images") {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                match fs::read(&path) {
                    Ok(bytes) => match image::load_from_memory(&bytes) {
                        Ok(img) => {
                            let hash = hasher.hash_image(&img);
                            hash_hex_lines.push(hex::encode(hash.as_bytes()));
                            blacklisted_hashes.push(hash);
                        }
                        Err(e) => eprintln!("Skipping {:?}: {:?}", path, e),
                    },
                    Err(e) => eprintln!("Failed to read {:?}: {:?}", path, e),
                }
            }
        }
        Err(e) => eprintln!("Failed to read data/bad_images: {:?}", e),
    }

    if let Err(e) = fs::write("data/BLACKLISTED_HASHES.txt", hash_hex_lines.join("\n")) {
        eprintln!("Failed to write hash file: {:?}", e);
    }

    blacklisted_hashes
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment variable 'DISCORD_TOKEN'");

    let log_channel_u64 = env::var("LOG_CHANNEL_ID")
        .expect("Expected LOG_CHANNEL_ID in environment")
        .parse::<u64>()
        .expect("LOG_CHANNEL_ID must be a valid integer");

    let blacklisted_hashes = prepare_blacklist_hashes();

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
        "Bot is starting... Monitoring for 4-attachment spam ({} blacklisted pHashes loaded).",
        hash_count
    );

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
