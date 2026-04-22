use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::env;

struct Handler {
    log_channel_id: ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.attachments.len() == 4 {
            match msg.delete(&ctx.http).await {
                Ok(_) => {
                    let log_entry = format!(
                        "🗑️ **Spam Filter Triggered**\n\
                         **User:** {} (`{}`)\n\
                         **Action:** Deleted message with 4 attachments\n\
                         **Channel:** <#{}>",
                        msg.author.tag(),
                        msg.author.id,
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

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        log_channel_id: ChannelId::new(log_channel_u64),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    println!("Bot is starting... Monitoring for 4-attachment spam.");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
