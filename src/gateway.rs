//! External agent gateway client boundary.
//!
//! This module is the integration point for bot-to-agent calls. It intentionally
//! uses the gateway `/message` contract instead of importing Daochang native
//! tools.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Request body for a gateway message turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentGatewayMessageRequest {
    /// Conversation or channel session identifier.
    pub session_id: String,
    /// User message or channel message content.
    pub message: String,
}

impl AgentGatewayMessageRequest {
    /// Build a gateway request from session and message text.
    #[must_use]
    pub fn new(session_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            message: message.into(),
        }
    }
}

/// Response body returned by the external gateway.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AgentGatewayMessageResponse {
    /// Assistant output returned by the gateway.
    pub response: String,
}

/// HTTP client for the external agent gateway.
#[derive(Debug, Clone)]
pub struct AgentGatewayClient {
    base_url: String,
    http: reqwest::Client,
}

impl AgentGatewayClient {
    /// Build a gateway client from the service base URL.
    ///
    /// # Errors
    ///
    /// Returns [`AgentGatewayError::InvalidBaseUrl`] when the provided URL is
    /// empty.
    pub fn new(base_url: impl Into<String>) -> Result<Self, AgentGatewayError> {
        let base_url = base_url.into();
        if base_url.trim().is_empty() {
            return Err(AgentGatewayError::InvalidBaseUrl);
        }

        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: reqwest::Client::new(),
        })
    }

    /// Return the concrete `/message` endpoint used by this client.
    #[must_use]
    pub fn message_endpoint(&self) -> String {
        format!("{}/message", self.base_url)
    }

    /// Send one user message to the external agent gateway.
    ///
    /// # Errors
    ///
    /// Returns [`AgentGatewayError::Request`] when the HTTP request fails or
    /// the gateway returns a non-success status.
    pub async fn send_message(
        &self,
        request: &AgentGatewayMessageRequest,
    ) -> Result<AgentGatewayMessageResponse, AgentGatewayError> {
        let response = self
            .http
            .post(self.message_endpoint())
            .json(request)
            .send()
            .await
            .map_err(|source| AgentGatewayError::Request {
                message: source.to_string(),
            })?;

        let response =
            response
                .error_for_status()
                .map_err(|source| AgentGatewayError::Request {
                    message: source.to_string(),
                })?;

        response
            .json::<AgentGatewayMessageResponse>()
            .await
            .map_err(|source| AgentGatewayError::Request {
                message: source.to_string(),
            })
    }
}

/// Error type for gateway client construction and calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentGatewayError {
    /// The configured base URL is empty.
    InvalidBaseUrl,
    /// The gateway request failed.
    Request {
        /// Human-readable request failure.
        message: String,
    },
}

impl Display for AgentGatewayError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBaseUrl => formatter.write_str("agent gateway base URL is empty"),
            Self::Request { message } => {
                write!(formatter, "agent gateway request failed: {message}")
            }
        }
    }
}

impl std::error::Error for AgentGatewayError {}
