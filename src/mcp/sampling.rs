//! Types related to LLM sampling requests and responses.

use super::{AudioContent, ImageContent, Request, RequestParams, ResultData, Role, TextContent}; // Use re-exports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// region:    --- Create Message Request

/// Specific parameters for the CreateMessageRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageParams {
	pub messages: Vec<SamplingMessage>,

	/// Server's preferences for model selection.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model_preferences: Option<ModelPreferences>,

	/// Optional system prompt.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_prompt: Option<String>,

	/// Request to include context from MCP servers.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_context: Option<IncludeContext>,

	/// Sampling temperature.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f64>,

	/// Maximum number of tokens to sample.
	pub max_tokens: u32, // Using u32 for token count

	/// Optional stop sequences.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop_sequences: Option<Vec<String>>,

	/// Optional provider-specific metadata.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Value>,
}

/// Type alias for the full CreateMessage Request structure using the wrapper.
pub type CreateMessageRequest = RequestParams<CreateMessageParams>;

/// Associates the params/result structures with the Request trait.
impl Request for CreateMessageParams {
	const METHOD: &'static str = "sampling/createMessage";
	type Params = Self;
	type Result = CreateMessageResultData;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IncludeContext {
	None,

	ThisServer,

	AllServers,
}

// endregion: --- Create Message Request

// region:    --- Create Message Result

/// Specific result data for the CreateMessage response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageResultData {
	#[serde(flatten)]
	pub message: SamplingMessage, // Embeds the sampled message fields

	/// The name of the model that generated the message.
	pub model: String,

	/// The reason why sampling stopped, if known. Could be an enum if reasons are standardized.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop_reason: Option<String>,
}

/// Type alias for the full CreateMessage Result structure using the wrapper.
pub type CreateMessageResult = ResultData<CreateMessageResultData>;

// endregion: --- Create Message Result

// region:    --- Sampling Message Definitions

/// Describes a message issued to or received from an LLM API during sampling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingMessage {
	pub role: Role,

	pub content: SamplingMessageContent,
}

/// Content types allowed within a SamplingMessage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SamplingMessageContent {
	Text(TextContent),

	Image(ImageContent),

	Audio(AudioContent),
	// Note: EmbeddedResource is NOT part of SamplingMessageContent, only PromptMessageContent
}

// Implement Default for SamplingMessageContent (optional, defaults to Text)
impl Default for SamplingMessageContent {
	fn default() -> Self {
		SamplingMessageContent::Text(Default::default())
	}
}

// endregion: --- Sampling Message Definitions

// region:    --- Model Preferences

/// Server's preferences for model selection (advisory).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPreferences {
	/// Optional hints for model selection (evaluated in order).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hints: Option<Vec<ModelHint>>,

	/// Priority for cost (0=low, 1=high).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cost_priority: Option<f64>,

	/// Priority for speed (0=low, 1=high).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed_priority: Option<f64>,

	/// Priority for intelligence (0=low, 1=high).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub intelligence_priority: Option<f64>,
}

/// Hints for model selection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelHint {
	/// Hint for a model name (substring match recommended).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,

	// Other hint keys are currently unspecified. Use flatten for extensibility.
	#[serde(flatten)]
	pub extra: HashMap<String, Value>,
}

// endregion: --- Model Preferences
