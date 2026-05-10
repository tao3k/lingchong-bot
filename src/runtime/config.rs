//! Environment-backed runtime configuration.

use std::fmt::{Display, Formatter};

const AGENT_GATEWAY_URL_ENV: &str = "LINGCHONG_AGENT_GATEWAY_URL";
const TELEGRAM_BOT_TOKEN_ENV: &str = "LINGCHONG_TELEGRAM_BOT_TOKEN";
const TELEGRAM_WEBHOOK_SECRET_ENV: &str = "LINGCHONG_TELEGRAM_WEBHOOK_SECRET";
const TELEGRAM_WEBHOOK_BIND_ENV: &str = "LINGCHONG_TELEGRAM_WEBHOOK_BIND";
const TELEGRAM_WEBHOOK_PORT_ENV: &str = "LINGCHONG_TELEGRAM_WEBHOOK_PORT";
const TELEGRAM_API_BASE_URL_ENV: &str = "LINGCHONG_TELEGRAM_API_BASE_URL";
const DISCORD_BOT_TOKEN_ENV: &str = "LINGCHONG_DISCORD_BOT_TOKEN";
const DISCORD_INGRESS_SECRET_ENV: &str = "LINGCHONG_DISCORD_INGRESS_SECRET";
const DISCORD_INGRESS_BIND_ENV: &str = "LINGCHONG_DISCORD_INGRESS_BIND";
const DISCORD_INGRESS_PATH_ENV: &str = "LINGCHONG_DISCORD_INGRESS_PATH";
const DISCORD_INGRESS_URL_ENV: &str = "LINGCHONG_DISCORD_INGRESS_URL";
const DISCORD_REQUIRE_MENTION_ENV: &str = "LINGCHONG_DISCORD_REQUIRE_MENTION";

const DEFAULT_TELEGRAM_API_BASE_URL: &str = "https://api.telegram.org";
const DEFAULT_TELEGRAM_WEBHOOK_PORT: u16 = 18081;
const DEFAULT_DISCORD_INGRESS_BIND: &str = "127.0.0.1:18082";
const DEFAULT_DISCORD_INGRESS_PATH: &str = "/discord/ingress";
const DEFAULT_LOCAL_HOST: &str = "127.0.0.1";

/// External agent gateway runtime configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRuntimeConfig {
    /// Base URL for the external agent gateway.
    pub gateway_url: AgentGatewayUrl,
}

/// External agent gateway URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentGatewayUrl(String);

impl AgentGatewayUrl {
    /// Create a gateway URL value.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the URL as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Channel HTTP bind address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelBindAddress(String);

impl ChannelBindAddress {
    /// Create a bind address value.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the bind address as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Channel HTTP port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelPort(u16);

impl ChannelPort {
    /// Create a port value.
    #[must_use]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Return the raw port value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

/// Channel HTTP path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelPath(String);

impl ChannelPath {
    /// Create a path value.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the path as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Local channel ingress or webhook URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelLocalUrl(String);

impl ChannelLocalUrl {
    /// Create a local URL value.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the URL as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Channel API base URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelApiBaseUrl(String);

impl ChannelApiBaseUrl {
    /// Create an API base URL value.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the URL as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Runtime configuration for one chat channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelRuntimeConfig {
    /// Whether the channel has enough local configuration to start.
    pub enabled: RuntimeFlag,
    /// Whether an inbound or outbound secret was configured.
    pub secret_configured: SecretFlag,
    /// Optional inbound HTTP bind address for webhook or ingress mode.
    pub inbound_bind: Option<ChannelBindAddress>,
    /// Optional inbound HTTP port for webhook or ingress mode.
    pub inbound_port: Option<ChannelPort>,
    /// Optional inbound HTTP path for webhook or ingress mode.
    pub inbound_path: Option<ChannelPath>,
    /// Optional local URL for webhook or ingress probes.
    pub inbound_url: Option<ChannelLocalUrl>,
    /// Optional channel API base URL.
    pub api_base_url: Option<ChannelApiBaseUrl>,
    /// Whether Discord messages must mention the bot before turn execution.
    pub require_mention: MentionPolicy,
}

/// Complete bot runtime configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// External agent gateway configuration.
    pub agent: AgentRuntimeConfig,
    /// Telegram runtime configuration.
    pub telegram: ChannelRuntimeConfig,
    /// Discord runtime configuration.
    pub discord: ChannelRuntimeConfig,
}

impl RuntimeConfig {
    /// Load runtime configuration from process environment variables.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeConfigError`] when required configuration is missing or
    /// an environment value is invalid.
    pub fn from_env() -> Result<Self, RuntimeConfigError> {
        Self::from_env_lookup(|key| std::env::var(key).ok())
    }

