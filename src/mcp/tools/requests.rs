use super::types::Tool;
use crate::mcp::{Cursor, GenericMeta, IntoMcpRequest, MessageContent, PaginationParams, ProgressToken, RequestMeta};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

// region:    --- ListToolsRequest

/// Sent from the client to request a list of tools the server has.
///
/// TS Ref: `ListToolsRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// Cursor for pagination
	#[serde(flatten)]
	pub pagination: PaginationParams,
}

/// Builders
impl ListToolsParams {
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

impl IntoMcpRequest<ListToolsParams> for ListToolsParams {
	const METHOD: &'static str = "tools/list";
	type McpResult = ListToolsResult;
}

/// The server's response to a tools/list request from the client.
///
/// TS Ref: `ListToolsResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// An opaque token representing the pagination position after the last returned result.
	/// If present, there may be more results available.
	pub next_cursor: Option<Cursor>,

	/// The list of tools
	pub tools: Vec<Tool>,
}

// endregion: --- ListToolsRequest

// region:    --- CallToolRequest

/// Used by the client to invoke a tool provided by the server.
///
/// TS Ref: `CallToolRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// The name of the tool.
	pub name: String,

	/// Arguments to use for the tool call.
	/// Note: The TS type uses `unknown`, so `Value` is appropriate here.
	pub arguments: Option<HashMap<String, Value>>,
}

/// Builders
impl CallToolParams {
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

	pub fn with_progress_token(mut self, progress_token: impl Into<ProgressToken>) -> Self {
		self.meta.get_or_insert_with(RequestMeta::default).progress_token = Some(progress_token.into());
		self
	}

	pub fn with_arguments(mut self, arguments: HashMap<String, Value>) -> Self {
		self.arguments = Some(arguments);
		self
	}

	pub fn append_argument(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
		self.arguments
			.get_or_insert_with(HashMap::new)
			.insert(name.into(), value.into());

		self
	}
}

impl IntoMcpRequest<CallToolParams> for CallToolParams {
	const METHOD: &'static str = "tools/call";
	type McpResult = CallToolResult;
}

/// The server's response to a tool call.
///
/// Any errors that originate from the tool SHOULD be reported inside the result
/// object, with `isError` set to true, _not_ as an MCP protocol-level error
/// response. Otherwise, the LLM would not be able to see that an error occurred
/// and self-correct.
///
/// However, any errors in _finding_ the tool, an error indicating that the
/// server does not support tool calls, or any other exceptional conditions,
/// should be reported as an MCP error response.
///
/// TS Ref: `CallToolResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The content resulting from the tool call.
	/// Reusing `MessageContent` as it covers Text, Image, Audio, and EmbeddedResource.
	pub content: Vec<MessageContent>,

	/// Whether the tool call ended in an error.
	/// If not set, this is assumed to be false (the call was successful).
	pub is_error: Option<bool>,
}

// endregion: --- CallToolRequest
