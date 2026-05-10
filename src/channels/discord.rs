//! Discord message formatting helpers.

/// Maximum message size accepted by Discord `Create Message`.
pub const DISCORD_MAX_MESSAGE_LENGTH: usize = 2000;

/// Split text into Discord-safe chunks using character count.
#[must_use]
pub fn split_message_for_discord(message: &str) -> Vec<String> {
    split_message_for_discord_with_limit(message, DISCORD_MAX_MESSAGE_LENGTH)
}

/// Split text into Discord-safe chunks using a caller-provided limit.
#[must_use]
pub fn split_message_for_discord_with_limit(message: &str, max_chars: usize) -> Vec<String> {
    if max_chars == 0 || message.is_empty() {
        return Vec::new();
    }

    let chars = message.chars().collect::<Vec<_>>();
    chars
        .chunks(max_chars)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}
