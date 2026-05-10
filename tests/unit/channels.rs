use lingchong_bot::channels::{
    DISCORD_MAX_MESSAGE_LENGTH, TELEGRAM_MAX_MESSAGE_LENGTH, decorate_chunk_for_telegram,
    split_message_for_discord, split_message_for_telegram,
};
use pretty_assertions::assert_eq;

#[test]
fn discord_split_respects_character_limit() {
    let message = "a".repeat(DISCORD_MAX_MESSAGE_LENGTH + 7);
    let chunks = split_message_for_discord(&message);

    assert_eq!(chunks.len(), 2);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk.chars().count() <= DISCORD_MAX_MESSAGE_LENGTH)
    );
    assert_eq!(chunks.concat(), message);
}

#[test]
fn telegram_split_respects_character_limit_and_preserves_text() {
    let message = "hello ".repeat(TELEGRAM_MAX_MESSAGE_LENGTH);
    let chunks = split_message_for_telegram(&message);

    assert!(chunks.len() > 1);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk.chars().count() < TELEGRAM_MAX_MESSAGE_LENGTH)
    );
    assert_eq!(chunks.concat(), message);
}

#[test]
fn telegram_chunk_decoration_marks_sequence_edges() {
    assert_eq!(decorate_chunk_for_telegram("one", 0, 1), "one");
    assert_eq!(decorate_chunk_for_telegram("one", 0, 2), "one\n[continues]");
    assert_eq!(decorate_chunk_for_telegram("two", 1, 2), "[continued]\ntwo");
}
