//! Live-gated bot launch planning.

use super::{
    AgentGatewayUrl, ChannelBindAddress, ChannelLocalUrl, ChannelPath, ChannelPort, RuntimeConfig,
    RuntimeFlag,
};

/// Live launch plan status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveLaunchPlanStatus {
    /// At least one channel is planned.
    Runnable,
    /// No channel can be planned from current configuration.
    Empty,
}

impl LiveLaunchPlanStatus {
    /// Return the stable diagnostic value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Runnable => "runnable",
            Self::Empty => "empty",
        }
    }
}

/// Live channel launch plan item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveLaunchChannel {
    /// Telegram webhook runtime.
    TelegramWebhook {
        /// Webhook bind address when configured.
        bind: Option<ChannelBindAddress>,
        /// Webhook port.
        port: Option<ChannelPort>,
        /// Webhook path.
        path: Option<ChannelPath>,
    },
    /// Discord ingress runtime.
    DiscordIngress {
        /// Ingress bind address.
        bind: Option<ChannelBindAddress>,
        /// Ingress path.
        path: Option<ChannelPath>,
        /// Local ingress URL.
        url: Option<ChannelLocalUrl>,
    },
}

impl LiveLaunchChannel {
    /// Return the stable channel id.
    #[must_use]
    pub const fn id(&self) -> &'static str {
        match self {
            Self::TelegramWebhook { .. } => "telegram_webhook",
            Self::DiscordIngress { .. } => "discord_ingress",
        }
    }

    fn render_lines(&self) -> Vec<String> {
        match self {
            Self::TelegramWebhook { bind, port, path } => vec![
                "live_channel=telegram_webhook".to_string(),
                format!(
                    "telegram_webhook_bind={}",
                    bind.as_ref().map_or("", ChannelBindAddress::as_str)
                ),
                format!(
                    "telegram_webhook_port={}",
                    port.map(|value| value.as_u16().to_string())
                        .unwrap_or_default()
                ),
                format!(
                    "telegram_webhook_path={}",
                    path.as_ref().map_or("", ChannelPath::as_str)
                ),
            ],
            Self::DiscordIngress { bind, path, url } => vec![
                "live_channel=discord_ingress".to_string(),
                format!(
                    "discord_ingress_bind={}",
                    bind.as_ref().map_or("", ChannelBindAddress::as_str)
                ),
                format!(
                    "discord_ingress_path={}",
                    path.as_ref().map_or("", ChannelPath::as_str)
                ),
                format!(
                    "discord_ingress_url={}",
                    url.as_ref().map_or("", ChannelLocalUrl::as_str)
                ),
            ],
        }
    }
}

/// Secret-safe live launch plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveLaunchPlan {
    /// Plan status.
    pub status: LiveLaunchPlanStatus,
    /// External agent gateway target.
    pub gateway_url: AgentGatewayUrl,
    /// Planned live channels.
    pub channels: Vec<LiveLaunchChannel>,
}

impl LiveLaunchPlan {
    /// Build a launch plan from runtime configuration.
    #[must_use]
    pub fn from_config(config: &RuntimeConfig) -> Self {
        let channels = planned_channels(config);
        let status = if channels.is_empty() {
            LiveLaunchPlanStatus::Empty
        } else {
            LiveLaunchPlanStatus::Runnable
        };

        Self {
            status,
            gateway_url: config.agent.gateway_url.clone(),
            channels,
        }
    }

    /// Return whether this plan has at least one live channel.
    #[must_use]
    pub const fn is_runnable(&self) -> bool {
        matches!(self.status, LiveLaunchPlanStatus::Runnable)
    }

    /// Render the plan as stable diagnostic lines.
    #[must_use]
    pub fn render_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("live_plan_status={}", self.status.as_str()),
            format!("live_gateway_url={}", self.gateway_url.as_str()),
            format!("live_channel_count={}", self.channels.len()),
        ];
        lines.extend(
            self.channels
                .iter()
                .flat_map(LiveLaunchChannel::render_lines),
        );
        lines
    }
}

fn planned_channels(config: &RuntimeConfig) -> Vec<LiveLaunchChannel> {
    let mut channels = Vec::new();
    if matches!(config.telegram.enabled, RuntimeFlag::Enabled) {
        channels.push(LiveLaunchChannel::TelegramWebhook {
            bind: config.telegram.inbound_bind.clone(),
            port: config.telegram.inbound_port,
            path: config.telegram.inbound_path.clone(),
        });
    }
    if matches!(config.discord.enabled, RuntimeFlag::Enabled) {
        channels.push(LiveLaunchChannel::DiscordIngress {
            bind: config.discord.inbound_bind.clone(),
            path: config.discord.inbound_path.clone(),
            url: config.discord.inbound_url.clone(),
        });
    }
    channels
}
