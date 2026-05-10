use lingchong_bot::channels::TelegramSessionPartition;
use lingchong_bot::harness::{
    TelegramAdminMatrixChatSelectionRequest, TelegramMatrixChatId, TelegramMatrixSessionKeyRequest,
    TelegramMatrixThreadId, TelegramMatrixUserId, build_telegram_matrix_session_key,
    build_telegram_session_memory_result_fields, select_telegram_admin_matrix_chat_ids,
};
use pretty_assertions::assert_eq;

#[test]
fn telegram_session_matrix_builds_expected_session_keys() {
    assert_eq!(
        build_telegram_matrix_session_key(TelegramMatrixSessionKeyRequest {
            partition: TelegramSessionPartition::ChatUser,
            chat_id: TelegramMatrixChatId::new(100),
            user_id: TelegramMatrixUserId::new(7),
            thread_id: None,
        }),
        "100:7"
    );
    assert_eq!(
        build_telegram_matrix_session_key(TelegramMatrixSessionKeyRequest {
            partition: TelegramSessionPartition::ChatThreadUser,
            chat_id: TelegramMatrixChatId::new(-100_228),
            user_id: TelegramMatrixUserId::new(7),
            thread_id: Some(TelegramMatrixThreadId::new(42)),
        }),
        "-100228:42:7"
    );
}

#[test]
fn telegram_session_matrix_memory_fields_include_session_scope() {
    let fields = build_telegram_session_memory_result_fields(TelegramMatrixSessionKeyRequest {
        partition: TelegramSessionPartition::ChatUser,
        chat_id: TelegramMatrixChatId::new(100),
        user_id: TelegramMatrixUserId::new(7),
        thread_id: None,
    });

    assert_eq!(
        fields,
        vec![
            "json_kind=session_memory".to_string(),
            "json_session_scope=telegram:100:7".to_string(),
        ]
    );
}

#[test]
fn telegram_admin_matrix_chat_selection_merges_dedups_and_keeps_groups() {
    let selected = select_telegram_admin_matrix_chat_ids(TelegramAdminMatrixChatSelectionRequest {
        explicit_matrix_chat_ids: vec![
            TelegramMatrixChatId::new(-5_101_776_367),
            TelegramMatrixChatId::new(-6_000_000_001),
        ],
        group_chat_id: Some(TelegramMatrixChatId::new(-5_020_317_863)),
        allow_chat_ids: vec![
            TelegramMatrixChatId::new(-5_292_802_281),
            TelegramMatrixChatId::new(1_304_799_691),
            TelegramMatrixChatId::new(-5_101_776_367),
        ],
    });

    assert_eq!(
        selected
            .into_iter()
            .map(TelegramMatrixChatId::into_raw)
            .collect::<Vec<_>>(),
        vec![
            -5_101_776_367,
            -6_000_000_001,
            -5_020_317_863,
            -5_292_802_281,
        ]
    );
}
