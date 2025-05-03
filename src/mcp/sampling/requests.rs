//! Defines the request and result structures for the `sampling/createMessage` MCP method.

use super::types::{ModelPreferences, SamplingContent, SamplingMessage}; // Use types from the same module tree
use crate::mcp::{GenericMeta, IncludeContext, IntoMcpRequest, RequestMeta, Role};
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

	/// The maximum number of tokens to sample, as requested by the server.
	pub max_tokens: i64, // Assuming integer tokens

	/// The server's preferences for which model to select. The client MAY ignore these preferences.
	pub model_preferences: Option<ModelPreferences>,

	/// An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.
	pub system_prompt: Option<String>,

	/// A request to include context from one or more MCP servers (including the caller), to be attached to the prompt.
	pub include_context: Option<IncludeContext>,

	/// Sampling temperature.
	pub temperature: Option<f64>,

	/// Sequences where the model should stop generating tokens.
	pub stop_sequences: Option<Vec<String>>,

	/// Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.
	pub metadata: Option<Value>,
}

/// Builders
impl CreateMessageParams {
	pub fn new(messages: Vec<SamplingMessage>, max_tokens: i64) -> Self {
		Self {
			meta: None,
			messages,
			max_tokens,
			model_preferences: None,
			system_prompt: None,
			include_context: None,
			temperature: None,
			stop_sequences: None,
			metadata: None,
		}
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}

	pub fn with_model_preferences(mut self, preferences: ModelPreferences) -> Self {
		self.model_preferences = Some(preferences);
		self
	}

	pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
		self.system_prompt = Some(prompt.into());
		self
	}

	pub fn with_include_context(mut self, context: IncludeContext) -> Self {
		self.include_context = Some(context);
		self
	}

	pub fn with_temperature(mut self, temp: f64) -> Self {
		self.temperature = Some(temp);
		self
	}

	pub fn with_stop_sequences(mut self, sequences: Vec<String>) -> Self {
		self.stop_sequences = Some(sequences);
		self
	}

	pub fn append_stop_sequence(mut self, sequence: impl Into<String>) -> Self {
		self.stop_sequences.get_or_insert_with(Vec::new).push(sequence.into());
		self
	}

	pub fn with_metadata(mut self, metadata: Value) -> Self {
		self.metadata = Some(metadata);
		self
	}
}

impl IntoMcpRequest<CreateMessageParams> for CreateMessageParams {
	const METHOD: &'static str = "sampling/createMessage";
	type McpResult = CreateMessageResult;
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
	pub role: Role,
	pub content: SamplingContent,

	/// The name of the model that generated the message.
	pub model: String,

	/// The reason why sampling stopped, if known.
	pub stop_reason: Option<String>,
}

/// Builders
impl CreateMessageResult {
	pub fn new(role: Role, content: SamplingContent, model: impl Into<String>) -> Self {
		Self {
			meta: None,
			role,
			content,
			model: model.into(),
			stop_reason: None,
		}
	}

	pub fn with_meta(mut self, meta: GenericMeta) -> Self {
		self.meta = Some(meta);
		self
	}

	pub fn with_stop_reason(mut self, reason: impl Into<String>) -> Self {
		self.stop_reason = Some(reason.into());
		self
	}
}

// endregion: --- CreateMessageRequest
