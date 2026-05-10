//! Telegram group profile extraction from runtime logs.

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

/// Telegram chat id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelegramChatId(i64);

impl TelegramChatId {
    /// Build a Telegram chat id.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Return the raw id.
    #[must_use]
    pub const fn into_raw(self) -> i64 {
        self.0
    }
}

/// Telegram forum topic/thread id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelegramProfileThreadId(i64);

impl TelegramProfileThreadId {
    /// Build a Telegram thread id.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Return the raw id.
    #[must_use]
    pub const fn into_raw(self) -> i64 {
        self.0
    }
}

/// Telegram runtime session key extracted from logs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramSessionKey(String);

impl TelegramSessionKey {
    /// Build a session key.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the raw session key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Telegram chat type from runtime logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelegramChatType {
    /// Private chat.
    Private,
    /// Group chat.
    Group,
    /// Supergroup chat.
    Supergroup,
    /// Channel chat.
    Channel,
}

impl TelegramChatType {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "private" => Some(Self::Private),
            "group" => Some(Self::Group),
            "supergroup" => Some(Self::Supergroup),
            "channel" => Some(Self::Channel),
            _ => None,
        }
    }
}

/// Parsed Telegram group profile keyed by title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramGroupProfile {
    groups_by_title: BTreeMap<String, TelegramGroup>,
}

impl TelegramGroupProfile {
    /// Parse a profile from runtime log text.
    ///
    /// # Errors
    ///
    /// Returns an error when any requested title is missing from the log.
    pub fn parse_required(
        log_text: &str,
        required_titles: &[&str],
    ) -> Result<Self, TelegramGroupProfileError> {
        let groups_by_title = parse_groups_by_title(log_text);
        let missing_titles = required_titles
            .iter()
            .filter(|title| !groups_by_title.contains_key(**title))
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        if missing_titles.is_empty() {
            Ok(Self { groups_by_title })
        } else {
            Err(TelegramGroupProfileError::MissingTitles(missing_titles))
        }
    }

    /// Return a group by exact title.
    #[must_use]
    pub fn group(&self, title: &str) -> Option<&TelegramGroup> {
        self.groups_by_title.get(title)
    }

    /// Return titles present in sorted order.
    #[must_use]
    pub fn present_titles(&self) -> Vec<&str> {
        self.groups_by_title.keys().map(String::as_str).collect()
    }
}

/// Telegram group evidence extracted from a log line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramGroup {
    /// Telegram chat id.
    pub chat_id: TelegramChatId,
    /// Telegram group title.
    pub title: String,
    /// Telegram chat type from the log line.
    pub chat_type: TelegramChatType,
    /// Session key used by the channel runtime.
    pub session_key: TelegramSessionKey,
    /// Optional Telegram forum topic/thread id.
    pub thread_id: Option<TelegramProfileThreadId>,
}

/// Profile extraction failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelegramGroupProfileError {
    /// At least one requested title was not present.
    MissingTitles(Vec<String>),
}

impl Display for TelegramGroupProfileError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTitles(titles) => {
                write!(
                    formatter,
                    "missing group titles in log: {}",
                    titles.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for TelegramGroupProfileError {}

fn parse_groups_by_title(log_text: &str) -> BTreeMap<String, TelegramGroup> {
    log_text
        .lines()
        .filter_map(parse_group_line)
        .map(|group| (group.title.clone(), group))
        .collect()
}

fn parse_group_line(line: &str) -> Option<TelegramGroup> {
    let chat_id = TelegramChatId::new(parse_i64_after(line, "chat_id=Some(")?);
    let title = parse_quoted_after(line, "chat_title=Some(\"")?;
    let chat_type = TelegramChatType::parse(&parse_quoted_after(line, "chat_type=Some(\"")?)?;
    let session_key = TelegramSessionKey::new(parse_token_after(line, "session_key=")?);
    let thread_id = parse_thread_id(line).map(TelegramProfileThreadId::new);

    Some(TelegramGroup {
        chat_id,
        title,
        chat_type,
        session_key,
        thread_id,
    })
}

fn parse_i64_after(line: &str, marker: &str) -> Option<i64> {
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let end = rest.find(')')?;
    rest[..end].parse().ok()
}

fn parse_quoted_after(line: &str, marker: &str) -> Option<String> {
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn parse_token_after(line: &str, marker: &str) -> Option<String> {
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

fn parse_thread_id(line: &str) -> Option<i64> {
    parse_i64_after(line, "message_thread_id=Some(")
}
