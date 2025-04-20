//! Types related to prompt listing and retrieval.

use super::{
	AudioContent, EmbeddedResource, ImageContent, Notification, NotificationParams, PaginatedParams,
	PaginatedResultData, Request, RequestParams, ResultData, Role, TextContent,
}; // Use re-exports
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// region:    --- List Prompts

/// Specific parameters for listing prompts, including pagination.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListPromptsParams {
	#[serde(flatten)]
	pub pagination: PaginatedParams,
}

/// Type alias for the full ListPrompts Request structure using the wrapper.
pub type ListPromptsRequest = RequestParams<ListPromptsParams>;

/// Specific result data for listing prompts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPromptsResultData {
	#[serde(flatten)]
	pub pagination: PaginatedResultData,

	pub prompts: Vec<Prompt>,
}

/// Type alias for the full ListPrompts Result structure using the wrapper.
pub type ListPromptsResult = ResultData<ListPromptsResultData>;

/// Associates the params/result structures with the Request trait.
impl Request for ListPromptsParams {
	const METHOD: &'static str = "prompts/list";
	type Params = Self;
	type Result = ListPromptsResultData;
}

// endregion: --- List Prompts

// region:    --- Get Prompt

/// Specific parameters for getting a single prompt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPromptParams {
	/// The name of the prompt or template.
	pub name: String,

	/// Arguments for templating.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<HashMap<String, String>>,
}

/// Type alias for the full GetPrompt Request structure using the wrapper.
pub type GetPromptRequest = RequestParams<GetPromptParams>;

/// Specific result data for getting a single prompt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPromptResultData {
	/// Optional description for the prompt.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,

	pub messages: Vec<PromptMessage>,
}

/// Type alias for the full GetPrompt Result structure using the wrapper.
pub type GetPromptResult = ResultData<GetPromptResultData>;

/// Associates the params/result structures with the Request trait.
impl Request for GetPromptParams {
	const METHOD: &'static str = "prompts/get";
	type Params = Self;
	type Result = GetPromptResultData;
}

// endregion: --- Get Prompt

// region:    --- Prompt List Changed Notification

/// Parameters for the PromptListChangedNotification (currently none).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PromptListChangedParams {}

/// Type alias for the full PromptListChanged Notification structure using the wrapper.
pub type PromptListChangedNotification = NotificationParams<PromptListChangedParams>;

/// Associates the params structure with the Notification trait.
impl Notification for PromptListChangedParams {
	const METHOD: &'static str = "notifications/prompts/list_changed";
	type Params = Self;
}

// endregion: --- Prompt List Changed Notification

// region:    --- Prompt Definitions

/// A prompt or prompt template offered by the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
	/// The name of the prompt or template.
	pub name: String,

	/// Optional description.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,

	/// Arguments the prompt accepts.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<Vec<PromptArgument>>,
}

/// Describes an argument a prompt can accept.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptArgument {
	/// The name of the argument.
	pub name: String,

	/// Human-readable description.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,

	/// Whether the argument is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required: Option<bool>,
}

/// Describes a message returned as part of a prompt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptMessage {
	pub role: Role,

	pub content: PromptMessageContent,
}

/// Content types allowed within a PromptMessage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")] // Rely on enum tag for type
pub enum PromptMessageContent {
	Text(TextContent),
	Image(ImageContent),
	Audio(AudioContent),
	Resource(EmbeddedResource),
}

// Implement Default for PromptMessageContent (optional, defaults to Text)
impl Default for PromptMessageContent {
	fn default() -> Self {
		PromptMessageContent::Text(Default::default())
	}
}

// endregion: --- Prompt Definitions
