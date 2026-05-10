use lingchong_bot::turn::{BotTurnService, ChannelAttachment, ChannelMessage};
use pretty_assertions::assert_eq;

fn sample_channel_message() -> ChannelMessage {
    ChannelMessage {
        id: "message-1".to_string(),
        sender: "alice".to_string(),
        recipient: "chat-1".to_string(),
        session_key: "chat-1:alice".to_string(),
        content: "hello".to_string(),
        attachments: vec![ChannelAttachment::ImageUrl {
            url: "https://example.test/image.png".to_string(),
        }],
        channel: "telegram".to_string(),
        timestamp: 1_774_203_200,
    }
}

#[test]
fn channel_message_builds_gateway_session_id() {
    let message = sample_channel_message();

    assert_eq!(message.gateway_session_id(), "telegram:chat-1:alice");
}

#[test]
fn turn_service_builds_gateway_request_without_domain_tools() {
    let message = sample_channel_message();
    let request = BotTurnService::build_gateway_request(&message);

    assert_eq!(request.session_id, "telegram:chat-1:alice");
    assert_eq!(request.message, "hello");
}

#[test]
fn channel_message_wire_shape_is_stable() {
    let message = sample_channel_message();
    let value = serde_json::to_value(&message).expect("message should serialize");

    assert_eq!(value["channel"], "telegram");
    assert_eq!(value["session_key"], "chat-1:alice");
    assert_eq!(
        value["attachments"][0]["ImageUrl"]["url"],
        "https://example.test/image.png"
    );
}
