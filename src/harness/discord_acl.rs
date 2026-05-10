//! Discord ACL command-reply validation helpers.

use super::discord_ingress::DiscordUserId;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Discord ACL reply observation extracted from runtime logs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordAclReplyObservation {
    /// Event name emitted by the command handler.
    pub event: String,
    /// Recipient channel/user id.
    pub recipient: String,
    /// Runtime session key.
    pub session_key: String,
}

/// Discord ACL JSON summary observation extracted from runtime logs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordAclJsonSummaryObservation {
    /// Event name emitted by the command handler.
    pub event: String,
    /// Recipient channel/user id.
    pub recipient: String,
    /// Runtime session key.
    pub session_key: String,
    /// JSON session scope returned by the command summary.
    pub json_session_scope: String,
}

/// Discord ACL probe case identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscordAclProbeCaseId {
    /// `/session admin add ...` should require admin authorization.
    ControlAdminDenied,
    /// `/session memory` slash-style command should require permission.
    SlashPermissionDenied,
}

impl DiscordAclProbeCaseId {
    /// Return the stable case id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlAdminDenied => "discord_control_admin_denied",
            Self::SlashPermissionDenied => "discord_slash_permission_denied",
        }
    }
}

impl Display for DiscordAclProbeCaseId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DiscordAclProbeCaseId {
    type Err = DiscordAclProbeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalize_selector_token(value).as_str() {
            "discord_control_admin_denied" | "control_admin_denied" | "admin_denied" => {
                Ok(Self::ControlAdminDenied)
            }
            "discord_slash_permission_denied" | "slash_permission_denied" | "permission_denied" => {
                Ok(Self::SlashPermissionDenied)
            }
            _ => Err(DiscordAclProbeParseError::InvalidCaseId(value.to_string())),
        }
    }
}

/// Discord ACL probe suite identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscordAclProbeSuite {
    /// All suites.
    All,
    /// Core ACL behavior.
    Core,
}

impl DiscordAclProbeSuite {
    /// Return the stable suite id.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Core => "core",
        }
    }
}

impl Display for DiscordAclProbeSuite {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DiscordAclProbeSuite {
    type Err = DiscordAclProbeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalize_selector_token(value).as_str() {
            "" | "all" => Ok(Self::All),
            "core" => Ok(Self::Core),
            _ => Err(DiscordAclProbeParseError::InvalidSuite(value.to_string())),
        }
    }
}

/// Discord ACL runtime event identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscordAclEventName {
    /// Admin-only control command reply.
    ControlAdminRequiredReply,
    /// Slash permission required reply.
    SlashPermissionRequiredReply,
}

impl DiscordAclEventName {
    /// Return the runtime event name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlAdminRequiredReply => "discord.command.control_admin_required.replied",
            Self::SlashPermissionRequiredReply => {
                "discord.command.slash_permission_required.replied"
            }
        }
    }
}

/// Definition of a Discord ACL probe case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordAclProbeCase {
    case_id: DiscordAclProbeCaseId,
    prompt: String,
    event_name: DiscordAclEventName,
    suites: Vec<DiscordAclProbeSuite>,
}

impl DiscordAclProbeCase {
    /// Return the stable case id.
    #[must_use]
    pub const fn case_id(&self) -> DiscordAclProbeCaseId {
        self.case_id
    }

    /// Return the prompt to send to Discord ingress.
    #[must_use]
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Return the expected runtime event.
    #[must_use]
    pub const fn event_name(&self) -> DiscordAclEventName {
        self.event_name
    }

    /// Return the suites this case belongs to.
    #[must_use]
    pub fn suites(&self) -> &[DiscordAclProbeSuite] {
        &self.suites
    }
}

/// Request used to filter Discord ACL probe cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscordAclProbeCaseFilterRequest<'a> {
    /// Selected suites.
    pub suites: &'a [DiscordAclProbeSuite],
    /// Explicit selected case ids.
    pub requested_case_ids: &'a [DiscordAclProbeCaseId],
}