    /// Load runtime configuration from an injected environment lookup.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeConfigError`] when required configuration is missing or
    /// an environment value is invalid.
    pub fn from_env_lookup(
        lookup: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, RuntimeConfigError> {
        let gateway_url = required_env(&lookup, AGENT_GATEWAY_URL_ENV)?;
        let telegram_token = optional_env(&lookup, TELEGRAM_BOT_TOKEN_ENV);
        let telegram_secret = optional_env(&lookup, TELEGRAM_WEBHOOK_SECRET_ENV);
        let telegram_webhook_bind = optional_env(&lookup, TELEGRAM_WEBHOOK_BIND_ENV);
        let telegram_webhook_port = resolve_optional_port(
            &lookup,
            TELEGRAM_WEBHOOK_PORT_ENV,
            telegram_webhook_bind.as_deref(),
        )?
        .or(Some(DEFAULT_TELEGRAM_WEBHOOK_PORT));
        let telegram_api_base_url = optional_env(&lookup, TELEGRAM_API_BASE_URL_ENV)
            .or_else(|| Some(DEFAULT_TELEGRAM_API_BASE_URL.to_string()));

        let discord_token = optional_env(&lookup, DISCORD_BOT_TOKEN_ENV);
        let discord_secret = optional_env(&lookup, DISCORD_INGRESS_SECRET_ENV);
        let discord_ingress_bind = optional_env(&lookup, DISCORD_INGRESS_BIND_ENV)
            .or_else(|| Some(DEFAULT_DISCORD_INGRESS_BIND.to_string()));
        let discord_ingress_path = optional_env(&lookup, DISCORD_INGRESS_PATH_ENV)
            .map(|path| normalize_path(&path))
            .or_else(|| Some(DEFAULT_DISCORD_INGRESS_PATH.to_string()));
        let discord_ingress_url = optional_env(&lookup, DISCORD_INGRESS_URL_ENV).or_else(|| {
            Some(default_local_url(
                discord_ingress_bind
                    .as_deref()
                    .unwrap_or(DEFAULT_DISCORD_INGRESS_BIND),
                discord_ingress_path
                    .as_deref()
                    .unwrap_or(DEFAULT_DISCORD_INGRESS_PATH),
            ))
        });
        let discord_require_mention =
            optional_bool_env(&lookup, DISCORD_REQUIRE_MENTION_ENV)?.unwrap_or(true);

        Ok(Self {
            agent: AgentRuntimeConfig {
                gateway_url: AgentGatewayUrl::new(gateway_url),
            },
            telegram: ChannelRuntimeConfig {
                enabled: RuntimeFlag::from_bool(telegram_token.is_some()),
                secret_configured: SecretFlag::from_bool(telegram_secret.is_some()),
                inbound_bind: telegram_webhook_bind.map(ChannelBindAddress::new),
                inbound_port: telegram_webhook_port.map(ChannelPort::new),
                inbound_path: Some(ChannelPath::new("/telegram/webhook")),
                inbound_url: None,
                api_base_url: telegram_api_base_url.map(ChannelApiBaseUrl::new),
                require_mention: MentionPolicy::NotRequired,
            },
            discord: ChannelRuntimeConfig {
                enabled: RuntimeFlag::from_bool(discord_token.is_some()),
                secret_configured: SecretFlag::from_bool(discord_secret.is_some()),
                inbound_bind: discord_ingress_bind.map(ChannelBindAddress::new),
                inbound_port: None,
                inbound_path: discord_ingress_path.map(ChannelPath::new),
                inbound_url: discord_ingress_url.map(ChannelLocalUrl::new),
                api_base_url: None,
                require_mention: MentionPolicy::from_bool(discord_require_mention),
            },
        })
    }

    /// Return a secret-safe summary for CLI and diagnostics.
    #[must_use]
    pub fn summary(&self) -> RuntimeConfigSummary {
        RuntimeConfigSummary {
            agent_gateway_url: self.agent.gateway_url.clone(),
            telegram_runtime: self.telegram.enabled,
            telegram_webhook_secret: self.telegram.secret_configured,
            telegram_webhook_bind: self.telegram.inbound_bind.clone(),
            telegram_webhook_port: self.telegram.inbound_port,
            telegram_api_base_url: self.telegram.api_base_url.clone(),
            discord_runtime: self.discord.enabled,
            discord_ingress_secret: self.discord.secret_configured,
            discord_ingress_bind: self.discord.inbound_bind.clone(),
            discord_ingress_path: self.discord.inbound_path.clone(),
            discord_ingress_url: self.discord.inbound_url.clone(),
            discord_mention_policy: self.discord.require_mention,
        }
    }
}

/// Secret-safe runtime configuration summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfigSummary {
    /// Base URL for the external agent gateway.
    pub agent_gateway_url: AgentGatewayUrl,
    /// Whether Telegram runtime is enabled.
    pub telegram_runtime: RuntimeFlag,
    /// Whether Telegram webhook secret is configured.
    pub telegram_webhook_secret: SecretFlag,
    /// Telegram webhook bind address.
    pub telegram_webhook_bind: Option<ChannelBindAddress>,
    /// Telegram webhook port.
    pub telegram_webhook_port: Option<ChannelPort>,
    /// Telegram API base URL.
    pub telegram_api_base_url: Option<ChannelApiBaseUrl>,
    /// Whether Discord runtime is enabled.
    pub discord_runtime: RuntimeFlag,
    /// Whether Discord ingress secret is configured.
    pub discord_ingress_secret: SecretFlag,
    /// Discord ingress bind address.
    pub discord_ingress_bind: Option<ChannelBindAddress>,
    /// Discord ingress path.
    pub discord_ingress_path: Option<ChannelPath>,
    /// Discord local ingress URL.
    pub discord_ingress_url: Option<ChannelLocalUrl>,
    /// Whether Discord requires bot mention before handling a message.
    pub discord_mention_policy: MentionPolicy,
}

impl RuntimeConfigSummary {
    /// Render the summary as stable diagnostic lines.
    #[must_use]
    pub fn render_lines(&self) -> Vec<String> {
        vec![
            format!("agent_gateway_url={}", self.agent_gateway_url.as_str()),
            format!("telegram_enabled={}", self.telegram_runtime.as_str()),
            format!(
                "telegram_webhook_secret_configured={}",
                self.telegram_webhook_secret.as_str()
            ),
            format!(
                "telegram_webhook_bind={}",
                self.telegram_webhook_bind
                    .as_ref()
                    .map_or("", ChannelBindAddress::as_str)
            ),
            format!(
                "telegram_webhook_port={}",
                self.telegram_webhook_port
                    .map(|port| port.as_u16().to_string())
                    .unwrap_or_default()
            ),
            format!(
                "telegram_api_base_url={}",
                self.telegram_api_base_url
                    .as_ref()
                    .map_or("", ChannelApiBaseUrl::as_str)
            ),
            format!("discord_enabled={}", self.discord_runtime.as_str()),
            format!(
                "discord_ingress_secret_configured={}",
                self.discord_ingress_secret.as_str()
            ),
            format!(
                "discord_ingress_bind={}",
                self.discord_ingress_bind
                    .as_ref()
                    .map_or("", ChannelBindAddress::as_str)
            ),
            format!(
                "discord_ingress_path={}",
                self.discord_ingress_path
                    .as_ref()
                    .map_or("", ChannelPath::as_str)
            ),
            format!(
                "discord_ingress_url={}",
                self.discord_ingress_url
                    .as_ref()
                    .map_or("", ChannelLocalUrl::as_str)
            ),
            format!(
                "discord_require_mention={}",
                self.discord_mention_policy.as_str()
            ),
        ]
    }
}

/// Runtime enablement state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeFlag {
    /// Runtime is enabled.
    Enabled,
    /// Runtime is disabled.
    Disabled,
}

impl RuntimeFlag {
    #[must_use]
    const fn from_bool(enabled: bool) -> Self {
        if enabled {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }

    #[must_use]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "true",
            Self::Disabled => "false",
        }
    }
}

/// Secret configuration state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecretFlag {
    /// Secret is configured.
    Configured,
    /// Secret is not configured.
    Missing,
}

impl SecretFlag {
    #[must_use]
    const fn from_bool(configured: bool) -> Self {
        if configured {
            Self::Configured
        } else {
            Self::Missing
        }
    }

