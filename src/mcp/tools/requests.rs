use super::types::Tool; // Import Tool from the same module tree
use crate::mcp::{Cursor, GenericMeta, IntoMcpRequest, MessageContent, PaginationParams, RequestMeta};
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

impl IntoMcpRequest for ListToolsParams {
	const METHOD: &'static str = "tools/list";
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

impl IntoMcpRequest for CallToolParams {
	const METHOD: &'static str = "tools/call";
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
