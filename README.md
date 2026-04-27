# rust-anti-spam

A Discord bot that automatically deletes spam messages containing exactly 4 attachments whose first image matches a blacklisted perceptual hash, and logs the action to a designated channel.

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

## How It Works

When a message with exactly 4 attachments is posted, the bot downloads the first attachment and computes its **perceptual hash** (pHash) using the Gradient algorithm. It then compares that hash against every entry in the blacklist using **Hamming distance**. If the closest match is within the threshold (≤ 5 bits), the message is deleted and the event is logged.

This approach is resilient to minor image edits — re-saves, slight crops, or compression artefacts that would defeat a simple byte-for-byte hash check.

## Blacklisting Images

Place spam images in the `data/bad_images/` directory. On startup the bot reads every file in that directory, computes its pHash, and writes the hex-encoded hashes to `data/BLACKLISTED_HASHES.txt` (which is regenerated each run — edit the image files, not the text file).

The `add_hash` utility prints the pHash of a single image so you can verify or inspect it:

```bash
cargo run --bin add_hash -- path/to/image.png
# pHash: a3f1c2...
```

To add a new spam image to the blacklist:

1. Copy the image into `data/bad_images/`
2. Restart the bot — it will pick it up automatically

## Running

```bash
cargo run
```

For a production/optimized build:

```bash
cargo build --release
./target/release/rust-anti-spam
```
