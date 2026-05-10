//! Runtime configuration for Lingchong bot processes.

/// Environment-backed configuration loading.
pub mod config;
/// Live-gated bot launch planning.
pub mod live_plan;
/// Live-gated bot runtime preflight checks.
pub mod preflight;

pub use config::{
    AgentGatewayUrl, AgentRuntimeConfig, ChannelApiBaseUrl, ChannelBindAddress, ChannelLocalUrl,
    ChannelPath, ChannelPort, ChannelRuntimeConfig, MentionPolicy, RuntimeConfig,
    RuntimeConfigError, RuntimeConfigSummary, RuntimeFlag, SecretFlag,
};
pub use live_plan::{LiveLaunchChannel, LiveLaunchPlan, LiveLaunchPlanStatus};
pub use preflight::{LivePreflightIssue, LivePreflightReport, LivePreflightStatus};