/// Discord ACL validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscordAclValidationError {
    /// Target event/recipient pair was not observed.
    MissingTargetReply,
    /// Reply session key was outside the expected scope set.
    ReplySessionMismatch {
        /// Observed runtime session key.
        observed_session: String,
    },
    /// JSON summary session key was outside the expected scope set.
    JsonSummarySessionMismatch {
        /// Observed runtime session key.
        observed_session: String,
    },
    /// JSON summary session scope was outside the expected scope set.
    JsonSummaryScopeMismatch {
        /// Observed JSON session scope.
        observed_scope: String,
    },
}

/// Discord ACL probe selector parse failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscordAclProbeParseError {
    /// Case id is not part of the ACL probe catalog.
    InvalidCaseId(String),
    /// Suite id is not part of the ACL probe catalog.
    InvalidSuite(String),
}

/// Request used to validate a target Discord ACL reply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscordAclReplyValidationRequest<'a> {
    /// Expected event name.
    pub event_name: &'a str,
    /// Expected recipient id.
    pub expected_recipient: &'a str,
    /// Accepted runtime session keys.
    pub expected_sessions: &'a [&'a str],
}

/// Request used to validate a target Discord ACL JSON summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscordAclJsonSummaryValidationRequest<'a> {
    /// Expected event name.
    pub event_name: &'a str,
    /// Expected recipient id.
    pub expected_recipient: &'a str,
    /// Accepted runtime session keys.
    pub expected_sessions: &'a [&'a str],
    /// Accepted JSON session scopes.
    pub expected_session_scopes: &'a [&'a str],
}

impl Display for DiscordAclValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTargetReply => formatter.write_str("missing target command reply"),
            Self::ReplySessionMismatch { observed_session } => {
                write!(
                    formatter,
                    "command reply session mismatch: {observed_session}"
                )
            }
            Self::JsonSummarySessionMismatch { observed_session } => {
                write!(
                    formatter,
                    "command reply JSON summary session mismatch: {observed_session}"
                )
            }
            Self::JsonSummaryScopeMismatch { observed_scope } => {
                write!(
                    formatter,
                    "command reply JSON summary scope mismatch: {observed_scope}"
                )
            }
        }
    }
}

impl std::error::Error for DiscordAclValidationError {}

impl Display for DiscordAclProbeParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCaseId(value) => write!(formatter, "invalid Discord ACL case id: {value}"),
            Self::InvalidSuite(value) => write!(formatter, "invalid Discord ACL suite: {value}"),
        }
    }
}

impl std::error::Error for DiscordAclProbeParseError {}

/// Parse and normalize selected Discord ACL suites.
///
/// # Errors
///
/// Returns an error when any suite selector is not part of the ACL probe
/// catalog.
pub fn parse_discord_acl_probe_suites(
    values: &[&str],
) -> Result<Vec<DiscordAclProbeSuite>, DiscordAclProbeParseError> {
    let suites = dedup_parsed_values(values)?;
    if suites.is_empty() || suites.contains(&DiscordAclProbeSuite::All) {
        Ok(vec![DiscordAclProbeSuite::All])
    } else {
        Ok(suites)
    }
}

/// Parse and normalize selected Discord ACL case ids.
///
/// # Errors
///
/// Returns an error when any case selector is not part of the ACL probe
/// catalog.
pub fn parse_discord_acl_probe_case_ids(
    values: &[&str],
) -> Result<Vec<DiscordAclProbeCaseId>, DiscordAclProbeParseError> {
    dedup_parsed_values(values)
}

