//! Discord ingress stress harness configuration.

use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

/// Discord channel id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordChannelId(String);

impl DiscordChannelId {
    /// Build a channel id.
    ///
    /// # Errors
    ///
    /// Returns an error when the id is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, DiscordIngressStressConfigError> {
        normalize_required(value.as_ref(), "channel_id").map(Self)
    }

    /// Return the raw id.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Discord user id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordUserId(String);

impl DiscordUserId {
    /// Build a user id.
    ///
    /// # Errors
    ///
    /// Returns an error when the id is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, DiscordIngressStressConfigError> {
        normalize_required(value.as_ref(), "user_id").map(Self)
    }

    /// Return the raw id.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Discord guild id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordGuildId(String);

impl DiscordGuildId {
    /// Build a guild id.
    #[must_use]
    pub fn new(value: impl AsRef<str>) -> Option<Self> {
        normalize_optional(value.as_ref()).map(Self)
    }

    /// Return the raw id.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Discord role id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordRoleId(String);

impl DiscordRoleId {
    /// Build a role id.
    #[must_use]
    pub fn new(value: impl AsRef<str>) -> Option<Self> {
        normalize_optional(value.as_ref()).map(Self)
    }

    /// Return the raw id.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Request used to build a Discord ingress stress-test configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct DiscordIngressStressConfigRequest {
    /// Stress rounds after warmup.
    pub rounds: u32,
    /// Warmup rounds before measurement.
    pub warmup_rounds: u32,
    /// Number of parallel workers.
    pub parallel: u32,
    /// Requests each worker sends per round.
    pub requests_per_worker: u32,
    /// Discord channel id.
    pub channel_id: DiscordChannelId,
    /// Discord user id.
    pub user_id: DiscordUserId,
    /// Optional Discord guild id.
    pub guild_id: Option<DiscordGuildId>,
    /// Role ids.
    pub role_ids: Vec<DiscordRoleId>,
    /// Runtime log path.
    pub log_file: PathBuf,
    /// JSON report path.
    pub output_json: PathBuf,
    /// Markdown report path.
    pub output_markdown: PathBuf,
    /// Optional p95 quality gate in milliseconds. Non-positive values disable the gate.
    pub quality_max_p95_ms: f64,
    /// Optional minimum requests-per-second quality gate. Non-positive values disable the gate.
    pub quality_min_rps: f64,
}

/// Discord ingress stress-test configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct DiscordIngressStressConfig {
    /// Stress rounds after warmup.
    pub rounds: u32,
    /// Warmup rounds before measurement.
    pub warmup_rounds: u32,
    /// Number of parallel workers.
    pub parallel: u32,
    /// Requests each worker sends per round.
    pub requests_per_worker: u32,
    /// Discord channel id.
    pub channel_id: DiscordChannelId,
    /// Discord user id.
    pub user_id: DiscordUserId,
    /// Optional Discord guild id.
    pub guild_id: Option<DiscordGuildId>,
    /// Deduplicated role ids.
    pub role_ids: Vec<DiscordRoleId>,
    /// Runtime log path.
    pub log_file: PathBuf,
    /// JSON report path.
    pub output_json: PathBuf,
    /// Markdown report path.
    pub output_markdown: PathBuf,
    /// Optional p95 quality gate in milliseconds.
    pub quality_max_p95_ms: Option<f64>,
    /// Optional minimum requests-per-second quality gate.
    pub quality_min_rps: Option<f64>,
}

impl DiscordIngressStressConfig {
    /// Build a normalized stress config.
    ///
    /// # Errors
    ///
    /// Returns an error when numeric fields are zero.
    pub fn build(
        request: DiscordIngressStressConfigRequest,
    ) -> Result<Self, DiscordIngressStressConfigError> {
        require_non_zero(request.rounds, "rounds")?;
        require_non_zero(request.parallel, "parallel")?;
        require_non_zero(request.requests_per_worker, "requests_per_worker")?;

        Ok(Self {
            rounds: request.rounds,
            warmup_rounds: request.warmup_rounds,
            parallel: request.parallel,
            requests_per_worker: request.requests_per_worker,
            channel_id: request.channel_id,
            user_id: request.user_id,
            guild_id: request.guild_id,
            role_ids: dedup_role_ids(request.role_ids),
            log_file: absolute_path(&request.log_file),
            output_json: absolute_path(&request.output_json),
            output_markdown: absolute_path(&request.output_markdown),
            quality_max_p95_ms: positive_gate(request.quality_max_p95_ms),
            quality_min_rps: positive_gate(request.quality_min_rps),
        })
    }
}

/// Discord ingress config failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscordIngressStressConfigError {
    /// Required text field was empty.
    MissingRequired(&'static str),
    /// Required numeric field was zero.
    ZeroValue(&'static str),
}

impl Display for DiscordIngressStressConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRequired(field) => write!(formatter, "{field} is required"),
            Self::ZeroValue(field) => write!(formatter, "{field} must be greater than zero"),
        }
    }
}

impl std::error::Error for DiscordIngressStressConfigError {}

const fn require_non_zero(
    value: u32,
    field: &'static str,
) -> Result<(), DiscordIngressStressConfigError> {
    if value == 0 {
        Err(DiscordIngressStressConfigError::ZeroValue(field))
    } else {
        Ok(())
    }
}

fn normalize_required(
    value: &str,
    field: &'static str,
) -> Result<String, DiscordIngressStressConfigError> {
    normalize_optional(value).ok_or(DiscordIngressStressConfigError::MissingRequired(field))
}

fn normalize_optional(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn dedup_role_ids(values: Vec<DiscordRoleId>) -> Vec<DiscordRoleId> {
    values.into_iter().fold(Vec::new(), |mut acc, value| {
        if !acc.contains(&value) {
            acc.push(value);
        }
        acc
    })
}

fn positive_gate(value: f64) -> Option<f64> {
    (value > 0.0).then_some(value)
}

fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}
