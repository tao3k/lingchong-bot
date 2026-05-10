use lingchong_bot::harness::{
    DiscordAclEventName, DiscordAclJsonSummaryObservation, DiscordAclJsonSummaryValidationRequest,
    DiscordAclProbeCaseFilterRequest, DiscordAclProbeCaseId, DiscordAclProbeSuite,
    DiscordAclReplyObservation, DiscordAclReplyValidationRequest, DiscordAclValidationError,
    DiscordUserId, build_discord_acl_probe_cases, filter_discord_acl_probe_cases,
    parse_discord_acl_probe_case_ids, parse_discord_acl_probe_suites,
    validate_discord_acl_json_summary, validate_discord_acl_reply,
};
use pretty_assertions::assert_eq;

#[test]
fn discord_acl_reply_validation_accepts_target_session() {
    let observations = vec![DiscordAclReplyObservation {
        event: "discord.command.control_admin_required.replied".to_string(),
        recipient: "1001".to_string(),
        session_key: "1001:2002".to_string(),
    }];

    let target = validate_discord_acl_reply(
        DiscordAclReplyValidationRequest {
            event_name: "discord.command.control_admin_required.replied",
            expected_recipient: "1001",
            expected_sessions: &["1001:2002"],
        },
        &observations,
    )
    .expect("target reply should validate");

    assert_eq!(target.session_key, "1001:2002");
}

#[test]
fn discord_acl_reply_validation_rejects_scope_mismatch() {
    let observations = vec![DiscordAclReplyObservation {
        event: "discord.command.control_admin_required.replied".to_string(),
        recipient: "1001".to_string(),
        session_key: "1001:7777".to_string(),
    }];

    let err = validate_discord_acl_reply(
        DiscordAclReplyValidationRequest {
            event_name: "discord.command.control_admin_required.replied",
            expected_recipient: "1001",
            expected_sessions: &["1001:2002"],
        },
        &observations,
    )
    .expect_err("mismatched session should fail");

    assert_eq!(
        err,
        DiscordAclValidationError::ReplySessionMismatch {
            observed_session: "1001:7777".to_string()
        }
    );
}

#[test]
fn discord_acl_json_summary_validation_rejects_scope_prefix_mismatch() {
    let summaries = vec![DiscordAclJsonSummaryObservation {
        event: "discord.command.session_memory_json.replied".to_string(),
        recipient: "1001".to_string(),
        session_key: "1001:2002".to_string(),
        json_session_scope: "telegram:1001:2002".to_string(),
    }];

    let err = validate_discord_acl_json_summary(
        DiscordAclJsonSummaryValidationRequest {
            event_name: "discord.command.session_memory_json.replied",
            expected_recipient: "1001",
            expected_sessions: &["1001:2002"],
            expected_session_scopes: &["discord:1001:2002"],
        },
        &summaries,
    )
    .expect_err("mismatched scope should fail");

    assert_eq!(
        err,
        DiscordAclValidationError::JsonSummaryScopeMismatch {
            observed_scope: "telegram:1001:2002".to_string()
        }
    );
}

#[test]
fn discord_acl_json_summary_validation_allows_missing_optional_summary() {
    let result = validate_discord_acl_json_summary(
        DiscordAclJsonSummaryValidationRequest {
            event_name: "discord.command.session_memory_json.replied",
            expected_recipient: "1001",
            expected_sessions: &["1001:2002"],
            expected_session_scopes: &["discord:1001:2002"],
        },
        &[],
    )
    .expect("missing optional summary should pass");

    assert!(result.is_none());
}

#[test]
fn discord_acl_probe_cases_build_default_catalog() {
    let cases =
        build_discord_acl_probe_cases(&DiscordUserId::new("1001").expect("user id should parse"));

    assert_eq!(cases.len(), 2);
    assert_eq!(
        cases[0].case_id(),
        DiscordAclProbeCaseId::ControlAdminDenied
    );
    assert_eq!(cases[0].prompt(), "/session admin add 1001");
    assert_eq!(
        cases[0].event_name(),
        DiscordAclEventName::ControlAdminRequiredReply
    );
    assert_eq!(cases[0].suites(), &[DiscordAclProbeSuite::Core]);
    assert_eq!(
        cases[1].case_id(),
        DiscordAclProbeCaseId::SlashPermissionDenied
    );
}

#[test]
fn discord_acl_probe_cases_filter_by_suite_and_case_id() {
    let cases =
        build_discord_acl_probe_cases(&DiscordUserId::new("1001").expect("user id should parse"));

    let filtered = filter_discord_acl_probe_cases(
        DiscordAclProbeCaseFilterRequest {
            suites: &[DiscordAclProbeSuite::Core],
            requested_case_ids: &[DiscordAclProbeCaseId::SlashPermissionDenied],
        },
        &cases,
    );

    assert_eq!(filtered.len(), 1);
    assert_eq!(
        filtered[0].case_id(),
        DiscordAclProbeCaseId::SlashPermissionDenied
    );

    let all = filter_discord_acl_probe_cases(
        DiscordAclProbeCaseFilterRequest {
            suites: &[DiscordAclProbeSuite::All],
            requested_case_ids: &[],
        },
        &cases,
    );

    assert_eq!(all.len(), 2);
}

#[test]
fn discord_acl_probe_selectors_parse_aliases_and_dedup() {
    let suites = parse_discord_acl_probe_suites(&["core", "core"]).expect("suites should parse");
    assert_eq!(suites, vec![DiscordAclProbeSuite::Core]);

    let all_suites = parse_discord_acl_probe_suites(&["core", "all"]).expect("all should parse");
    assert_eq!(all_suites, vec![DiscordAclProbeSuite::All]);

    let case_ids = parse_discord_acl_probe_case_ids(&[
        "control-admin-denied",
        "discord_control_admin_denied",
        "slash_permission_denied",
    ])
    .expect("case ids should parse");
    assert_eq!(
        case_ids,
        vec![
            DiscordAclProbeCaseId::ControlAdminDenied,
            DiscordAclProbeCaseId::SlashPermissionDenied,
        ]
    );
}

#[test]
fn discord_acl_probe_selectors_reject_unknown_values() {
    assert!(parse_discord_acl_probe_suites(&["admin"]).is_err());
    assert!(parse_discord_acl_probe_case_ids(&["unknown_case"]).is_err());
}