    #[must_use]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Configured => "true",
            Self::Missing => "false",
        }
    }
}

/// Discord mention gate policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MentionPolicy {
    /// Discord messages must mention the bot before execution.
    Required,
    /// Discord messages can execute without mention.
    NotRequired,
}

impl MentionPolicy {
    #[must_use]
    const fn from_bool(required: bool) -> Self {
        if required {
            Self::Required
        } else {
            Self::NotRequired
        }
    }

    #[must_use]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "true",
            Self::NotRequired => "false",
        }
    }
}

/// Runtime configuration loading error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeConfigError {
    /// Required environment variable is missing or blank.
    MissingRequiredEnv {
        /// Environment variable name.
        name: &'static str,
    },
    /// Boolean environment variable has an invalid value.
    InvalidBoolEnv {
        /// Environment variable name.
        name: &'static str,
        /// Invalid raw value.
        value: String,
    },
    /// Port environment variable has an invalid value.
    InvalidPortEnv {
        /// Environment variable name.
        name: &'static str,
        /// Invalid raw value.
        value: String,
    },
}

impl Display for RuntimeConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRequiredEnv { name } => {
                write!(formatter, "required environment variable {name} is missing")
            }
            Self::InvalidBoolEnv { name, value } => {
                write!(
                    formatter,
                    "environment variable {name} has invalid boolean value {value:?}"
                )
            }
            Self::InvalidPortEnv { name, value } => {
                write!(
                    formatter,
                    "environment variable {name} has invalid port value {value:?}"
                )
            }
        }
    }
}

