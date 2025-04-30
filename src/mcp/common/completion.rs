use crate::mcp::{GenericMeta, IntoMcpRequest, PromptReference, RequestMeta};
use serde::{Deserialize, Serialize};
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

/// Builders
impl ResourceReference {
	pub fn new(uri: impl Into<String>) -> Self {
		Self { uri: uri.into() }
	}
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

/// Builders
impl CompletionArgument {
	pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			value: value.into(),
		}
	}
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

/// Builders
impl CompleteParams {
	pub fn new(reference: CompletionReference, argument: CompletionArgument) -> Self {
		Self {
			meta: None,
			reference,
			argument,
		}
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}

	pub fn with_reference(mut self, reference: CompletionReference) -> Self {
		self.reference = reference;
		self
	}

	pub fn with_argument(mut self, argument: CompletionArgument) -> Self {
		self.argument = argument;
		self
	}
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// Builders
impl CompletionResultData {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_values(mut self, values: Vec<String>) -> Self {
		self.values = values;
		self
	}

	pub fn append_value(mut self, value: impl Into<String>) -> Self {
		self.values.push(value.into());
		self
	}

	pub fn with_total(mut self, total: u64) -> Self {
		self.total = Some(total);
		self
	}

	pub fn with_has_more(mut self, has_more: bool) -> Self {
		self.has_more = Some(has_more);
		self
	}
}