/// Build the default Discord ACL probe case set.
#[must_use]
pub fn build_discord_acl_probe_cases(target_user_id: &DiscordUserId) -> Vec<DiscordAclProbeCase> {
    vec![
        DiscordAclProbeCase {
            case_id: DiscordAclProbeCaseId::ControlAdminDenied,
            prompt: format!("/session admin add {}", target_user_id.as_str()),
            event_name: DiscordAclEventName::ControlAdminRequiredReply,
            suites: vec![DiscordAclProbeSuite::Core],
        },
        DiscordAclProbeCase {
            case_id: DiscordAclProbeCaseId::SlashPermissionDenied,
            prompt: "/session memory".to_string(),
            event_name: DiscordAclEventName::SlashPermissionRequiredReply,
            suites: vec![DiscordAclProbeSuite::Core],
        },
    ]
}

fn dedup_parsed_values<T>(values: &[&str]) -> Result<Vec<T>, T::Err>
where
    T: FromStr + PartialEq,
{
    values.iter().try_fold(Vec::new(), |mut selected, value| {
        let parsed = value.parse::<T>()?;
        if !selected.contains(&parsed) {
            selected.push(parsed);
        }
        Ok(selected)
    })
}

fn normalize_selector_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('-', "_")
}

/// Filter Discord ACL probe cases by suite and explicit case id selection.
#[must_use]
pub fn filter_discord_acl_probe_cases(
    request: DiscordAclProbeCaseFilterRequest<'_>,
    cases: &[DiscordAclProbeCase],
) -> Vec<DiscordAclProbeCase> {
    cases
        .iter()
        .filter(|case| {
            request.requested_case_ids.is_empty()
                || request.requested_case_ids.contains(&case.case_id)
        })
        .filter(|case| {
            request.suites.contains(&DiscordAclProbeSuite::All)
                || case
                    .suites
                    .iter()
                    .any(|suite| request.suites.contains(suite))
        })
        .cloned()
        .collect()
}

/// Validate that a target Discord command reply belongs to an expected session.
///
/// # Errors
///
/// Returns an error when the target reply is missing or its session key is not
/// in the expected set.
pub fn validate_discord_acl_reply<'a>(
    request: DiscordAclReplyValidationRequest<'_>,
    observations: &'a [DiscordAclReplyObservation],
) -> Result<&'a DiscordAclReplyObservation, DiscordAclValidationError> {
    let Some(target) = observations.iter().find(|observation| {
        observation.event == request.event_name
            && observation.recipient == request.expected_recipient
    }) else {
        return Err(DiscordAclValidationError::MissingTargetReply);
    };

    if !target.session_key.is_empty()
        && !request
            .expected_sessions
            .contains(&target.session_key.as_str())
    {
        return Err(DiscordAclValidationError::ReplySessionMismatch {
            observed_session: target.session_key.clone(),
        });
    }

    Ok(target)
}

/// Validate optional JSON summary scope for a Discord command reply.
///
/// # Errors
///
/// Returns an error when a matching summary exists but its session key or JSON
/// session scope is outside the expected sets.
pub fn validate_discord_acl_json_summary<'a>(
    request: DiscordAclJsonSummaryValidationRequest<'_>,
    summaries: &'a [DiscordAclJsonSummaryObservation],
) -> Result<Option<&'a DiscordAclJsonSummaryObservation>, DiscordAclValidationError> {
    let Some(target) = summaries.iter().find(|summary| {
        summary.event == request.event_name && summary.recipient == request.expected_recipient
    }) else {
        return Ok(None);
    };

    if !target.session_key.is_empty()
        && !request
            .expected_sessions
            .contains(&target.session_key.as_str())
    {
        return Err(DiscordAclValidationError::JsonSummarySessionMismatch {
            observed_session: target.session_key.clone(),
        });
    }

    if !target.json_session_scope.is_empty()
        && !request
            .expected_session_scopes
            .contains(&target.json_session_scope.as_str())
    {
        return Err(DiscordAclValidationError::JsonSummaryScopeMismatch {
            observed_scope: target.json_session_scope.clone(),
        });
    }

    Ok(Some(target))
}
