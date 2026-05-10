use super::lookup_fixture;
use lingchong_bot::runtime::{
    ChannelApiBaseUrl, ChannelBindAddress, ChannelLocalUrl, ChannelPath, ChannelPort,
    MentionPolicy, RuntimeConfig, RuntimeConfigError, RuntimeFlag, SecretFlag,
};
use pretty_assertions::assert_eq;

#[test]
fn runtime_config_loads_secret_safe_channel_state() {
    let config = RuntimeConfig::from_env_lookup(lookup_fixture).expect("config should load");

    assert_eq!(config.agent.gateway_url.as_str(), "http://127.0.0.1:18093/");
    assert_eq!(config.telegram.enabled, RuntimeFlag::Enabled);
    assert_eq!(config.telegram.secret_configured, SecretFlag::Configured);
    assert_eq!(
        config
            .telegram
            .inbound_bind
            .as_ref()
            .map(ChannelBindAddress::as_str),
        Some("127.0.0.1:28081")
    );
    assert_eq!(
        config.telegram.inbound_port.map(ChannelPort::as_u16),
        Some(28081)
    );
    assert_eq!(
        config
            .telegram
            .api_base_url
            .as_ref()
            .map(ChannelApiBaseUrl::as_str),
        Some("https://api.telegram.org")
    );
    assert_eq!(config.discord.enabled, RuntimeFlag::Enabled);
    assert_eq!(config.discord.secret_configured, SecretFlag::Configured);
    assert_eq!(
        config
            .discord
            .inbound_bind
            .as_ref()
            .map(ChannelBindAddress::as_str),
        Some("0.0.0.0:29082")
    );
    assert_eq!(
        config
            .discord
            .inbound_path
            .as_ref()
            .map(ChannelPath::as_str),
        Some("/discord/custom")
    );
    assert_eq!(
        config
            .discord
            .inbound_url
            .as_ref()
            .map(ChannelLocalUrl::as_str),
        Some("http://127.0.0.1:29082/discord/custom")
    );
    assert_eq!(config.discord.require_mention, MentionPolicy::NotRequired);
}

#[test]
fn runtime_config_requires_gateway_url() {
    let error = RuntimeConfig::from_env_lookup(|_| None).expect_err("gateway URL is required");

    assert_eq!(
        error,
        RuntimeConfigError::MissingRequiredEnv {
            name: "LINGCHONG_AGENT_GATEWAY_URL"
        }
    );
}

#[test]
fn runtime_config_rejects_invalid_boolean() {
    let error = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        "LINGCHONG_DISCORD_REQUIRE_MENTION" => Some("sometimes".to_string()),
        _ => None,
    })
    .expect_err("invalid boolean should fail");

    assert_eq!(
        error,
        RuntimeConfigError::InvalidBoolEnv {
            name: "LINGCHONG_DISCORD_REQUIRE_MENTION",
            value: "sometimes".to_string()
        }
    );
}

#[test]
fn runtime_config_telegram_webhook_port_env_overrides_bind_port() {
    let config = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        "LINGCHONG_TELEGRAM_WEBHOOK_BIND" => Some("127.0.0.1:28081".to_string()),
        "LINGCHONG_TELEGRAM_WEBHOOK_PORT" => Some("38081".to_string()),
        _ => None,
    })
    .expect("config should load");

    assert_eq!(
        config
            .telegram
            .inbound_bind
            .as_ref()
            .map(ChannelBindAddress::as_str),
        Some("127.0.0.1:28081")
    );
    assert_eq!(
        config.telegram.inbound_port.map(ChannelPort::as_u16),
        Some(38081)
    );
}

#[test]
fn runtime_config_telegram_webhook_port_defaults_when_unset() {
    let config = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        _ => None,
    })
    .expect("config should load");

    assert_eq!(
        config.telegram.inbound_port.map(ChannelPort::as_u16),
        Some(18081)
    );
}

#[test]
fn runtime_config_rejects_invalid_telegram_webhook_port() {
    let error = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        "LINGCHONG_TELEGRAM_WEBHOOK_PORT" => Some("99999".to_string()),
        _ => None,
    })
    .expect_err("invalid port should fail");

    assert_eq!(
        error,
        RuntimeConfigError::InvalidPortEnv {
            name: "LINGCHONG_TELEGRAM_WEBHOOK_PORT",
            value: "99999".to_string()
        }
    );
}

#[test]
fn runtime_config_discord_ingress_url_env_overrides_bind_and_path() {
    let config = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        "LINGCHONG_DISCORD_INGRESS_BIND" => Some("0.0.0.0:19082".to_string()),
        "LINGCHONG_DISCORD_INGRESS_PATH" => Some("ingress/discord".to_string()),
        "LINGCHONG_DISCORD_INGRESS_URL" => Some("https://example.test/discord".to_string()),
        _ => None,
    })
    .expect("config should load");

    assert_eq!(
        config
            .discord
            .inbound_bind
            .as_ref()
            .map(ChannelBindAddress::as_str),
        Some("0.0.0.0:19082")
    );
    assert_eq!(
        config
            .discord
            .inbound_path
            .as_ref()
            .map(ChannelPath::as_str),
        Some("/ingress/discord")
    );
    assert_eq!(
        config
            .discord
            .inbound_url
            .as_ref()
            .map(ChannelLocalUrl::as_str),
        Some("https://example.test/discord")
    );
}

#[test]
fn runtime_config_discord_ingress_defaults_to_local_url() {
    let config = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        _ => None,
    })
    .expect("config should load");

    assert_eq!(
        config
            .discord
            .inbound_url
            .as_ref()
            .map(ChannelLocalUrl::as_str),
        Some("http://127.0.0.1:18082/discord/ingress")
    );
}

#[test]
fn runtime_summary_does_not_render_secret_values() {
    let config = RuntimeConfig::from_env_lookup(lookup_fixture).expect("config should load");
    let rendered = config.summary().render_lines().join("\n");

    assert!(rendered.contains("telegram_enabled=true"));
    assert!(rendered.contains("telegram_webhook_bind=127.0.0.1:28081"));
    assert!(rendered.contains("telegram_webhook_port=28081"));
    assert!(rendered.contains("discord_enabled=true"));
    assert!(rendered.contains("discord_ingress_bind=0.0.0.0:29082"));
    assert!(rendered.contains("discord_ingress_path=/discord/custom"));
    assert!(rendered.contains("discord_ingress_url=http://127.0.0.1:29082/discord/custom"));
    assert!(!rendered.contains("telegram-token"));
    assert!(!rendered.contains("discord-token"));
    assert!(!rendered.contains("telegram-secret"));
    assert!(!rendered.contains("discord-secret"));
}
