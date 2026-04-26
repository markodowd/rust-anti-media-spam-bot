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

## Blacklisting Image Hashes

The bot compares attachment hashes against `BLACKLISTED_HASHES.txt`. Each line is a SHA-256 hex digest.

To add an image to the blacklist, use the included `add_hash` utility:

```bash
cargo run --bin add_hash -- path/to/image.png
```

This will:
1. Read the file and compute its SHA-256 hash
2. Check whether the hash is already in `BLACKLISTED_HASHES.txt`
3. Append it if not, or print a message if it's already present

You can pass any file type — the hash is computed from raw bytes regardless of format. To blacklist a spam image you've downloaded, just point the tool at it:

```bash
cargo run --bin add_hash -- ~/Downloads/spam.jpg
# Added hash: a3f1c2...
```

Restart the bot after updating `BLACKLISTED_HASHES.txt` for changes to take effect.

## Running

```bash
cargo run
```

For a production/optimized build:

```bash
cargo build --release
./target/release/rust-anti-spam
```
