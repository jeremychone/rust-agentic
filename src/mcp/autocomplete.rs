//! Types related to autocompletion requests.

use super::{Request, RequestParams, ResultData}; // Use re-exports from super
use serde::{Deserialize, Serialize};

// region:    --- Complete Request

/// Specific parameters for the CompleteRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteParams {
	/// Reference to the context (prompt or resource) for completion.
	#[serde(rename = "ref")]
	pub reference: CompletionReference,
	/// Information about the argument being completed.
	pub argument: ArgumentCompletionInfo,
}

/// Type alias for the full Complete Request structure using the wrapper.
pub type CompleteRequest = RequestParams<CompleteParams>;

/// Associates the params/result structures with the Request trait.
impl Request for CompleteParams {
	const METHOD: &'static str = "completion/complete";
	type Params = Self;
	type Result = CompleteResultData;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArgumentCompletionInfo {
	/// Name of the argument.
	pub name: String,
	/// Current value of the argument for matching.
	pub value: String,
}

// endregion: --- Complete Request

// region:    --- Complete Result

/// Specific result data for the Complete response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteResultData {
	pub completion: CompletionInfo,
}

/// Type alias for the full Complete Result structure using the wrapper.
pub type CompleteResult = ResultData<CompleteResultData>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionInfo {
	/// Array of completion values (max 100 items suggested by spec).
	pub values: Vec<String>,
	/// Total number of options available, if known.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total: Option<u32>,
	/// Indicates if more options exist beyond the response.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_more: Option<bool>,
}

// endregion: --- Complete Result

// region:    --- Completion References

/// Enum identifying the context for completion (Prompt or Resource).
/// Uses `serde(tag = "type")` for proper serialization/deserialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CompletionReference {
	#[serde(rename = "ref/resource")]
	Resource(ResourceReference),
	#[serde(rename = "ref/prompt")]
	Prompt(PromptReference),
}

/// A reference to a resource or resource template definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceReference {
	/// The URI or URI template of the resource.
	pub uri: String,
}

/// Identifies a prompt or prompt template.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptReference {
	/// The name of the prompt or template.
	pub name: String,
}

// endregion: --- Completion References
