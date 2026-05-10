use lingchong_bot::gateway::{AgentGatewayClient, AgentGatewayError, AgentGatewayMessageRequest};

#[test]
fn gateway_client_rejects_empty_base_url() {
    let error = AgentGatewayClient::new("   ").expect_err("empty base URL should fail");

    assert_eq!(error, AgentGatewayError::InvalidBaseUrl);
}

#[test]
fn gateway_client_builds_message_endpoint() {
    let client =
        AgentGatewayClient::new("http://127.0.0.1:18093/").expect("base URL should be accepted");

    assert_eq!(client.message_endpoint(), "http://127.0.0.1:18093/message");
}

#[test]
fn gateway_message_request_preserves_session_and_message() {
    let request = AgentGatewayMessageRequest::new("telegram:chat:user", "hello");

    assert_eq!(request.session_id, "telegram:chat:user");
    assert_eq!(request.message, "hello");
}
