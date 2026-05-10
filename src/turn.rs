//! Channel turn orchestration.
//!
//! This module maps channel messages into external gateway requests. It is the
//! bot-side replacement for direct native-tool execution.

use serde::{Deserialize, Serialize};

use crate::gateway::{
    AgentGatewayClient, AgentGatewayError, AgentGatewayMessageRequest, AgentGatewayMessageResponse,
};

/// Structured attachment attached to a channel message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelAttachment {
    /// Attachment represented as a remotely fetchable image URL.
    ImageUrl {
        /// Remote image location.
        url: String,
    },
}

/// A message received from Telegram, Discord, or another chat adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelMessage {
    /// Unique message identifier from the source channel.
    pub id: String,
    /// Sender identity for diagnostics and policy.
    pub sender: String,
    /// Reply target for outbound channel send operations.
    pub recipient: String,
    /// Session partition key selected by the channel adapter.
    pub session_key: String,
    /// Message text content.
    pub content: String,
    /// Structured inbound attachments.
    pub attachments: Vec<ChannelAttachment>,
    /// Channel name such as `telegram` or `discord`.
    pub channel: String,
    /// Unix timestamp.
    pub timestamp: u64,
}

impl ChannelMessage {
    /// Build the external agent session identifier for this message.
    #[must_use]
    pub fn gateway_session_id(&self) -> String {
        format!("{}:{}", self.channel, self.session_key)
    }
}

/// Successful bot turn response ready for channel delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotTurnReply {
    /// Reply target copied from the inbound channel message.
    pub recipient: String,
    /// Text to send to the channel.
    pub content: String,
}

/// Gateway-backed bot turn service.
#[derive(Debug, Clone)]
pub struct BotTurnService {
    gateway: AgentGatewayClient,
}

impl BotTurnService {
    /// Build a turn service from an external agent gateway client.
    #[must_use]
    pub const fn new(gateway: AgentGatewayClient) -> Self {
        Self { gateway }
    }

    /// Build the gateway request for an inbound channel message.
    #[must_use]
    pub fn build_gateway_request(message: &ChannelMessage) -> AgentGatewayMessageRequest {
        AgentGatewayMessageRequest::new(message.gateway_session_id(), message.content.clone())
    }

    /// Run one channel turn through the external gateway.
    ///
    /// # Errors
    ///
    /// Returns [`AgentGatewayError`] when the external gateway rejects or fails
    /// the request.
    pub async fn run_turn(
        &self,
        message: &ChannelMessage,
    ) -> Result<BotTurnReply, AgentGatewayError> {
        let request = Self::build_gateway_request(message);
        let AgentGatewayMessageResponse { response } = self.gateway.send_message(&request).await?;
        Ok(BotTurnReply {
            recipient: message.recipient.clone(),
            content: response,
        })
    }
}
