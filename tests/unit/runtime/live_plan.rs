use super::lookup_fixture;
use lingchong_bot::runtime::{
    LiveLaunchChannel, LiveLaunchPlan, LiveLaunchPlanStatus, RuntimeConfig,
};
use pretty_assertions::assert_eq;

#[test]
fn live_launch_plan_is_empty_without_enabled_channels() {
    let config = RuntimeConfig::from_env_lookup(|key| match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some("http://127.0.0.1:18093".to_string()),
        _ => None,
    })
    .expect("config should load");

    let plan = LiveLaunchPlan::from_config(&config);

    assert_eq!(plan.status, LiveLaunchPlanStatus::Empty);
    assert!(plan.channels.is_empty());
    assert!(!plan.is_runnable());
}

#[test]
fn live_launch_plan_includes_enabled_channels() {
    let config = RuntimeConfig::from_env_lookup(lookup_fixture).expect("config should load");

    let plan = LiveLaunchPlan::from_config(&config);

    assert_eq!(plan.status, LiveLaunchPlanStatus::Runnable);
    assert_eq!(plan.channels.len(), 2);
    assert!(matches!(
        plan.channels[0],
        LiveLaunchChannel::TelegramWebhook { .. }
    ));
    assert!(matches!(
        plan.channels[1],
        LiveLaunchChannel::DiscordIngress { .. }
    ));
    assert_eq!(plan.channels[0].id(), "telegram_webhook");
    assert_eq!(plan.channels[1].id(), "discord_ingress");
}

#[test]
fn live_launch_plan_render_is_secret_safe() {
    let config = RuntimeConfig::from_env_lookup(lookup_fixture).expect("config should load");
    let rendered = LiveLaunchPlan::from_config(&config)
        .render_lines()
        .join("\n");

    assert!(rendered.contains("live_plan_status=runnable"));
    assert!(rendered.contains("live_channel_count=2"));
    assert!(rendered.contains("live_channel=telegram_webhook"));
    assert!(rendered.contains("live_channel=discord_ingress"));
    assert!(rendered.contains("live_gateway_url=http://127.0.0.1:18093"));
    assert!(!rendered.contains("telegram-token"));
    assert!(!rendered.contains("discord-token"));
    assert!(!rendered.contains("telegram-secret"));
    assert!(!rendered.contains("discord-secret"));
}
