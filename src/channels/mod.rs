//! Channel-safe formatting helpers for bot adapters.

/// Discord channel formatting helpers.
pub mod discord;
/// Channel session partition and mention-gate policies.
pub mod session;
/// Telegram channel formatting helpers.
pub mod telegram;

pub use discord::{DISCORD_MAX_MESSAGE_LENGTH, split_message_for_discord};
pub use session::{
    DiscordMentionGate, DiscordSessionPartition, SessionPartitionParseError,
    TelegramSessionPartition, TelegramThreadId,
};
pub use telegram::{
    TELEGRAM_MAX_MESSAGE_LENGTH, chunk_marker_reserve_chars, decorate_chunk_for_telegram,
    split_message_for_telegram,
};
