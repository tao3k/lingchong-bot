use lingchong_bot::harness::{
    DiscordChannelId, DiscordGuildId, DiscordIngressStressConfig, DiscordIngressStressConfigError,
    DiscordIngressStressConfigRequest, DiscordRoleId, DiscordUserId,
};
use pretty_assertions::assert_eq;

#[test]
fn discord_ingress_stress_config_normalizes_inputs() {
    let cfg = DiscordIngressStressConfig::build(DiscordIngressStressConfigRequest {
        rounds: 2,
        warmup_rounds: 1,
        parallel: 2,
        requests_per_worker: 3,
        channel_id: DiscordChannelId::new(" 2001 ").expect("channel id should parse"),
        user_id: DiscordUserId::new(" 1001 ").expect("user id should parse"),
        guild_id: DiscordGuildId::new(" 3001 "),
        role_ids: ["role-a", "role-a", "", "role-b"]
            .into_iter()
            .filter_map(DiscordRoleId::new)
            .collect(),
        log_file: "runtime.log".into(),
        output_json: "report.json".into(),
        output_markdown: "report.md".into(),
        quality_max_p95_ms: 0.0,
        quality_min_rps: 5.0,
    })
    .expect("config should build");

    assert_eq!(cfg.rounds, 2);
    assert_eq!(cfg.warmup_rounds, 1);
    assert_eq!(cfg.parallel, 2);
    assert_eq!(cfg.requests_per_worker, 3);
    assert_eq!(cfg.channel_id.as_str(), "2001");
    assert_eq!(cfg.user_id.as_str(), "1001");
    assert_eq!(
        cfg.guild_id
            .as_ref()
            .expect("guild id should be present")
            .as_str(),
        "3001"
    );
    assert_eq!(
        cfg.role_ids
            .iter()
            .map(DiscordRoleId::as_str)
            .collect::<Vec<_>>(),
        vec!["role-a", "role-b"]
    );
    assert!(cfg.log_file.is_absolute());
    assert!(cfg.output_json.is_absolute());
    assert!(cfg.output_markdown.is_absolute());
    assert_eq!(cfg.quality_max_p95_ms, None);
    assert_eq!(cfg.quality_min_rps, Some(5.0));
}

#[test]
fn discord_ingress_stress_config_rejects_missing_channel_id() {
    let err = DiscordChannelId::new(" ").expect_err("missing channel id should fail");

    assert_eq!(
        err,
        DiscordIngressStressConfigError::MissingRequired("channel_id")
    );
}
