use crate::mcp::{ClientCapabilities, GenericMeta, Implementation, RequestMeta, ServerCapabilities};
use rpc_router::RpcId;
use serde::{Deserialize, Serialize};
use serde_json::Value; // Added for potential future _meta content

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

impl InitializeParams {
	pub const METHOD: &'static str = "initialize";
}

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

impl PingParams {
	pub const METHOD: &'static str = "ping";
}

// Note: The result for PingRequest is `EmptyResult`, which translates to a standard JSON-RPC success response
// with an empty `result` object (potentially containing only `_meta`).
// No specific struct is needed for `EmptyResult` itself beyond standard result handling.

// endregion: --- PingRequest
