//! Channel session partition and mention-gate policies.

use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// How incoming Telegram messages map to a logical conversation session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TelegramSessionPartition {
    /// Share one session for everyone in the same chat.
    #[default]
    ChatOnly,
    /// Isolate by chat and user.
    ChatUser,
    /// Share one session for the same user across all chats.
    UserOnly,
    /// Isolate by chat, thread, and user.
    ChatThreadUser,
}

impl TelegramSessionPartition {
    /// Build a session key from Telegram identifiers.
    #[must_use]
    pub fn build_session_key(
        self,
        chat_id: impl AsRef<str>,
        user_identity: impl AsRef<str>,
        thread_id: Option<TelegramThreadId>,
    ) -> String {
        let chat_id = chat_id.as_ref();
        let user_identity = user_identity.as_ref();
        match self {
            Self::ChatOnly => chat_id.to_string(),
            Self::ChatUser => format!("{chat_id}:{user_identity}"),
            Self::UserOnly => user_identity.to_string(),
            Self::ChatThreadUser => {
                let thread_id = thread_id.map_or(0, TelegramThreadId::into_raw);
                format!("{chat_id}:{thread_id}:{user_identity}")
            }
        }
    }
}

/// Telegram forum-topic thread identifier used for session keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelegramThreadId(i64);

impl TelegramThreadId {
    /// Build a Telegram thread identifier.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Return the raw Telegram thread id.
    #[must_use]
    pub const fn into_raw(self) -> i64 {
        self.0
    }
}

impl Display for TelegramSessionPartition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::ChatOnly => "chat",
            Self::ChatUser => "chat_user",
            Self::UserOnly => "user",
            Self::ChatThreadUser => "chat_thread_user",
        };
        formatter.write_str(value)
    }
}

impl FromStr for TelegramSessionPartition {
    type Err = SessionPartitionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "chat" | "chat_only" | "chat-only" | "chatonly" => Ok(Self::ChatOnly),
            "chat_user" | "chat-user" | "chatuser" => Ok(Self::ChatUser),
            "user" | "user_only" | "user-only" | "useronly" => Ok(Self::UserOnly),
            "chat_thread_user" | "chat-thread-user" | "chatthreaduser" | "topic_user"
            | "topic-user" | "topicuser" => Ok(Self::ChatThreadUser),
            _ => Err(SessionPartitionParseError),
        }
    }
}

/// How incoming Discord messages map to a logical conversation session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiscordSessionPartition {
    /// Isolate by guild or DM scope, channel, and user.
    #[default]
    GuildChannelUser,
    /// Share one session per channel.
    ChannelOnly,
    /// Share one session per user across all guilds and channels.
    UserOnly,
    /// Share one session per user within one guild or DM scope.
    GuildUser,
}

impl DiscordSessionPartition {
    /// Build a session key from Discord identifiers.
    #[must_use]
    pub fn build_session_key(
        self,
        scope: impl AsRef<str>,
        channel_id: impl AsRef<str>,
        user_identity: impl AsRef<str>,
    ) -> String {
        let scope = scope.as_ref();
        let channel_id = channel_id.as_ref();
        let user_identity = user_identity.as_ref();
        match self {
            Self::GuildChannelUser => format!("{scope}:{channel_id}:{user_identity}"),
            Self::ChannelOnly => format!("{scope}:{channel_id}"),
            Self::UserOnly => user_identity.to_string(),
            Self::GuildUser => format!("{scope}:{user_identity}"),
        }
    }

    /// Build a session scope value from a prefix and Discord identifiers.
    #[must_use]
    pub fn build_session_scope(
        self,
        prefix: impl AsRef<str>,
        scope: impl AsRef<str>,
        channel_id: impl AsRef<str>,
        user_identity: impl AsRef<str>,
    ) -> String {
        format!(
            "{}{}",
            prefix.as_ref(),
            self.build_session_key(scope, channel_id, user_identity)
        )
    }
}

impl Display for DiscordSessionPartition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::GuildChannelUser => "guild_channel_user",
            Self::ChannelOnly => "channel",
            Self::UserOnly => "user",
            Self::GuildUser => "guild_user",
        };
        formatter.write_str(value)
    }
}

impl FromStr for DiscordSessionPartition {
    type Err = SessionPartitionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "guild_channel_user" | "guild-channel-user" | "guildchanneluser" | "channel_user"
            | "channel-user" | "channeluser" => Ok(Self::GuildChannelUser),
            "channel" | "channel_only" | "channel-only" | "channelonly" => Ok(Self::ChannelOnly),
            "user" | "user_only" | "user-only" | "useronly" => Ok(Self::UserOnly),
            "guild_user" | "guild-user" | "guilduser" => Ok(Self::GuildUser),
            _ => Err(SessionPartitionParseError),
        }
    }
}

/// Session partition parse failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionPartitionParseError;

impl Display for SessionPartitionParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("invalid session partition")
    }
}

impl std::error::Error for SessionPartitionParseError {}

/// Discord mention gate for foreground channel turns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordMentionGate {
    /// Whether a bot mention is required before handling the message.
    pub require_mention: bool,
    /// Discord bot user id, without mention markup.
    pub bot_user_id: Option<String>,
}

impl DiscordMentionGate {
    /// Decide whether a Discord message should enter the bot turn pipeline.
    #[must_use]
    pub fn accepts_message(&self, content: &str, mentioned_user_ids: &[String]) -> bool {
        if !self.require_mention {
            return true;
        }

        let Some(bot_user_id) = self.bot_user_id.as_deref() else {
            return false;
        };

        mentioned_user_ids
            .iter()
            .any(|user_id| user_id == bot_user_id)
            || content.contains(&format!("<@{bot_user_id}>"))
            || content.contains(&format!("<@!{bot_user_id}>"))
    }
}
