use super::lookup_fixture;
use lingchong_bot::runtime::{
    LivePreflightIssue, LivePreflightReport, LivePreflightStatus, RuntimeConfig,
};
use pretty_assertions::assert_eq;

#[test]
fn live_preflight_blocks_without_enabled_channels() {
    let config = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        _ => None,
    })
    .expect("config should load");

    let report = LivePreflightReport::from_config(&config);

    assert_eq!(report.status, LivePreflightStatus::Blocked);
    assert_eq!(
        report.issues,
        vec![LivePreflightIssue::MissingEnabledChannel]
    );
    assert!(!report.is_ready());
}

#[test]
fn live_preflight_allows_any_enabled_channel() {
    let config = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        "LINGCHONG_DISCORD_BOT_TOKEN" => Some("discord-token".to_string()),
        _ => None,
    })
    .expect("config should load");

    let report = LivePreflightReport::from_config(&config);

    assert_eq!(report.status, LivePreflightStatus::Ready);
    assert!(report.issues.is_empty());
    assert!(report.is_ready());
}

#[test]
fn live_preflight_report_is_secret_safe() {
    let config = RuntimeConfig::from_env_lookup(lookup_fixture).expect("config should load");
    let rendered = LivePreflightReport::from_config(&config)
        .render_lines()
        .join("\n");

    assert!(rendered.contains("live_preflight_status=ready"));
    assert!(!rendered.contains("telegram-token"));
    assert!(!rendered.contains("discord-token"));
    assert!(!rendered.contains("telegram-secret"));
    assert!(!rendered.contains("discord-secret"));
}