impl std::error::Error for RuntimeConfigError {}

fn required_env(
    lookup: &impl Fn(&str) -> Option<String>,
    name: &'static str,
) -> Result<String, RuntimeConfigError> {
    optional_env(lookup, name).ok_or(RuntimeConfigError::MissingRequiredEnv { name })
}

fn optional_env(lookup: &impl Fn(&str) -> Option<String>, name: &str) -> Option<String> {
    lookup(name)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn optional_bool_env(
    lookup: &impl Fn(&str) -> Option<String>,
    name: &'static str,
) -> Result<Option<bool>, RuntimeConfigError> {
    let Some(value) = optional_env(lookup, name) else {
        return Ok(None);
    };

    match value.as_str() {
        "1" | "true" | "TRUE" | "True" | "yes" | "YES" | "on" | "ON" => Ok(Some(true)),
        "0" | "false" | "FALSE" | "False" | "no" | "NO" | "off" | "OFF" => Ok(Some(false)),
        _ => Err(RuntimeConfigError::InvalidBoolEnv { name, value }),
    }
}

fn resolve_optional_port(
    lookup: &impl Fn(&str) -> Option<String>,
    name: &'static str,
    bind: Option<&str>,
) -> Result<Option<u16>, RuntimeConfigError> {
    if let Some(value) = optional_env(lookup, name) {
        return parse_port(&value, name).map(Some);
    }

    Ok(bind.and_then(port_from_bind))
}

fn parse_port(value: &str, name: &'static str) -> Result<u16, RuntimeConfigError> {
    value
        .parse::<u16>()
        .ok()
        .filter(|port| *port > 0)
        .ok_or_else(|| RuntimeConfigError::InvalidPortEnv {
            name,
            value: value.to_string(),
        })
}

fn port_from_bind(bind: &str) -> Option<u16> {
    bind.rsplit_once(':')
        .and_then(|(_, raw_port)| raw_port.parse::<u16>().ok())
        .filter(|port| *port > 0)
}

fn normalize_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        DEFAULT_DISCORD_INGRESS_PATH.to_string()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

fn default_local_url(bind: &str, path: &str) -> String {
    format!(
        "http://{}{}",
        normalize_bind_for_local_url(bind),
        normalize_path(path)
    )
}

fn normalize_bind_for_local_url(bind: &str) -> String {
    let trimmed = bind.trim();
    if trimmed.is_empty() {
        return DEFAULT_DISCORD_INGRESS_BIND.to_string();
    }

    let Some((host, port)) = trimmed.rsplit_once(':') else {
        return format!("{DEFAULT_LOCAL_HOST}:{trimmed}");
    };

    let normalized_host = match host.trim().trim_matches(['[', ']']) {
        "" | "0.0.0.0" | "::" => DEFAULT_LOCAL_HOST,
        value => value,
    };
    format!("{normalized_host}:{}", port.trim())
}
