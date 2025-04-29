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

impl IntoMcpRequest<ListPromptsParams> for ListPromptsParams {
	const METHOD: &'static str = "prompts/list";
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

impl IntoMcpRequest<GetPromptParams> for GetPromptParams {
	const METHOD: &'static str = "prompts/get";
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
