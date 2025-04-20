//! Defines the request and result structures for the `sampling/createMessage` MCP method.

use crate::mcp::{Annotations, GenericMeta, IncludeContext, RequestMeta, Role};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::base64::Base64;

// region:    --- Sampling Types

/// Hints to use for model selection.
///
/// Keys not declared here are currently left unspecified by the spec and are up
/// to the client to interpret.
///
/// TS Ref: `ModelHint`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelHint {
	/// A hint for a model name.
	pub name: Option<String>,
	// Note: Does not capture arbitrary extra fields as per schema description.
}

/// The server's preferences for model selection, requested of the client during sampling.
///
/// These preferences are always advisory. The client MAY ignore them.
///
/// TS Ref: `ModelPreferences`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelPreferences {
	/// Optional hints to use for model selection.
	pub hints: Option<Vec<ModelHint>>,

	/// How much to prioritize cost when selecting a model (0-1).
	pub cost_priority: Option<f64>,

	/// How much to prioritize sampling speed (latency) when selecting a model (0-1).
	pub speed_priority: Option<f64>,

	/// How much to prioritize intelligence and capabilities when selecting a model (0-1).
	pub intelligence_priority: Option<f64>,
}

/// Text provided to or from an LLM.
///
/// TS Ref: `TextContent`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
	/// The text content of the message.
	pub text: String,

	/// Optional annotations for the client.
	pub annotations: Option<Annotations>,
}

/// An image provided to or from an LLM.
///
/// TS Ref: `ImageContent`
#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageContent {
	/// The base64-encoded image data.
	/// @format byte
	#[serde_as(as = "Base64")]
	pub data: Vec<u8>,

	/// The MIME type of the image.
	pub mime_type: String,

	/// Optional annotations for the client.
	pub annotations: Option<Annotations>,
}

/// Audio provided to or from an LLM.
///
/// TS Ref: `AudioContent`
#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioContent {
	/// The base64-encoded audio data.
	/// @format byte
	#[serde_as(as = "Base64")]
	pub data: Vec<u8>,

	/// The MIME type of the audio.
	pub mime_type: String,

	/// Optional annotations for the client.
	pub annotations: Option<Annotations>,
}

/// Represents the content part of a `SamplingMessage`.
/// Based on TS `SamplingMessage.content: TextContent | ImageContent | AudioContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SamplingContent {
	Text(TextContent),
	Image(ImageContent),
	Audio(AudioContent),
}

/// Describes a message issued to or received from an LLM API.
///
/// TS Ref: `SamplingMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SamplingMessage {
	pub role: Role,
	pub content: SamplingContent,
}

// endregion: --- Sampling Types

// region:    --- CreateMessageRequest

/// A request from the server to sample an LLM via the client.
///
/// TS Ref: `CreateMessageRequest`
#[serde_with::skip_serializing_none]
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

impl CreateMessageParams {
	pub const METHOD: &'static str = "sampling/createMessage";
}

/// The client's response to a sampling/createMessage request from the server.
///
/// TS Ref: `CreateMessageResult`
/// Note: The TS definition extends `SamplingMessage`, so we include `role` and `content` directly.
#[serde_with::skip_serializing_none]
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

// endregion: --- CreateMessageRequest
