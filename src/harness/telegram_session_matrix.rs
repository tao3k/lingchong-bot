//! Telegram session-matrix planning helpers.

use crate::channels::TelegramSessionPartition;

/// Telegram chat id used by session-matrix planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelegramMatrixChatId(i64);

impl TelegramMatrixChatId {
    /// Build a chat id.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Parse a chat id string.
    #[must_use]
    pub fn parse(value: impl AsRef<str>) -> Option<Self> {
        value.as_ref().trim().parse().ok().map(Self)
    }

    /// Return the raw id.
    #[must_use]
    pub const fn into_raw(self) -> i64 {
        self.0
    }

    const fn is_group_chat(self) -> bool {
        self.0 < 0
    }
}

/// Telegram user id used by session-matrix planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelegramMatrixUserId(i64);

impl TelegramMatrixUserId {
    /// Build a user id.
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

/// Telegram topic/thread id used by session-matrix planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelegramMatrixThreadId(i64);

impl TelegramMatrixThreadId {
    /// Build a thread id.
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

/// Request used to build the expected Telegram session key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelegramMatrixSessionKeyRequest {
    /// Session partition mode.
    pub partition: TelegramSessionPartition,
    /// Telegram chat id.
    pub chat_id: TelegramMatrixChatId,
    /// Telegram user id.
    pub user_id: TelegramMatrixUserId,
    /// Optional Telegram thread id.
    pub thread_id: Option<TelegramMatrixThreadId>,
}

/// Request used to select group chats for admin matrix probes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramAdminMatrixChatSelectionRequest {
    /// Explicit matrix chat ids.
    pub explicit_matrix_chat_ids: Vec<TelegramMatrixChatId>,
    /// Primary group chat id.
    pub group_chat_id: Option<TelegramMatrixChatId>,
    /// Allowed chat ids parsed from profile/environment configuration.
    pub allow_chat_ids: Vec<TelegramMatrixChatId>,
}

/// Build the expected session key for a Telegram matrix case.
#[must_use]
pub fn build_telegram_matrix_session_key(request: TelegramMatrixSessionKeyRequest) -> String {
    let chat_id = request.chat_id.into_raw().to_string();
    let user_id = request.user_id.into_raw().to_string();
    match request.partition {
        TelegramSessionPartition::ChatOnly => chat_id,
        TelegramSessionPartition::ChatUser => format!("{chat_id}:{user_id}"),
        TelegramSessionPartition::UserOnly => user_id,
        TelegramSessionPartition::ChatThreadUser => {
            let thread_id = request
                .thread_id
                .map_or(0, TelegramMatrixThreadId::into_raw);
            format!("{chat_id}:{thread_id}:{user_id}")
        }
    }
}

/// Build expected JSON fields for `/session memory json` assertions.
#[must_use]
pub fn build_telegram_session_memory_result_fields(
    request: TelegramMatrixSessionKeyRequest,
) -> Vec<String> {
    let session_key = build_telegram_matrix_session_key(request);
    vec![
        "json_kind=session_memory".to_string(),
        format!("json_session_scope=telegram:{session_key}"),
    ]
}

/// Select admin matrix group chats, preserving first-seen order.
#[must_use]
pub fn select_telegram_admin_matrix_chat_ids(
    request: TelegramAdminMatrixChatSelectionRequest,
) -> Vec<TelegramMatrixChatId> {
    request
        .explicit_matrix_chat_ids
        .into_iter()
        .chain(request.group_chat_id)
        .chain(
            request
                .allow_chat_ids
                .into_iter()
                .filter(|id| id.is_group_chat()),
        )
        .fold(Vec::new(), append_unique_chat_id)
}

fn append_unique_chat_id(
    mut selected: Vec<TelegramMatrixChatId>,
    chat_id: TelegramMatrixChatId,
) -> Vec<TelegramMatrixChatId> {
    if !selected.contains(&chat_id) {
        selected.push(chat_id);
    }
    selected
}
