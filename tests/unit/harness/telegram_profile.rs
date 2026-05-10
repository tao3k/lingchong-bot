use lingchong_bot::harness::{
    TelegramChatType, TelegramGroupProfile, TelegramGroupProfileError, TelegramProfileThreadId,
};
use pretty_assertions::assert_eq;

#[test]
fn telegram_group_profile_extracts_required_groups() {
    let log = r#"
2026-02-20T00:00:01Z INFO lingchong_bot::channels::telegram::runtime::webhook: Parsed message, forwarding to agent session_key=-5101776367:1304799691 chat_id=Some(-5101776367) chat_title=Some("Test1") chat_type=Some("group") message_thread_id=None content_preview=/help
2026-02-20T00:00:02Z INFO lingchong_bot::channels::telegram::runtime::webhook: Parsed message, forwarding to agent session_key=-5020317863:1304799691 chat_id=Some(-5020317863) chat_title=Some("Test2") chat_type=Some("group") message_thread_id=Some(44) content_preview=/help
"#;

    let profile = TelegramGroupProfile::parse_required(log, &["Test1", "Test2"])
        .expect("required groups should parse");

    assert_eq!(profile.present_titles(), vec!["Test1", "Test2"]);
    assert_eq!(
        profile
            .group("Test1")
            .expect("group should exist")
            .chat_id
            .into_raw(),
        -5_101_776_367
    );
    assert_eq!(
        profile
            .group("Test1")
            .expect("group should exist")
            .chat_type,
        TelegramChatType::Group
    );
    assert_eq!(
        profile
            .group("Test2")
            .expect("group should exist")
            .thread_id
            .map(TelegramProfileThreadId::into_raw),
        Some(44)
    );
}

#[test]
fn telegram_group_profile_reports_missing_titles() {
    let err = TelegramGroupProfile::parse_required("", &["Missing"])
        .expect_err("missing title should fail");

    assert_eq!(
        err,
        TelegramGroupProfileError::MissingTitles(vec!["Missing".to_string()])
    );
    assert_eq!(err.to_string(), "missing group titles in log: Missing");
}
