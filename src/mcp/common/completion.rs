use crate::mcp::{GenericMeta, IntoMcpRequest, LoggingLevel, PromptReference, RequestMeta, Root};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

/// Reference to the context for which completion is requested.
/// Can be either a prompt or a resource.
///
/// TS Ref: `PromptReference | ResourceReference`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CompletionReference {
	#[serde(rename = "ref/prompt")]
	Prompt(PromptReference), // Uses the imported PromptReference
	#[serde(rename = "ref/resource")]
	Resource(ResourceReference),
}

// Note: PromptReference moved to src/mcp/prompts/types.rs

/// Identifies a resource or template for completion context.
///
/// TS Ref: `ResourceReference`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceReference {
	/// The URI or URI template of the resource.
	/// `@format uri-template`
	pub uri: String,
}

/// Information about the argument being completed.
///
/// TS Ref: `CompleteRequest.params.argument`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionArgument {
	/// The name of the argument
	pub name: String,
	/// The value of the argument to use for completion matching.
	pub value: String,
}

/// A request from the client to the server, to ask for completion options.
///
/// TS Ref: `CompleteRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// Reference to the prompt or resource definition.
	#[serde(rename = "ref")]
	pub reference: CompletionReference,

	/// The argument's information
	pub argument: CompletionArgument,
}

impl IntoMcpRequest<CompleteParams> for CompleteParams {
	const METHOD: &'static str = "completion/complete";
	type McpResult = CompleteResult;
}

/// The server's response to a completion/complete request
///
/// TS Ref: `CompleteResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The completion data.
	pub completion: CompletionResultData,
}

/// Data containing the completion results.
///
/// TS Ref: `CompleteResult.completion`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionResultData {
	/// An array of completion values. Must not exceed 100 items.
	pub values: Vec<String>,

	/// The total number of completion options available. This can exceed the number of values actually sent in the response.
	/// Using u64 as count should be non-negative.
	pub total: Option<u64>,

	/// Indicates whether there are additional completion options beyond those provided in the current response,
	/// even if the exact total is unknown.
	pub has_more: Option<bool>,
}
