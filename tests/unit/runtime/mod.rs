mod config;
mod live_plan;
mod preflight;

pub fn lookup_fixture(key: &str) -> Option<String> {
    match key {
        "LINGCHONG_AGENT_GATEWAY_URL" => Some(" http://127.0.0.1:18093/ ".to_string()),
        "LINGCHONG_TELEGRAM_BOT_TOKEN" => Some("telegram-token".to_string()),
        "LINGCHONG_TELEGRAM_WEBHOOK_SECRET" => Some("telegram-secret".to_string()),
        "LINGCHONG_TELEGRAM_WEBHOOK_BIND" => Some("127.0.0.1:28081".to_string()),
        "LINGCHONG_DISCORD_BOT_TOKEN" => Some("discord-token".to_string()),
        "LINGCHONG_DISCORD_INGRESS_SECRET" => Some("discord-secret".to_string()),
        "LINGCHONG_DISCORD_INGRESS_BIND" => Some("0.0.0.0:29082".to_string()),
        "LINGCHONG_DISCORD_INGRESS_PATH" => Some("discord/custom".to_string()),
        "LINGCHONG_DISCORD_REQUIRE_MENTION" => Some("false".to_string()),
        _ => None,
    }
}
