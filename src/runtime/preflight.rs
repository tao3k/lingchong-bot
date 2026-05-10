//! Live-gated runtime preflight checks.

use super::{RuntimeConfig, RuntimeFlag};
use std::fmt::{Display, Formatter};

/// Live preflight status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LivePreflightStatus {
    /// Runtime has enough configuration for a live-gated bot probe.
    Ready,
    /// Runtime is missing required live-gated configuration.
    Blocked,
}

impl LivePreflightStatus {
    /// Return the stable diagnostic value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
        }
    }
}

/// Live preflight issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LivePreflightIssue {
    /// Neither Telegram nor Discord runtime is enabled.
    MissingEnabledChannel,
}

impl Display for LivePreflightIssue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingEnabledChannel => {
                formatter.write_str("no enabled Telegram or Discord bot channel")
            }
        }
    }
}

/// Secret-safe preflight report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivePreflightReport {
    /// Preflight status.
    pub status: LivePreflightStatus,
    /// Issues that block live-gated execution.
    pub issues: Vec<LivePreflightIssue>,
}

impl LivePreflightReport {
    /// Build a preflight report from runtime configuration.
    #[must_use]
    pub fn from_config(config: &RuntimeConfig) -> Self {
        let has_enabled_channel = matches!(config.telegram.enabled, RuntimeFlag::Enabled)
            || matches!(config.discord.enabled, RuntimeFlag::Enabled);
        let issues = if has_enabled_channel {
            Vec::new()
        } else {
            vec![LivePreflightIssue::MissingEnabledChannel]
        };
        let status = if issues.is_empty() {
            LivePreflightStatus::Ready
        } else {
            LivePreflightStatus::Blocked
        };

        Self { status, issues }
    }

    /// Return whether this report allows live-gated execution.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self.status, LivePreflightStatus::Ready)
    }

    /// Render the report as stable diagnostic lines.
    #[must_use]
    pub fn render_lines(&self) -> Vec<String> {
        let mut lines = vec![format!("live_preflight_status={}", self.status.as_str())];
        lines.extend(
            self.issues
                .iter()
                .map(|issue| format!("live_preflight_issue={issue}")),
        );
        lines
    }
}
