use lingchong_bot::channels::{
    DiscordMentionGate, DiscordSessionPartition, TelegramSessionPartition, TelegramThreadId,
};
use pretty_assertions::assert_eq;

#[test]
fn telegram_session_partition_builds_expected_keys() {
    assert_eq!(
        TelegramSessionPartition::ChatOnly.build_session_key("chat-1", "user-1", None),
        "chat-1"
    );
    assert_eq!(
        TelegramSessionPartition::ChatUser.build_session_key("chat-1", "user-1", None),
        "chat-1:user-1"
    );
    assert_eq!(
        TelegramSessionPartition::UserOnly.build_session_key("chat-1", "user-1", None),
        "user-1"
    );
    assert_eq!(
        TelegramSessionPartition::ChatThreadUser.build_session_key(
            "chat-1",
            "user-1",
            Some(TelegramThreadId::new(42)),
        ),
        "chat-1:42:user-1"
    );
}

#[test]
fn telegram_session_partition_parses_aliases() {
    assert_eq!(
        "chat-thread-user"
            .parse::<TelegramSessionPartition>()
            .expect("alias should parse"),
        TelegramSessionPartition::ChatThreadUser
    );
    assert!("workspace".parse::<TelegramSessionPartition>().is_err());
}

#[test]
fn discord_session_partition_builds_expected_keys() {
    assert_eq!(
        DiscordSessionPartition::GuildChannelUser.build_session_key("guild-1", "channel-1", "u1"),
        "guild-1:channel-1:u1"
    );
    assert_eq!(
        DiscordSessionPartition::ChannelOnly.build_session_key("guild-1", "channel-1", "u1"),
        "guild-1:channel-1"
    );
    assert_eq!(
        DiscordSessionPartition::UserOnly.build_session_key("guild-1", "channel-1", "u1"),
        "u1"
    );
    assert_eq!(
        DiscordSessionPartition::GuildUser.build_session_key("guild-1", "channel-1", "u1"),
        "guild-1:u1"
    );
}

#[test]
fn discord_session_partition_builds_expected_scopes() {
    assert_eq!(
        DiscordSessionPartition::GuildChannelUser.build_session_scope(
            "discord:",
            "guild-1",
            "channel-1",
            "u1",
        ),
        "discord:guild-1:channel-1:u1"
    );
    assert_eq!(
        DiscordSessionPartition::ChannelOnly.build_session_scope(
            "discord:",
            "dm",
            "channel-1",
            "u1",
        ),
        "discord:dm:channel-1"
    );
}

#[test]
fn discord_session_partition_parses_aliases() {
    assert_eq!(
        "guild-channel-user"
            .parse::<DiscordSessionPartition>()
            .expect("alias should parse"),
        DiscordSessionPartition::GuildChannelUser
    );
    assert!("thread".parse::<DiscordSessionPartition>().is_err());
}

#[test]
fn discord_mention_gate_allows_when_requirement_is_disabled() {
    let gate = DiscordMentionGate {
        require_mention: false,
        bot_user_id: None,
    };

    assert!(gate.accepts_message("hello", &[]));
}

#[test]
fn discord_mention_gate_requires_bot_identity_when_enabled() {
    let gate = DiscordMentionGate {
        require_mention: true,
        bot_user_id: None,
    };

    assert!(!gate.accepts_message("<@bot-1> hello", &[]));
}

#[test]
fn discord_mention_gate_accepts_structured_or_inline_mentions() {
    let gate = DiscordMentionGate {
        require_mention: true,
        bot_user_id: Some("bot-1".to_string()),
    };

    assert!(gate.accepts_message("hello", &["bot-1".to_string()]));
    assert!(gate.accepts_message("<@bot-1> hello", &[]));
    assert!(gate.accepts_message("<@!bot-1> hello", &[]));
    assert!(!gate.accepts_message("hello", &["other".to_string()]));
}
