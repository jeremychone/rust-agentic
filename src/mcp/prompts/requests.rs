use crate::mcp::{Cursor, GenericMeta, IntoMcpRequest, PaginationParams, Prompt, PromptMessage, RequestMeta};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

// region:    --- ListPromptsRequest

/// Sent from the client to request a list of prompts and prompt templates the server has.
///
/// TS Ref: `ListPromptsRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListPromptsParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// Cursor for pagination
	#[serde(flatten)]
	pub pagination: PaginationParams,
}

/// Builders
impl ListPromptsParams {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}

	pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
		self.pagination = pagination;
		self
	}

	pub fn with_cursor(mut self, cursor: impl Into<Cursor>) -> Self {
		self.pagination.cursor = Some(cursor.into());
		self
	}
}

impl IntoMcpRequest<ListPromptsParams> for ListPromptsParams {
	const METHOD: &'static str = "prompts/list";
	type McpResult = ListPromptsResult;
}

/// The server's response to a prompts/list request from the client.
///
/// TS Ref: `ListPromptsResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPromptsResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// An opaque token representing the pagination position after the last returned result.
	/// If present, there may be more results available.
	pub next_cursor: Option<Cursor>,

	/// The list of prompts
	pub prompts: Vec<Prompt>,
}

// endregion: --- ListPromptsRequest

// region:    --- GetPromptRequest

/// Used by the client to get a prompt provided by the server.
///
/// TS Ref: `GetPromptRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPromptParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// The name of the prompt or prompt template.
	pub name: String,

	/// Arguments to use for templating the prompt.
	pub arguments: Option<HashMap<String, String>>,
}

/// Builders
impl GetPromptParams {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			meta: None,
			name: name.into(),
			arguments: None,
		}
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}

	pub fn with_arguments(mut self, arguments: HashMap<String, String>) -> Self {
		self.arguments = Some(arguments);
		self
	}

	pub fn append_argument(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
		self.arguments
			.get_or_insert_with(HashMap::new)
			.insert(name.into(), value.into());
		self
	}
}

impl IntoMcpRequest<GetPromptParams> for GetPromptParams {
	const METHOD: &'static str = "prompts/get";
	type McpResult = GetPromptResult;
}

/// The server's response to a prompts/get request from the client.
///
/// TS Ref: `GetPromptResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPromptResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// An optional description for the prompt.
	pub description: Option<String>,

	/// The messages constituting the prompt.
	pub messages: Vec<PromptMessage>,
}

// endregion: --- GetPromptRequest
