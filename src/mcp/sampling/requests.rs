//! Defines the request and result structures for the `sampling/createMessage` MCP method.

use super::types::{ModelPreferences, SamplingContent, SamplingMessage}; // Use types from the same module tree
use crate::mcp::{GenericMeta, IncludeContext, IntoMcpRequest, RequestMeta};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

// region:    --- CreateMessageRequest

/// A request from the server to sample an LLM via the client.
///
/// TS Ref: `CreateMessageRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// Messages to provide context for the LLM.
	pub messages: Vec<SamplingMessage>,

	/// The server's preferences for which model to select. The client MAY ignore these preferences.
	pub model_preferences: Option<ModelPreferences>,

	/// An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.
	pub system_prompt: Option<String>,

	/// A request to include context from one or more MCP servers (including the caller), to be attached to the prompt.
	pub include_context: Option<IncludeContext>,

	/// Sampling temperature.
	pub temperature: Option<f64>,

	/// The maximum number of tokens to sample, as requested by the server.
	pub max_tokens: i64, // Assuming integer tokens

	/// Sequences where the model should stop generating tokens.
	pub stop_sequences: Option<Vec<String>>,

	/// Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.
	pub metadata: Option<Value>,
}

impl IntoMcpRequest for CreateMessageParams {
	const METHOD: &'static str = "sampling/createMessage";
}

/// The client's response to a sampling/createMessage request from the server.
///
/// TS Ref: `CreateMessageResult`
/// Note: The TS definition extends `SamplingMessage`, so we include `role` and `content` directly.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	// Fields from SamplingMessage
	pub role: crate::mcp::Role, // Qualify Role to avoid ambiguity if types module re-exports Role
	pub content: SamplingContent,

	/// The name of the model that generated the message.
	pub model: String,

	/// The reason why sampling stopped, if known.
	pub stop_reason: Option<String>,
}

// endregion: --- CreateMessageRequest
