use crate::mcp::{ClientCapabilities, GenericMeta, Implementation, IntoMcpRequest, RequestMeta, ServerCapabilities};
use serde::{Deserialize, Serialize};

// region:    --- InitializeRequest

/// This request is sent from the client to the server when it first connects, asking it to begin initialization.
///
/// TS Ref: InitializeRequest
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // Ensure consistent renaming
pub struct InitializeParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well.
	pub protocol_version: String,

	pub capabilities: ClientCapabilities,
	pub client_info: Implementation,
}

// region:    --- IntoRequest

// Example for a param struct
impl IntoMcpRequest<InitializeParams> for InitializeParams {
	const METHOD: &'static str = "initialize";
	type McpResult = InitializeResult;
}

// endregion: --- IntoRequest

// region:    --- Contructor
impl InitializeParams {
	pub fn from_client_info(name: impl Into<String>, version: impl Into<String>) -> Self {
		Self {
			meta: None,
			protocol_version: crate::mcp::LATEST_PROTOCOL_VERSION.to_string(),
			capabilities: ClientCapabilities::default(),
			client_info: Implementation {
				name: name.into(),
				version: version.into(),
			},
		}
	}
}
// endregion: --- Contructor

// region:    --- Withs

impl InitializeParams {
	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}
	pub fn with_capabilities(mut self, capabilities: ClientCapabilities) -> Self {
		self.capabilities = capabilities;
		self
	}
	pub fn with_client_info(mut self, client_info: Implementation) -> Self {
		self.client_info = client_info;
		self
	}
}

// endregion: --- Withs

/// After receiving an initialize request from the client, the server sends this response.
///
/// TS Ref: InitializeResult
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // Ensure consistent renaming
pub struct InitializeResult {
	/// Optional metadata (_meta) for the result.
	/// Note: `Result` in TS has `_meta` and `[key: string]: unknown`.
	/// We use `RequestMeta` here for consistency, assuming result meta follows a similar pattern or is handled generically.
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>, // Reusing RequestMeta, adjust if ResultMeta needs differentiation

	/// The version of the Model Context Protocol that the server wants to use.
	/// This may not match the version that the client requested. If the client cannot support this version, it MUST disconnect.
	pub protocol_version: String,
	pub capabilities: ServerCapabilities,
	pub server_info: Implementation,

	/// Instructions describing how to use the server and its features.
	/// This can be used by clients to improve the LLM's understanding of available tools, resources, etc.
	/// It can be thought of like a "hint" to the model. For example, this information MAY be added to the system prompt.
	pub instructions: Option<String>,
}

// endregion: --- InitializeRequest

// region:    --- PingRequest

/// A ping, issued by either the server or the client, to check that the other party is still alive.
/// The receiver must promptly respond with an EmptyResult, or else may be disconnected.
///
/// TS Ref: PingRequest
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)] // Default for easy creation
#[serde(rename_all = "camelCase")]
pub struct PingParams {
	/// Optional metadata (_meta) for the request.
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,
	// Note: PingRequest has no specific parameters beyond the optional _meta inherited from Request.
	// Serde will serialize this as an empty object `{}` or just `{"_meta": ...}` if meta is Some.
}

impl IntoMcpRequest<PingParams> for PingParams {
	const METHOD: &'static str = "ping";
	type McpResult = ();
}

// Note: The result for PingRequest is `EmptyResult`, which translates to a standard JSON-RPC success response
// with an empty `result` object (potentially containing only `_meta`).
// No specific struct is needed for `EmptyResult` itself beyond standard result handling.

// endregion: --- PingRequest
