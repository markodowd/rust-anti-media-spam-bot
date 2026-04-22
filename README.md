# rust-anti-spam

A Discord bot that automatically deletes spam messages containing exactly 4 attachments and logs the action to a designated channel.

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- A Discord bot token ([Discord Developer Portal](https://discord.com/developers/applications))
- A Discord server where you have permission to add bots

## Environment Setup

Create a `.env` file in the project root:

```
DISCORD_TOKEN=your_bot_token_here
LOG_CHANNEL_ID=your_channel_id_here
```

| Variable | Description |
|---|---|
| `DISCORD_TOKEN` | Your Discord bot's authentication token, found in the Discord Developer Portal under your application's **Bot** tab |
| `LOG_CHANNEL_ID` | The ID of the Discord channel where the bot will log spam deletions. Right-click a channel in Discord and select **Copy Channel ID** (requires Developer Mode to be enabled in Discord settings) |

## Discord Bot Setup

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications) and create a new application
2. Under the **Bot** tab, create a bot and copy the token into `DISCORD_TOKEN`
3. Under **Privileged Gateway Intents**, enable **Message Content Intent**
4. Under **OAuth2 > URL Generator**, select the `bot` scope and the following permissions:
   - Read Messages / View Channels
   - Manage Messages (required to delete spam)
   - Send Messages (required to post logs)
5. Use the generated URL to invite the bot to your server

## Running

```bash
cargo run
```

For a production/optimized build:

```bash
cargo build --release
./target/release/rust-anti-spam
```
