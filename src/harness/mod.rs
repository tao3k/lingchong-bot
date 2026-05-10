//! Deterministic channel harness helpers.

/// Discord ACL command-reply validation helpers.
pub mod discord_acl;
/// Discord ingress stress harness configuration helpers.
pub mod discord_ingress;
/// Telegram group profile extraction helpers.
pub mod telegram_profile;
/// Telegram session-matrix planning helpers.
pub mod telegram_session_matrix;

pub use discord_acl::{
    DiscordAclEventName, DiscordAclJsonSummaryObservation, DiscordAclJsonSummaryValidationRequest,
    DiscordAclProbeCase, DiscordAclProbeCaseFilterRequest, DiscordAclProbeCaseId,
    DiscordAclProbeParseError, DiscordAclProbeSuite, DiscordAclReplyObservation,
    DiscordAclReplyValidationRequest, DiscordAclValidationError, build_discord_acl_probe_cases,
    filter_discord_acl_probe_cases, parse_discord_acl_probe_case_ids,
    parse_discord_acl_probe_suites, validate_discord_acl_json_summary, validate_discord_acl_reply,
};
pub use discord_ingress::{
    DiscordChannelId, DiscordGuildId, DiscordIngressStressConfig, DiscordIngressStressConfigError,
    DiscordIngressStressConfigRequest, DiscordRoleId, DiscordUserId,
};
pub use telegram_profile::{
    TelegramChatId, TelegramChatType, TelegramGroupProfile, TelegramGroupProfileError,
    TelegramProfileThreadId, TelegramSessionKey,
};
pub use telegram_session_matrix::{
    TelegramAdminMatrixChatSelectionRequest, TelegramMatrixChatId, TelegramMatrixSessionKeyRequest,
    TelegramMatrixThreadId, TelegramMatrixUserId, build_telegram_matrix_session_key,
    build_telegram_session_memory_result_fields, select_telegram_admin_matrix_chat_ids,
};
